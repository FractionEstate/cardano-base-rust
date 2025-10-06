use cardano_crypto_class::dsign::DsignAlgorithm;
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::compact_single::OptimizedKesSignature;
use cardano_crypto_class::kes::{
    CompactSingleKes, CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, CompactSum4Kes,
    CompactSum5Kes, CompactSum6Kes, CompactSum7Kes, KesAlgorithm, SingleKes, Sum1Kes, Sum2Kes,
    Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes, Sum7Kes,
};
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

#[derive(Deserialize)]
struct SumKesVectors {
    levels: Vec<SumKesLevel>,
}

#[derive(Deserialize)]
struct CompactSumKesVectors {
    levels: Vec<SumKesLevel>,
}

#[derive(Deserialize)]
struct SumKesLevel {
    level: u8,
    total_periods: u64,
    vectors: Vec<SumKesVectorEntry>,
}

#[derive(Deserialize)]
struct SumKesVectorEntry {
    seed: String,
    verification_key: String,
    tracked_periods: Vec<SumKesPeriodEntry>,
}

#[derive(Deserialize)]
struct SumKesPeriodEntry {
    period: u64,
    message: String,
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

#[test]
fn sum_kes_vectors_cover_period_boundaries() {
    let fixture = kes::get("sum_kes_test_vectors.json").expect("sum KES vectors present");
    let parsed: SumKesVectors = serde_json::from_str(fixture).expect("valid sum KES JSON");

    for level in &parsed.levels {
        match level.level {
            1 => exercise_sum_level::<Sum1Kes>(level),
            2 => exercise_sum_level::<Sum2Kes>(level),
            3 => exercise_sum_level::<Sum3Kes>(level),
            4 => exercise_sum_level::<Sum4Kes>(level),
            5 => exercise_sum_level::<Sum5Kes>(level),
            6 => exercise_sum_level::<Sum6Kes>(level),
            7 => exercise_sum_level::<Sum7Kes>(level),
            other => panic!("unexpected SumKES level {other}"),
        }
    }
}

#[test]
fn compact_sum_kes_vectors_cover_all_levels() {
    let fixture =
        kes::get("compact_sum_kes_test_vectors.json").expect("compact sum KES vectors present");
    let parsed: CompactSumKesVectors =
        serde_json::from_str(fixture).expect("valid compact sum KES JSON");

    for level in &parsed.levels {
        match level.level {
            1 => exercise_sum_level::<CompactSum1Kes>(level),
            2 => exercise_sum_level::<CompactSum2Kes>(level),
            3 => exercise_sum_level::<CompactSum3Kes>(level),
            4 => exercise_sum_level::<CompactSum4Kes>(level),
            5 => exercise_sum_level::<CompactSum5Kes>(level),
            6 => exercise_sum_level::<CompactSum6Kes>(level),
            7 => exercise_sum_level::<CompactSum7Kes>(level),
            other => panic!("unexpected CompactSumKES level {other}"),
        }
    }
}

fn exercise_sum_level<K>(level: &SumKesLevel)
where
    K: KesAlgorithm<Context = ()>,
{
    assert_eq!(level.total_periods, K::total_periods());

    for vector in &level.vectors {
        let seed_bytes = decode_hex(&vector.seed);
        let mut signing_key =
            K::gen_key_kes_from_seed_bytes(&seed_bytes).expect("sum signing key generation");
        let verification_key =
            K::derive_verification_key(&signing_key).expect("sum verification key");

        let vk_bytes = K::raw_serialize_verification_key_kes(&verification_key);
        assert_eq!(
            vector.verification_key,
            encode_upper(vk_bytes),
            "verification key mismatch for level {}",
            level.level
        );

        let mut period_entries = vector.tracked_periods.iter().peekable();
        let total_periods = K::total_periods();

        for period in 0..total_periods {
            if let Some(expected) = period_entries.peek() {
                if expected.period == period {
                    let message = decode_hex(&expected.message);
                    assert_eq!(
                        expected.signature, expected.raw_signature,
                        "signature/raw mismatch at level {} period {}",
                        level.level, period
                    );
                    let expected_signature = decode_hex(&expected.raw_signature);

                    let signature =
                        K::sign_kes(&(), period, &message, &signing_key).expect("sum signing");
                    let raw_signature = K::raw_serialize_signature_kes(&signature);

                    assert_eq!(
                        expected_signature, raw_signature,
                        "signature mismatch at level {} period {}",
                        level.level, period
                    );

                    let deserialised = K::raw_deserialize_signature_kes(&raw_signature)
                        .expect("sum signature deserialise");
                    K::verify_kes(&(), &verification_key, period, &message, &deserialised)
                        .expect("sum verification");

                    period_entries.next();
                }
            }

            if period + 1 == total_periods {
                K::forget_signing_key_kes(signing_key);
                break;
            }

            signing_key = K::update_kes(&(), signing_key, period)
                .expect("sum key update")
                .expect("sum key remains valid");
        }

        assert!(period_entries.next().is_none(), "unused period entries");
    }
}

fn decode_hex(input: &str) -> Vec<u8> {
    if input.is_empty() {
        Vec::new()
    } else {
        decode(input).expect("valid hex input")
    }
}
