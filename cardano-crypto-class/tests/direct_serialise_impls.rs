// Test DirectSerialise implementations for zero-copy serialization
// These tests verify that DirectSerialise provides correct and efficient serialization

use cardano_crypto_class::direct_serialise::{direct_deserialise_buf, direct_serialise_buf};
use cardano_crypto_class::dsign::ed25519::{Ed25519, Ed25519Signature, Ed25519VerificationKey};
use cardano_crypto_class::dsign::{DsignAlgorithm, DsignMAlgorithm};
use cardano_crypto_class::mlocked_seed::MLockedSeed;
use cardano_crypto_class::vrf::VRFAlgorithm;
use cardano_crypto_class::vrf::praos::{PraosProof, PraosSeed, PraosVRF, PraosVerificationKey};

#[test]
fn test_ed25519_signature_direct_serialise_roundtrip() {
    // Generate a signature
    let mut seed = MLockedSeed::<32>::new_zeroed().expect("zeroed seed allocation must succeed");
    seed.as_mut_bytes().copy_from_slice(&[42u8; 32]);

    let sk = Ed25519::gen_key_m(&seed).expect("Ed25519 key generation must succeed");
    let message = b"DirectSerialise test message";
    let signature = Ed25519::sign_bytes_m(&(), message, &sk).expect("Ed25519 signing must succeed");

    // Serialize using DirectSerialise
    let mut buffer = vec![0u8; Ed25519::SIGNATURE_SIZE];
    let written = direct_serialise_buf(&mut buffer, &signature)
        .expect("DirectSerialise should serialize signature");

    assert_eq!(
        written,
        Ed25519::SIGNATURE_SIZE,
        "Should write exactly SIGNATURE_SIZE bytes"
    );

    // Deserialize using DirectDeserialise
    let (deserialized, read): (Ed25519Signature, usize) =
        direct_deserialise_buf(&buffer).expect("DirectDeserialise should deserialize signature");

    assert_eq!(
        read,
        Ed25519::SIGNATURE_SIZE,
        "Should read exactly SIGNATURE_SIZE bytes"
    );
    assert_eq!(
        deserialized, signature,
        "Deserialized signature should match original"
    );
}

#[test]
fn test_ed25519_verification_key_direct_serialise_roundtrip() {
    // Generate a verification key
    let mut seed = MLockedSeed::<32>::new_zeroed().expect("zeroed seed allocation must succeed");
    seed.as_mut_bytes().copy_from_slice(&[123u8; 32]);

    let sk = Ed25519::gen_key_m(&seed).expect("Ed25519 key generation must succeed");
    let vk = Ed25519::derive_verification_key_m(&sk)
        .expect("Ed25519 verification key derivation must succeed");

    // Serialize using DirectSerialise
    let mut buffer = vec![0u8; Ed25519::VERIFICATION_KEY_SIZE];
    let written = direct_serialise_buf(&mut buffer, &vk)
        .expect("DirectSerialise should serialize verification key");

    assert_eq!(
        written,
        Ed25519::VERIFICATION_KEY_SIZE,
        "Should write exactly VERIFICATION_KEY_SIZE bytes"
    );

    // Deserialize using DirectDeserialise
    let (deserialized, read): (Ed25519VerificationKey, usize) = direct_deserialise_buf(&buffer)
        .expect("DirectDeserialise should deserialize verification key");

    assert_eq!(
        read,
        Ed25519::VERIFICATION_KEY_SIZE,
        "Should read exactly VERIFICATION_KEY_SIZE bytes"
    );
    assert_eq!(deserialized, vk, "Deserialized VK should match original");
}

#[test]
fn test_direct_serialise_signature_can_verify() {
    // Ensure DirectSerialise roundtrip preserves signature validity
    let mut seed = MLockedSeed::<32>::new_zeroed().expect("zeroed seed allocation must succeed");
    seed.as_mut_bytes().copy_from_slice(&[99u8; 32]);

    let sk = Ed25519::gen_key_m(&seed).expect("Ed25519 key generation must succeed");
    let vk = Ed25519::derive_verification_key_m(&sk)
        .expect("Ed25519 verification key derivation must succeed");
    let message = b"Verification test";
    let signature = Ed25519::sign_bytes_m(&(), message, &sk).expect("Ed25519 signing must succeed");

    // Serialize and deserialize signature
    let mut buffer = vec![0u8; Ed25519::SIGNATURE_SIZE];
    direct_serialise_buf(&mut buffer, &signature)
        .expect("DirectSerialise should serialize signature");

    let (deserialized_sig, _): (Ed25519Signature, usize) =
        direct_deserialise_buf(&buffer).expect("DirectDeserialise should deserialize signature");

    // Verify the deserialized signature
    let verification = Ed25519::verify_bytes(&(), &vk, message, &deserialized_sig);
    assert!(
        verification.is_ok(),
        "Deserialized signature should verify successfully"
    );
}

#[test]
fn test_direct_serialise_deterministic() {
    // Verify that DirectSerialise is deterministic
    let mut seed = MLockedSeed::<32>::new_zeroed().expect("zeroed seed allocation must succeed");
    seed.as_mut_bytes().copy_from_slice(&[77u8; 32]);

    let sk = Ed25519::gen_key_m(&seed).expect("Ed25519 key generation must succeed");
    let message = b"Deterministic test";
    let signature = Ed25519::sign_bytes_m(&(), message, &sk).expect("Ed25519 signing must succeed");

    // Serialize twice
    let mut buffer1 = vec![0u8; Ed25519::SIGNATURE_SIZE];
    let mut buffer2 = vec![0u8; Ed25519::SIGNATURE_SIZE];

    direct_serialise_buf(&mut buffer1, &signature)
        .expect("DirectSerialise should serialize signature");
    direct_serialise_buf(&mut buffer2, &signature)
        .expect("DirectSerialise should serialize signature");

    assert_eq!(buffer1, buffer2, "DirectSerialise should be deterministic");
}

#[test]
fn test_direct_serialise_buffer_too_small() {
    // Test error handling when buffer is too small
    let mut seed = MLockedSeed::<32>::new_zeroed().expect("zeroed seed allocation must succeed");
    seed.as_mut_bytes().copy_from_slice(&[1u8; 32]);

    let sk = Ed25519::gen_key_m(&seed).expect("Ed25519 key generation must succeed");
    let message = b"Buffer size test";
    let signature = Ed25519::sign_bytes_m(&(), message, &sk).expect("Ed25519 signing must succeed");

    // Try to serialize into a buffer that's too small
    let mut buffer = vec![0u8; Ed25519::SIGNATURE_SIZE - 1];
    let result = direct_serialise_buf(&mut buffer, &signature);

    assert!(result.is_err(), "Should fail with buffer too small");
}

// VRF DirectSerialise tests

#[test]
fn test_praos_verification_key_direct_serialise_roundtrip() {
    // Generate Praos VRF key from seed
    let seed = PraosSeed::generate().expect("Praos seed generation must succeed");
    let sk = PraosVRF::gen_key_from_seed_bytes(seed.as_bytes());
    let vk = PraosVRF::derive_verification_key(&sk);

    // Serialize verification key using DirectSerialise
    let mut buffer = vec![0u8; PraosVRF::VERIFICATION_KEY_SIZE];
    let written = direct_serialise_buf(&mut buffer, &vk)
        .expect("DirectSerialise should serialize verification key");

    assert_eq!(
        written,
        PraosVRF::VERIFICATION_KEY_SIZE,
        "Should write exactly VERIFICATION_KEY_SIZE bytes"
    );

    // Deserialize using DirectDeserialise
    let (deserialized, read): (PraosVerificationKey, usize) = direct_deserialise_buf(&buffer)
        .expect("DirectDeserialise should deserialize verification key");

    assert_eq!(
        read,
        PraosVRF::VERIFICATION_KEY_SIZE,
        "Should read exactly VERIFICATION_KEY_SIZE bytes"
    );
    assert_eq!(deserialized, vk, "Deserialized VK should match original");
}

#[test]
fn test_praos_proof_direct_serialise_roundtrip() {
    // Generate Praos proof
    let seed = PraosSeed::generate().expect("Praos seed generation must succeed");
    let sk = PraosVRF::gen_key_from_seed_bytes(seed.as_bytes());
    let message = b"VRF test message";
    let (_output, proof) = PraosVRF::evaluate_bytes(&(), message, &sk);

    // Serialize proof using DirectSerialise
    let mut buffer = vec![0u8; PraosVRF::PROOF_SIZE];
    let written =
        direct_serialise_buf(&mut buffer, &proof).expect("DirectSerialise should serialize proof");

    assert_eq!(
        written,
        PraosVRF::PROOF_SIZE,
        "Should write exactly PROOF_SIZE bytes"
    );

    // Deserialize using DirectDeserialise
    let (deserialized, read): (PraosProof, usize) =
        direct_deserialise_buf(&buffer).expect("DirectDeserialise should deserialize proof");

    assert_eq!(
        read,
        PraosVRF::PROOF_SIZE,
        "Should read exactly PROOF_SIZE bytes"
    );
    assert_eq!(
        deserialized, proof,
        "Deserialized proof should match original"
    );
}

#[test]
fn test_praos_proof_direct_serialise_can_verify() {
    // Ensure DirectSerialise roundtrip preserves proof validity
    let seed = PraosSeed::generate().expect("Praos seed generation must succeed");
    let sk = PraosVRF::gen_key_from_seed_bytes(seed.as_bytes());
    let vk = PraosVRF::derive_verification_key(&sk);
    let message = b"Verification test message";
    let (expected_output, proof) = PraosVRF::evaluate_bytes(&(), message, &sk);

    // Serialize and deserialize proof
    let mut buffer = vec![0u8; PraosVRF::PROOF_SIZE];
    direct_serialise_buf(&mut buffer, &proof).expect("DirectSerialise should serialize proof");

    let (deserialized_proof, _): (PraosProof, usize) =
        direct_deserialise_buf(&buffer).expect("DirectDeserialise should deserialize proof");

    // Verify the deserialized proof
    let verified_output = PraosVRF::verify_bytes(&(), &vk, message, &deserialized_proof);
    assert!(
        verified_output.is_some(),
        "Deserialized proof should verify successfully"
    );
    assert_eq!(
        verified_output.expect("Verified output should be present"),
        expected_output,
        "Output should match"
    );
}

#[test]
fn test_praos_direct_serialise_deterministic() {
    // Verify that DirectSerialise is deterministic for VRF types
    let seed = PraosSeed::generate().expect("Praos seed generation must succeed");
    let sk = PraosVRF::gen_key_from_seed_bytes(seed.as_bytes());
    let message = b"Deterministic test";
    let (_output, proof) = PraosVRF::evaluate_bytes(&(), message, &sk);

    // Serialize twice
    let mut buffer1 = vec![0u8; PraosVRF::PROOF_SIZE];
    let mut buffer2 = vec![0u8; PraosVRF::PROOF_SIZE];

    direct_serialise_buf(&mut buffer1, &proof).expect("DirectSerialise should serialize proof");
    direct_serialise_buf(&mut buffer2, &proof).expect("DirectSerialise should serialize proof");

    assert_eq!(buffer1, buffer2, "DirectSerialise should be deterministic");
}
