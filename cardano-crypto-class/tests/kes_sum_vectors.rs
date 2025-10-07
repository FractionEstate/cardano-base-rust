#![cfg(feature = "serde")]

use cardano_crypto_class::kes::{
    KesAlgorithm, Sum1Kes, Sum2Kes, Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes, Sum7Kes,
};
use cardano_test_vectors::kes;
use hex::encode_upper;
use serde::Deserialize;

#[derive(Deserialize)]
struct SumKesVectors {
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
    test_name: String,
    seed: String,
    verification_key: String,
    tracked_periods: Vec<PeriodVectorEntry>,
}

#[derive(Deserialize)]
struct PeriodVectorEntry {
    period: u64,
    message: String,
    signature: String,
    raw_signature: String,
}

#[test]
fn sum_kes_vectors_match_generated_data() {
    let fixture = kes::get("sum_kes_test_vectors.json").expect("embedded sum KES vectors");
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
            other => panic!("unexpected sum level {other}"),
        }
    }
}

fn exercise_sum_level<K>(level: &SumKesLevel)
where
    K: KesAlgorithm<Context = ()>,
{
    assert_eq!(
        level.total_periods,
        K::total_periods(),
        "total periods mismatch"
    );

    for vector in &level.vectors {
        let seed_bytes = decode_seed(&vector.seed, K::SEED_SIZE);
        let mut signing_key =
            K::gen_key_kes_from_seed_bytes(&seed_bytes).expect("sum signing key generation");
        let verification_key =
            K::derive_verification_key(&signing_key).expect("sum verification key derivation");

        let vk_bytes = K::raw_serialize_verification_key_kes(&verification_key);
        assert_eq!(
            vector.verification_key,
            encode_upper(vk_bytes),
            "verification key mismatch for {}",
            vector.test_name
        );

        let mut tracked = vector.tracked_periods.iter().peekable();
        let total_periods = K::total_periods();

        for period in 0..total_periods {
            if let Some(entry) = tracked.peek() {
                if entry.period == period {
                    let message = decode_hex(&entry.message);
                    let expected_signature = decode_hex(&entry.raw_signature);

                    let signature =
                        K::sign_kes(&(), period, &message, &signing_key).expect("sum signing");
                    let raw_signature = K::raw_serialize_signature_kes(&signature);

                    assert_eq!(
                        expected_signature, raw_signature,
                        "signature mismatch at period {period} for {}",
                        vector.test_name
                    );
                    assert_eq!(
                        entry.signature,
                        encode_upper(&raw_signature),
                        "hex signature mismatch at period {period} for {}",
                        vector.test_name
                    );

                    let deserialised = K::raw_deserialize_signature_kes(&raw_signature)
                        .expect("sum signature deserialise");
                    K::verify_kes(&(), &verification_key, period, &message, &deserialised)
                        .expect("sum verification");

                    tracked.next();
                }
            }

            if period + 1 == total_periods {
                K::forget_signing_key_kes(signing_key);
                break;
            }

            signing_key = K::update_kes(&(), signing_key, period)
                .expect("sum key update")
                .expect("sum key should remain valid before final period");
        }

        assert!(tracked.next().is_none(), "unused tracked entries remain");
    }
}

fn decode_seed(hex_seed: &str, expected_len: usize) -> Vec<u8> {
    let bytes = decode_hex(hex_seed);
    assert_eq!(bytes.len(), expected_len, "seed must match expected length");
    bytes
}

fn decode_hex(input: &str) -> Vec<u8> {
    if input.is_empty() {
        Vec::new()
    } else {
        hex::decode(input).expect("valid hex input")
    }
}
