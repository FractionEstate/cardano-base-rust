// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Schnorr signatures over the Secp256k1 elliptic curve (BIP340).
//!
//! This module provides Schnorr signature support for the Secp256k1 curve
//! following the BIP340 specification, commonly used in Bitcoin Taproot
//! and other blockchain systems for cross-chain compatibility.
//!
//! # Security Note
//!
//! This implementation is provided for cross-chain bridge compatibility only.
//! For Cardano consensus, use Ed25519 signatures instead.

use crate::dsign::{DsignAlgorithm, DsignError};
use rand_core::{CryptoRng, RngCore};
use secp256k1::{
    Keypair, Secp256k1, SecretKey, XOnlyPublicKey, schnorr::Signature as SchnorrSignature,
};
use std::fmt;

/// Schnorr Secp256k1 digital signature algorithm (BIP340).
///
/// Used for cross-chain compatibility with Bitcoin Taproot and other systems.
pub struct SchnorrSecp256k1DSIGN;

/// Schnorr Secp256k1 signing key (32 bytes).
#[derive(Clone)]
pub struct SigningKey(Keypair);

impl fmt::Debug for SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SchnorrSecp256k1SigningKey(<secret>)")
    }
}

/// Schnorr Secp256k1 verification key (32 bytes x-only public key).
#[derive(Clone, PartialEq, Eq)]
pub struct VerificationKey(XOnlyPublicKey);

impl fmt::Debug for VerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SchnorrSecp256k1VerificationKey({:?})", self.0)
    }
}

/// Schnorr Secp256k1 signature (64 bytes).
#[derive(Clone, PartialEq, Eq)]
pub struct Signature(SchnorrSignature);

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SchnorrSecp256k1Signature({:?})", self.0)
    }
}

/// Context type for Schnorr Secp256k1 (no context needed).
#[derive(Clone, Copy, Debug)]
pub struct Context;

impl Default for Context {
    fn default() -> Self {
        Context
    }
}

impl DsignAlgorithm for SchnorrSecp256k1DSIGN {
    type SigningKey = SigningKey;
    type VerificationKey = VerificationKey;
    type Signature = Signature;
    type Context = Context;

    const ALGORITHM_NAME: &'static str = "SchnorrSecp256k1DSIGN";
    const SEED_SIZE: usize = 32;
    const SIGNING_KEY_SIZE: usize = 32;
    const VERIFICATION_KEY_SIZE: usize = 32; // X-only public key
    const SIGNATURE_SIZE: usize = 64; // Schnorr signature

    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey {
        let (xonly, _parity) = signing_key.0.x_only_public_key();
        VerificationKey(xonly)
    }

    fn sign_bytes(
        _context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> Self::Signature {
        let secp = Secp256k1::new();

        // For Schnorr, we need a 32-byte message hash
        // If message is not 32 bytes, hash it first
        let message_hash = if message.len() == 32 {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(message);
            arr
        } else {
            use sha2::{Digest, Sha256};
            let digest = Sha256::digest(message);
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&digest);
            arr
        };

        // Schnorr sign takes raw bytes, not a Message object
        let signature = secp.sign_schnorr(&message_hash, &signing_key.0);
        Signature(signature)
    }

    fn verify_bytes(
        _context: &Self::Context,
        verification_key: &Self::VerificationKey,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<(), DsignError> {
        let secp = Secp256k1::new();

        // Hash the message if it's not already 32 bytes
        let message_hash = if message.len() == 32 {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(message);
            arr
        } else {
            use sha2::{Digest, Sha256};
            let digest = Sha256::digest(message);
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&digest);
            arr
        };

        // Schnorr verify takes raw bytes, not a Message object
        secp.verify_schnorr(&signature.0, &message_hash, &verification_key.0)
            .map_err(|_| DsignError::VerificationFailed)
    }

    fn gen_key_from_seed_bytes(seed: &[u8]) -> Self::SigningKey {
        assert_eq!(
            seed.len(),
            Self::SEED_SIZE,
            "seed must be exactly {} bytes",
            Self::SEED_SIZE
        );
        let secp = Secp256k1::new();
        // Convert slice to array for from_byte_array
        let mut seed_arr = [0u8; 32];
        seed_arr.copy_from_slice(seed);
        let secret_key =
            SecretKey::from_byte_array(seed_arr).expect("seed must be valid for Secp256k1");
        let keypair = Keypair::from_secret_key(&secp, &secret_key);
        SigningKey(keypair)
    }

    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8> {
        key.0.serialize().to_vec()
    }

    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey> {
        if bytes.len() != Self::VERIFICATION_KEY_SIZE {
            return None;
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        XOnlyPublicKey::from_byte_array(arr)
            .ok()
            .map(VerificationKey)
    }

    fn raw_serialize_signing_key(signing_key: &Self::SigningKey) -> Vec<u8> {
        signing_key.0.secret_bytes().to_vec()
    }

    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey> {
        if bytes.len() != Self::SIGNING_KEY_SIZE {
            return None;
        }
        let secp = Secp256k1::new();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        SecretKey::from_byte_array(arr)
            .ok()
            .map(|sk| SigningKey(Keypair::from_secret_key(&secp, &sk)))
    }

    fn raw_serialize_signature(signature: &Self::Signature) -> Vec<u8> {
        signature.0.as_ref().to_vec()
    }

    fn raw_deserialize_signature(bytes: &[u8]) -> Option<Self::Signature> {
        if bytes.len() != Self::SIGNATURE_SIZE {
            return None;
        }
        let mut arr = [0u8; 64];
        arr.copy_from_slice(bytes);
        Some(Signature(SchnorrSignature::from_byte_array(arr)))
    }
}

/// Generate a keypair using a cryptographic RNG.
pub fn generate_keypair<R: RngCore + CryptoRng>(rng: &mut R) -> (SigningKey, VerificationKey) {
    let secp = Secp256k1::new();
    // Generate random 32 bytes for secret key
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let secret_key = SecretKey::from_byte_array(bytes).expect("32 random bytes should be valid");
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let (xonly, _parity) = keypair.x_only_public_key();

    (SigningKey(keypair), VerificationKey(xonly))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::Seed;

    #[test]
    fn test_schnorr_secp256k1_round_trip() {
        let mut rng = rand::rng();
        let (signing_key, verification_key) = generate_keypair(&mut rng);

        let context = Context::default();
        let message = b"Hello, Bitcoin Taproot!";
        let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, message, &signing_key);

        assert!(
            SchnorrSecp256k1DSIGN::verify_bytes(&context, &verification_key, message, &signature)
                .is_ok()
        );
    }

    #[test]
    fn test_schnorr_secp256k1_serialization() {
        let mut rng = rand::rng();
        let (signing_key, verification_key) = generate_keypair(&mut rng);

        // Test signing key serialization
        let sk_bytes = SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&signing_key);
        assert_eq!(sk_bytes.len(), SchnorrSecp256k1DSIGN::SIGNING_KEY_SIZE);
        let sk_restored = SchnorrSecp256k1DSIGN::raw_deserialize_signing_key(&sk_bytes).unwrap();
        assert_eq!(
            SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&sk_restored),
            sk_bytes
        );

        // Test verification key serialization
        let vk_bytes = SchnorrSecp256k1DSIGN::raw_serialize_verification_key(&verification_key);
        assert_eq!(vk_bytes.len(), SchnorrSecp256k1DSIGN::VERIFICATION_KEY_SIZE);
        let vk_restored =
            SchnorrSecp256k1DSIGN::raw_deserialize_verification_key(&vk_bytes).unwrap();
        assert_eq!(vk_restored, verification_key);
    }

    #[test]
    fn test_schnorr_secp256k1_signature_format() {
        let mut rng = rand::rng();
        let (signing_key, _) = generate_keypair(&mut rng);
        let context = Context::default();
        let message = b"Test message";

        let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, message, &signing_key);
        let sig_bytes = SchnorrSecp256k1DSIGN::raw_serialize_signature(&signature);

        assert_eq!(sig_bytes.len(), SchnorrSecp256k1DSIGN::SIGNATURE_SIZE);

        let sig_restored = SchnorrSecp256k1DSIGN::raw_deserialize_signature(&sig_bytes).unwrap();
        assert_eq!(sig_restored, signature);
    }

    #[test]
    fn test_schnorr_secp256k1_deterministic_keygen() {
        let seed_bytes = [42u8; 32];
        let seed = Seed::from_bytes(&seed_bytes);

        let sk1 = SchnorrSecp256k1DSIGN::gen_key(&seed);
        let sk2 = SchnorrSecp256k1DSIGN::gen_key(&seed);

        assert_eq!(
            SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&sk1),
            SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&sk2)
        );
    }

    #[test]
    fn test_schnorr_secp256k1_wrong_signature() {
        let mut rng = rand::rng();
        let (signing_key, verification_key) = generate_keypair(&mut rng);
        let context = Context::default();

        let message = b"Original message";
        let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, message, &signing_key);

        let wrong_message = b"Modified message";
        assert!(
            SchnorrSecp256k1DSIGN::verify_bytes(
                &context,
                &verification_key,
                wrong_message,
                &signature
            )
            .is_err()
        );
    }

    #[test]
    fn test_schnorr_vs_ecdsa_different_signatures() {
        // Verify that Schnorr and ECDSA produce different signatures
        // for the same key material and message
        use crate::dsign::ecdsa_secp256k1;

        let seed_bytes = [99u8; 32];
        let seed = Seed::from_bytes(&seed_bytes);

        let schnorr_sk = SchnorrSecp256k1DSIGN::gen_key(&seed);
        let ecdsa_sk = ecdsa_secp256k1::EcdsaSecp256k1DSIGN::gen_key(&seed);

        let message = b"Same message, different algorithms";
        let schnorr_ctx = Context::default();
        let ecdsa_ctx = ecdsa_secp256k1::Context::default();

        let schnorr_sig = SchnorrSecp256k1DSIGN::sign_bytes(&schnorr_ctx, message, &schnorr_sk);
        let ecdsa_sig =
            ecdsa_secp256k1::EcdsaSecp256k1DSIGN::sign_bytes(&ecdsa_ctx, message, &ecdsa_sk);

        // Signatures should be different (different algorithms)
        assert_ne!(
            SchnorrSecp256k1DSIGN::raw_serialize_signature(&schnorr_sig),
            ecdsa_secp256k1::EcdsaSecp256k1DSIGN::raw_serialize_signature(&ecdsa_sig)
        );
    }
}
