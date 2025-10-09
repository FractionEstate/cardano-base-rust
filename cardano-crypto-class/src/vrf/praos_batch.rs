use std::fmt;

use cardano_vrf_pure::{VrfDraft13, VrfError as VrfPureError, common};
use thiserror::Error;

use crate::mlocked_bytes::{MLockedBytes, MLockedError};
use crate::seed::Seed;

use super::{OutputVRF, VRFAlgorithm};

fn seed_size() -> usize {
    32
}

fn verification_key_size() -> usize {
    32
}

fn signing_key_size() -> usize {
    64
}

fn proof_size() -> usize {
    128 // draft-13 batch-compatible uses 128-byte proofs
}

fn output_size() -> usize {
    64
}

fn io_verification_key_size() -> usize {
    32
}

fn io_signing_key_size() -> usize {
    64
}

#[derive(Debug, Error)]
pub enum PraosBatchConstructionError {
    #[error("mlocked allocation failed: {0}")]
    Memory(#[from] MLockedError),
    #[error("vrf error: {0}")]
    Vrf(#[from] VrfPureError),
    #[error("invalid length: expected {expected}, got {actual}")]
    WrongLength { expected: usize, actual: usize },
    #[error("invalid length: expected {expected} or alternate {alternate}, got {actual}")]
    WrongLengthWithAlternate {
        expected: usize,
        alternate: usize,
        actual: usize,
    },
}

pub struct PraosBatchCompatSeed {
    bytes: MLockedBytes,
}

impl fmt::Debug for PraosBatchCompatSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PraosBatchCompatSeed(<mlocked>)")
    }
}

impl PraosBatchCompatSeed {
    /// Generates a new random Praos batch-compatible seed.
    ///
    /// # Errors
    ///
    /// Returns an error if memory-locked allocation fails.
    pub fn generate() -> Result<Self, PraosBatchConstructionError> {
        let mut bytes = MLockedBytes::new_zeroed(seed_size())?;
        // Use Rust's rand crate
        use rand_core::RngCore;
        let mut rng = rand::rng();
        rng.fill_bytes(bytes.as_mut_slice());
        Ok(Self { bytes })
    }

    /// Creates a Praos batch-compatible seed from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The byte length is not 32 bytes
    /// - Memory-locked allocation fails
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosBatchConstructionError> {
        if bytes.len() != seed_size() {
            return Err(PraosBatchConstructionError::WrongLength {
                expected: seed_size(),
                actual: bytes.len(),
            });
        }
        let mut allocated = MLockedBytes::new_zeroed(seed_size())?;
        allocated.as_mut_slice().copy_from_slice(bytes);
        Ok(Self { bytes: allocated })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.as_slice().to_vec()
    }
}

impl Clone for PraosBatchCompatSeed {
    fn clone(&self) -> Self {
        Self {
            bytes: self
                .bytes
                .try_clone()
                .expect("mlocked seed cloning failed - memory allocation error"),
        }
    }
}

/// Generates a new random Praos batch-compatible seed.
///
/// # Errors
///
/// Returns an error if memory-locked allocation fails.
pub fn gen_seed() -> Result<PraosBatchCompatSeed, PraosBatchConstructionError> {
    PraosBatchCompatSeed::generate()
}

pub struct PraosBatchCompatSigningKey {
    secret: MLockedBytes,
}

impl fmt::Debug for PraosBatchCompatSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PraosBatchCompatSigningKey(<mlocked>)")
    }
}

impl PraosBatchCompatSigningKey {
    /// Creates a Praos batch-compatible signing key from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The byte length is not 64 bytes (standard) or 32 bytes (alternative)
    /// - Memory-locked allocation fails
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosBatchConstructionError> {
        let len = bytes.len();
        let expected = signing_key_size();
        let alternate = io_signing_key_size();
        if len != expected && len != alternate {
            return Err(PraosBatchConstructionError::WrongLengthWithAlternate {
                expected,
                alternate,
                actual: len,
            });
        }
        let mut secret = MLockedBytes::new_zeroed(expected)?;
        if len == expected {
            secret.as_mut_slice().copy_from_slice(bytes);
        } else {
            secret.as_mut_slice()[..alternate].copy_from_slice(bytes);
        }
        Ok(Self { secret })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.secret.as_slice()
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.secret.as_slice().to_vec()
    }

    /// Derives the verification key from this signing key.
    ///
    /// # Errors
    ///
    /// This function should not fail under normal circumstances as it performs
    /// deterministic cryptographic key derivation.
    pub fn derive_verification_key(
        &self,
    ) -> Result<PraosBatchCompatVerificationKey, PraosBatchConstructionError> {
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&self.as_bytes()[0..32]);
        let pk = common::seed_to_public_key(&seed);
        Ok(PraosBatchCompatVerificationKey { bytes: pk.to_vec() })
    }

    /// Extracts the seed (first 32 bytes) from this signing key.
    ///
    /// # Errors
    ///
    /// Returns an error if memory-locked allocation fails.
    pub fn to_seed(&self) -> Result<PraosBatchCompatSeed, PraosBatchConstructionError> {
        // Extract seed from first 32 bytes
        let mut seed = MLockedBytes::new_zeroed(seed_size())?;
        seed.as_mut_slice().copy_from_slice(&self.as_bytes()[0..32]);
        Ok(PraosBatchCompatSeed { bytes: seed })
    }

    /// Generates a VRF proof for the given message.
    ///
    /// # Errors
    ///
    /// Returns an error if the VRF proof generation fails.
    pub fn prove(
        &self,
        message: &[u8],
    ) -> Result<PraosBatchCompatProof, PraosBatchConstructionError> {
        let mut sk = [0u8; 64];
        sk.copy_from_slice(self.as_bytes());
        let proof = VrfDraft13::prove(&sk, message)?;
        Ok(PraosBatchCompatProof {
            bytes: proof.to_vec(),
        })
    }
}

impl Clone for PraosBatchCompatSigningKey {
    fn clone(&self) -> Self {
        Self {
            secret: self.secret.try_clone().expect("failed to clone secret key"),
        }
    }
}

pub struct PraosBatchCompatVerificationKey {
    bytes: Vec<u8>,
}

impl fmt::Debug for PraosBatchCompatVerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PraosBatchCompatVerificationKey")
            .field(&hex::encode(&self.bytes))
            .finish()
    }
}

impl PraosBatchCompatVerificationKey {
    /// Creates a Praos batch-compatible verification key from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte length is not 32 bytes (standard) or 32 bytes (alternative).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosBatchConstructionError> {
        let len = bytes.len();
        let expected = verification_key_size();
        let alternate = io_verification_key_size();
        if len != expected && len != alternate {
            return Err(PraosBatchConstructionError::WrongLengthWithAlternate {
                expected,
                alternate,
                actual: len,
            });
        }
        let mut owned = vec![0u8; expected];
        if len == expected {
            owned.copy_from_slice(bytes);
        } else {
            owned[..alternate].copy_from_slice(bytes);
        }
        Ok(Self { bytes: owned })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// Verify a VRF proof against this verification key.
    ///
    /// # Errors
    ///
    /// Returns an error if the verification routine fails unexpectedly.
    pub fn verify(
        &self,
        message: &[u8],
        proof: &PraosBatchCompatProof,
    ) -> Result<Option<Vec<u8>>, PraosBatchConstructionError> {
        let mut pk = [0u8; 32];
        pk.copy_from_slice(self.bytes.as_slice());
        let mut proof_bytes = [0u8; 128];
        proof_bytes.copy_from_slice(proof.bytes.as_slice());

        match VrfDraft13::verify(&pk, &proof_bytes, message) {
            Ok(output) => Ok(Some(output.to_vec())),
            Err(_) => Ok(None),
        }
    }
}

impl Clone for PraosBatchCompatVerificationKey {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
        }
    }
}

impl PartialEq for PraosBatchCompatVerificationKey {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for PraosBatchCompatVerificationKey {}

#[derive(Clone, PartialEq, Eq)]
pub struct PraosBatchCompatProof {
    bytes: Vec<u8>,
}

impl fmt::Debug for PraosBatchCompatProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PraosBatchCompatProof")
            .field(&hex::encode(&self.bytes))
            .finish()
    }
}

impl PraosBatchCompatProof {
    /// Creates a Praos batch-compatible VRF proof from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte length is not 128 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosBatchConstructionError> {
        if bytes.len() != proof_size() {
            return Err(PraosBatchConstructionError::WrongLength {
                expected: proof_size(),
                actual: bytes.len(),
            });
        }
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    /// Extracts the VRF output bytes from this proof.
    ///
    /// # Errors
    ///
    /// This function should not fail under normal circumstances as extraction
    /// is deterministic. Returns `Ok(None)` if the proof is malformed.
    pub fn to_output_bytes(&self) -> Result<Option<Vec<u8>>, PraosBatchConstructionError> {
        let mut proof_bytes = [0u8; 128];
        proof_bytes.copy_from_slice(self.bytes.as_slice());
        match VrfDraft13::proof_to_hash(&proof_bytes) {
            Ok(output) => Ok(Some(output.to_vec())),
            Err(_) => Ok(None),
        }
    }
}

/// Generates a Praos batch-compatible keypair from a seed.
///
/// # Errors
///
/// Returns an error if memory-locked allocation fails.
pub fn keypair_from_seed(
    seed: &PraosBatchCompatSeed,
) -> Result<
    (PraosBatchCompatVerificationKey, PraosBatchCompatSigningKey),
    PraosBatchConstructionError,
> {
    let mut seed_bytes = [0u8; 32];
    seed_bytes.copy_from_slice(seed.as_bytes());
    let (sk_array, pk_array) = VrfDraft13::keypair_from_seed(&seed_bytes);

    // Store in mlocked memory
    let mut sk = MLockedBytes::new_zeroed(signing_key_size())?;
    sk.as_mut_slice().copy_from_slice(&sk_array);

    Ok((
        PraosBatchCompatVerificationKey {
            bytes: pk_array.to_vec(),
        },
        PraosBatchCompatSigningKey { secret: sk },
    ))
}

/// Generates a Praos batch-compatible keypair from seed bytes.
///
/// # Errors
///
/// Returns an error if:
/// - The seed byte length is not 32 bytes
/// - Memory-locked allocation fails
/// - Key derivation fails
pub fn keypair_from_seed_bytes(
    seed_bytes: &[u8],
) -> Result<
    (PraosBatchCompatVerificationKey, PraosBatchCompatSigningKey),
    PraosBatchConstructionError,
> {
    let seed = PraosBatchCompatSeed::from_bytes(seed_bytes)?;
    keypair_from_seed(&seed)
}

/// Derive a batch-compatible signing key deterministically from a [`Seed`].
///
/// # Panics
///
/// Panics if the seed does not provide enough bytes or if the resulting
/// material is not a valid signing key.
#[must_use]
pub fn signing_key_from_seed(seed: &Seed) -> PraosBatchCompatSigningKey {
    let (material, _) = crate::seed::get_bytes_from_seed(signing_key_size(), seed.clone())
        .expect("seed produced insufficient material for signing key");
    signing_key_from_bytes(&material).expect("seed produced invalid praos batch signing key")
}

/// Creates a Praos batch-compatible signing key from raw bytes.
///
/// # Errors
///
/// Returns an error if:
/// - The byte length is not 64 bytes (standard) or 32 bytes (alternative)
/// - Memory-locked allocation fails
pub fn signing_key_from_bytes(
    bytes: &[u8],
) -> Result<PraosBatchCompatSigningKey, PraosBatchConstructionError> {
    PraosBatchCompatSigningKey::from_bytes(bytes)
}

#[must_use]
pub fn signing_key_to_bytes(signing_key: &PraosBatchCompatSigningKey) -> Vec<u8> {
    signing_key.to_vec()
}

/// Creates a Praos batch-compatible verification key from raw bytes.
///
/// # Errors
///
/// Returns an error if the byte length is not 32 bytes (standard) or 32 bytes (alternative).
pub fn verification_key_from_bytes(
    bytes: &[u8],
) -> Result<PraosBatchCompatVerificationKey, PraosBatchConstructionError> {
    PraosBatchCompatVerificationKey::from_bytes(bytes)
}

#[must_use]
pub fn verification_key_to_bytes(verification_key: &PraosBatchCompatVerificationKey) -> Vec<u8> {
    verification_key.to_vec()
}

/// Creates a Praos batch-compatible VRF proof from raw bytes.
///
/// # Errors
///
/// Returns an error if the byte length is not 128 bytes.
pub fn proof_from_bytes(
    bytes: &[u8],
) -> Result<PraosBatchCompatProof, PraosBatchConstructionError> {
    PraosBatchCompatProof::from_bytes(bytes)
}

#[must_use]
pub fn proof_to_bytes(proof: &PraosBatchCompatProof) -> Vec<u8> {
    proof.to_vec()
}

/// Creates a Praos batch-compatible seed from raw bytes.
///
/// # Errors
///
/// Returns an error if:
/// - The byte length is not 32 bytes
/// - Memory-locked allocation fails
pub fn seed_from_bytes(bytes: &[u8]) -> Result<PraosBatchCompatSeed, PraosBatchConstructionError> {
    PraosBatchCompatSeed::from_bytes(bytes)
}

#[must_use]
pub fn seed_to_bytes(seed: &PraosBatchCompatSeed) -> Vec<u8> {
    seed.to_vec()
}

#[must_use]
pub fn unsafe_raw_seed(seed: &PraosBatchCompatSeed) -> Vec<u8> {
    seed.to_vec()
}

/// Extracts a batch-compatible VRF output from a proof.
///
/// # Errors
///
/// Returns an error if the proof cannot be converted into an output or if the
/// resulting byte string has the wrong length.
pub fn output_from_proof(
    proof: &PraosBatchCompatProof,
) -> Result<Option<OutputVRF<PraosBatchCompatVRF>>, PraosBatchConstructionError> {
    match proof.to_output_bytes()? {
        Some(bytes) => {
            let actual = bytes.len();
            if actual != output_size() {
                return Err(PraosBatchConstructionError::WrongLength {
                    expected: output_size(),
                    actual,
                });
            }
            let output = OutputVRF::from_bytes(bytes).map_err(|_| {
                PraosBatchConstructionError::WrongLength {
                    expected: output_size(),
                    actual,
                }
            })?;
            Ok(Some(output))
        },
        None => Ok(None),
    }
}

pub struct PraosBatchCompatVRF;

impl VRFAlgorithm for PraosBatchCompatVRF {
    type VerificationKey = PraosBatchCompatVerificationKey;
    type SigningKey = PraosBatchCompatSigningKey;
    type Proof = PraosBatchCompatProof;
    type Context = ();

    const ALGORITHM_NAME: &'static str = "PraosBatchCompatVRF";
    const SEED_SIZE: usize = 32;
    const VERIFICATION_KEY_SIZE: usize = 32;
    const SIGNING_KEY_SIZE: usize = 64;
    const PROOF_SIZE: usize = 128;
    const OUTPUT_SIZE: usize = 64;

    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey {
        signing_key
            .derive_verification_key()
            .expect("praos batch sk_to_pk failed")
    }

    fn evaluate_bytes(
        _context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> (OutputVRF<Self>, Self::Proof) {
        let proof = signing_key
            .prove(message)
            .expect("praos batch prove failed");
        let output_bytes = proof
            .to_output_bytes()
            .expect("praos batch proof_to_hash failed")
            .expect("invalid praos batch proof");
        let output = OutputVRF::from_bytes(output_bytes).expect("output size mismatch");
        (output, proof)
    }

    fn verify_bytes(
        _context: &Self::Context,
        verification_key: &Self::VerificationKey,
        message: &[u8],
        proof: &Self::Proof,
    ) -> Option<OutputVRF<Self>> {
        match verification_key.verify(message, proof) {
            Ok(Some(bytes)) => OutputVRF::copy_from_slice(&bytes).ok(),
            Ok(None) => None,
            Err(_) => None,
        }
    }

    fn gen_key_from_seed_bytes(seed: &[u8]) -> Self::SigningKey {
        let (_, sk) = keypair_from_seed_bytes(seed).expect("invalid praos batch seed bytes");
        sk
    }

    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8> {
        key.to_vec()
    }

    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey> {
        PraosBatchCompatVerificationKey::from_bytes(bytes).ok()
    }

    fn raw_serialize_signing_key(key: &Self::SigningKey) -> Vec<u8> {
        key.to_vec()
    }

    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey> {
        PraosBatchCompatSigningKey::from_bytes(bytes).ok()
    }

    fn raw_serialize_proof(proof: &Self::Proof) -> Vec<u8> {
        proof.to_vec()
    }

    fn raw_deserialize_proof(bytes: &[u8]) -> Option<Self::Proof> {
        PraosBatchCompatProof::from_bytes(bytes).ok()
    }
}
