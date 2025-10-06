//! Test vectors for Ed25519 DSIGN implementation
//!
//! This module loads test vectors from the cardano-test-vectors crate
//! and validates the Ed25519 implementation against them.

use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::seed::mk_seed_from_bytes;
use cardano_test_vectors::dsign;
use serde_json::Value;

/// Helper to decode hex strings
fn decode_hex(s: &str) -> Vec<u8> {
    hex::decode(s).expect("valid hex string")
}

/// Parse the Ed25519 test vectors JSON
fn parse_ed25519_vectors() -> Value {
    let json =
        dsign::get("ed25519_test_vectors.json").expect("Ed25519 test vectors should be available");
    serde_json::from_str(json).expect("Should parse Ed25519 test vectors JSON")
}

#[test]
fn test_ed25519_vectors_exist() {
    let json =
        dsign::get("ed25519_test_vectors.json").expect("Ed25519 test vectors should be available");
    assert!(!json.is_empty(), "Ed25519 test vectors should not be empty");
}

#[test]
fn test_ed25519_vectors_parse() {
    let vectors = parse_ed25519_vectors();

    assert_eq!(vectors["algorithm"].as_str().unwrap(), "Ed25519DSIGN");
    let vector_array = vectors["vectors"].as_array().unwrap();
    assert!(
        !vector_array.is_empty(),
        "Should have at least one test vector"
    );

    println!("Loaded {} Ed25519 test vectors", vector_array.len());
}

#[test]
fn test_ed25519_key_generation_from_seed() {
    let vectors = parse_ed25519_vectors();
    let vector_array = vectors["vectors"].as_array().unwrap();

    for vector in vector_array {
        let test_name = vector["test_name"].as_str().unwrap();
        let description = vector["description"].as_str().unwrap();

        println!("\n=== Testing: {} ===", test_name);
        println!("Description: {}", description);

        // Decode seed
        let seed_hex = vector["seed"].as_str().unwrap();
        let seed_bytes = decode_hex(seed_hex);
        assert_eq!(
            seed_bytes.len(),
            Ed25519::SEED_SIZE,
            "Seed should be {} bytes for {}",
            Ed25519::SEED_SIZE,
            test_name
        );

        // Generate signing key from seed
        let seed = mk_seed_from_bytes(seed_bytes);
        let signing_key = Ed25519::gen_key(&seed);

        println!("✓ Generated signing key from seed");

        // Derive verification key
        let verification_key = Ed25519::derive_verification_key(&signing_key);

        let vk_bytes = Ed25519::raw_serialize_verification_key(&verification_key);
        println!("✓ Derived verification key: {}", hex::encode(&vk_bytes));

        // If expected verification key is provided, validate it
        if let Some(expected_vk) = vector["expected_verification_key"].as_str() {
            let expected_vk_bytes = decode_hex(expected_vk);
            assert_eq!(
                vk_bytes, expected_vk_bytes,
                "Verification key mismatch for {}",
                test_name
            );
            println!("✓ Verification key matches expected value");
        } else {
            println!("⚠ No expected verification key provided (will need to extract from Haskell)");
        }
    }
}

#[test]
fn test_ed25519_rfc8032_test_vectors() {
    println!("\n=== RFC 8032 Ed25519 Test Vector Validation ===");

    let vectors = parse_ed25519_vectors();
    let vector_array = vectors["vectors"].as_array().unwrap();

    // Filter for RFC 8032 test vectors
    let rfc_vectors: Vec<&serde_json::Value> = vector_array
        .iter()
        .filter(|v| v["test_name"].as_str().unwrap().starts_with("RFC_8032"))
        .collect();

    println!("Found {} RFC 8032 test vectors\n", rfc_vectors.len());

    for vector in rfc_vectors {
        let test_name = vector["test_name"].as_str().unwrap();
        println!("=== {} ===", test_name);

        // Generate keys from seed
        let seed_hex = vector["seed"].as_str().unwrap();
        let seed_bytes = decode_hex(seed_hex);
        let seed = mk_seed_from_bytes(seed_bytes);
        let signing_key = Ed25519::gen_key(&seed);
        let verification_key = Ed25519::derive_verification_key(&signing_key);

        // Check public key against expected
        if let Some(expected_pk) = vector["expected_public_key"].as_str() {
            let expected_pk_bytes = decode_hex(expected_pk);
            let vk_bytes = Ed25519::raw_serialize_verification_key(&verification_key);

            println!("Expected public key: {}", expected_pk);
            println!("Generated public key: {}", hex::encode(&vk_bytes));

            assert_eq!(
                vk_bytes, expected_pk_bytes,
                "Public key mismatch for {}",
                test_name
            );
            println!("✅ Public key matches RFC 8032");
        }

        // Sign message
        let message_hex = vector["message"].as_str().unwrap();
        let message = if message_hex.is_empty() {
            vec![]
        } else {
            decode_hex(message_hex)
        };

        let signature = Ed25519::sign_bytes(&(), &message, &signing_key);
        let sig_bytes = Ed25519::raw_serialize_signature(&signature);

        // Check signature against expected
        if let Some(expected_sig) = vector["expected_signature"].as_str() {
            let expected_sig_str = expected_sig.replace(" ", ""); // Remove spaces
            let expected_sig_bytes = decode_hex(&expected_sig_str);

            println!("Expected signature: {}", expected_sig_str);
            println!("Generated signature: {}", hex::encode(&sig_bytes));

            assert_eq!(
                sig_bytes, expected_sig_bytes,
                "Signature mismatch for {}",
                test_name
            );
            println!("✅ Signature matches RFC 8032");
        }

        // Verify signature
        let verify_result = Ed25519::verify_bytes(&(), &verification_key, &message, &signature);
        assert!(
            verify_result.is_ok(),
            "Signature verification failed for {}",
            test_name
        );
        println!("✅ Signature verified\n");
    }

    println!("=== All RFC 8032 Test Vectors Pass ===");
}

#[test]
fn test_ed25519_sign_and_verify() {
    let vectors = parse_ed25519_vectors();
    let vector_array = vectors["vectors"].as_array().unwrap();

    for vector in vector_array {
        let test_name = vector["test_name"].as_str().unwrap();

        println!("\n=== Testing Sign/Verify: {} ===", test_name);

        // Generate keys from seed
        let seed_hex = vector["seed"].as_str().unwrap();
        let seed_bytes = decode_hex(seed_hex);
        let seed = mk_seed_from_bytes(seed_bytes);
        let signing_key = Ed25519::gen_key(&seed);
        let verification_key = Ed25519::derive_verification_key(&signing_key);

        // Decode message
        let message_hex = vector["message"].as_str().unwrap();
        let message = decode_hex(message_hex);
        println!("Message: {} bytes", message.len());

        // Sign the message
        let signature = Ed25519::sign_bytes(&(), &message, &signing_key);
        let sig_bytes = Ed25519::raw_serialize_signature(&signature);

        println!("✓ Generated signature: {}", hex::encode(&sig_bytes));
        assert_eq!(
            sig_bytes.len(),
            Ed25519::SIGNATURE_SIZE,
            "Signature should be {} bytes",
            Ed25519::SIGNATURE_SIZE
        );

        // Verify the signature
        let verify_result = Ed25519::verify_bytes(&(), &verification_key, &message, &signature);
        assert!(
            verify_result.is_ok(),
            "Signature verification should succeed for {}",
            test_name
        );
        println!("✓ Signature verified successfully");

        // If expected signature is provided, validate it
        if let Some(expected_sig) = vector["expected_signature"].as_str() {
            let expected_sig_bytes = decode_hex(expected_sig);
            assert_eq!(
                sig_bytes, expected_sig_bytes,
                "Signature mismatch for {}",
                test_name
            );
            println!("✓ Signature matches expected value");
        } else {
            println!("⚠ No expected signature provided (will need to extract from Haskell)");
        }
    }
}

#[test]
fn test_ed25519_verify_fails_wrong_message() {
    let vectors = parse_ed25519_vectors();
    let vector_array = vectors["vectors"].as_array().unwrap();

    // Use the first vector
    let vector = &vector_array[0];

    println!("\n=== Testing Wrong Message Verification ===");

    // Generate keys and sign original message
    let seed_hex = vector["seed"].as_str().unwrap();
    let seed_bytes = decode_hex(seed_hex);
    let seed = mk_seed_from_bytes(seed_bytes);
    let signing_key = Ed25519::gen_key(&seed);
    let verification_key = Ed25519::derive_verification_key(&signing_key);

    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);
    let signature = Ed25519::sign_bytes(&(), &message, &signing_key);

    // Try to verify with a different message
    let wrong_message = b"this is the wrong message";
    let verify_result = Ed25519::verify_bytes(&(), &verification_key, wrong_message, &signature);

    assert!(
        verify_result.is_err(),
        "Verification should fail with wrong message"
    );
    println!("✓ Verification correctly failed for wrong message");
}

#[test]
fn test_ed25519_verify_fails_wrong_key() {
    let vectors = parse_ed25519_vectors();
    let vector_array = vectors["vectors"].as_array().unwrap();

    // Use the first vector
    let vector = &vector_array[0];

    println!("\n=== Testing Wrong Key Verification ===");

    // Generate keys and sign message
    let seed_hex = vector["seed"].as_str().unwrap();
    let seed_bytes = decode_hex(seed_hex);
    let seed = mk_seed_from_bytes(seed_bytes);
    let signing_key = Ed25519::gen_key(&seed);

    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);
    let signature = Ed25519::sign_bytes(&(), &message, &signing_key);

    // Generate a different key
    let wrong_seed = mk_seed_from_bytes(vec![0xFF; Ed25519::SEED_SIZE]);
    let wrong_signing_key = Ed25519::gen_key(&wrong_seed);
    let wrong_verification_key = Ed25519::derive_verification_key(&wrong_signing_key);

    // Try to verify with wrong key
    let verify_result = Ed25519::verify_bytes(&(), &wrong_verification_key, &message, &signature);

    assert!(
        verify_result.is_err(),
        "Verification should fail with wrong key"
    );
    println!("✓ Verification correctly failed for wrong key");
}

#[test]
fn test_ed25519_deterministic_signatures() {
    let vectors = parse_ed25519_vectors();
    let vector_array = vectors["vectors"].as_array().unwrap();

    // Use the first vector
    let vector = &vector_array[0];

    println!("\n=== Testing Deterministic Signatures ===");

    let seed_hex = vector["seed"].as_str().unwrap();
    let seed_bytes = decode_hex(seed_hex);
    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);

    // Sign the same message multiple times with the same key
    let seed = mk_seed_from_bytes(seed_bytes.clone());
    let signing_key_1 = Ed25519::gen_key(&seed);
    let signature_1 = Ed25519::sign_bytes(&(), &message, &signing_key_1);
    let sig_bytes_1 = Ed25519::raw_serialize_signature(&signature_1);

    let seed_2 = mk_seed_from_bytes(seed_bytes);
    let signing_key_2 = Ed25519::gen_key(&seed_2);
    let signature_2 = Ed25519::sign_bytes(&(), &message, &signing_key_2);
    let sig_bytes_2 = Ed25519::raw_serialize_signature(&signature_2);

    assert_eq!(
        sig_bytes_1, sig_bytes_2,
        "Signatures should be deterministic (same input should produce same signature)"
    );
    println!("✓ Signatures are deterministic");
}

#[test]
fn test_ed25519_serialization_roundtrip() {
    let vectors = parse_ed25519_vectors();
    let vector_array = vectors["vectors"].as_array().unwrap();

    // Use the first vector
    let vector = &vector_array[0];

    println!("\n=== Testing Serialization Roundtrip ===");

    let seed_hex = vector["seed"].as_str().unwrap();
    let seed_bytes = decode_hex(seed_hex);
    let seed = mk_seed_from_bytes(seed_bytes);
    let signing_key = Ed25519::gen_key(&seed);
    let verification_key = Ed25519::derive_verification_key(&signing_key);

    let message_hex = vector["message"].as_str().unwrap();
    let message = decode_hex(message_hex);
    let signature = Ed25519::sign_bytes(&(), &message, &signing_key);

    // Serialize keys and signature
    let vk_bytes = Ed25519::raw_serialize_verification_key(&verification_key);
    let sk_bytes = Ed25519::raw_serialize_signing_key(&signing_key);
    let sig_bytes = Ed25519::raw_serialize_signature(&signature);

    println!("Verification key: {} bytes", vk_bytes.len());
    println!("Signing key: {} bytes", sk_bytes.len());
    println!("Signature: {} bytes", sig_bytes.len());

    // Deserialize
    let vk_restored = Ed25519::raw_deserialize_verification_key(&vk_bytes)
        .expect("Should deserialize verification key");
    let sk_restored =
        Ed25519::raw_deserialize_signing_key(&sk_bytes).expect("Should deserialize signing key");
    let sig_restored =
        Ed25519::raw_deserialize_signature(&sig_bytes).expect("Should deserialize signature");

    // Verify with restored key and signature
    let verify_result = Ed25519::verify_bytes(&(), &vk_restored, &message, &sig_restored);
    assert!(
        verify_result.is_ok(),
        "Verification with restored keys should succeed"
    );
    println!("✓ Serialization roundtrip successful");

    // Verify that signing with restored key produces same signature
    let sig_from_restored = Ed25519::sign_bytes(&(), &message, &sk_restored);
    let sig_from_restored_bytes = Ed25519::raw_serialize_signature(&sig_from_restored);

    assert_eq!(
        sig_bytes, sig_from_restored_bytes,
        "Restored signing key should produce same signature"
    );
    println!("✓ Restored signing key produces same signature");
}

#[test]
fn test_ed25519_empty_message() {
    println!("\n=== Testing Empty Message ===");

    let seed = mk_seed_from_bytes(vec![42u8; Ed25519::SEED_SIZE]);
    let signing_key = Ed25519::gen_key(&seed);
    let verification_key = Ed25519::derive_verification_key(&signing_key);

    let empty_message: &[u8] = &[];
    let signature = Ed25519::sign_bytes(&(), empty_message, &signing_key);

    let verify_result = Ed25519::verify_bytes(&(), &verification_key, empty_message, &signature);
    assert!(
        verify_result.is_ok(),
        "Should be able to sign and verify empty message"
    );
    println!("✓ Empty message signing/verification works");
}

#[test]
fn test_ed25519_large_message() {
    println!("\n=== Testing Large Message ===");

    let seed = mk_seed_from_bytes(vec![99u8; Ed25519::SEED_SIZE]);
    let signing_key = Ed25519::gen_key(&seed);
    let verification_key = Ed25519::derive_verification_key(&signing_key);

    // Create a large message (10 KB)
    let large_message = vec![0xAB; 10_000];
    let signature = Ed25519::sign_bytes(&(), &large_message, &signing_key);

    let verify_result = Ed25519::verify_bytes(&(), &verification_key, &large_message, &signature);
    assert!(
        verify_result.is_ok(),
        "Should be able to sign and verify large message"
    );
    println!("✓ Large message (10 KB) signing/verification works");
}
