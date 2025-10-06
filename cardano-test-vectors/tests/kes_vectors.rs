use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::compact_single::OptimizedKesSignature;
use cardano_crypto_class::kes::{CompactSingleKes, KesAlgorithm, SingleKes};
use cardano_test_vectors::kes;
use hex::{decode, encode_upper};
use serde::Deserialize;

#[derive(Deserialize)]
struct SingleKesVectors {
    vectors: Vec<SingleKesVectorEntry>,
}

#[derive(Deserialize)]
struct SingleKesVectorEntry {
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

#[derive(Deserialize)]
struct CompactSingleKesVectors {
    vectors: Vec<CompactSingleKesVectorEntry>,
}

#[derive(Deserialize)]
struct CompactSingleKesVectorEntry {
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
fn single_kes_vectors_match_generated_data() {
    let fixture = kes::get("single_kes_test_vectors.json").expect("single KES vectors present");
    let parsed: SingleKesVectors = serde_json::from_str(fixture).expect("valid single KES JSON");

    for (index, vector) in parsed.vectors.iter().enumerate() {
        assert_eq!(0, vector.period, "SingleKES supports only period 0");

        let seed_bytes = decode_hex(&vector.seed);
        let message = decode_hex(&vector.message);

        let signing_key = SingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed_bytes)
            .expect("signing key generation");
        let verification_key = SingleKes::<Ed25519>::derive_verification_key(&signing_key)
            .expect("verification key derivation");

        let vk_bytes = SingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
        assert_eq!(
            vector.expected.verification_key,
            encode_upper(vk_bytes),
            "vk mismatch for vector {index}"
        );

        let signature_bytes = decode_hex(&vector.expected.signature);
        let raw_signature_bytes = decode_hex(&vector.expected.raw_signature);
        assert_eq!(
            signature_bytes, raw_signature_bytes,
            "raw signature mismatch for vector {index}"
        );

        let signature = SingleKes::<Ed25519>::raw_deserialize_signature_kes(&raw_signature_bytes)
            .expect("signature deserialise");

        SingleKes::<Ed25519>::verify_kes(
            &(),
            &verification_key,
            vector.period,
            &message,
            &signature,
        )
        .expect("signature verification");

        let recomputed = SingleKes::<Ed25519>::sign_kes(&(), vector.period, &message, &signing_key)
            .expect("signing");
        let recomputed_bytes = SingleKes::<Ed25519>::raw_serialize_signature_kes(&recomputed);
        assert_eq!(
            raw_signature_bytes, recomputed_bytes,
            "roundtrip signature mismatch for vector {index}"
        );

        SingleKes::<Ed25519>::forget_signing_key_kes(signing_key);
    }
}

#[test]
fn compact_single_kes_vectors_match_generated_data() {
    let fixture = kes::get("compact_single_kes_test_vectors.json")
        .expect("compact single KES vectors present");
    let parsed: CompactSingleKesVectors =
        serde_json::from_str(fixture).expect("valid compact single KES JSON");

    let signature_len = <Ed25519 as DsignAlgorithm>::SIGNATURE_SIZE;

    for (index, vector) in parsed.vectors.iter().enumerate() {
        assert_eq!(0, vector.period, "CompactSingleKES supports only period 0");

        let seed_bytes = decode_hex(&vector.seed);
        let message = decode_hex(&vector.message);

        let signing_key = CompactSingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed_bytes)
            .expect("signing key generation");
        let verification_key = CompactSingleKes::<Ed25519>::derive_verification_key(&signing_key)
            .expect("verification key derivation");

        let derived_vk_bytes =
            CompactSingleKes::<Ed25519>::raw_serialize_verification_key_kes(&verification_key);
        assert_eq!(
            vector.expected.derived_verification_key,
            encode_upper(derived_vk_bytes),
            "derived vk mismatch for vector {index}"
        );

        let raw_signature_bytes = decode_hex(&vector.expected.raw_signature);
        let signature_bytes = decode_hex(&vector.expected.signature);
        assert_eq!(
            signature_bytes,
            raw_signature_bytes[..signature_len],
            "signature fragment mismatch for vector {index}"
        );

        let signature =
            CompactSingleKes::<Ed25519>::raw_deserialize_signature_kes(&raw_signature_bytes)
                .expect("signature deserialise");

        let embedded_vk_bytes = CompactSingleKes::<Ed25519>::raw_serialize_verification_key_kes(
            signature.extract_verification_key(),
        );
        assert_eq!(
            vector.expected.embedded_verification_key,
            encode_upper(embedded_vk_bytes),
            "embedded vk mismatch for vector {index}"
        );

        CompactSingleKes::<Ed25519>::verify_kes(
            &(),
            &verification_key,
            vector.period,
            &message,
            &signature,
        )
        .expect("signature verification");

        let recomputed =
            CompactSingleKes::<Ed25519>::sign_kes(&(), vector.period, &message, &signing_key)
                .expect("signing");
        let recomputed_bytes =
            CompactSingleKes::<Ed25519>::raw_serialize_signature_kes(&recomputed);
        assert_eq!(
            raw_signature_bytes, recomputed_bytes,
            "roundtrip signature mismatch for vector {index}"
        );

        CompactSingleKes::<Ed25519>::forget_signing_key_kes(signing_key);
    }
}

fn decode_hex(input: &str) -> Vec<u8> {
    if input.is_empty() {
        Vec::new()
    } else {
        decode(input).expect("valid hex input")
    }
}
