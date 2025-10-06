//! VRF implementation following IETF draft-03
//!
//! This implements ECVRF-ED25519-SHA512-Elligator2 as specified in
//! draft-irtf-cfrg-vrf-03

#![allow(clippy::unwrap_used)]

use crate::VrfResult;
use crate::cardano_compat::{cardano_vrf_prove, cardano_vrf_verify, point::cardano_clear_cofactor};
use crate::common::{
    SUITE_DRAFT03, THREE, bytes_to_point, point_to_bytes, secret_key_to_public, seed_to_secret_key,
};
use sha2::{Digest, Sha512};

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
        cardano_vrf_prove(secret_key, message)
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
        cardano_vrf_verify(public_key, proof, message)
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

        // Clear cofactor using Cardano-specific method
        let gamma_cleared = cardano_clear_cofactor(&gamma);
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
