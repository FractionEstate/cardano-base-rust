// SPDX-License-Identifier: Apache-2.0 OR MIT

//! ECDSA signatures over the Secp256k1 elliptic curve.
//!
//! This module provides ECDSA (Elliptic Curve Digital Signature Algorithm) support
//! for the Secp256k1 curve, commonly used in Bitcoin and Ethereum for cross-chain
//! compatibility.
//!
//! # Security Note
//!
//! This implementation is provided for cross-chain bridge compatibility only.
//! For Cardano consensus, use Ed25519 signatures instead.

use crate::dsign::{DsignAlgorithm, DsignError};
use rand::{CryptoRng, RngCore};
use secp256k1::{ecdsa::Signature as Secp256k1Signature, Message, PublicKey, Secp256k1, SecretKey};
use std::fmt;

/// ECDSA Secp256k1 digital signature algorithm.
///
/// Used for cross-chain compatibility with Bitcoin and Ethereum.
pub struct EcdsaSecp256k1DSIGN;

/// ECDSA Secp256k1 signing key (32 bytes).
#[derive(Clone)]
pub struct SigningKey(SecretKey);

impl fmt::Debug for SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EcdsaSecp256k1SigningKey(<secret>)")
    }
}

/// ECDSA Secp256k1 verification key (33 bytes compressed).
#[derive(Clone, PartialEq, Eq)]
pub struct VerificationKey(PublicKey);

impl fmt::Debug for VerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EcdsaSecp256k1VerificationKey({:?})", self.0)
    }
}

/// ECDSA Secp256k1 signature (64 bytes compact).
#[derive(Clone, PartialEq, Eq)]
pub struct Signature(Secp256k1Signature);

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EcdsaSecp256k1Signature({:?})", self.0)
    }
}

/// Context type for ECDSA Secp256k1 (no context needed).
#[derive(Clone, Copy, Debug)]
pub struct Context;

impl Default for Context {
    fn default() -> Self {
        Context
    }
}

impl DsignAlgorithm for EcdsaSecp256k1DSIGN {
    type SigningKey = SigningKey;
    type VerificationKey = VerificationKey;
    type Signature = Signature;
    type Context = Context;

    const ALGORITHM_NAME: &'static str = "EcdsaSecp256k1DSIGN";
    const SEED_SIZE: usize = 32;
    const SIGNING_KEY_SIZE: usize = 32;
    const VERIFICATION_KEY_SIZE: usize = 33; // Compressed public key
    const SIGNATURE_SIZE: usize = 64; // Compact signature

    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &signing_key.0);
        VerificationKey(public_key)
    }

    fn sign_bytes(
        _context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> Self::Signature {
        let secp = Secp256k1::new();

        // For ECDSA, we need a 32-byte message hash
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

        let message_obj = Message::from_digest(message_hash);

        let signature = secp.sign_ecdsa(message_obj, &signing_key.0);
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

        let message_obj = Message::from_digest(message_hash);

        secp.verify_ecdsa(message_obj, &signature.0, &verification_key.0)
            .map_err(|_| DsignError::VerificationFailed)
    }

    fn gen_key_from_seed_bytes(seed: &[u8]) -> Self::SigningKey {
        assert_eq!(
            seed.len(),
            Self::SEED_SIZE,
            "seed must be exactly {} bytes",
            Self::SEED_SIZE
        );
        // Convert slice to array for from_byte_array
        let mut seed_arr = [0u8; 32];
        seed_arr.copy_from_slice(seed);
        let secret_key =
            SecretKey::from_byte_array(seed_arr).expect("seed must be valid for Secp256k1");
        SigningKey(secret_key)
    }

    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8> {
        key.0.serialize().to_vec()
    }

    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey> {
        if bytes.len() != Self::VERIFICATION_KEY_SIZE {
            return None;
        }
        PublicKey::from_slice(bytes).ok().map(VerificationKey)
    }

    fn raw_serialize_signing_key(signing_key: &Self::SigningKey) -> Vec<u8> {
        signing_key.0.secret_bytes().to_vec()
    }

    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey> {
        if bytes.len() != Self::SIGNING_KEY_SIZE {
            return None;
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        SecretKey::from_byte_array(arr).ok().map(SigningKey)
    }

    fn raw_serialize_signature(signature: &Self::Signature) -> Vec<u8> {
        signature.0.serialize_compact().to_vec()
    }

    fn raw_deserialize_signature(bytes: &[u8]) -> Option<Self::Signature> {
        if bytes.len() != Self::SIGNATURE_SIZE {
            return None;
        }
        Secp256k1Signature::from_compact(bytes).ok().map(Signature)
    }
}

/// Generate a keypair using a cryptographic RNG.
pub fn generate_keypair<R: RngCore + CryptoRng>(rng: &mut R) -> (SigningKey, VerificationKey) {
    let secp = Secp256k1::new();
    // Generate random 32 bytes for secret key
    let mut bytes = [0u8; 32];
    rng.fill_bytes(&mut bytes);

    let secret_key = SecretKey::from_byte_array(bytes).expect("32 random bytes should be valid");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    (SigningKey(secret_key), VerificationKey(public_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::Seed;

    #[test]
    fn test_ecdsa_secp256k1_round_trip() {
        let mut rng = rand::thread_rng();
        let (signing_key, verification_key) = generate_keypair(&mut rng);

        let context = Context::default();
        let message = b"Hello, cross-chain world!";
        let signature = EcdsaSecp256k1DSIGN::sign_bytes(&context, message, &signing_key);

        assert!(EcdsaSecp256k1DSIGN::verify_bytes(
            &context,
            &verification_key,
            message,
            &signature
        )
        .is_ok());
    }

    #[test]
    fn test_ecdsa_secp256k1_serialization() {
        let mut rng = rand::thread_rng();
        let (signing_key, verification_key) = generate_keypair(&mut rng);

        // Test signing key serialization
        let sk_bytes = EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&signing_key);
        assert_eq!(sk_bytes.len(), EcdsaSecp256k1DSIGN::SIGNING_KEY_SIZE);
        let sk_restored = EcdsaSecp256k1DSIGN::raw_deserialize_signing_key(&sk_bytes).unwrap();
        assert_eq!(
            EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&sk_restored),
            sk_bytes
        );

        // Test verification key serialization
        let vk_bytes = EcdsaSecp256k1DSIGN::raw_serialize_verification_key(&verification_key);
        assert_eq!(vk_bytes.len(), EcdsaSecp256k1DSIGN::VERIFICATION_KEY_SIZE);
        let vk_restored = EcdsaSecp256k1DSIGN::raw_deserialize_verification_key(&vk_bytes).unwrap();
        assert_eq!(vk_restored, verification_key);
    }

    #[test]
    fn test_ecdsa_secp256k1_signature_format() {
        let mut rng = rand::thread_rng();
        let (signing_key, _) = generate_keypair(&mut rng);
        let context = Context::default();
        let message = b"Test message";

        let signature = EcdsaSecp256k1DSIGN::sign_bytes(&context, message, &signing_key);
        let sig_bytes = EcdsaSecp256k1DSIGN::raw_serialize_signature(&signature);

        assert_eq!(sig_bytes.len(), EcdsaSecp256k1DSIGN::SIGNATURE_SIZE);

        let sig_restored = EcdsaSecp256k1DSIGN::raw_deserialize_signature(&sig_bytes).unwrap();
        assert_eq!(sig_restored, signature);
    }

    #[test]
    fn test_ecdsa_secp256k1_deterministic_keygen() {
        let seed_bytes = [42u8; 32];
        let seed = Seed::from_bytes(&seed_bytes);

        let sk1 = EcdsaSecp256k1DSIGN::gen_key(&seed);
        let sk2 = EcdsaSecp256k1DSIGN::gen_key(&seed);

        assert_eq!(
            EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&sk1),
            EcdsaSecp256k1DSIGN::raw_serialize_signing_key(&sk2)
        );
    }

    #[test]
    fn test_ecdsa_secp256k1_wrong_signature() {
        let mut rng = rand::thread_rng();
        let (signing_key, verification_key) = generate_keypair(&mut rng);
        let context = Context::default();

        let message = b"Original message";
        let signature = EcdsaSecp256k1DSIGN::sign_bytes(&context, message, &signing_key);

        let wrong_message = b"Modified message";
        assert!(EcdsaSecp256k1DSIGN::verify_bytes(
            &context,
            &verification_key,
            wrong_message,
            &signature
        )
        .is_err());
    }
}
