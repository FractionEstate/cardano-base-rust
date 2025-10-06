//! Debug-oriented trace test mirroring the VRF crate strategy.
//! Run with `cargo test -p cardano-test-vectors --test debug_ed25519_trace -- --nocapture`
//! to inspect the intermediate values while generating signatures.

use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_test_vectors::dsign;
use hex::encode_upper;
use serde::Deserialize;

#[derive(Deserialize)]
struct TestVectorFile {
    vectors: Vec<TestVector>,
}

#[derive(Deserialize)]
struct TestVector {
    #[serde(rename = "test_name")]
    test_name: String,
    seed: String,
    message: String,
    #[serde(rename = "expected_public_key")]
    expected_public_key: Option<String>,
    #[serde(rename = "expected_signature")]
    expected_signature: Option<String>,
}

#[test]
fn debug_trace_first_vector() {
    println!("\n=== Ed25519 Debug Trace ===");

    let json = dsign::get("ed25519_test_vectors.json").expect("vector file is embedded");
    let vectors: TestVectorFile = serde_json::from_str(json).expect("valid JSON structure");
    let vector = vectors
        .vectors
        .first()
        .expect("at least one Ed25519 test vector present");

    println!("Vector: {}", vector.test_name);
    println!("Seed: {}", vector.seed);
    println!("Message: {}", vector.message);

    let seed_bytes = decode_hex(&vector.seed);
    let message_bytes = decode_hex(&vector.message);

    println!("Seed bytes: {}", encode_upper(&seed_bytes));
    println!("Message bytes: {}", encode_upper(&message_bytes));

    let signing_key = <Ed25519 as DsignAlgorithm>::gen_key_from_seed_bytes(&seed_bytes);
    let verification_key = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing_key);
    let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), &message_bytes, &signing_key);

    let vk_bytes = <Ed25519 as DsignAlgorithm>::raw_serialize_verification_key(&verification_key);
    let sig_bytes = <Ed25519 as DsignAlgorithm>::raw_serialize_signature(&signature);

    println!("Derived verification key: {}", encode_upper(&vk_bytes));
    println!("Generated signature: {}", encode_upper(&sig_bytes));

    if let Some(expected_vk) = vector.expected_public_key.as_deref() {
        println!(
            "Expected verification key: {}",
            expected_vk.to_ascii_uppercase()
        );
    }

    if let Some(expected_sig) = vector.expected_signature.as_deref() {
        println!("Expected signature: {}", expected_sig.to_ascii_uppercase());
    }

    <Ed25519 as DsignAlgorithm>::verify_bytes(&(), &verification_key, &message_bytes, &signature)
        .expect("debug trace signature should verify");

    println!("✅ Signature verification succeeded");
}

fn decode_hex(input: &str) -> Vec<u8> {
    if input.is_empty() {
        Vec::new()
    } else {
        hex::decode(input).expect("hex input should be valid")
    }
}
