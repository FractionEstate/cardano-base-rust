//! VRF implementation following IETF draft-03
//!
//! This implements ECVRF-ED25519-SHA512-Elligator2 as specified in
//! draft-irtf-cfrg-vrf-03

#![allow(clippy::unwrap_used)]

use curve25519_dalek::{edwards::EdwardsPoint, scalar::Scalar, traits::VartimeMultiscalarMul};
use sha2::{Digest, Sha512};
use zeroize::Zeroizing;

use crate::common::*;
use crate::{VrfError, VrfResult};

/// VRF proof size for draft-03 (80 bytes)
pub const PROOF_SIZE: usize = 80;

/// Public key size (32 bytes)
pub const PUBLIC_KEY_SIZE: usize = 32;

/// Secret key size (64 bytes: 32-byte seed + 32-byte public key)
pub const SECRET_KEY_SIZE: usize = 64;

/// Seed size (32 bytes)
pub const SEED_SIZE: usize = 32;

/// Output size (64 bytes)
pub const OUTPUT_SIZE: usize = 64;

/// VRF Draft-03 implementation
#[derive(Clone)]
pub struct VrfDraft03;

impl VrfDraft03 {
    /// Generate a VRF proof
    ///
    /// # Arguments
    /// * `secret_key` - 64-byte secret key (32-byte seed + 32-byte public key)
    /// * `message` - Message to prove
    ///
    /// # Returns
    /// 80-byte proof
    ///
    /// # Errors
    ///
    /// Returns `VrfError` if the proof generation fails.
    ///
    /// # Panics
    ///
    /// May panic if internal cryptographic operations fail (extremely unlikely).
    pub fn prove(
        secret_key: &[u8; SECRET_KEY_SIZE],
        message: &[u8],
    ) -> VrfResult<[u8; PROOF_SIZE]> {
        // Expand the secret key
        let mut az = Zeroizing::new([0u8; 64]);
        let mut hasher = Sha512::new();
        hasher.update(&secret_key[0..32]); // Hash the seed part
        let hash = hasher.finalize();
        az.copy_from_slice(&hash);

        // Clamp the scalar
        az[0] &= 248;
        az[31] &= 127;
        az[31] |= 64;

        let x = Scalar::from_bytes_mod_order(az[0..32].try_into().unwrap());

        // Extract public key
        let pk = &secret_key[32..64];

        // Compute H = hash_to_curve(pk || message)
        let mut h_hasher = Sha512::new();
        h_hasher.update(&[SUITE_DRAFT03]);
        h_hasher.update(&[ONE]);
        h_hasher.update(pk);
        h_hasher.update(message);
        let r_string = h_hasher.finalize();

        // Apply Elligator2
        let mut r_bytes = [0u8; 32];
        r_bytes.copy_from_slice(&r_string[0..32]);
        r_bytes[31] &= 0x7f; // Clear sign bit

        let h_string = elligator2_hash_to_curve(&r_bytes);
        let h_point = bytes_to_point(&h_string)?;

        // Gamma = x * H
        let gamma = h_point * x;

        // Compute nonce = hash(az[32..64] || H_string)
        let mut nonce_hasher = Sha512::new();
        nonce_hasher.update(&az[32..64]);
        nonce_hasher.update(&h_string);
        let nonce_hash = nonce_hasher.finalize();
        let k = Scalar::from_bytes_mod_order_wide(&nonce_hash.as_slice().try_into().unwrap());

        // k*B and k*H
        let k_b = EdwardsPoint::mul_base(&k);
        let k_h = h_point * k;

        let gamma_bytes = point_to_bytes(&gamma);
        let k_b_bytes = point_to_bytes(&k_b);
        let k_h_bytes = point_to_bytes(&k_h);

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
        let mut proof = [0u8; PROOF_SIZE];
        proof[0..32].copy_from_slice(&gamma_bytes);
        proof[32..48].copy_from_slice(&c_hash[0..16]);
        proof[48..80].copy_from_slice(s.as_bytes());

        Ok(proof)
    }

    /// Verify a VRF proof and return the output
    ///
    /// # Arguments
    /// * `public_key` - 32-byte public key
    /// * `proof` - 80-byte proof
    /// * `message` - Message that was proven
    ///
    /// # Returns
    /// 64-byte VRF output on success
    pub fn verify(
        public_key: &[u8; PUBLIC_KEY_SIZE],
        proof: &[u8; PROOF_SIZE],
        message: &[u8],
    ) -> VrfResult<[u8; OUTPUT_SIZE]> {
        // Parse public key
        let y_point = bytes_to_point(public_key)?;

        // Check for small order
        if has_small_order(&y_point) {
            return Err(VrfError::InvalidPublicKey);
        }

        // Parse proof: Gamma || c || s
        let gamma_bytes = &proof[0..32];
        let c_bytes_short = &proof[32..48];
        let s_bytes: [u8; 32] = proof[48..80].try_into().unwrap();

        // Validate Gamma
        let gamma_array: [u8; 32] = gamma_bytes.try_into().unwrap();
        let gamma = bytes_to_point(&gamma_array)?;

        // Validate s
        if !is_canonical_scalar(&s_bytes) {
            return Err(VrfError::InvalidProof);
        }

        let s = Scalar::from_canonical_bytes(s_bytes)
            .map(|s| s)
            .unwrap_or(Scalar::ZERO);
        if s == Scalar::ZERO && s_bytes != [0u8; 32] {
            return Err(VrfError::InvalidScalar);
        }

        // Reconstruct c (pad with zeros)
        let mut c_bytes = [0u8; 32];
        c_bytes[0..16].copy_from_slice(c_bytes_short);
        let c = Scalar::from_bytes_mod_order(c_bytes);

        // Compute H = hash_to_curve(pk || message)
        let mut h_hasher = Sha512::new();
        h_hasher.update(&[SUITE_DRAFT03]);
        h_hasher.update(&[ONE]);
        h_hasher.update(public_key);
        h_hasher.update(message);
        let r_string = h_hasher.finalize();

        let mut r_bytes = [0u8; 32];
        r_bytes.copy_from_slice(&r_string[0..32]);
        r_bytes[31] &= 0x7f;

        let h_string = elligator2_hash_to_curve(&r_bytes);
        let h_point = bytes_to_point(&h_string)?;

        // Compute U = s*B - c*Y
        let neg_c = scalar_negate(&c);
        let u = EdwardsPoint::vartime_multiscalar_mul(
            &[s, neg_c],
            &[EdwardsPoint::mul_base(&Scalar::ONE), y_point],
        );

        // Compute V = s*H - c*Gamma
        let v = EdwardsPoint::vartime_multiscalar_mul(&[s, neg_c], &[h_point, gamma]);

        let u_bytes = point_to_bytes(&u);
        let v_bytes = point_to_bytes(&v);

        // Recompute challenge c' = hash(suite || 0x02 || H || Gamma || U || V)
        let mut c_prime_hasher = Sha512::new();
        c_prime_hasher.update(&[SUITE_DRAFT03]);
        c_prime_hasher.update(&[TWO]);
        c_prime_hasher.update(&h_string);
        c_prime_hasher.update(&gamma_array);
        c_prime_hasher.update(&u_bytes);
        c_prime_hasher.update(&v_bytes);
        let c_prime_hash = c_prime_hasher.finalize();

        // Verify challenge matches
        let c_prime_short: [u8; 16] = c_prime_hash[0..16].try_into().unwrap();
        let c_short: [u8; 16] = c_bytes_short.try_into().unwrap();
        if !verify_16(&c_short, &c_prime_short) {
            return Err(VrfError::VerificationFailed);
        }

        // Compute output
        Self::proof_to_hash(proof)
    }

    /// Convert a proof to VRF output hash
    ///
    /// # Arguments
    /// * `proof` - 80-byte proof
    ///
    /// # Returns
    /// 64-byte VRF output
    pub fn proof_to_hash(proof: &[u8; PROOF_SIZE]) -> VrfResult<[u8; OUTPUT_SIZE]> {
        let gamma_bytes: [u8; 32] = proof[0..32].try_into().unwrap();
        let gamma = bytes_to_point(&gamma_bytes)?;

        // Clear cofactor
        let gamma_cleared = clear_cofactor(&gamma);
        let gamma_cleared_bytes = point_to_bytes(&gamma_cleared);

        // beta = hash(suite || 0x03 || cofactor*Gamma)
        let mut hasher = Sha512::new();
        hasher.update(&[SUITE_DRAFT03]);
        hasher.update(&[THREE]);
        hasher.update(&gamma_cleared_bytes);
        let beta = hasher.finalize();

        let mut output = [0u8; OUTPUT_SIZE];
        output.copy_from_slice(&beta);
        Ok(output)
    }

    /// Generate keypair from seed
    #[must_use]
    pub fn keypair_from_seed(
        seed: &[u8; SEED_SIZE],
    ) -> ([u8; SECRET_KEY_SIZE], [u8; PUBLIC_KEY_SIZE]) {
        let sk = seed_to_secret_key(seed);
        let pk = secret_key_to_public(&sk);
        (sk, pk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prove_verify_roundtrip() {
        let seed = [42u8; SEED_SIZE];
        let (sk, pk) = VrfDraft03::keypair_from_seed(&seed);
        let message = b"test message";

        let proof = VrfDraft03::prove(&sk, message).expect("prove failed");
        let output = VrfDraft03::verify(&pk, &proof, message).expect("verify failed");

        assert_eq!(output.len(), OUTPUT_SIZE);
    }

    #[test]
    fn test_verify_rejects_invalid_proof() {
        let seed = [42u8; SEED_SIZE];
        let (_, pk) = VrfDraft03::keypair_from_seed(&seed);
        let message = b"test message";

        let mut bad_proof = [0u8; PROOF_SIZE];
        bad_proof[0] = 1; // Invalid proof

        assert!(VrfDraft03::verify(&pk, &bad_proof, message).is_err());
    }

    #[test]
    fn test_proof_to_hash_deterministic() {
        let seed = [123u8; SEED_SIZE];
        let (sk, _) = VrfDraft03::keypair_from_seed(&seed);
        let message = b"deterministic test";

        let proof = VrfDraft03::prove(&sk, message).expect("prove failed");
        let output1 = VrfDraft03::proof_to_hash(&proof).expect("proof_to_hash failed");
        let output2 = VrfDraft03::proof_to_hash(&proof).expect("proof_to_hash failed");

        assert_eq!(output1, output2, "proof_to_hash should be deterministic");
    }
}
