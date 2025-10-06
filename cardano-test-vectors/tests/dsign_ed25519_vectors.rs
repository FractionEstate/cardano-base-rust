use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_test_vectors::dsign;
use hex::{decode, encode_upper};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestVectorFile {
    vectors: Vec<TestVector>,
}

#[derive(Debug, Deserialize)]
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
fn ed25519_vectors_produce_expected_outputs() {
    let json = dsign::get("ed25519_test_vectors.json")
        .expect("Ed25519 test vector file should be embedded");
    let parsed: TestVectorFile =
        serde_json::from_str(json).expect("Ed25519 test vectors JSON should parse");

    assert!(
        !parsed.vectors.is_empty(),
        "should have at least one test vector"
    );

    for (index, vector) in parsed.vectors.iter().enumerate() {
        let seed_bytes = decode_hex(&vector.seed);
        assert_eq!(seed_bytes.len(), <Ed25519 as DsignAlgorithm>::SEED_SIZE);

        let message_bytes = decode_hex(&vector.message);

        let signing_key = <Ed25519 as DsignAlgorithm>::gen_key_from_seed_bytes(&seed_bytes);
        let verification_key = <Ed25519 as DsignAlgorithm>::derive_verification_key(&signing_key);
        let signature = <Ed25519 as DsignAlgorithm>::sign_bytes(&(), &message_bytes, &signing_key);

        let vk_bytes =
            <Ed25519 as DsignAlgorithm>::raw_serialize_verification_key(&verification_key);
        let sig_bytes = <Ed25519 as DsignAlgorithm>::raw_serialize_signature(&signature);

        let vk_hex = encode_upper(&vk_bytes);
        let sig_hex = encode_upper(&sig_bytes);

        if let Some(expected_vk) = vector.expected_public_key.as_deref() {
            assert_eq!(
                vk_hex,
                expected_vk.to_ascii_uppercase(),
                "verification key mismatch for {} (index {})",
                vector.test_name,
                index
            );
        }

        if let Some(expected_sig) = vector.expected_signature.as_deref() {
            assert_eq!(
                sig_hex,
                expected_sig.to_ascii_uppercase(),
                "signature mismatch for {} (index {})",
                vector.test_name,
                index
            );
        }

        <Ed25519 as DsignAlgorithm>::verify_bytes(
            &(),
            &verification_key,
            &message_bytes,
            &signature,
        )
        .unwrap_or_else(|err| {
            panic!(
                "signature verification failed for {} (index {}): {:?}",
                vector.test_name, index, err
            )
        });
    }
}

fn decode_hex(input: &str) -> Vec<u8> {
    if input.is_empty() {
        Vec::new()
    } else {
        decode(input).expect("valid hex input")
    }
}
