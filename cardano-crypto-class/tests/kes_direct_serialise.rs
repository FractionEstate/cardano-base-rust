//! Test DirectSerialise/DirectDeserialise for KES SigningKeys
//!
//! This test verifies that KES signing keys can be serialized/deserialized using
//! DirectSerialise. For SingleKES and CompactSingleKES, the SigningKey type is
//! D::MLockedSigningKey which already implements DirectSerialise (e.g., Ed25519MLockedSigningKey).

use cardano_crypto_class::direct_serialise::{DirectDeserialise, DirectSerialise};
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{CompactSingleKes, KesAlgorithm, SingleKes};

type SingleKesEd25519 = SingleKes<Ed25519>;
type CompactSingleKesEd25519 = CompactSingleKes<Ed25519>;

#[test]
fn test_single_kes_signing_key_direct_serialise_roundtrip() {
    // Generate a signing key from a seed
    let seed_bytes = [42u8; 32];
    let sk = SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes)
        .expect("Failed to generate signing key");

    // Serialize the signing key
    let mut serialized = Vec::new();
    let mut push = |ptr: *const u8, len: usize| {
        unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            serialized.extend_from_slice(slice);
        }
        Ok(())
    };
    sk.direct_serialise(&mut push)
        .expect("Failed to serialize signing key");

    // Should serialize to 32 bytes (the seed)
    assert_eq!(
        serialized.len(),
        32,
        "Serialized signing key should be 32 bytes"
    );

    // Deserialize the signing key
    let mut offset = 0;
    let mut pull = |ptr: *mut u8, len: usize| {
        unsafe {
            std::ptr::copy_nonoverlapping(serialized[offset..].as_ptr(), ptr, len);
        }
        offset += len;
        Ok(())
    };

    type SigningKeyType = <SingleKesEd25519 as KesAlgorithm>::SigningKey;
    let sk_restored =
        SigningKeyType::direct_deserialise(&mut pull).expect("Failed to deserialize signing key");

    // Verify both keys produce the same verification key
    let vk1 = SingleKesEd25519::derive_verification_key(&sk).expect("Failed to derive vk1");
    let vk2 =
        SingleKesEd25519::derive_verification_key(&sk_restored).expect("Failed to derive vk2");

    assert_eq!(vk1, vk2, "Verification keys should match after roundtrip");

    // Verify both keys can sign and produce the same signature
    let message = b"Test message for KES DirectSerialise";
    let sig1 = SingleKesEd25519::sign_kes(&(), 0, message, &sk).expect("Failed to sign with sk1");
    let sig2 =
        SingleKesEd25519::sign_kes(&(), 0, message, &sk_restored).expect("Failed to sign with sk2");

    assert_eq!(sig1, sig2, "Signatures should match after roundtrip");

    // Clean up
    SingleKesEd25519::forget_signing_key_kes(sk);
    SingleKesEd25519::forget_signing_key_kes(sk_restored);
}

#[test]
fn test_compact_single_kes_signing_key_direct_serialise_roundtrip() {
    // Generate a signing key from a seed
    let seed_bytes = [123u8; 32];
    let sk = CompactSingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes)
        .expect("Failed to generate signing key");

    // Serialize the signing key
    let mut serialized = Vec::new();
    let mut push = |ptr: *const u8, len: usize| {
        unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            serialized.extend_from_slice(slice);
        }
        Ok(())
    };
    sk.direct_serialise(&mut push)
        .expect("Failed to serialize signing key");

    // Should serialize to 32 bytes (the seed)
    assert_eq!(
        serialized.len(),
        32,
        "Serialized signing key should be 32 bytes"
    );

    // Deserialize the signing key
    let mut offset = 0;
    let mut pull = |ptr: *mut u8, len: usize| {
        unsafe {
            std::ptr::copy_nonoverlapping(serialized[offset..].as_ptr(), ptr, len);
        }
        offset += len;
        Ok(())
    };

    type SigningKeyType = <CompactSingleKesEd25519 as KesAlgorithm>::SigningKey;
    let sk_restored =
        SigningKeyType::direct_deserialise(&mut pull).expect("Failed to deserialize signing key");

    // Verify both keys produce the same verification key
    let vk1 = CompactSingleKesEd25519::derive_verification_key(&sk).expect("Failed to derive vk1");
    let vk2 = CompactSingleKesEd25519::derive_verification_key(&sk_restored)
        .expect("Failed to derive vk2");

    assert_eq!(vk1, vk2, "Verification keys should match after roundtrip");

    // Verify both keys can sign and produce valid signatures
    let message = b"Test message for CompactSingleKES DirectSerialise";
    let sig1 =
        CompactSingleKesEd25519::sign_kes(&(), 0, message, &sk).expect("Failed to sign with sk1");
    let sig2 = CompactSingleKesEd25519::sign_kes(&(), 0, message, &sk_restored)
        .expect("Failed to sign with sk2");

    // For CompactSingleKES, signatures include the verification key,
    // so they should be identical if the keys are the same
    assert_eq!(sig1, sig2, "Signatures should match after roundtrip");

    // Clean up
    CompactSingleKesEd25519::forget_signing_key_kes(sk);
    CompactSingleKesEd25519::forget_signing_key_kes(sk_restored);
}

#[test]
fn test_single_kes_direct_serialise_security() {
    // Verify that DirectSerialise only exposes the seed (32 bytes), not the full key (64 bytes)
    let seed_bytes = [7u8; 32];
    let sk = SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes)
        .expect("Failed to generate signing key");

    let mut serialized = Vec::new();
    let mut push = |ptr: *const u8, len: usize| {
        unsafe {
            let slice = std::slice::from_raw_parts(ptr, len);
            serialized.extend_from_slice(slice);
        }
        Ok(())
    };
    sk.direct_serialise(&mut push)
        .expect("Failed to serialize signing key");

    // CRITICAL: Should only serialize 32 bytes (seed), NOT 64 bytes (seed+pubkey)
    // This prevents unnecessary exposure of key material
    assert_eq!(
        serialized.len(),
        32,
        "DirectSerialise should only expose the 32-byte seed, not the 64-byte compound key"
    );

    SingleKesEd25519::forget_signing_key_kes(sk);
}

#[test]
fn test_multiple_keys_independent() {
    // Verify that multiple keys from different seeds are independent
    let seed1 = [1u8; 32];
    let seed2 = [2u8; 32];

    let sk1 =
        SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed1).expect("Failed to generate sk1");
    let sk2 =
        SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed2).expect("Failed to generate sk2");

    let vk1 = SingleKesEd25519::derive_verification_key(&sk1).expect("Failed to derive vk1");
    let vk2 = SingleKesEd25519::derive_verification_key(&sk2).expect("Failed to derive vk2");

    assert_ne!(vk1, vk2, "Different seeds should produce different keys");

    // Verify signatures are different
    let message = b"Test message";
    let sig1 = SingleKesEd25519::sign_kes(&(), 0, message, &sk1).expect("Failed to sign");
    let sig2 = SingleKesEd25519::sign_kes(&(), 0, message, &sk2).expect("Failed to sign");

    assert_ne!(
        sig1, sig2,
        "Different keys should produce different signatures"
    );

    SingleKesEd25519::forget_signing_key_kes(sk1);
    SingleKesEd25519::forget_signing_key_kes(sk2);
}
