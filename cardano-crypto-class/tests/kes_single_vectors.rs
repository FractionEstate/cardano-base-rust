#![cfg(feature = "serde")]

use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{KesAlgorithm, KesError, KesMError, SingleKes};
use cardano_test_vectors::kes;
use hex::encode_upper;
use serde::Deserialize;

#[derive(Deserialize)]
struct SingleKesVectors {
    vectors: Vec<SingleKesVectorEntry>,
}

#[derive(Deserialize)]
struct SingleKesVectorEntry {
    test_name: String,
    seed: String,
    message: String,
    period: u64,
    expected: SingleKesExpected,
}

#[derive(Deserialize)]
struct SingleKesExpected {
    verification_key: String,
    signature: String,
    raw_signature: String,
}

#[test]
fn single_kes_vectors_match_generated_data() {
    let fixture = kes::get("single_kes_test_vectors.json").expect("embedded single KES vectors");
    let parsed: SingleKesVectors = serde_json::from_str(fixture).expect("valid single KES JSON");

    for vector in parsed.vectors {
        let seed_bytes = decode_seed(&vector.seed, SingleKes::<Ed25519>::SEED_SIZE);
        let message = decode_hex(&vector.message);

        let signing_key = SingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed_bytes)
            .expect(&vector.test_name);
        let verification_key = SingleKes::<Ed25519>::derive_verification_key(&signing_key)
            .expect("derive verification key");

        let vk_bytes = SingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
        assert_eq!(
            vector.expected.verification_key,
            encode_upper(vk_bytes),
            "verification key mismatch for {}",
            vector.test_name
        );

        let signature = SingleKes::<Ed25519>::sign_kes(&(), vector.period, &message, &signing_key)
            .expect("single signing");
        let raw_signature = SingleKes::<Ed25519>::raw_serialize_signature_kes(&signature);

        assert_eq!(
            vector.expected.signature,
            encode_upper(&raw_signature),
            "signature mismatch for {}",
            vector.test_name
        );
        assert_eq!(
            vector.expected.raw_signature,
            encode_upper(&raw_signature),
            "raw signature mismatch for {}",
            vector.test_name
        );

        let deserialised = SingleKes::<Ed25519>::raw_deserialize_signature_kes(&raw_signature)
            .expect("single signature deserialise");
        SingleKes::<Ed25519>::verify_kes(
            &(),
            &verification_key,
            vector.period,
            &message,
            &deserialised,
        )
        .expect("single verification");

        let err = SingleKes::<Ed25519>::sign_kes(&(), vector.period + 1, &message, &signing_key)
            .expect_err("signing beyond allowed period must fail");
        assert!(matches!(
            err,
            KesMError::Kes(KesError::PeriodOutOfRange { .. })
        ));

        let next = SingleKes::<Ed25519>::update_kes(&(), signing_key, vector.period)
            .expect("single update succeeds");
        assert!(next.is_none(), "single KES must expire after period 0");
    }
}

fn decode_seed(hex_seed: &str, expected_len: usize) -> Vec<u8> {
    let bytes = decode_hex(hex_seed);
    assert_eq!(bytes.len(), expected_len, "seed must fit expected length");
    bytes
}

fn decode_hex(input: &str) -> Vec<u8> {
    if input.is_empty() {
        Vec::new()
    } else {
        hex::decode(input).expect("valid hex input")
    }
}
