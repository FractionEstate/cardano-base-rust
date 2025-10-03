use std::fmt;

use core::convert::TryFrom;

use ed25519_dalek::{Signature as DalekSignature, SigningKey, VerifyingKey};
use ed25519_dalek::{Signer, Verifier};

use crate::direct_serialise::{DirectDeserialise, DirectResult, DirectSerialise, SizeCheckError};
use crate::dsign::{DsignAlgorithm, DsignError};
use crate::pinned_sized_bytes::PinnedSizedBytes;

pub(crate) const SEED_BYTES: usize = 32;
pub(crate) const VERIFICATION_KEY_BYTES: usize = 32;
pub(crate) const SIGNATURE_BYTES: usize = 64;
pub(crate) const SECRET_COMPOUND_BYTES: usize = 64;

/// Newtype representing an Ed25519 verification key stored as pinned bytes.
#[derive(Clone, PartialEq, Eq)]
pub struct Ed25519VerificationKey(PinnedSizedBytes<VERIFICATION_KEY_BYTES>);

impl fmt::Debug for Ed25519VerificationKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ed25519VerificationKey({})",
            hex::encode(self.0.as_bytes())
        )
    }
}

impl Ed25519VerificationKey {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != VERIFICATION_KEY_BYTES {
            return None;
        }
        let mut array = [0u8; VERIFICATION_KEY_BYTES];
        array.copy_from_slice(bytes);
        VerifyingKey::from_bytes(&array).ok()?;
        Some(Self(PinnedSizedBytes::from_array(array)))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; VERIFICATION_KEY_BYTES] {
        self.0.as_bytes()
    }
}

impl DirectSerialise for Ed25519VerificationKey {
    fn direct_serialise(
        &self,
        push: &mut dyn FnMut(*const u8, usize) -> DirectResult<()>,
    ) -> DirectResult<()> {
        self.0.with_c_ptr(|ptr| push(ptr, VERIFICATION_KEY_BYTES))
    }
}

impl DirectDeserialise for Ed25519VerificationKey {
    fn direct_deserialise(
        pull: &mut dyn FnMut(*mut u8, usize) -> DirectResult<()>,
    ) -> DirectResult<Self> {
        let (bytes, result) = PinnedSizedBytes::<VERIFICATION_KEY_BYTES>::create_result(|ptr| {
            pull(ptr, VERIFICATION_KEY_BYTES)
        });
        result?;
        Ed25519VerificationKey::from_bytes(bytes.as_bytes()).ok_or_else(|| SizeCheckError {
            expected_size: VERIFICATION_KEY_BYTES,
            actual_size: VERIFICATION_KEY_BYTES,
        })
    }
}

/// Compound signing key stored as the 64-byte libsodium-style secret structure.
#[derive(Clone, PartialEq, Eq)]
pub struct Ed25519SigningKey(PinnedSizedBytes<SECRET_COMPOUND_BYTES>);

impl fmt::Debug for Ed25519SigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519SigningKey(..)")
    }
}

impl Ed25519SigningKey {
    pub(crate) fn from_seed_bytes(seed: &[u8]) -> Self {
        let mut seed_array = [0u8; SEED_BYTES];
        seed_array.copy_from_slice(seed);
        let signing_key = SigningKey::from_bytes(&seed_array);
        let verifying_key = signing_key.verifying_key();
        let mut compound = [0u8; SECRET_COMPOUND_BYTES];
        compound[..SEED_BYTES].copy_from_slice(&seed_array);
        compound[SEED_BYTES..].copy_from_slice(&verifying_key.to_bytes());
        Self(PinnedSizedBytes::from_array(compound))
    }

    pub(crate) fn seed_bytes(&self) -> [u8; SEED_BYTES] {
        let mut seed = [0u8; SEED_BYTES];
        seed.copy_from_slice(&self.0.as_bytes()[..SEED_BYTES]);
        seed
    }

    pub(crate) fn verifying_bytes(&self) -> [u8; VERIFICATION_KEY_BYTES] {
        let mut vk = [0u8; VERIFICATION_KEY_BYTES];
        vk.copy_from_slice(&self.0.as_bytes()[SEED_BYTES..]);
        vk
    }

    fn signing_key(&self) -> SigningKey {
        let seed = self.seed_bytes();
        SigningKey::from_bytes(&seed)
    }

    pub(crate) fn compound_bytes(&self) -> &[u8; SECRET_COMPOUND_BYTES] {
        self.0.as_bytes()
    }
}

/// Ed25519 signature stored as pinned bytes.
#[derive(Clone, PartialEq, Eq)]
pub struct Ed25519Signature(PinnedSizedBytes<SIGNATURE_BYTES>);

impl fmt::Debug for Ed25519Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519Signature({})", hex::encode(self.0.as_bytes()))
    }
}

impl Ed25519Signature {
    pub(crate) fn from_dalek(signature: &DalekSignature) -> Self {
        Self(PinnedSizedBytes::from_array(signature.to_bytes()))
    }

    pub(crate) fn as_bytes(&self) -> &[u8; SIGNATURE_BYTES] {
        self.0.as_bytes()
    }
}

/// Marker type implementing [`DsignAlgorithm`] for Ed25519.
pub struct Ed25519;

impl DsignAlgorithm for Ed25519 {
    type SigningKey = Ed25519SigningKey;
    type VerificationKey = Ed25519VerificationKey;
    type Signature = Ed25519Signature;
    type Context = ();

    const ALGORITHM_NAME: &'static str = "ed25519";
    const SEED_SIZE: usize = SEED_BYTES;
    const VERIFICATION_KEY_SIZE: usize = VERIFICATION_KEY_BYTES;
    const SIGNING_KEY_SIZE: usize = SEED_BYTES;
    const SIGNATURE_SIZE: usize = SIGNATURE_BYTES;

    fn derive_verification_key(signing_key: &Self::SigningKey) -> Self::VerificationKey {
        let mut bytes = [0u8; VERIFICATION_KEY_BYTES];
        bytes.copy_from_slice(&signing_key.verifying_bytes());
        Ed25519VerificationKey(PinnedSizedBytes::from_array(bytes))
    }

    fn sign_bytes(
        _context: &Self::Context,
        message: &[u8],
        signing_key: &Self::SigningKey,
    ) -> Self::Signature {
        let signing_key = signing_key.signing_key();
        let signature = signing_key.sign(message);
        Ed25519Signature::from_dalek(&signature)
    }

    fn verify_bytes(
        _context: &Self::Context,
        verification_key: &Self::VerificationKey,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<(), DsignError> {
        let verifying_key = VerifyingKey::from_bytes(verification_key.as_bytes())
            .map_err(|err| DsignError::Message(err.to_string()))?;
        let signature = DalekSignature::try_from(signature.as_bytes().as_ref())
            .map_err(|err| DsignError::Message(err.to_string()))?;
        verifying_key
            .verify(message, &signature)
            .map_err(|_| DsignError::VerificationFailed)
    }

    fn gen_key_from_seed_bytes(seed: &[u8]) -> Self::SigningKey {
        assert_eq!(seed.len(), SEED_BYTES, "invalid seed length");
        Ed25519SigningKey::from_seed_bytes(seed)
    }

    fn raw_serialize_verification_key(key: &Self::VerificationKey) -> Vec<u8> {
        key.as_bytes().to_vec()
    }

    fn raw_deserialize_verification_key(bytes: &[u8]) -> Option<Self::VerificationKey> {
        Ed25519VerificationKey::from_bytes(bytes)
    }

    fn raw_serialize_signing_key(signing_key: &Self::SigningKey) -> Vec<u8> {
        signing_key.seed_bytes().to_vec()
    }

    fn raw_deserialize_signing_key(bytes: &[u8]) -> Option<Self::SigningKey> {
        if bytes.len() != SEED_BYTES {
            return None;
        }
        Some(Ed25519SigningKey::from_seed_bytes(bytes))
    }

    fn raw_serialize_signature(signature: &Self::Signature) -> Vec<u8> {
        signature.as_bytes().to_vec()
    }

    fn raw_deserialize_signature(bytes: &[u8]) -> Option<Self::Signature> {
        if bytes.len() != SIGNATURE_BYTES {
            return None;
        }
        let mut array = [0u8; SIGNATURE_BYTES];
        array.copy_from_slice(bytes);
        DalekSignature::try_from(array.as_ref())
            .ok()
            .map(|sig| Ed25519Signature::from_dalek(&sig))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dsign::{signed_dsign, verify_signed_dsign, DsignError};
    use crate::seed::{mk_seed_from_bytes, Seed};

    #[test]
    fn key_generation_is_deterministic() {
        let seed_bytes = [7u8; SEED_BYTES];
        let seed = Seed::from_bytes(seed_bytes.to_vec());
        let signing = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let signing_again = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        assert_eq!(signing.0.as_bytes(), signing_again.0.as_bytes());
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let seed = mk_seed_from_bytes(vec![42u8; SEED_BYTES]);
        let signing = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let verifying = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing);
        let message = b"cardano";
        let signed = signed_dsign::<Ed25519, _>(&(), message, &signing);
        assert!(verify_signed_dsign::<Ed25519, _>(&(), &verifying, message, &signed).is_ok());
    }

    #[test]
    fn raw_serialise_roundtrip() {
        let seed = mk_seed_from_bytes(vec![1u8; SEED_BYTES]);
        let signing = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let verifying = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing);
        let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), b"msg", &signing);

        let vk_raw = <Ed25519 as DsignAlgorithm>::raw_serialize_verification_key(&verifying);
        let sk_raw = <Ed25519 as DsignAlgorithm>::raw_serialize_signing_key(&signing);
        let sig_raw = <Ed25519 as DsignAlgorithm>::raw_serialize_signature(&signature);

        assert!(<Ed25519 as DsignAlgorithm>::raw_deserialize_verification_key(&vk_raw).is_some());
        assert!(<Ed25519 as DsignAlgorithm>::raw_deserialize_signing_key(&sk_raw).is_some());
        assert!(<Ed25519 as DsignAlgorithm>::raw_deserialize_signature(&sig_raw).is_some());
    }

    #[test]
    fn verify_fails_for_wrong_message() {
        let seed = mk_seed_from_bytes(vec![9u8; SEED_BYTES]);
        let signing = <Ed25519 as DsignAlgorithm>::gen_key(&seed);
        let verifying = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing);
        let signed = signed_dsign::<Ed25519, _>(&(), b"hello", &signing);
        let result = verify_signed_dsign::<Ed25519, _>(&(), &verifying, b"world", &signed);
        assert!(matches!(result, Err(DsignError::VerificationFailed)));
    }
}
