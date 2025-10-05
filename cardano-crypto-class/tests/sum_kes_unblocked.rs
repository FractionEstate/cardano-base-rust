// Test for Sum KES types with gen_key_kes_from_seed_bytes
// This verifies that Sum KES is now unblocked and can generate keys from seed bytes

use cardano_crypto_class::kes::{KesAlgorithm, Sum1Kes, Sum2Kes};

#[test]
fn test_sum_kes_1_gen_key_from_seed_bytes() {
    // Test that Sum1Kes (2 periods) can generate keys from seed bytes
    let seed_bytes = vec![0u8; Sum1Kes::SEED_SIZE];

    let signing_key = Sum1Kes::gen_key_kes_from_seed_bytes(&seed_bytes);
    assert!(
        signing_key.is_ok(),
        "Sum1Kes should be able to generate key from seed bytes: {:?}",
        signing_key.err()
    );

    // Verify we can derive a verification key
    let sk = signing_key.unwrap();
    let vk = Sum1Kes::derive_verification_key(&sk);
    assert!(vk.is_ok(), "derive_verification_key should succeed");
}

#[test]
fn test_sum_kes_2_gen_key_from_seed_bytes() {
    // Test that Sum2Kes (4 periods) can generate keys from seed bytes
    let seed_bytes = vec![0u8; Sum2Kes::SEED_SIZE];

    let signing_key = Sum2Kes::gen_key_kes_from_seed_bytes(&seed_bytes);
    assert!(
        signing_key.is_ok(),
        "Sum2Kes should be able to generate key from seed bytes: {:?}",
        signing_key.err()
    );

    let sk = signing_key.unwrap();
    let vk = Sum2Kes::derive_verification_key(&sk);
    assert!(vk.is_ok(), "derive_verification_key should succeed");
}

#[test]
fn test_sum_kes_can_sign_at_different_periods() {
    // Verify that Sum KES keys can sign at different periods
    let seed_bytes = vec![42u8; Sum2Kes::SEED_SIZE];
    let message = b"Sum KES test message";

    let sk = Sum2Kes::gen_key_kes_from_seed_bytes(&seed_bytes).unwrap();
    let vk = Sum2Kes::derive_verification_key(&sk).unwrap();

    // Sign at period 0
    let sig0 = Sum2Kes::sign_kes(&(), 0, message, &sk).unwrap();
    assert!(
        Sum2Kes::verify_kes(&(), &vk, 0, message, &sig0).is_ok(),
        "Signature at period 0 should verify"
    );

    // Evolve to period 1
    let sk1 = Sum2Kes::update_kes(&(), sk, 0).unwrap().unwrap();
    let sig1 = Sum2Kes::sign_kes(&(), 1, message, &sk1).unwrap();
    assert!(
        Sum2Kes::verify_kes(&(), &vk, 1, message, &sig1).is_ok(),
        "Signature at period 1 should verify"
    );
}

#[test]
fn test_sum_kes_deterministic_generation() {
    // Verify that Sum KES generates deterministic keys from same seed
    let seed_bytes = vec![123u8; Sum1Kes::SEED_SIZE];

    let sk1 = Sum1Kes::gen_key_kes_from_seed_bytes(&seed_bytes).unwrap();
    let sk2 = Sum1Kes::gen_key_kes_from_seed_bytes(&seed_bytes).unwrap();

    let vk1 = Sum1Kes::derive_verification_key(&sk1).unwrap();
    let vk2 = Sum1Kes::derive_verification_key(&sk2).unwrap();

    assert_eq!(vk1, vk2, "Same seed should produce same verification key");
}
