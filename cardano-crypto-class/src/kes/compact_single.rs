use std::marker::PhantomData;

use crate::dsign::DsignMAlgorithm;
use crate::kes::{KesAlgorithm, KesError, KesMError, Period};

/// CompactSingleKES wraps a DSIGNM algorithm with an embedded verification key.
///
/// Unlike SingleKES, the signature includes the verification key. This allows
/// CompactSumKES to reconstruct verification keys from signatures, reducing
/// the number of keys that need to be stored in the Merkle tree.
pub struct CompactSingleKes<D: DsignMAlgorithm>(PhantomData<D>);

/// Signature type that embeds the verification key.
#[derive(Clone, PartialEq, Eq)]
pub struct CompactSingleSig<D: DsignMAlgorithm> {
    pub(crate) signature: D::Signature,
    pub(crate) verification_key: D::VerificationKey,
}

impl<D> KesAlgorithm for CompactSingleKes<D>
where
    D: DsignMAlgorithm,
    D::VerificationKey: Clone,
{
    type VerificationKey = D::VerificationKey;
    type SigningKey = D::MLockedSigningKey;
    type Signature = CompactSingleSig<D>;
    type Context = D::Context;

    const ALGORITHM_NAME: &'static str = D::ALGORITHM_NAME;
    const SEED_SIZE: usize = D::SEED_SIZE;
    const VERIFICATION_KEY_SIZE: usize = D::VERIFICATION_KEY_SIZE;
    const SIGNING_KEY_SIZE: usize = D::SIGNING_KEY_SIZE;
    // Signature size is DSIGN signature + verification key
    const SIGNATURE_SIZE: usize = D::SIGNATURE_SIZE + D::VERIFICATION_KEY_SIZE;

    fn total_periods() -> Period {
        1
    }

    fn derive_verification_key(
        signing_key: &Self::SigningKey,
    ) -> Result<Self::VerificationKey, KesMError> {
        D::derive_verification_key_m(signing_key).map_err(|e| KesMError::Dsign(format!("{:?}", e)))
    }

    fn sign_kes(
        context: &Self::Context,
        period: Period,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> Result<Self::Signature, KesMError> {
        if period != 0 {
            return Err(KesMError::Kes(KesError::PeriodOutOfRange {
                period,
                max_period: 1,
            }));
        }
        let signature = D::sign_bytes_m(context, message, signing_key)
            .map_err(|e| KesMError::Dsign(format!("{:?}", e)))?;
        let verification_key = D::derive_verification_key_m(signing_key)
            .map_err(|e| KesMError::Dsign(format!("{:?}", e)))?;
        Ok(CompactSingleSig {
            signature,
            verification_key,
        })
    }

    fn verify_kes(
        context: &Self::Context,
        _verification_key: &Self::VerificationKey,
        period: Period,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<(), KesError> {
        if period != 0 {
            return Err(KesError::PeriodOutOfRange {
                period,
                max_period: 1,
            });
        }
        // Verify using the embedded verification key
        D::verify_bytes(
            context,
            &signature.verification_key,
            message,
            &signature.signature,
        )
        .map_err(|_| KesError::VerificationFailed)
    }

    fn update_kes(
        _context: &Self::Context,
        signing_key: Self::SigningKey,
        period: Period,
    ) -> Result<Option<Self::SigningKey>, KesMError> {
        if period >= 1 {
            D::forget_signing_key_m(signing_key);
            Ok(None)
        } else {
            Ok(Some(signing_key))
        }
    }

    fn gen_key_kes_from_seed_bytes(_seed: &[u8]) -> Result<Self::SigningKey, KesMError> {
        Err(KesMError::Dsign(
            "gen_key_kes_from_seed_bytes not yet fully implemented for generic DSIGNM".to_owned(),
        ))
    }

    fn raw_serialize_verification_key_kes(key: &Self::VerificationKey) -> Vec<u8> {
        D::raw_serialize_verification_key(key)
    }

    fn raw_deserialize_verification_key_kes(bytes: &[u8]) -> Option<Self::VerificationKey> {
        D::raw_deserialize_verification_key(bytes)
    }

    fn raw_serialize_signature_kes(signature: &Self::Signature) -> Vec<u8> {
        let mut result = D::raw_serialize_signature(&signature.signature);
        result.extend_from_slice(&D::raw_serialize_verification_key(
            &signature.verification_key,
        ));
        result
    }

    fn raw_deserialize_signature_kes(bytes: &[u8]) -> Option<Self::Signature> {
        if bytes.len() != Self::SIGNATURE_SIZE {
            return None;
        }
        let sig_bytes = &bytes[0..D::SIGNATURE_SIZE];
        let vk_bytes = &bytes[D::SIGNATURE_SIZE..];

        let signature = D::raw_deserialize_signature(sig_bytes)?;
        let verification_key = D::raw_deserialize_verification_key(vk_bytes)?;

        Some(CompactSingleSig {
            signature,
            verification_key,
        })
    }

    fn forget_signing_key_kes(signing_key: Self::SigningKey) {
        D::forget_signing_key_m(signing_key);
    }
}

/// Helper trait to extract the verification key from a CompactSingle signature.
pub trait OptimizedKesSignature {
    type VerificationKey;

    fn extract_verification_key(&self) -> &Self::VerificationKey;
}

impl<D: DsignMAlgorithm> OptimizedKesSignature for CompactSingleSig<D> {
    type VerificationKey = D::VerificationKey;

    fn extract_verification_key(&self) -> &Self::VerificationKey {
        &self.verification_key
    }
}
