// Test for gen_key_kes_from_seed_bytes functionality
// This test verifies that the recently implemented gen_key_kes_from_seed_bytes
// method works correctly for SingleKes and CompactSingleKes

use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{CompactSingleKes, KesAlgorithm, SingleKes};

type SingleKesEd25519 = SingleKes<Ed25519>;
type CompactSingleKesEd25519 = CompactSingleKes<Ed25519>;

#[test]
fn test_single_kes_gen_key_from_seed_bytes() {
    // Test that gen_key_kes_from_seed_bytes works for SingleKes
    let seed_bytes = [0u8; 32]; // All zeros seed

    // This should no longer return an error
    let signing_key = SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes);
    assert!(
        signing_key.is_ok(),
        "gen_key_kes_from_seed_bytes should succeed"
    );

    // Verify we can derive a verification key
    let sk = signing_key.unwrap();
    let vk = SingleKesEd25519::derive_verification_key(&sk);
    assert!(vk.is_ok(), "derive_verification_key should succeed");
}

#[test]
fn test_compact_single_kes_gen_key_from_seed_bytes() {
    // Test that gen_key_kes_from_seed_bytes works for CompactSingleKes
    let seed_bytes = [0u8; 32]; // All zeros seed

    // This should no longer return an error
    let signing_key = CompactSingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes);
    assert!(
        signing_key.is_ok(),
        "gen_key_kes_from_seed_bytes should succeed"
    );

    // Verify we can derive a verification key
    let sk = signing_key.unwrap();
    let vk = CompactSingleKesEd25519::derive_verification_key(&sk);
    assert!(vk.is_ok(), "derive_verification_key should succeed");
}

#[test]
fn test_gen_key_from_seed_bytes_deterministic() {
    // Verify that the same seed produces the same keys
    let seed_bytes = [42u8; 32];

    let sk1 = SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes).unwrap();
    let sk2 = SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes).unwrap();

    let vk1 = SingleKesEd25519::derive_verification_key(&sk1).unwrap();
    let vk2 = SingleKesEd25519::derive_verification_key(&sk2).unwrap();

    // Verification keys should be identical
    assert_eq!(vk1, vk2, "Same seed should produce same verification key");
}

#[test]
fn test_gen_key_from_seed_bytes_can_sign() {
    // Verify that keys generated from seed bytes can actually sign
    let seed_bytes = [123u8; 32];
    let message = b"Hello, KES!";

    let sk = SingleKesEd25519::gen_key_kes_from_seed_bytes(&seed_bytes).unwrap();
    let vk = SingleKesEd25519::derive_verification_key(&sk).unwrap();

    // Sign a message at period 0
    let signature = SingleKesEd25519::sign_kes(&(), 0, message, &sk);
    assert!(signature.is_ok(), "Signing should succeed");

    // Verify the signature
    let sig = signature.unwrap();
    let verification = SingleKesEd25519::verify_kes(&(), &vk, 0, message, &sig);
    assert!(verification.is_ok(), "Verification should succeed");
}

#[test]
fn test_gen_key_from_seed_bytes_wrong_length() {
    // Test that providing wrong seed length returns an error
    let short_seed = [0u8; 16]; // Too short

    let result = SingleKesEd25519::gen_key_kes_from_seed_bytes(&short_seed);
    assert!(result.is_err(), "Should fail with wrong seed length");
}
