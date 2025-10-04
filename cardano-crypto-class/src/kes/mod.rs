use std::fmt;
use std::marker::PhantomData;

use thiserror::Error;

use crate::mlocked_bytes::MLockedError;
use crate::seed::{get_bytes_from_seed_t, Seed};
use crate::util::SignableRepresentation;

pub mod compact_single;
pub mod compact_sum;
pub mod hash;
pub mod single;
pub mod sum;
pub mod verify_hash;

// Re-export hash algorithms for convenience
pub use hash::{Blake2b256, Blake2b512, KesHashAlgorithm};

// Re-export SingleKes types
pub use single::{SingleKes, SingleSignature, SingleSigningKey};

// Re-export CompactSingleKes types
pub use compact_single::{
    CompactSingleKes, CompactSingleSig, CompactSingleSigningKey, OptimizedKesSignature,
};

// Re-export Sum type aliases (using Blake2b256)
pub use sum::{Sum0Kes, Sum1Kes, Sum2Kes, Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes, Sum7Kes};

// Re-export CompactSum type aliases (using Blake2b256)
pub use compact_sum::{
    CompactSum0Kes, CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, CompactSum4Kes, CompactSum5Kes,
    CompactSum6Kes, CompactSum7Kes,
};

/// The KES period. Periods are enumerated from zero.
pub type Period = u64;

/// Error raised by KES operations.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum KesError {
    #[error("KES signature verification failed")]
    VerificationFailed,
    #[error("{context}: wrong length, expected {expected} bytes but got {actual}")]
    WrongLength {
        context: &'static str,
        expected: usize,
        actual: usize,
    },
    #[error("{0}")]
    Message(String),
    #[error("KES key evolved beyond max period")]
    KeyExpired,
    #[error("period {period} out of range [0, {max_period})")]
    PeriodOutOfRange { period: Period, max_period: Period },
}

impl KesError {
    #[must_use]
    pub fn wrong_length(context: &'static str, expected: usize, actual: usize) -> Self {
        KesError::WrongLength {
            context,
            expected,
            actual,
        }
    }
}

/// Error raised by mlocked KES operations.
#[derive(Debug, Error)]
pub enum KesMError {
    #[error(transparent)]
    Kes(#[from] KesError),
    #[error(transparent)]
    Mlocked(#[from] MLockedError),
    #[error("DSIGN error: {0}")]
    Dsign(String),
}

/// Trait capturing the common KES (Key Evolving Signature) interface.
///
/// This follows the design from "Composition and Efficiency Tradeoffs for
/// Forward-Secure Digital Signatures" by Tal Malkin, Daniele Micciancio,
/// and Sara Miner (https://eprint.iacr.org/2001/034).
pub trait KesAlgorithm {
    /// Verification key type.
    type VerificationKey;
    /// Signing key type (may contain mlocked memory).
    type SigningKey;
    /// Signature type.
    type Signature;
    /// Optional context parameter.
    type Context;

    /// Name of the algorithm.
    const ALGORITHM_NAME: &'static str;
    /// Number of seed bytes required.
    const SEED_SIZE: usize;
    /// Size of the verification key when serialized.
    const VERIFICATION_KEY_SIZE: usize;
    /// Size of the signing key when serialized.
    const SIGNING_KEY_SIZE: usize;
    /// Size of signatures produced.
    const SIGNATURE_SIZE: usize;

    /// Total number of periods this KES scheme supports.
    fn total_periods() -> Period;

    /// Derive the verification key from a signing key.
    fn derive_verification_key(
        signing_key: &Self::SigningKey,
    ) -> Result<Self::VerificationKey, KesMError>;

    /// Sign a message at a specific period.
    fn sign_kes(
        context: &Self::Context,
        period: Period,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> Result<Self::Signature, KesMError>;

    /// Verify a KES signature at a specific period.
    fn verify_kes(
        context: &Self::Context,
        verification_key: &Self::VerificationKey,
        period: Period,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<(), KesError>;

    /// Update (evolve) the signing key to the next period.
    ///
    /// Returns None if the key has expired (reached max period).
    fn update_kes(
        context: &Self::Context,
        signing_key: Self::SigningKey,
        period: Period,
    ) -> Result<Option<Self::SigningKey>, KesMError>;

    /// Generate a signing key from a seed.
    fn gen_key_kes(seed: &Seed) -> Result<Self::SigningKey, KesMError> {
        let (material, _) = get_bytes_from_seed_t(Self::SEED_SIZE, seed.clone());
        Self::gen_key_kes_from_seed_bytes(&material)
    }

    /// Generate a signing key from raw seed bytes.
    fn gen_key_kes_from_seed_bytes(seed: &[u8]) -> Result<Self::SigningKey, KesMError>;

    /// Serialize the verification key.
    fn raw_serialize_verification_key_kes(key: &Self::VerificationKey) -> Vec<u8>;

    /// Deserialize a verification key.
    fn raw_deserialize_verification_key_kes(bytes: &[u8]) -> Option<Self::VerificationKey>;

    /// Serialize a signature.
    fn raw_serialize_signature_kes(signature: &Self::Signature) -> Vec<u8>;

    /// Deserialize a signature.
    fn raw_deserialize_signature_kes(bytes: &[u8]) -> Option<Self::Signature>;

    /// Securely forget/zeroize a signing key.
    fn forget_signing_key_kes(signing_key: Self::SigningKey);
}

/// Trait for unsound KES operations (exposing signing key serialization).
pub trait UnsoundKesAlgorithm: KesAlgorithm {
    /// Serialize a signing key (UNSOUND - use only for testing).
    fn raw_serialize_signing_key_kes(signing_key: &Self::SigningKey) -> Result<Vec<u8>, KesMError>;

    /// Deserialize a signing key (UNSOUND - use only for testing).
    fn raw_deserialize_signing_key_kes(bytes: &[u8]) -> Result<Self::SigningKey, KesMError>;
}

/// Wrapper around a KES signature carrying algorithm and message types.
#[derive(Clone)]
pub struct SignedKes<A, M>
where
    A: KesAlgorithm,
    M: ?Sized,
{
    signature: A::Signature,
    period: Period,
    _marker: PhantomData<fn(&M)>,
}

impl<A, M> SignedKes<A, M>
where
    A: KesAlgorithm,
    M: ?Sized,
{
    pub fn new(signature: A::Signature, period: Period) -> Self {
        Self {
            signature,
            period,
            _marker: PhantomData,
        }
    }

    pub fn signature(&self) -> &A::Signature {
        &self.signature
    }

    pub fn period(&self) -> Period {
        self.period
    }

    pub fn into_inner(self) -> (A::Signature, Period) {
        (self.signature, self.period)
    }
}

impl<A, M> fmt::Debug for SignedKes<A, M>
where
    A: KesAlgorithm,
    A::Signature: fmt::Debug,
    M: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignedKes")
            .field("signature", &self.signature)
            .field("period", &self.period)
            .finish()
    }
}

/// Convenience function to create a signed KES value.
pub fn signed_kes<A, M>(
    context: &A::Context,
    period: Period,
    message: &M,
    signing_key: &A::SigningKey,
) -> Result<SignedKes<A, M>, KesMError>
where
    A: KesAlgorithm,
    M: SignableRepresentation + ?Sized,
{
    let representation = message.signable_representation();
    let signature = A::sign_kes(context, period, representation.as_ref(), signing_key)?;
    Ok(SignedKes::new(signature, period))
}

/// Verify a signed KES value.
pub fn verify_signed_kes<A, M>(
    context: &A::Context,
    verification_key: &A::VerificationKey,
    message: &M,
    signed: &SignedKes<A, M>,
) -> Result<(), KesError>
where
    A: KesAlgorithm,
    M: SignableRepresentation + ?Sized,
{
    let representation = message.signable_representation();
    A::verify_kes(
        context,
        verification_key,
        signed.period(),
        representation.as_ref(),
        signed.signature(),
    )
}

/// Helper functions
#[must_use]
pub const fn seed_size_kes<A: KesAlgorithm>() -> usize {
    A::SEED_SIZE
}

#[must_use]
pub const fn size_verification_key_kes<A: KesAlgorithm>() -> usize {
    A::VERIFICATION_KEY_SIZE
}

#[must_use]
pub const fn size_signing_key_kes<A: KesAlgorithm>() -> usize {
    A::SIGNING_KEY_SIZE
}

#[must_use]
pub const fn size_signature_kes<A: KesAlgorithm>() -> usize {
    A::SIGNATURE_SIZE
}

#[must_use]
pub fn total_periods_kes<A: KesAlgorithm>() -> Period {
    A::total_periods()
}
