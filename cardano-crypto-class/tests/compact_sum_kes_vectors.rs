#![cfg(feature = "serde")]

use cardano_crypto_class::kes::{
    CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, CompactSum4Kes, CompactSum5Kes, CompactSum6Kes,
    CompactSum7Kes, KesAlgorithm,
};
use cardano_test_vectors::kes;
use hex::decode;
use serde::Deserialize;

#[derive(Deserialize)]
struct CompactSumKesVectors {
    levels: Vec<CompactSumLevel>,
}

#[derive(Deserialize)]
struct CompactSumLevel {
    level: u8,
    total_periods: u64,
    vectors: Vec<CompactSumVectorEntry>,
}

#[derive(Deserialize)]
struct CompactSumVectorEntry {
    seed: String,
    verification_key: String,
    tracked_periods: Vec<CompactSumPeriodEntry>,
}

#[derive(Deserialize)]
struct CompactSumPeriodEntry {
    period: u64,
    message: String,
    signature: String,
    raw_signature: String,
}

#[test]
fn compact_sum_vectors_match_generated_data() {
    let fixture =
        kes::get("compact_sum_kes_test_vectors.json").expect("compact sum KES vectors embedded");
    let parsed: CompactSumKesVectors =
        serde_json::from_str(fixture).expect("valid compact sum JSON");

    for level in &parsed.levels {
        let exercise: fn(&CompactSumLevel) = match level.level {
            1 => exercise_compact_sum_level::<CompactSum1Kes>,
            2 => exercise_compact_sum_level::<CompactSum2Kes>,
            3 => exercise_compact_sum_level::<CompactSum3Kes>,
            4 => exercise_compact_sum_level::<CompactSum4Kes>,
            5 => exercise_compact_sum_level::<CompactSum5Kes>,
            6 => exercise_compact_sum_level::<CompactSum6Kes>,
            7 => exercise_compact_sum_level::<CompactSum7Kes>,
            other => {
                assert!(
                    (1..=7).contains(&other),
                    "unexpected compact sum level {other}"
                );
                continue;
            },
        };
        exercise(level);
    }
}

fn exercise_compact_sum_level<K>(level: &CompactSumLevel)
where
    K: KesAlgorithm<Context = ()>,
{
    assert_eq!(level.total_periods, K::total_periods());

    for vector in &level.vectors {
        let seed_bytes = decode_hex(&vector.seed);
        let mut signing_key = K::gen_key_kes_from_seed_bytes(&seed_bytes)
            .expect("compact sum signing key generation");
        let verification_key =
            K::derive_verification_key(&signing_key).expect("compact sum verification key");

        let vk_bytes = K::raw_serialize_verification_key_kes(&verification_key);
        assert_eq!(
            vector.verification_key,
            hex::encode_upper(vk_bytes),
            "verification key mismatch"
        );

        let mut expected_periods = vector.tracked_periods.iter().peekable();
        let total_periods = K::total_periods();

        for period in 0..total_periods {
            if let Some(entry) = expected_periods.peek() {
                if entry.period == period {
                    assert_eq!(
                        entry.signature, entry.raw_signature,
                        "raw signature mismatch for period {period}"
                    );

                    let message = decode_hex(&entry.message);
                    let expected_sig = decode_hex(&entry.raw_signature);

                    let signature = K::sign_kes(&(), period, &message, &signing_key)
                        .expect("compact sum signing");
                    let raw_signature = K::raw_serialize_signature_kes(&signature);

                    assert_eq!(
                        expected_sig, raw_signature,
                        "signature mismatch at period {period}"
                    );

                    let deserialised = K::raw_deserialize_signature_kes(&raw_signature)
                        .expect("compact sum deserialise");
                    K::verify_kes(&(), &verification_key, period, &message, &deserialised)
                        .expect("compact sum verification");

                    expected_periods.next();
                }
            }

            if period + 1 == total_periods {
                K::forget_signing_key_kes(signing_key);
                break;
            }

            signing_key = K::update_kes(&(), signing_key, period)
                .expect("compact sum key update")
                .expect("compact sum key still valid");
        }

        assert!(
            expected_periods.next().is_none(),
            "unused compact sum entries"
        );
    }
}

fn decode_hex(input: &str) -> Vec<u8> {
    if input.is_empty() {
        Vec::new()
    } else {
        decode(input).expect("valid hex input")
    }
}
