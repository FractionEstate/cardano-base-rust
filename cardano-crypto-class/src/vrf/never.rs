use std::fmt;

use crate::seed::Seed;

use super::{OutputVRF, VRFAlgorithm};

/// VRF algorithm that is deliberately unavailable.
pub struct NeverVRF;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NeverVerificationKey;

impl fmt::Debug for NeverVerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NeverVerificationKey")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NeverSigningKey;

impl fmt::Debug for NeverSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NeverSigningKey")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NeverCertificate;

impl fmt::Debug for NeverCertificate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NeverCertificate")
    }
}

impl VRFAlgorithm for NeverVRF {
    type VerificationKey = NeverVerificationKey;
    type SigningKey = NeverSigningKey;
    type Proof = NeverCertificate;
    type Context = ();

    const ALGORITHM_NAME: &'static str = "never";
    const SEED_SIZE: usize = 0;
    const VERIFICATION_KEY_SIZE: usize = 0;
    const SIGNING_KEY_SIZE: usize = 0;
    const PROOF_SIZE: usize = 0;
    const OUTPUT_SIZE: usize = 0;

    fn derive_verification_key(_signing_key: &Self::SigningKey) -> Self::VerificationKey {
        NeverVerificationKey
    }

    fn evaluate_bytes(
        _context: &Self::Context,
        _message: &[u8],
        _signing_key: &Self::SigningKey,
    ) -> (OutputVRF<Self>, Self::Proof) {
        panic!("VRF unavailable")
    }

    fn verify_bytes(
        _context: &Self::Context,
        _verification_key: &Self::VerificationKey,
        _message: &[u8],
        _proof: &Self::Proof,
    ) -> Option<OutputVRF<Self>> {
        panic!("VRF unavailable")
    }

    fn gen_key_from_seed_bytes(_seed: &[u8]) -> Self::SigningKey {
        NeverSigningKey
    }

    fn raw_serialize_verification_key(_key: &Self::VerificationKey) -> Vec<u8> {
        Vec::new()
    }

    fn raw_deserialize_verification_key(_bytes: &[u8]) -> Option<Self::VerificationKey> {
        Some(NeverVerificationKey)
    }

    fn raw_serialize_signing_key(_key: &Self::SigningKey) -> Vec<u8> {
        Vec::new()
    }

    fn raw_deserialize_signing_key(_bytes: &[u8]) -> Option<Self::SigningKey> {
        Some(NeverSigningKey)
    }

    fn raw_serialize_proof(_proof: &Self::Proof) -> Vec<u8> {
        Vec::new()
    }

    fn raw_deserialize_proof(_bytes: &[u8]) -> Option<Self::Proof> {
        Some(NeverCertificate)
    }
}

/// Deterministic key generation helper for consistency with the Haskell API.
#[must_use]
pub fn gen_key(_seed: &Seed) -> NeverSigningKey {
    NeverSigningKey
}

/// Helper that mirrors the Haskell `genKeyPairVRF` behaviour.
#[must_use]
pub fn gen_keypair(_seed: &Seed) -> (NeverSigningKey, NeverVerificationKey) {
    (NeverSigningKey, NeverVerificationKey)
}
