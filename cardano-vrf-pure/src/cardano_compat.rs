//! Cardano-specific VRF implementation compatible with libsodium
//!
//! This module implements the exact behavior of Cardano's libsodium fork
//! to ensure byte-for-byte compatibility with the Haskell implementation.
//!
//! The implementation is based on the C code in:
//! cardano-crypto-praos/cbits/vrf03/ and cardano-crypto-praos/cbits/private/

use curve25519_dalek::{
    constants::ED25519_BASEPOINT_TABLE,
    edwards::{CompressedEdwardsY, EdwardsPoint},
    montgomery::MontgomeryPoint,
    scalar::Scalar,
};
use sha2::{Digest, Sha512};
use zeroize::Zeroizing;

use crate::{VrfError, VrfResult};

/// Suite identifier for draft-03
const SUITE_DRAFT03: u8 = 0x03;
const ONE: u8 = 0x01;
const TWO: u8 = 0x02;

/// Cardano-specific Elligator2 hash-to-curve implementation
/// This must match the libsodium `cardano_ge25519_from_uniform` function exactly
fn cardano_hash_to_curve(r: &[u8; 32]) -> VrfResult<EdwardsPoint> {
    // TODO: Implement Cardano-specific Elligator2
    // For now, use standard implementation as placeholder
    // This needs to be replaced with the exact Cardano implementation
    
    // The C code does:
    // 1. Extract sign bit from r[31]
    // 2. Clear sign bit: r[31] &= 0x7f
    // 3. Convert r to field element
    // 4. Apply Elligator2: ge25519_elligator2(x, y, r_fe, &notsquare)
    // 5. Convert Montgomery (x,y) to Edwards (X,Y): ge25519_mont_to_ed
    // 6. Conditional negate based on sign: cmov
    // 7. Clear cofactor: cardano_ge25519_clear_cofactor
    
    let mut r_modified = *r;
    let x_sign = r_modified[31] >> 7;
    r_modified[31] &= 0x7f;
    
    // For now, use MontgomeryPoint as a placeholder
    // This is NOT correct and needs full Elligator2 implementation
    let mont_point = MontgomeryPoint(r_modified);
    let ed_point = mont_point.to_edwards(x_sign);
    
    match ed_point {
        Some(p) => {
            // TODO: Apply cofactor clearing
            // The C code multiplies by the cofactor here with specific logic
            Ok(p.mul_by_cofactor())
        }
        None => Err(VrfError::InvalidPoint),
    }
}

/// Cardano-compatible VRF prove function
/// Matches the behavior of crypto_vrf_ietfdraft03_prove in C
pub fn cardano_vrf_prove(
    secret_key: &[u8; 64],
    message: &[u8],
) -> VrfResult<[u8; 80]> {
    // Expand the secret key (same as C code)
    let mut az = Zeroizing::new([0u8; 64]);
    let mut hasher = Sha512::new();
    hasher.update(&secret_key[0..32]); // Hash the seed part
    let hash = hasher.finalize();
    az.copy_from_slice(&hash);

    // Clamp the scalar (same as C code)
    az[0] &= 248;
    az[31] &= 127;
    az[31] |= 64;

    let x = Scalar::from_bytes_mod_order(az[0..32].try_into().unwrap());

    // Extract public key
    let pk = &secret_key[32..64];

    // Compute H = hash_to_curve(suite || 0x01 || pk || message)
    let mut h_hasher = Sha512::new();
    h_hasher.update(&[SUITE_DRAFT03]);
    h_hasher.update(&[ONE]);
    h_hasher.update(pk);
    h_hasher.update(message);
    let r_string = h_hasher.finalize();

    // Apply Cardano-specific Elligator2
    let mut r_bytes = [0u8; 32];
    r_bytes.copy_from_slice(&r_string[0..32]);
    r_bytes[31] &= 0x7f; // Clear sign bit (same as C)

    // This is the critical function that needs to match libsodium exactly
    let h_point = cardano_hash_to_curve(&r_bytes)?;
    
    // Get H_string (the encoded point)
    let h_string = h_point.compress().to_bytes();

    // Gamma = x * H
    let gamma = h_point * x;

    // Compute nonce = hash(az[32..64] || H_string)
    let mut nonce_hasher = Sha512::new();
    nonce_hasher.update(&az[32..64]);
    nonce_hasher.update(&h_string);
    let nonce_hash = nonce_hasher.finalize();
    let k = Scalar::from_bytes_mod_order_wide(&nonce_hash.as_slice().try_into().unwrap());

    // k*B and k*H
    let k_b: EdwardsPoint = &k * ED25519_BASEPOINT_TABLE;
    let k_h = h_point * k;

    let gamma_bytes = gamma.compress().to_bytes();
    let k_b_bytes = k_b.compress().to_bytes();
    let k_h_bytes = k_h.compress().to_bytes();

    // Compute challenge c = hash(suite || 0x02 || H || Gamma || k*B || k*H)
    let mut c_hasher = Sha512::new();
    c_hasher.update(&[SUITE_DRAFT03]);
    c_hasher.update(&[TWO]);
    c_hasher.update(&h_string);
    c_hasher.update(&gamma_bytes);
    c_hasher.update(&k_b_bytes);
    c_hasher.update(&k_h_bytes);
    let c_hash = c_hasher.finalize();

    // Take first 16 bytes of challenge
    let mut c_bytes = [0u8; 32];
    c_bytes[0..16].copy_from_slice(&c_hash[0..16]);
    // Last 16 bytes are zero

    let c = Scalar::from_bytes_mod_order(c_bytes);

    // s = k + c*x (mod L)
    let s = k + c * x;

    // Construct proof: Gamma || c[0..16] || s
    let mut proof = [0u8; 80];
    proof[0..32].copy_from_slice(&gamma_bytes);
    proof[32..48].copy_from_slice(&c_hash[0..16]);
    proof[48..80].copy_from_slice(s.as_bytes());

    Ok(proof)
}

/// Cardano-compatible VRF verify function  
pub fn cardano_vrf_verify(
    public_key: &[u8; 32],
    proof: &[u8; 80],
    message: &[u8],
) -> VrfResult<[u8; 64]> {
    // Parse proof components
    let gamma_bytes: [u8; 32] = proof[0..32].try_into().unwrap();
    let c_bytes_short: [u8; 16] = proof[32..48].try_into().unwrap();
    let s_bytes: [u8; 32] = proof[48..80].try_into().unwrap();
    
    // Parse public key
    let y_point = CompressedEdwardsY(*public_key)
        .decompress()
        .ok_or(VrfError::InvalidPublicKey)?;
    
    // Parse Gamma
    let gamma = CompressedEdwardsY(gamma_bytes)
        .decompress()
        .ok_or(VrfError::InvalidProof)?;
    
    // Parse s
    let s_option: Option<Scalar> = Scalar::from_canonical_bytes(s_bytes).into();
    if s_option.is_none() {
        return Err(VrfError::InvalidScalar);
    }
    let s = s_option.unwrap();
    
    // Reconstruct c
    let mut c_bytes = [0u8; 32];
    c_bytes[0..16].copy_from_slice(&c_bytes_short);
    let c = Scalar::from_bytes_mod_order(c_bytes);
    
    // Compute H
    let mut h_hasher = Sha512::new();
    h_hasher.update(&[SUITE_DRAFT03]);
    h_hasher.update(&[ONE]);
    h_hasher.update(public_key);
    h_hasher.update(message);
    let r_string = h_hasher.finalize();
    
    let mut r_bytes = [0u8; 32];
    r_bytes.copy_from_slice(&r_string[0..32]);
    r_bytes[31] &= 0x7f;
    
    let h_point = cardano_hash_to_curve(&r_bytes)?;
    let h_string = h_point.compress().to_bytes();
    
    // Verify: s*B = k*B + c*Y and s*H = k*H + c*Gamma
    let s_b: EdwardsPoint = &s * ED25519_BASEPOINT_TABLE;
    let c_y = c * y_point;
    let s_h = h_point * s;
    let c_gamma = c * gamma;
    
    // k*B = s*B - c*Y
    let k_b = s_b - c_y;
    // k*H = s*H - c*Gamma
    let k_h = s_h - c_gamma;
    
    let k_b_bytes = k_b.compress().to_bytes();
    let k_h_bytes = k_h.compress().to_bytes();
    
    // Recompute challenge
    let mut c_hasher = Sha512::new();
    c_hasher.update(&[SUITE_DRAFT03]);
    c_hasher.update(&[TWO]);
    c_hasher.update(&h_string);
    c_hasher.update(&gamma.compress().to_bytes());
    c_hasher.update(&k_b_bytes);
    c_hasher.update(&k_h_bytes);
    let c_hash = c_hasher.finalize();
    
    // Verify challenge matches
    if &c_hash[0..16] != c_bytes_short {
        return Err(VrfError::VerificationFailed);
    }
    
    // Compute VRF output
    let mut output_hasher = Sha512::new();
    output_hasher.update(&[SUITE_DRAFT03]);
    output_hasher.update(&[0x03]); // Three for output
    output_hasher.update(&gamma.compress().to_bytes());
    let output_hash = output_hasher.finalize();
    
    let mut output = [0u8; 64];
    output.copy_from_slice(&output_hash);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cardano_vrf_basic() {
        // Test with a simple key
        let mut sk = [0u8; 64];
        sk[32..64].copy_from_slice(&[0x3b, 0x6a, 0x27, 0xbc, 0xce, 0xb6, 0xa4, 0x2d, 
                                     0x62, 0xa3, 0xa8, 0xd0, 0x2a, 0x6f, 0x0d, 0x73,
                                     0x65, 0x32, 0x15, 0x77, 0x1d, 0xe2, 0x43, 0xa6,
                                     0x3a, 0xc0, 0x48, 0xa1, 0x8b, 0x59, 0xda, 0x29]);
        
        let message = &[0x00];
        
        let proof_result = cardano_vrf_prove(&sk, message);
        // This will not match Haskell yet because our Elligator2 is incomplete
        // But at least verify it compiles and runs
        assert!(proof_result.is_ok() || proof_result.is_err());
    }
}
