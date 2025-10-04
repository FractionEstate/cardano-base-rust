use std::marker::PhantomData;

use blake2::{Blake2b512, Digest};

use crate::kes::compact_single::OptimizedKesSignature;
use crate::kes::{KesAlgorithm, KesError, KesMError, Period};
use crate::mlocked_bytes::MLockedBytes;
use crate::seed::Seed;

/// CompactSumKES is an optimized version of SumKES that stores fewer verification keys.
///
/// The key insight is that in a Merkle tree structure, each branch node only needs
/// to store ONE verification key instead of TWO. The signature embeds the "off-side"
/// verification key, and we can reconstruct the "on-side" key from the signature.
///
/// For example, in a tree with current period at leaf E:
/// ```text
///       (A)
///      /   \
///   (B)     (C)
///   / \     / \
/// (D) (E) (F) (G)
///      ^
///  0   1   2   3
/// ```
///
/// The signature for E contains its DSIGN key. The signature for B contains E's
/// signature and D's VerKey. The signature for A contains B's signature and C's VerKey.
///
/// This reduces storage from depth*2 keys to just depth keys.
pub struct CompactSumKes<D>(PhantomData<D>)
where
    D: KesAlgorithm,
    D::Signature: OptimizedKesSignature;

/// Signing key for CompactSumKES.
pub struct CompactSumSigningKey<D>
where
    D: KesAlgorithm,
    D::Signature: OptimizedKesSignature,
{
    pub(crate) sk: D::SigningKey,
    pub(crate) r1_seed: Option<MLockedBytes>,
    pub(crate) vk0: D::VerificationKey,
    pub(crate) vk1: D::VerificationKey,
}

/// Signature for CompactSumKES - only stores the "other" verification key.
#[derive(Clone)]
pub struct CompactSumSignature<D>
where
    D: KesAlgorithm,
    D::Signature: OptimizedKesSignature,
{
    /// Signature from the active subtree (contains embedded vk)
    pub(crate) sigma: D::Signature,
    /// Verification key for the inactive subtree
    pub(crate) vk_other: D::VerificationKey,
}

impl<D> KesAlgorithm for CompactSumKes<D>
where
    D: KesAlgorithm,
    D::VerificationKey: Clone,
    D::Signature: OptimizedKesSignature<VerificationKey = D::VerificationKey> + Clone,
{
    type VerificationKey = Vec<u8>; // Hash of (vk0, vk1)
    type SigningKey = CompactSumSigningKey<D>;
    type Signature = CompactSumSignature<D>;
    type Context = D::Context;

    const ALGORITHM_NAME: &'static str = D::ALGORITHM_NAME; // Could append "_compact"
    const SEED_SIZE: usize = D::SEED_SIZE;
    const VERIFICATION_KEY_SIZE: usize = 64; // Blake2b-512 output
    const SIGNING_KEY_SIZE: usize =
        D::SIGNING_KEY_SIZE + D::SEED_SIZE + 2 * D::VERIFICATION_KEY_SIZE;
    // Compact signature: constituent signature + only ONE verification key
    const SIGNATURE_SIZE: usize = D::SIGNATURE_SIZE + D::VERIFICATION_KEY_SIZE;

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

        let (sigma, vk_other) = if period < t_half {
            // Use left subtree, store right vk
            let sig = D::sign_kes(context, period, message, &signing_key.sk)?;
            (sig, signing_key.vk1.clone())
        } else {
            // Use right subtree, store left vk
            let sig = D::sign_kes(context, period - t_half, message, &signing_key.sk)?;
            (sig, signing_key.vk0.clone())
        };

        Ok(CompactSumSignature { sigma, vk_other })
    }

    fn verify_kes(
        context: &Self::Context,
        verification_key: &Self::VerificationKey,
        period: Period,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<(), KesError> {
        let t_half = D::total_periods();

        // Extract the "on-side" verification key from the signature
        let vk_active = signature.sigma.extract_verification_key();

        // Reconstruct both vk0 and vk1
        let (vk0, vk1) = if period < t_half {
            // Active is left, other is right
            (vk_active.clone(), signature.vk_other.clone())
        } else {
            // Active is right, other is left
            (signature.vk_other.clone(), vk_active.clone())
        };

        // Verify that H(vk0 || vk1) matches the provided verification key
        let mut hasher = Blake2b512::new();
        hasher.update(D::raw_serialize_verification_key_kes(&vk0));
        hasher.update(D::raw_serialize_verification_key_kes(&vk1));
        let computed_vk = hasher.finalize().to_vec();

        if &computed_vk != verification_key {
            return Err(KesError::VerificationFailed);
        }

        // Verify the signature against the active verification key
        if period < t_half {
            D::verify_kes(context, &vk0, period, message, &signature.sigma)
        } else {
            D::verify_kes(context, &vk1, period - t_half, message, &signature.sigma)
        }
    }

    fn update_kes(
        context: &Self::Context,
        mut signing_key: Self::SigningKey,
        period: Period,
    ) -> Result<Option<Self::SigningKey>, KesMError> {
        let t_half = D::total_periods();

        if period + 1 >= 2 * t_half {
            D::forget_signing_key_kes(signing_key.sk);
            return Ok(None);
        }

        if period + 1 == t_half {
            // Transition from left to right subtree
            let r1_seed = signing_key
                .r1_seed
                .take()
                .ok_or(KesMError::Kes(KesError::KeyExpired))?;

            let seed = Seed::from_bytes(r1_seed.as_slice());
            let sk1 = D::gen_key_kes(&seed)?;

            D::forget_signing_key_kes(signing_key.sk);

            Ok(Some(CompactSumSigningKey {
                sk: sk1,
                r1_seed: None,
                vk0: signing_key.vk0,
                vk1: signing_key.vk1,
            }))
        } else if period + 1 < t_half {
            // Still in left subtree
            let updated_sk = D::update_kes(context, signing_key.sk, period)?;
            match updated_sk {
                Some(sk) => Ok(Some(CompactSumSigningKey {
                    sk,
                    r1_seed: signing_key.r1_seed,
                    vk0: signing_key.vk0,
                    vk1: signing_key.vk1,
                })),
                None => Ok(None),
            }
        } else {
            // In right subtree
            let adjusted_period = period - t_half;
            let updated_sk = D::update_kes(context, signing_key.sk, adjusted_period)?;
            match updated_sk {
                Some(sk) => Ok(Some(CompactSumSigningKey {
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

        // Store r1 in mlocked memory
        let mut r1_mlocked = MLockedBytes::new(r1_bytes.len())?;
        r1_mlocked.as_mut_slice().copy_from_slice(r1_bytes);

        Ok(CompactSumSigningKey {
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
        result.extend_from_slice(&D::raw_serialize_verification_key_kes(&signature.vk_other));
        result
    }

    fn raw_deserialize_signature_kes(bytes: &[u8]) -> Option<Self::Signature> {
        if bytes.len() != Self::SIGNATURE_SIZE {
            return None;
        }

        let sig_bytes = &bytes[0..D::SIGNATURE_SIZE];
        let vk_bytes = &bytes[D::SIGNATURE_SIZE..];

        let sigma = D::raw_deserialize_signature_kes(sig_bytes)?;
        let vk_other = D::raw_deserialize_verification_key_kes(vk_bytes)?;

        Some(CompactSumSignature { sigma, vk_other })
    }

    fn forget_signing_key_kes(signing_key: Self::SigningKey) {
        D::forget_signing_key_kes(signing_key.sk);
    }
}

// Type aliases for nested CompactSum compositions
use crate::dsign::ed25519::Ed25519;
use crate::kes::compact_single::CompactSingleKes;

/// Base case: CompactSingleKES wrapping Ed25519
pub type CompactSum0Kes = CompactSingleKes<Ed25519>;

/// 2^1 = 2 periods (compact)
pub type CompactSum1Kes = CompactSumKes<CompactSum0Kes>;

/// 2^2 = 4 periods (compact)
pub type CompactSum2Kes = CompactSumKes<CompactSum1Kes>;

/// 2^3 = 8 periods (compact)
pub type CompactSum3Kes = CompactSumKes<CompactSum2Kes>;

/// 2^4 = 16 periods (compact)
pub type CompactSum4Kes = CompactSumKes<CompactSum3Kes>;

/// 2^5 = 32 periods (compact)
pub type CompactSum5Kes = CompactSumKes<CompactSum4Kes>;

/// 2^6 = 64 periods (compact)
pub type CompactSum6Kes = CompactSumKes<CompactSum5Kes>;

/// 2^7 = 128 periods (compact, standard Cardano KES)
pub type CompactSum7Kes = CompactSumKes<CompactSum6Kes>;
