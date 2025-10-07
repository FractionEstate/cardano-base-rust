#![cfg(feature = "serde")]

use cardano_crypto_class::dsign::{DsignAlgorithm, ed25519::Ed25519};
use cardano_crypto_class::kes::{
    CompactSingleKes, KesAlgorithm, KesError, KesMError, OptimizedKesSignature,
};
use cardano_test_vectors::kes;
use hex::encode_upper;
use serde::Deserialize;

#[derive(Deserialize)]
struct CompactSingleKesVectors {
    vectors: Vec<CompactSingleKesVectorEntry>,
}

#[derive(Deserialize)]
struct CompactSingleKesVectorEntry {
    test_name: String,
    seed: String,
    message: String,
    period: u64,
    expected: CompactSingleExpected,
}

#[derive(Deserialize)]
struct CompactSingleExpected {
    derived_verification_key: String,
    embedded_verification_key: String,
    signature: String,
    raw_signature: String,
}

#[test]
fn compact_single_kes_vectors_match_generated_data() {
    let fixture = kes::get("compact_single_kes_test_vectors.json")
        .expect("embedded compact single KES vectors");
    let parsed: CompactSingleKesVectors =
        serde_json::from_str(fixture).expect("valid compact single KES JSON");

    let signature_len = <Ed25519 as DsignAlgorithm>::SIGNATURE_SIZE;

    for vector in parsed.vectors {
        let seed_bytes = decode_seed(&vector.seed, CompactSingleKes::<Ed25519>::SEED_SIZE);
        let message = decode_hex(&vector.message);

        let signing_key = CompactSingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed_bytes)
            .expect(&vector.test_name);
        let verification_key = CompactSingleKes::<Ed25519>::derive_verification_key(&signing_key)
            .expect("derive compact single verification key");

        let derived_bytes =
            CompactSingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
        assert_eq!(
            vector.expected.derived_verification_key,
            encode_upper(&derived_bytes),
            "derived verification key mismatch for {}",
            vector.test_name
        );

        let signature =
            CompactSingleKes::<Ed25519>::sign_kes(&(), vector.period, &message, &signing_key)
                .expect("compact single signing");
        let raw_signature = CompactSingleKes::<Ed25519>::raw_serialize_signature_kes(&signature);

        assert_eq!(
            vector.expected.raw_signature,
            encode_upper(&raw_signature),
            "raw signature mismatch for {}",
            vector.test_name
        );

        let (dsign_sig, embedded_vk) = raw_signature.split_at(signature_len);
        assert_eq!(
            vector.expected.signature,
            encode_upper(dsign_sig),
            "DSIGN portion mismatch for {}",
            vector.test_name
        );
        assert_eq!(
            vector.expected.embedded_verification_key,
            encode_upper(embedded_vk),
            "embedded verification key mismatch for {}",
            vector.test_name
        );

        let extracted_vk = signature.extract_verification_key();
        let embedded_bytes =
            CompactSingleKes::<Ed25519>::raw_serialize_verification_key_kes(extracted_vk);
        assert_eq!(
            embedded_bytes, embedded_vk,
            "extracted verification key mismatch for {}",
            vector.test_name
        );

        let deserialised =
            CompactSingleKes::<Ed25519>::raw_deserialize_signature_kes(&raw_signature)
                .expect("compact single signature deserialise");
        CompactSingleKes::<Ed25519>::verify_kes(
            &(),
            &verification_key,
            vector.period,
            &message,
            &deserialised,
        )
        .expect("compact single verification");

        let err =
            CompactSingleKes::<Ed25519>::sign_kes(&(), vector.period + 1, &message, &signing_key)
                .expect_err("signing beyond compact single period must fail");
        assert!(matches!(
            err,
            KesMError::Kes(KesError::PeriodOutOfRange { .. })
        ));

        let next = CompactSingleKes::<Ed25519>::update_kes(&(), signing_key, vector.period)
            .expect("compact single update succeeds");
        assert!(
            next.is_none(),
            "compact single KES must expire after period 0"
        );
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
