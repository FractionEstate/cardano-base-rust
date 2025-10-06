use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

pub mod mock;
pub mod never;
pub mod praos;
pub mod praos_batch;
pub mod simple;

pub use praos::{
    PraosConstructionError, PraosProof, PraosSeed, PraosSigningKey, PraosVRF, PraosVerificationKey,
    gen_seed as praos_gen_seed, keypair_from_seed as praos_keypair_from_seed,
    keypair_from_seed_bytes as praos_keypair_from_seed_bytes,
    output_from_proof as praos_output_from_proof,
    output_to_batch_compat as praos_output_to_batch_compat,
    proof_from_bytes as praos_proof_from_bytes, proof_to_bytes as praos_proof_to_bytes,
    seed_from_bytes as praos_seed_from_bytes, seed_to_bytes as praos_seed_to_bytes,
    signing_key_from_bytes as praos_signing_key_from_bytes,
    signing_key_to_bytes as praos_signing_key_to_bytes,
    sk_to_batch_compat as praos_sk_to_batch_compat,
    verification_key_from_bytes as praos_verification_key_from_bytes,
    verification_key_to_bytes as praos_verification_key_to_bytes,
    vk_to_batch_compat as praos_vk_to_batch_compat,
};

pub use praos_batch::{
    PraosBatchCompatProof, PraosBatchCompatSeed, PraosBatchCompatSigningKey, PraosBatchCompatVRF,
    PraosBatchCompatVerificationKey, PraosBatchConstructionError, gen_seed as praos_batch_gen_seed,
    keypair_from_seed as praos_batch_keypair_from_seed,
    keypair_from_seed_bytes as praos_batch_keypair_from_seed_bytes,
    output_from_proof as praos_batch_output_from_proof,
    proof_from_bytes as praos_batch_proof_from_bytes, proof_to_bytes as praos_batch_proof_to_bytes,
    seed_from_bytes as praos_batch_seed_from_bytes, seed_to_bytes as praos_batch_seed_to_bytes,
    signing_key_from_bytes as praos_batch_signing_key_from_bytes,
    signing_key_to_bytes as praos_batch_signing_key_to_bytes,
    r#unsafe_raw_seed as praos_batch_unsafe_raw_seed,
    verification_key_from_bytes as praos_batch_verification_key_from_bytes,
    verification_key_to_bytes as praos_batch_verification_key_to_bytes,
};

pub use mock::{
    MockCertificate, MockSigningKey, MockVRF, MockVerificationKey, gen_key as mock_gen_key,
    gen_keypair as mock_gen_keypair,
};

pub use never::{
    NeverCertificate, NeverSigningKey, NeverVRF, NeverVerificationKey, gen_key as never_gen_key,
    gen_keypair as never_gen_keypair,
};

pub use simple::{
    SimpleCertificate, SimpleSigningKey, SimpleVRF, SimpleVerificationKey,
    gen_key as simple_gen_key, gen_keypair as simple_gen_keypair,
};

use num_bigint::BigUint;
use thiserror::Error;

use crate::seed::{Seed, get_bytes_from_seed_t};
use crate::util::{SignableRepresentation, bytes_to_natural, natural_to_bytes};

/// Errors that can occur when working with VRF helpers.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum VRFError {
    #[error("{context}: wrong length, expected {expected} bytes but got {actual}")]
    WrongLength {
        context: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("value exceeds {expected} bytes")]
    ValueTooLarge { expected: usize },
}

impl VRFError {
    #[must_use]
    pub fn wrong_length(context: &'static str, expected: usize, actual: usize) -> Self {
        VRFError::WrongLength {
            context,
            expected,
            actual,
        }
    }

    #[must_use]
    pub fn value_too_large(expected: usize) -> Self {
        VRFError::ValueTooLarge { expected }
    }
}

/// Output bytes produced by a VRF evaluation.
#[derive(Clone)]
pub struct OutputVRF<A: VRFAlgorithm> {
    bytes: Vec<u8>,
    _marker: PhantomData<A>,
}

impl<A: VRFAlgorithm> PartialEq for OutputVRF<A> {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl<A: VRFAlgorithm> Eq for OutputVRF<A> {}

impl<A: VRFAlgorithm> Hash for OutputVRF<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}
impl<A: VRFAlgorithm> fmt::Debug for OutputVRF<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OutputVRF")
            .field(&hex::encode(&self.bytes))
            .finish()
    }
}

impl<A: VRFAlgorithm> OutputVRF<A> {
    /// Construct an output from raw bytes, validating the expected size.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte length does not match the expected output size for the VRF algorithm.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, VRFError> {
        if bytes.len() != A::OUTPUT_SIZE {
            return Err(VRFError::wrong_length(
                "OutputVRF",
                A::OUTPUT_SIZE,
                bytes.len(),
            ));
        }
        Ok(Self {
            bytes,
            _marker: PhantomData,
        })
    }

    /// Construct an output by copying the provided slice.
    ///
    /// # Errors
    ///
    /// Returns an error if the byte length does not match the expected output size for the VRF algorithm.
    pub fn copy_from_slice(bytes: &[u8]) -> Result<Self, VRFError> {
        Self::from_bytes(bytes.to_vec())
    }

    /// Immutable view of the output bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consume the wrapper and return the owned bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Interpret the output bytes as a natural number.
    #[must_use]
    pub fn to_natural(&self) -> BigUint {
        bytes_to_natural(&self.bytes)
    }

    /// Construct an output from a natural number, big-endian encoded to the expected length.
    ///
    /// # Errors
    ///
    /// Returns an error if the natural number is too large to fit in the expected output size.
    pub fn from_natural(value: &BigUint) -> Result<Self, VRFError> {
        let mut bytes = natural_to_bytes(A::OUTPUT_SIZE, value);
        if bytes.len() > A::OUTPUT_SIZE {
            return Err(VRFError::value_too_large(A::OUTPUT_SIZE));
        }
        if bytes.len() < A::OUTPUT_SIZE {
            let mut padded = vec![0u8; A::OUTPUT_SIZE];
            let offset = A::OUTPUT_SIZE - bytes.len();
            padded[offset..].copy_from_slice(&bytes);
            bytes = padded;
        }
        Self::from_bytes(bytes)
    }
}

/// Certified output pairing the VRF output with its proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertifiedVRF<A: VRFAlgorithm> {
    pub output: OutputVRF<A>,
    pub proof: A::Proof,
}

impl<A: VRFAlgorithm> CertifiedVRF<A> {
    pub fn new(output: OutputVRF<A>, proof: A::Proof) -> Self {
        Self { output, proof }
    }
}

/// Trait capturing the common interface exposed by VRF algorithms.
pub trait VRFAlgorithm {
    /// Verification key type.
    type VerificationKey; // VerKeyVRF
    /// Signing key type.
    type SigningKey; // SignKeyVRF
    /// Proof/certificate type produced by the VRF.
    type Proof; // CertVRF
    /// Optional context parameter.
    type Context;

    /// Name of the algorithm.
    const ALGORITHM_NAME: &'static str;
    /// Number of bytes required to seed key generation.
    const SEED_SIZE: usize;
    /// Size of the verification key when serialised.
    const VERIFICATION_KEY_SIZE: usize;
    /// Size of the signing key when serialised.
    const SIGNING_KEY_SIZE: usize;
    /// Size of the certificate/proof when serialised.
    const PROOF_SIZE: usize;
    /// Size of the VRF output in bytes.
    const OUTPUT_SIZE: usize;

    /// Derive a verification key from a signing key.
    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey;

    /// Evaluate the VRF over a signable message.
    fn eval<M>(
        context: &Self::Context,
        message: &M,
        signing_key: &Self::SigningKey,
    ) -> (OutputVRF<Self>, Self::Proof)
    where
        M: SignableRepresentation + ?Sized,
        Self: Sized,
    {
        let (output_bytes, proof) = Self::evaluate_bytes(
            context,
            message.signable_representation().as_ref(),
            signing_key,
        );
        (output_bytes, proof)
    }

    /// Evaluate the VRF on raw bytes.
    fn evaluate_bytes(
        context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> (OutputVRF<Self>, Self::Proof)
    where
        Self: Sized;

    /// Verify a VRF proof over a signable message.
    fn verify<M>(
        context: &Self::Context,
        verification_key: &Self::VerificationKey,
        message: &M,
        proof: &Self::Proof,
    ) -> Option<OutputVRF<Self>>
    where
        M: SignableRepresentation + ?Sized,
        Self: Sized,
    {
        Self::verify_bytes(
            context,
            verification_key,
            message.signable_representation().as_ref(),
            proof,
        )
    }

    /// Verify a VRF proof over raw bytes.
    fn verify_bytes(
        context: &Self::Context,
        verification_key: &Self::VerificationKey,
        message: &[u8],
        proof: &Self::Proof,
    ) -> Option<OutputVRF<Self>>
    where
        Self: Sized;

    /// Deterministically derive a signing key from the supplied seed.
    #[must_use]
    fn gen_key(seed: &Seed) -> Self::SigningKey
    where
        Self: Sized,
    {
        let (material, _) = get_bytes_from_seed_t(Self::SEED_SIZE, seed.clone());
        Self::gen_key_from_seed_bytes(&material)
    }

    /// Deterministically derive a signing key from seed bytes.
    fn gen_key_from_seed_bytes(seed: &[u8]) -> Self::SigningKey
    where
        Self: Sized;

    /// Deterministically derive a keypair from the supplied seed.
    #[must_use]
    fn gen_keypair(seed: &Seed) -> (Self::SigningKey, Self::VerificationKey)
    where
        Self: Sized,
    {
        let sk = Self::gen_key(seed);
        let vk = Self::derive_verification_key(&sk);
        (sk, vk)
    }

    /// Serialise a verification key into raw bytes.
    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8>
    where
        Self: Sized;

    /// Deserialise a verification key from raw bytes.
    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey>
    where
        Self: Sized;

    /// Serialise a signing key into raw bytes.
    fn raw_serialize_signing_key(key: &Self::SigningKey) -> Vec<u8>
    where
        Self: Sized;

    /// Deserialise a signing key from raw bytes.
    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey>
    where
        Self: Sized;

    /// Serialise a proof into raw bytes.
    fn raw_serialize_proof(proof: &Self::Proof) -> Vec<u8>
    where
        Self: Sized;

    /// Deserialise a proof from raw bytes.
    fn raw_deserialize_proof(bytes: &[u8]) -> Option<Self::Proof>
    where
        Self: Sized;
}

/// Convenience helper mirroring `evalCertified` from the Haskell implementation.
pub fn eval_certified<A, M>(
    context: &A::Context,
    message: &M,
    signing_key: &A::SigningKey,
) -> CertifiedVRF<A>
where
    A: VRFAlgorithm,
    M: SignableRepresentation + ?Sized,
{
    let (output, proof) = A::eval(context, message, signing_key);
    CertifiedVRF::new(output, proof)
}

/// Verify a certified VRF proof, returning whether it matches the derived output.
pub fn verify_certified<A, M>(
    context: &A::Context,
    verification_key: &A::VerificationKey,
    message: &M,
    certified: &CertifiedVRF<A>,
) -> bool
where
    A: VRFAlgorithm,
    M: SignableRepresentation + ?Sized,
{
    match A::verify(context, verification_key, message, &certified.proof) {
        Some(output) => output == certified.output,
        None => false,
    }
}
