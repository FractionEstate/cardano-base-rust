use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use std::fmt;

use crate::seed::Seed;
use crate::util::{read_binary_word64, write_binary_word64};

use super::{OutputVRF, VRFAlgorithm};

/// Mock verifiable random function used for testing and benchmarking.
pub struct MockVRF;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MockVerificationKey(u64);

impl fmt::Debug for MockVerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MockVerificationKey({:#x})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MockSigningKey(u64);

impl fmt::Debug for MockSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MockSigningKey({:#x})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MockCertificate(u64);

impl MockVerificationKey {
    #[must_use] 
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl MockSigningKey {
    #[must_use] 
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl MockCertificate {
    #[must_use] 
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl MockSigningKey {
    fn to_bytes(&self) -> [u8; MockVRF::SIGNING_KEY_SIZE] {
        write_binary_word64(self.0).try_into().expect("length 8")
    }
}

fn cbor_bytes(bytes: &[u8]) -> Vec<u8> {
    let len = bytes.len();
    let mut encoded = Vec::new();

    if len <= 23 {
        encoded.push(0x40 | (len as u8));
    } else if len <= u8::MAX as usize {
        encoded.push(0x58);
        encoded.push(len as u8);
    } else if len <= u16::MAX as usize {
        encoded.push(0x59);
        encoded.extend_from_slice(&(len as u16).to_be_bytes());
    } else if len <= u32::MAX as usize {
        encoded.push(0x5a);
        encoded.extend_from_slice(&(len as u32).to_be_bytes());
    } else {
        encoded.push(0x5b);
        encoded.extend_from_slice(&(len as u64).to_be_bytes());
    }

    encoded.extend_from_slice(bytes);
    encoded
}

fn short_hash(data: &[u8]) -> [u8; MockVRF::OUTPUT_SIZE] {
    let mut hasher =
        Blake2bVar::new(MockVRF::OUTPUT_SIZE).expect("Blake2bVar accepts lengths up to 64 bytes");
    hasher.update(data);
    let mut output = [0u8; MockVRF::OUTPUT_SIZE];
    hasher
        .finalize_variable(&mut output)
        .expect("Blake2bVar finalize succeeded");
    output
}

impl VRFAlgorithm for MockVRF {
    type VerificationKey = MockVerificationKey;
    type SigningKey = MockSigningKey;
    type Proof = MockCertificate;
    type Context = ();

    const ALGORITHM_NAME: &'static str = "mock";
    const SEED_SIZE: usize = 8;
    const VERIFICATION_KEY_SIZE: usize = 8;
    const SIGNING_KEY_SIZE: usize = 8;
    const PROOF_SIZE: usize = 8;
    const OUTPUT_SIZE: usize = 8;

    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey {
        MockVerificationKey(signing_key.0)
    }

    fn evaluate_bytes(
        _context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> (OutputVRF<Self>, Self::Proof) {
        let mut serialized = cbor_bytes(message);
        serialized.extend_from_slice(&cbor_bytes(&signing_key.to_bytes()));

        let digest = short_hash(&serialized);
        let output = OutputVRF::<Self>::from_bytes(digest.to_vec())
            .expect("digest length must match OUTPUT_SIZE");
        let cert = MockCertificate(signing_key.0);
        (output, cert)
    }

    fn verify_bytes(
        context: &Self::Context,
        verification_key: &Self::VerificationKey,
        message: &[u8],
        proof: &Self::Proof,
    ) -> Option<OutputVRF<Self>> {
        let signing_key = MockSigningKey(verification_key.0);
        let (output, cert) = Self::evaluate_bytes(context, message, &signing_key);
        if cert == *proof {
            Some(output)
        } else {
            None
        }
    }

    fn gen_key_from_seed_bytes(seed: &[u8]) -> Self::SigningKey {
        let value = read_binary_word64(seed);
        MockSigningKey(value)
    }

    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8> {
        write_binary_word64(key.0)
    }

    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey> {
        if bytes.len() == Self::VERIFICATION_KEY_SIZE {
            Some(MockVerificationKey(read_binary_word64(bytes)))
        } else {
            None
        }
    }

    fn raw_serialize_signing_key(key: &Self::SigningKey) -> Vec<u8> {
        write_binary_word64(key.0)
    }

    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey> {
        if bytes.len() == Self::SIGNING_KEY_SIZE {
            Some(MockSigningKey(read_binary_word64(bytes)))
        } else {
            None
        }
    }

    fn raw_serialize_proof(proof: &Self::Proof) -> Vec<u8> {
        write_binary_word64(proof.0)
    }

    fn raw_deserialize_proof(bytes: &[u8]) -> Option<Self::Proof> {
        if bytes.len() == Self::PROOF_SIZE {
            Some(MockCertificate(read_binary_word64(bytes)))
        } else {
            None
        }
    }
}

impl From<MockSigningKey> for MockVerificationKey {
    fn from(value: MockSigningKey) -> Self {
        MockVerificationKey(value.0)
    }
}

impl From<&MockSigningKey> for MockVerificationKey {
    fn from(value: &MockSigningKey) -> Self {
        MockVerificationKey(value.0)
    }
}

impl From<MockSigningKey> for MockCertificate {
    fn from(value: MockSigningKey) -> Self {
        MockCertificate(value.0)
    }
}

impl From<&MockSigningKey> for MockCertificate {
    fn from(value: &MockSigningKey) -> Self {
        MockCertificate(value.0)
    }
}

/// Convenience helper for deterministic key generation matching the Haskell helper.
#[must_use] 
pub fn gen_key(seed: &Seed) -> MockSigningKey {
    MockVRF::gen_key(seed)
}

/// Convenience helper returning the keypair from a seed.
#[must_use] 
pub fn gen_keypair(seed: &Seed) -> (MockSigningKey, MockVerificationKey) {
    MockVRF::gen_keypair(seed)
}
