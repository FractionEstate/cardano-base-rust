use std::marker::PhantomData;

use blake2::{Blake2b512, Digest};

use crate::kes::{KesAlgorithm, KesError, KesMError, Period};
use crate::mlocked_bytes::MLockedBytes;
use crate::seed::Seed;

/// SumKES composes two KES schemes to create a scheme with double the periods.
///
/// This implements the binary sum composition from Section 3.1 of the MMM paper
/// "Composition and Efficiency Tradeoffs for Forward-Secure Digital Signatures".
///
/// The signing key contains:
/// - sk_0: signing key for the first half of periods
/// - r_1: seed for generating sk_1 (second half)
/// - vk_0, vk_1: verification keys for both halves
///
/// The verification key is: H(vk_0 || vk_1)
pub struct SumKes<D: KesAlgorithm>(PhantomData<D>);

/// Signing key for SumKES contains both constituent keys.
pub struct SumSigningKey<D: KesAlgorithm> {
    /// Current signing key (either for left or right subtree)
    pub(crate) sk: D::SigningKey,
    /// Seed for the right subtree (mlocked)
    pub(crate) r1_seed: Option<MLockedBytes>,
    /// Verification key for left subtree
    pub(crate) vk0: D::VerificationKey,
    /// Verification key for right subtree
    pub(crate) vk1: D::VerificationKey,
}

/// Signature for SumKES includes constituent signature and both verification keys.
#[derive(Clone)]
pub struct SumSignature<D: KesAlgorithm> {
    pub(crate) sigma: D::Signature,
    pub(crate) vk0: D::VerificationKey,
    pub(crate) vk1: D::VerificationKey,
}

impl<D> KesAlgorithm for SumKes<D>
where
    D: KesAlgorithm,
    D::VerificationKey: Clone,
{
    type VerificationKey = Vec<u8>; // Hash of (vk0, vk1)
    type SigningKey = SumSigningKey<D>;
    type Signature = SumSignature<D>;
    type Context = D::Context;

    const ALGORITHM_NAME: &'static str = D::ALGORITHM_NAME; // Could append "_sum"
    const SEED_SIZE: usize = D::SEED_SIZE;
    const VERIFICATION_KEY_SIZE: usize = 64; // Blake2b-512 output
    const SIGNING_KEY_SIZE: usize =
        D::SIGNING_KEY_SIZE + D::SEED_SIZE + 2 * D::VERIFICATION_KEY_SIZE;
    const SIGNATURE_SIZE: usize = D::SIGNATURE_SIZE + 2 * D::VERIFICATION_KEY_SIZE;

    fn total_periods() -> Period {
        2 * D::total_periods()
    }

    fn derive_verification_key(
        signing_key: &Self::SigningKey,
    ) -> Result<Self::VerificationKey, KesMError> {
        // vk = H(vk0 || vk1)
        let mut hasher = Blake2b512::new();
        hasher.update(D::raw_serialize_verification_key_kes(&signing_key.vk0));
        hasher.update(D::raw_serialize_verification_key_kes(&signing_key.vk1));
        Ok(hasher.finalize().to_vec())
    }

    fn sign_kes(
        context: &Self::Context,
        period: Period,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> Result<Self::Signature, KesMError> {
        let t_half = D::total_periods();

        let sigma = if period < t_half {
            // Use left subtree (sk_0)
            D::sign_kes(context, period, message, &signing_key.sk)?
        } else {
            // Use right subtree (sk_1)
            D::sign_kes(context, period - t_half, message, &signing_key.sk)?
        };

        Ok(SumSignature {
            sigma,
            vk0: signing_key.vk0.clone(),
            vk1: signing_key.vk1.clone(),
        })
    }

    fn verify_kes(
        context: &Self::Context,
        verification_key: &Self::VerificationKey,
        period: Period,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<(), KesError> {
        // Verify that H(vk0 || vk1) matches the provided verification key
        let mut hasher = Blake2b512::new();
        hasher.update(D::raw_serialize_verification_key_kes(&signature.vk0));
        hasher.update(D::raw_serialize_verification_key_kes(&signature.vk1));
        let computed_vk = hasher.finalize().to_vec();

        if &computed_vk != verification_key {
            return Err(KesError::VerificationFailed);
        }

        let t_half = D::total_periods();

        if period < t_half {
            // Verify against left subtree
            D::verify_kes(context, &signature.vk0, period, message, &signature.sigma)
        } else {
            // Verify against right subtree
            D::verify_kes(
                context,
                &signature.vk1,
                period - t_half,
                message,
                &signature.sigma,
            )
        }
    }

    fn update_kes(
        context: &Self::Context,
        mut signing_key: Self::SigningKey,
        period: Period,
    ) -> Result<Option<Self::SigningKey>, KesMError> {
        let t_half = D::total_periods();

        if period + 1 >= 2 * t_half {
            // Key has expired
            D::forget_signing_key_kes(signing_key.sk);
            return Ok(None);
        }

        if period + 1 == t_half {
            // Transition from left to right subtree
            // Generate sk_1 from r1_seed
            let r1_seed = signing_key
                .r1_seed
                .take()
                .ok_or(KesMError::Kes(KesError::KeyExpired))?;

            let seed = Seed::from_bytes(r1_seed.as_slice());
            let sk1 = D::gen_key_kes(&seed)?;

            // Forget the old signing key
            D::forget_signing_key_kes(signing_key.sk);

            Ok(Some(SumSigningKey {
                sk: sk1,
                r1_seed: None, // Already used
                vk0: signing_key.vk0,
                vk1: signing_key.vk1,
            }))
        } else if period + 1 < t_half {
            // Still in left subtree, update sk_0
            let updated_sk = D::update_kes(context, signing_key.sk, period)?;
            match updated_sk {
                Some(sk) => Ok(Some(SumSigningKey {
                    sk,
                    r1_seed: signing_key.r1_seed,
                    vk0: signing_key.vk0,
                    vk1: signing_key.vk1,
                })),
                None => Ok(None),
            }
        } else {
            // In right subtree, update sk_1
            let adjusted_period = period - t_half;
            let updated_sk = D::update_kes(context, signing_key.sk, adjusted_period)?;
            match updated_sk {
                Some(sk) => Ok(Some(SumSigningKey {
                    sk,
                    r1_seed: None,
                    vk0: signing_key.vk0,
                    vk1: signing_key.vk1,
                })),
                None => Ok(None),
            }
        }
    }

    fn gen_key_kes_from_seed_bytes(seed: &[u8]) -> Result<Self::SigningKey, KesMError> {
        // Split seed into r0 and r1 using hash
        let mut hasher = Blake2b512::new();
        hasher.update(&[1u8]);
        hasher.update(seed);
        let r0_hash = hasher.finalize();
        let r0_bytes = &r0_hash[..D::SEED_SIZE];

        let mut hasher = Blake2b512::new();
        hasher.update(&[2u8]);
        hasher.update(seed);
        let r1_hash = hasher.finalize();
        let r1_bytes = &r1_hash[..D::SEED_SIZE];

        // Generate sk_0 from r0
        let sk0 = D::gen_key_kes_from_seed_bytes(r0_bytes)?;
        let vk0 = D::derive_verification_key(&sk0)?;

        // Generate sk_1 from r1 (only to derive vk1, then forget)
        let sk1 = D::gen_key_kes_from_seed_bytes(r1_bytes)?;
        let vk1 = D::derive_verification_key(&sk1)?;
        D::forget_signing_key_kes(sk1);

        // Store r1 in mlocked memory for later
        let mut r1_mlocked = MLockedBytes::new(r1_bytes.len())?;
        r1_mlocked.as_mut_slice().copy_from_slice(r1_bytes);

        Ok(SumSigningKey {
            sk: sk0,
            r1_seed: Some(r1_mlocked),
            vk0,
            vk1,
        })
    }

    fn raw_serialize_verification_key_kes(key: &Self::VerificationKey) -> Vec<u8> {
        key.clone()
    }

    fn raw_deserialize_verification_key_kes(bytes: &[u8]) -> Option<Self::VerificationKey> {
        if bytes.len() == Self::VERIFICATION_KEY_SIZE {
            Some(bytes.to_vec())
        } else {
            None
        }
    }

    fn raw_serialize_signature_kes(signature: &Self::Signature) -> Vec<u8> {
        let mut result = D::raw_serialize_signature_kes(&signature.sigma);
        result.extend_from_slice(&D::raw_serialize_verification_key_kes(&signature.vk0));
        result.extend_from_slice(&D::raw_serialize_verification_key_kes(&signature.vk1));
        result
    }

    fn raw_deserialize_signature_kes(bytes: &[u8]) -> Option<Self::Signature> {
        if bytes.len() != Self::SIGNATURE_SIZE {
            return None;
        }

        let sig_bytes = &bytes[0..D::SIGNATURE_SIZE];
        let vk0_offset = D::SIGNATURE_SIZE;
        let vk1_offset = vk0_offset + D::VERIFICATION_KEY_SIZE;

        let sigma = D::raw_deserialize_signature_kes(sig_bytes)?;
        let vk0 = D::raw_deserialize_verification_key_kes(&bytes[vk0_offset..vk1_offset])?;
        let vk1 = D::raw_deserialize_verification_key_kes(&bytes[vk1_offset..])?;

        Some(SumSignature { sigma, vk0, vk1 })
    }

    fn forget_signing_key_kes(signing_key: Self::SigningKey) {
        D::forget_signing_key_kes(signing_key.sk);
        // r1_seed will be dropped and zeroized automatically
    }
}

// Type aliases for nested compositions
use crate::dsign::ed25519::Ed25519;
use crate::kes::single::SingleKes;

/// Base case: SingleKES wrapping Ed25519
pub type Sum0Kes = SingleKes<Ed25519>;

/// 2^1 = 2 periods
pub type Sum1Kes = SumKes<Sum0Kes>;

/// 2^2 = 4 periods
pub type Sum2Kes = SumKes<Sum1Kes>;

/// 2^3 = 8 periods
pub type Sum3Kes = SumKes<Sum2Kes>;

/// 2^4 = 16 periods
pub type Sum4Kes = SumKes<Sum3Kes>;

/// 2^5 = 32 periods
pub type Sum5Kes = SumKes<Sum4Kes>;

/// 2^6 = 64 periods
pub type Sum6Kes = SumKes<Sum5Kes>;

/// 2^7 = 128 periods (standard Cardano KES)
pub type Sum7Kes = SumKes<Sum6Kes>;
