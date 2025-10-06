//! Test vectors for Schnorr Secp256k1 DSIGN implementation
//!
//! This module loads test vectors from the cardano-test-vectors crate
//! and validates the Schnorr Secp256k1 (BIP340) implementation against them.

use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::schnorr_secp256k1::SchnorrSecp256k1DSIGN;
use cardano_crypto_class::seed::mk_seed_from_bytes;
use cardano_test_vectors::dsign;
use serde_json::Value;

/// Helper to decode hex strings
fn decode_hex(s: &str) -> Vec<u8> {
    hex::decode(s).expect("valid hex string")
}

/// Parse the Schnorr Secp256k1 test vectors JSON
fn parse_schnorr_vectors() -> Value {
    let json = dsign::get("schnorr_secp256k1_test_vectors.json")
        .expect("Schnorr Secp256k1 test vectors should be available");
    serde_json::from_str(json).expect("Should parse Schnorr Secp256k1 test vectors JSON")
}

#[test]
fn test_schnorr_vectors_exist() {
    let json = dsign::get("schnorr_secp256k1_test_vectors.json")
        .expect("Schnorr Secp256k1 test vectors should be available");
    assert!(!json.is_empty(), "Schnorr test vectors should not be empty");
}

#[test]
fn test_schnorr_vectors_parse() {
    let vectors = parse_schnorr_vectors();

    assert_eq!(
        vectors["algorithm"].as_str().unwrap(),
        "SchnorrSecp256k1DSIGN"
    );

    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();
    assert!(
        !sign_verify.is_empty(),
        "Should have sign/verify test vectors"
    );

    println!(
        "Loaded {} Schnorr sign/verify test vectors",
        sign_verify.len()
    );
}

#[test]
fn test_schnorr_key_generation_from_seed() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    for vector in sign_verify {
        let test_name = vector["test_name"].as_str().unwrap();
        println!("\n=== Testing Key Generation: {} ===", test_name);

        // Generate keys from secret key
        let sk_hex = vector["secret_key"].as_str().unwrap();
        let sk_bytes = decode_hex(sk_hex);
        let seed = mk_seed_from_bytes(sk_bytes);
        let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);
        let verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key);

        // Serialize verification key
        let vk_bytes = SchnorrSecp256k1DSIGN::raw_serialize_verification_key(&verification_key);

        println!("Verification key: {}", hex::encode(&vk_bytes));
        assert_eq!(
            vk_bytes.len(),
            SchnorrSecp256k1DSIGN::VERIFICATION_KEY_SIZE,
            "Verification key should be {} bytes",
            SchnorrSecp256k1DSIGN::VERIFICATION_KEY_SIZE
        );
    }
}

#[test]
fn test_schnorr_sign_and_verify() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    for vector in sign_verify {
        let test_name = vector["test_name"].as_str().unwrap();
        println!("\n=== Testing Sign/Verify: {} ===", test_name);

        // Generate keys from secret key
        let sk_hex = vector["secret_key"].as_str().unwrap();
        let sk_bytes = decode_hex(sk_hex);
        let seed = mk_seed_from_bytes(sk_bytes);
        let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);
        let verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key);

        // Decode message
        let message_hex = vector["message"].as_str().unwrap();
        let message = decode_hex(message_hex);
        println!("Message: {} bytes", message.len());

        // Sign the message
        let context = Default::default();
        let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, &message, &signing_key);
        let sig_bytes = SchnorrSecp256k1DSIGN::raw_serialize_signature(&signature);

        println!("✓ Generated signature: {}", hex::encode(&sig_bytes));
        assert_eq!(
            sig_bytes.len(),
            SchnorrSecp256k1DSIGN::SIGNATURE_SIZE,
            "Signature should be {} bytes",
            SchnorrSecp256k1DSIGN::SIGNATURE_SIZE
        );

        // Verify the signature
        let verify_result =
            SchnorrSecp256k1DSIGN::verify_bytes(&context, &verification_key, &message, &signature);
        assert!(
            verify_result.is_ok(),
            "Signature verification should succeed for {}",
            test_name
        );
        println!("✓ Signature verified successfully");
    }
}

#[test]
fn test_schnorr_randomized_signatures() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    // Use first vector
    let vector = &sign_verify[0];
    let test_name = vector["test_name"].as_str().unwrap();

    println!("\n=== Testing Randomized Signatures: {} ===", test_name);

    let sk_hex = vector["secret_key"].as_str().unwrap();
    let sk_bytes = decode_hex(sk_hex);
    let seed = mk_seed_from_bytes(sk_bytes);
    let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);
    let verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key);

    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);

    // Generate signature twice - BIP340 allows randomized nonces for additional security
    let context = Default::default();
    let sig1 = SchnorrSecp256k1DSIGN::sign_bytes(&context, &message, &signing_key);
    let sig2 = SchnorrSecp256k1DSIGN::sign_bytes(&context, &message, &signing_key);

    let sig1_bytes = SchnorrSecp256k1DSIGN::raw_serialize_signature(&sig1);
    let sig2_bytes = SchnorrSecp256k1DSIGN::raw_serialize_signature(&sig2);

    // Signatures may differ (randomized nonce), but both should verify
    let verify1 = SchnorrSecp256k1DSIGN::verify_bytes(&context, &verification_key, &message, &sig1);
    let verify2 = SchnorrSecp256k1DSIGN::verify_bytes(&context, &verification_key, &message, &sig2);

    assert!(verify1.is_ok(), "First signature should verify");
    assert!(verify2.is_ok(), "Second signature should verify");

    println!("✓ Randomized signing confirmed: both signatures verify successfully");
    println!("  Sig1: {}", hex::encode(&sig1_bytes[..16]));
    println!("  Sig2: {}", hex::encode(&sig2_bytes[..16]));
}

#[test]
fn test_schnorr_verify_fails_wrong_message() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    let vector = &sign_verify[0];
    println!("\n=== Testing Wrong Message Verification ===");

    let sk_hex = vector["secret_key"].as_str().unwrap();
    let sk_bytes = decode_hex(sk_hex);
    let seed = mk_seed_from_bytes(sk_bytes);
    let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);
    let verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key);

    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);

    let context = Default::default();
    let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, &message, &signing_key);

    // Try to verify with a different message
    let wrong_message = b"this is the wrong message for schnorr";
    let verify_result =
        SchnorrSecp256k1DSIGN::verify_bytes(&context, &verification_key, wrong_message, &signature);

    assert!(
        verify_result.is_err(),
        "Verification should fail with wrong message"
    );
    println!("✓ Verification correctly failed for wrong message");
}

#[test]
fn test_schnorr_verify_fails_wrong_key() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    println!("\n=== Testing Wrong Key Verification ===");

    // Use first vector to sign
    let vector = &sign_verify[0];
    let sk_hex = vector["secret_key"].as_str().unwrap();
    let sk_bytes = decode_hex(sk_hex);
    let seed = mk_seed_from_bytes(sk_bytes);
    let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);

    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);

    let context = Default::default();
    let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, &message, &signing_key);

    // Use a different key for verification
    let vector2 = &sign_verify[1];
    let sk2_hex = vector2["secret_key"].as_str().unwrap();
    let sk2_bytes = decode_hex(sk2_hex);
    let seed2 = mk_seed_from_bytes(sk2_bytes);
    let signing_key2 = SchnorrSecp256k1DSIGN::gen_key(&seed2);
    let wrong_verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key2);

    let verify_result = SchnorrSecp256k1DSIGN::verify_bytes(
        &context,
        &wrong_verification_key,
        &message,
        &signature,
    );

    assert!(
        verify_result.is_err(),
        "Verification should fail with wrong key"
    );
    println!("✓ Verification correctly failed for wrong key");
}

#[test]
fn test_schnorr_serialization_roundtrip() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    let vector = &sign_verify[0];
    println!("\n=== Testing Serialization Roundtrip ===");

    let sk_hex = vector["secret_key"].as_str().unwrap();
    let sk_bytes = decode_hex(sk_hex);
    let seed = mk_seed_from_bytes(sk_bytes);
    let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);
    let verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key);

    // Serialize and deserialize verification key
    let vk_bytes = SchnorrSecp256k1DSIGN::raw_serialize_verification_key(&verification_key);
    let vk_restored = SchnorrSecp256k1DSIGN::raw_deserialize_verification_key(&vk_bytes)
        .expect("Should deserialize verification key");

    // Verification keys should be identical after roundtrip
    let vk_bytes2 = SchnorrSecp256k1DSIGN::raw_serialize_verification_key(&vk_restored);
    assert_eq!(
        vk_bytes, vk_bytes2,
        "Verification key should survive serialization roundtrip"
    );

    // Serialize and deserialize signing key
    let sk_serialized = SchnorrSecp256k1DSIGN::raw_serialize_signing_key(&signing_key);
    let sk_restored = SchnorrSecp256k1DSIGN::raw_deserialize_signing_key(&sk_serialized)
        .expect("Should deserialize signing key");

    // Verify that restored signing key produces same verification key
    let vk_from_restored = SchnorrSecp256k1DSIGN::derive_verification_key(&sk_restored);
    let vk_from_restored_bytes =
        SchnorrSecp256k1DSIGN::raw_serialize_verification_key(&vk_from_restored);
    assert_eq!(
        vk_bytes, vk_from_restored_bytes,
        "Restored signing key should derive same verification key"
    );

    // Sign and verify with restored keys
    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);

    let context = Default::default();
    let sig_original = SchnorrSecp256k1DSIGN::sign_bytes(&context, &message, &signing_key);
    let sig_restored = SchnorrSecp256k1DSIGN::sign_bytes(&context, &message, &sk_restored);

    // Both signatures should verify (even if they differ due to randomization)
    let verify_original =
        SchnorrSecp256k1DSIGN::verify_bytes(&context, &verification_key, &message, &sig_original);
    let verify_restored =
        SchnorrSecp256k1DSIGN::verify_bytes(&context, &vk_restored, &message, &sig_restored);

    assert!(verify_original.is_ok(), "Original signature should verify");
    assert!(verify_restored.is_ok(), "Restored signature should verify");

    // Cross-verify: original key's signature should verify with restored verification key
    let cross_verify =
        SchnorrSecp256k1DSIGN::verify_bytes(&context, &vk_restored, &message, &sig_original);
    assert!(
        cross_verify.is_ok(),
        "Restored verification key should verify original signature"
    );

    println!("✓ Serialization roundtrip successful");
}

#[test]
fn test_schnorr_empty_message() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    let vector = &sign_verify[0];
    println!("\n=== Testing Empty Message ===");

    let sk_hex = vector["secret_key"].as_str().unwrap();
    let sk_bytes = decode_hex(sk_hex);
    let seed = mk_seed_from_bytes(sk_bytes);
    let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);
    let verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key);

    let empty_message = b"";

    let context = Default::default();
    let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, empty_message, &signing_key);
    let verify_result =
        SchnorrSecp256k1DSIGN::verify_bytes(&context, &verification_key, empty_message, &signature);

    assert!(
        verify_result.is_ok(),
        "Should sign and verify empty message"
    );
    println!("✓ Empty message signing/verification successful");
}

#[test]
fn test_schnorr_large_message() {
    let vectors = parse_schnorr_vectors();
    let sign_verify = vectors["sign_and_verify_vectors"].as_array().unwrap();

    let vector = &sign_verify[0];
    println!("\n=== Testing Large Message (10KB) ===");

    let sk_hex = vector["secret_key"].as_str().unwrap();
    let sk_bytes = decode_hex(sk_hex);
    let seed = mk_seed_from_bytes(sk_bytes);
    let signing_key = SchnorrSecp256k1DSIGN::gen_key(&seed);
    let verification_key = SchnorrSecp256k1DSIGN::derive_verification_key(&signing_key);

    // Create a 10KB message
    let large_message = vec![0x42u8; 10240];

    let context = Default::default();
    let signature = SchnorrSecp256k1DSIGN::sign_bytes(&context, &large_message, &signing_key);
    let verify_result = SchnorrSecp256k1DSIGN::verify_bytes(
        &context,
        &verification_key,
        &large_message,
        &signature,
    );

    assert!(
        verify_result.is_ok(),
        "Should sign and verify large message"
    );
    println!("✓ Large message signing/verification successful");
}
