use std::fmt;

use cardano_vrf_pure::{common, VrfDraft03, VrfError as VrfPureError};
use thiserror::Error;

use crate::direct_serialise::{DirectDeserialise, DirectResult, DirectSerialise, SizeCheckError};
use crate::mlocked_bytes::{MLockedBytes, MLockedError};
use crate::seed::Seed;

use super::praos_batch::{
    PraosBatchCompatSigningKey, PraosBatchCompatVRF, PraosBatchCompatVerificationKey,
};
use super::{OutputVRF, VRFAlgorithm, VRFError};

const fn seed_size() -> usize {
    32
}

const fn verification_key_size() -> usize {
    32
}

const fn signing_key_size() -> usize {
    64
}

const fn proof_size() -> usize {
    80
}

const fn output_size() -> usize {
    64
}

#[derive(Debug, Error)]
pub enum PraosConstructionError {
    #[error("mlocked allocation failed: {0}")]
    Memory(#[from] MLockedError),
    #[error("vrf error: {0}")]
    Vrf(#[from] VrfPureError),
    #[error("invalid length: expected {expected}, got {actual}")]
    WrongLength { expected: usize, actual: usize },
}

pub struct PraosSeed {
    bytes: MLockedBytes,
}

impl fmt::Debug for PraosSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PraosSeed(<mlocked>)")
    }
}

impl PraosSeed {
    /// Generates a new random Praos seed.
    ///
    /// # Errors
    ///
    /// Returns an error if memory-locked allocation fails.
    pub fn generate() -> Result<Self, PraosConstructionError> {
        let mut bytes = MLockedBytes::new_zeroed(seed_size())?;
        let slice = bytes.as_mut_slice();
        // Use Rust's rand crate instead of FFI
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        rng.fill_bytes(slice);
        Ok(Self { bytes })
    }

    /// Creates a Praos seed from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The byte length is not 32 bytes
    /// - Memory-locked allocation fails
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosConstructionError> {
        if bytes.len() != seed_size() {
            return Err(PraosConstructionError::WrongLength {
                expected: seed_size(),
                actual: bytes.len(),
            });
        }
        let mut mlocked = MLockedBytes::new_zeroed(seed_size())?;
        mlocked.as_mut_slice().copy_from_slice(bytes);
        Ok(Self { bytes: mlocked })
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

impl Clone for PraosSeed {
    fn clone(&self) -> Self {
        let cloned = self.bytes.try_clone().expect("failed to clone seed");
        Self { bytes: cloned }
    }
}

/// Generates a new random Praos seed.
///
/// # Errors
///
/// Returns an error if memory-locked allocation fails.
pub fn gen_seed() -> Result<PraosSeed, PraosConstructionError> {
    PraosSeed::generate()
}

pub struct PraosSigningKey {
    secret: MLockedBytes,
}

impl fmt::Debug for PraosSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PraosSigningKey(<mlocked>)")
    }
}

impl PraosSigningKey {
    /// Creates a Praos signing key from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The byte length is not 64 bytes
    /// - Memory-locked allocation fails
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosConstructionError> {
        if bytes.len() != signing_key_size() {
            return Err(PraosConstructionError::WrongLength {
                expected: signing_key_size(),
                actual: bytes.len(),
            });
        }
        let mut secret = MLockedBytes::new_zeroed(signing_key_size())?;
        secret.as_mut_slice().copy_from_slice(bytes);
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
    pub fn derive_verification_key(&self) -> Result<PraosVerificationKey, PraosConstructionError> {
        // Extract seed (first 32 bytes) from sk
        let seed: [u8; 32] = self.as_bytes()[0..32].try_into().unwrap();
        // Use common::seed_to_public_key to derive public key
        let pk = common::seed_to_public_key(&seed);
        Ok(PraosVerificationKey { bytes: pk.to_vec() })
    }

    /// Extracts the seed (first 32 bytes) from this signing key.
    ///
    /// # Errors
    ///
    /// Returns an error if memory-locked allocation fails.
    pub fn to_seed(&self) -> Result<PraosSeed, PraosConstructionError> {
        // Extract seed from first 32 bytes of secret key
        let mut seed = MLockedBytes::new_zeroed(seed_size())?;
        seed.as_mut_slice().copy_from_slice(&self.as_bytes()[0..32]);
        Ok(PraosSeed { bytes: seed })
    }

    /// Generates a VRF proof for the given message.
    ///
    /// # Errors
    ///
    /// Returns an error if the VRF proof generation fails.
    pub fn prove(&self, message: &[u8]) -> Result<PraosProof, PraosConstructionError> {
        // Use VrfDraft03::prove with the 64-byte secret key
        let sk: [u8; 64] = self.as_bytes().try_into().unwrap();
        let proof = VrfDraft03::prove(&sk, message)?;
        Ok(PraosProof {
            bytes: proof.to_vec(),
        })
    }
}

impl Clone for PraosSigningKey {
    fn clone(&self) -> Self {
        let secret = self.secret.try_clone().expect("failed to clone secret key");
        Self { secret }
    }
}

impl Drop for PraosSigningKey {
    fn drop(&mut self) {
        // MLockedBytes already zeroes memory on drop.
    }
}

pub struct PraosVerificationKey {
    bytes: Vec<u8>,
}

impl PraosVerificationKey {
    /// Creates a Praos verification key from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte length is not 32 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosConstructionError> {
        if bytes.len() != verification_key_size() {
            return Err(PraosConstructionError::WrongLength {
                expected: verification_key_size(),
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

    /// Verifies a VRF proof against this verification key.
    ///
    /// # Errors
    ///
    /// This function should not fail under normal circumstances as verification
    /// is deterministic. Returns `Ok(None)` if the proof is invalid, or `Ok(Some(output))`
    /// if the proof is valid.
    pub fn verify(
        &self,
        message: &[u8],
        proof: &PraosProof,
    ) -> Result<Option<Vec<u8>>, PraosConstructionError> {
        // Use VrfDraft03::verify
        let pk: [u8; 32] = self.bytes.as_slice().try_into().unwrap();
        let proof_bytes: [u8; 80] = proof.bytes.as_slice().try_into().unwrap();

        match VrfDraft03::verify(&pk, &proof_bytes, message) {
            Ok(output) => Ok(Some(output.to_vec())),
            Err(_) => Ok(None),
        }
    }
}

impl Clone for PraosVerificationKey {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
        }
    }
}

impl fmt::Debug for PraosVerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PraosVerificationKey")
            .field(&hex::encode(&self.bytes))
            .finish()
    }
}

impl PartialEq for PraosVerificationKey {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for PraosVerificationKey {}

// CBOR Serialization for PraosVerificationKey
#[cfg(feature = "serde")]
impl serde::Serialize for PraosVerificationKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PraosVerificationKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BytesVisitor;

        impl<'de> serde::de::Visitor<'de> for BytesVisitor {
            type Value = PraosVerificationKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "Praos VRF verification key bytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                PraosVerificationKey::from_bytes(v)
                    .map_err(|e| E::custom(format!("invalid Praos verification key: {}", e)))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_bytes(&v)
            }
        }

        deserializer.deserialize_bytes(BytesVisitor)
    }
}

// DirectSerialise implementation for zero-copy serialization
impl DirectSerialise for PraosVerificationKey {
    fn direct_serialise(
        &self,
        push: &mut dyn FnMut(*const u8, usize) -> DirectResult<()>,
    ) -> DirectResult<()> {
        push(self.bytes.as_ptr(), verification_key_size())
    }
}

impl DirectDeserialise for PraosVerificationKey {
    fn direct_deserialise(
        pull: &mut dyn FnMut(*mut u8, usize) -> DirectResult<()>,
    ) -> DirectResult<Self> {
        let mut bytes = vec![0u8; verification_key_size()];
        pull(bytes.as_mut_ptr(), verification_key_size())?;
        Self::from_bytes(&bytes).map_err(|_| SizeCheckError {
            expected_size: verification_key_size(),
            actual_size: bytes.len(),
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PraosProof {
    bytes: Vec<u8>,
}

impl fmt::Debug for PraosProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PraosProof")
            .field(&hex::encode(&self.bytes))
            .finish()
    }
}

impl PraosProof {
    /// Creates a Praos VRF proof from raw bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte length is not 80 bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PraosConstructionError> {
        if bytes.len() != proof_size() {
            return Err(PraosConstructionError::WrongLength {
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
    pub fn to_output_bytes(&self) -> Result<Option<Vec<u8>>, PraosConstructionError> {
        // Use VrfDraft03::proof_to_hash
        let proof_bytes: [u8; 80] = self.bytes.as_slice().try_into().unwrap();
        match VrfDraft03::proof_to_hash(&proof_bytes) {
            Ok(output) => Ok(Some(output.to_vec())),
            Err(_) => Ok(None),
        }
    }
}

// CBOR Serialization for PraosProof
#[cfg(feature = "serde")]
impl serde::Serialize for PraosProof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PraosProof {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BytesVisitor;

        impl<'de> serde::de::Visitor<'de> for BytesVisitor {
            type Value = PraosProof;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "Praos VRF proof bytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                PraosProof::from_bytes(v)
                    .map_err(|e| E::custom(format!("invalid Praos proof: {}", e)))
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_bytes(&v)
            }
        }

        deserializer.deserialize_bytes(BytesVisitor)
    }
}

// DirectSerialise implementation for zero-copy serialization
impl DirectSerialise for PraosProof {
    fn direct_serialise(
        &self,
        push: &mut dyn FnMut(*const u8, usize) -> DirectResult<()>,
    ) -> DirectResult<()> {
        push(self.bytes.as_ptr(), proof_size())
    }
}

impl DirectDeserialise for PraosProof {
    fn direct_deserialise(
        pull: &mut dyn FnMut(*mut u8, usize) -> DirectResult<()>,
    ) -> DirectResult<Self> {
        let mut bytes = vec![0u8; proof_size()];
        pull(bytes.as_mut_ptr(), proof_size())?;
        Self::from_bytes(&bytes).map_err(|_| SizeCheckError {
            expected_size: proof_size(),
            actual_size: bytes.len(),
        })
    }
}

/// Generates a Praos keypair from a seed.
///
/// # Errors
///
/// Returns an error if memory-locked allocation fails.
pub fn keypair_from_seed(
    seed: &PraosSeed,
) -> Result<(PraosVerificationKey, PraosSigningKey), PraosConstructionError> {
    // Use VrfDraft03::keypair_from_seed or common functions
    let seed_bytes: [u8; 32] = seed.as_bytes().try_into().unwrap();
    let (sk_array, pk_array) = VrfDraft03::keypair_from_seed(&seed_bytes);

    // Store in mlocked memory
    let mut sk = MLockedBytes::new_zeroed(signing_key_size())?;
    sk.as_mut_slice().copy_from_slice(&sk_array);

    Ok((
        PraosVerificationKey {
            bytes: pk_array.to_vec(),
        },
        PraosSigningKey { secret: sk },
    ))
}

/// Generates a Praos keypair from seed bytes.
///
/// # Errors
///
/// Returns an error if:
/// - The seed byte length is not 32 bytes
/// - Memory-locked allocation fails
/// - Key derivation fails
pub fn keypair_from_seed_bytes(
    seed_bytes: &[u8],
) -> Result<(PraosVerificationKey, PraosSigningKey), PraosConstructionError> {
    let seed = PraosSeed::from_bytes(seed_bytes)?;
    keypair_from_seed(&seed)
}

#[must_use]
pub fn signing_key_from_seed(seed: &Seed) -> PraosSigningKey {
    let (material, _) = crate::seed::get_bytes_from_seed_t(signing_key_size(), seed.clone());
    signing_key_from_bytes(&material).expect("seed produced invalid signing key")
}

/// Creates a Praos signing key from raw bytes.
///
/// # Errors
///
/// Returns an error if:
/// - The byte length is not 64 bytes
/// - Memory-locked allocation fails
pub fn signing_key_from_bytes(bytes: &[u8]) -> Result<PraosSigningKey, PraosConstructionError> {
    PraosSigningKey::from_bytes(bytes)
}

#[must_use]
pub fn signing_key_to_bytes(signing_key: &PraosSigningKey) -> Vec<u8> {
    signing_key.to_vec()
}

/// Creates a Praos verification key from raw bytes.
///
/// # Errors
///
/// Returns an error if the byte length is not 32 bytes.
pub fn verification_key_from_bytes(
    bytes: &[u8],
) -> Result<PraosVerificationKey, PraosConstructionError> {
    PraosVerificationKey::from_bytes(bytes)
}

#[must_use]
pub fn verification_key_to_bytes(verification_key: &PraosVerificationKey) -> Vec<u8> {
    verification_key.to_vec()
}

/// Creates a Praos VRF proof from raw bytes.
///
/// # Errors
///
/// Returns an error if the byte length is not 80 bytes.
pub fn proof_from_bytes(bytes: &[u8]) -> Result<PraosProof, PraosConstructionError> {
    PraosProof::from_bytes(bytes)
}

#[must_use]
pub fn proof_to_bytes(proof: &PraosProof) -> Vec<u8> {
    proof.to_vec()
}

/// Creates a Praos seed from raw bytes.
///
/// # Errors
///
/// Returns an error if:
/// - The byte length is not 32 bytes
/// - Memory-locked allocation fails
pub fn seed_from_bytes(bytes: &[u8]) -> Result<PraosSeed, PraosConstructionError> {
    PraosSeed::from_bytes(bytes)
}

#[must_use]
pub fn seed_to_bytes(seed: &PraosSeed) -> Vec<u8> {
    seed.to_vec()
}

/// Extracts VRF output from a proof.
///
/// # Errors
///
/// Returns an error if:
/// - The proof output extraction fails
/// - The output length is not 64 bytes
pub fn output_from_proof(
    proof: &PraosProof,
) -> Result<Option<OutputVRF<PraosVRF>>, PraosConstructionError> {
    match proof.to_output_bytes()? {
        Some(bytes) => {
            let actual = bytes.len();
            if actual != output_size() {
                return Err(PraosConstructionError::WrongLength {
                    expected: output_size(),
                    actual,
                });
            }
            let output =
                OutputVRF::from_bytes(bytes).map_err(|_| PraosConstructionError::WrongLength {
                    expected: output_size(),
                    actual,
                })?;
            Ok(Some(output))
        },
        None => Ok(None),
    }
}

/// Converts a Praos verification key to batch-compatible format.
///
/// # Errors
///
/// Returns an error if the key length is invalid.
pub fn vk_to_batch_compat(
    verification_key: &PraosVerificationKey,
) -> Result<PraosBatchCompatVerificationKey, PraosConstructionError> {
    PraosBatchCompatVerificationKey::from_bytes(verification_key.as_bytes()).map_err(|_| {
        PraosConstructionError::WrongLength {
            expected: verification_key_size(),
            actual: verification_key.as_bytes().len(),
        }
    })
}

/// Converts a Praos signing key to batch-compatible format.
///
/// # Errors
///
/// Returns an error if the key length is invalid.
pub fn sk_to_batch_compat(
    signing_key: &PraosSigningKey,
) -> Result<PraosBatchCompatSigningKey, PraosConstructionError> {
    PraosBatchCompatSigningKey::from_bytes(signing_key.as_bytes()).map_err(|_| {
        PraosConstructionError::WrongLength {
            expected: signing_key_size(),
            actual: signing_key.as_bytes().len(),
        }
    })
}

/// Converts a Praos VRF output to batch-compatible format.
///
/// # Errors
///
/// Returns an error if the output length is invalid.
pub fn output_to_batch_compat(
    output: &OutputVRF<PraosVRF>,
) -> Result<OutputVRF<PraosBatchCompatVRF>, VRFError> {
    OutputVRF::copy_from_slice(output.as_bytes())
}

pub struct PraosVRF;

impl VRFAlgorithm for PraosVRF {
    type VerificationKey = PraosVerificationKey;
    type SigningKey = PraosSigningKey;
    type Proof = PraosProof;
    type Context = ();

    const ALGORITHM_NAME: &'static str = "PraosVRF";
    const SEED_SIZE: usize = seed_size();
    const VERIFICATION_KEY_SIZE: usize = verification_key_size();
    const SIGNING_KEY_SIZE: usize = signing_key_size();
    const PROOF_SIZE: usize = proof_size();
    const OUTPUT_SIZE: usize = output_size();

    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey {
        signing_key
            .derive_verification_key()
            .expect("praos sk_to_pk failed")
    }

    fn evaluate_bytes(
        _context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> (OutputVRF<Self>, Self::Proof) {
        let proof = signing_key.prove(message).expect("praos prove failed");
        let output_bytes = proof
            .to_output_bytes()
            .expect("praos proof_to_hash failed")
            .expect("invalid praos proof");
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
        let (_, sk) = keypair_from_seed_bytes(seed).expect("invalid praos seed bytes");
        sk
    }

    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8> {
        key.to_vec()
    }

    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey> {
        PraosVerificationKey::from_bytes(bytes).ok()
    }

    fn raw_serialize_signing_key(key: &Self::SigningKey) -> Vec<u8> {
        key.to_vec()
    }

    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey> {
        PraosSigningKey::from_bytes(bytes).ok()
    }

    fn raw_serialize_proof(proof: &Self::Proof) -> Vec<u8> {
        proof.to_vec()
    }

    fn raw_deserialize_proof(bytes: &[u8]) -> Option<Self::Proof> {
        PraosProof::from_bytes(bytes).ok()
    }
}
