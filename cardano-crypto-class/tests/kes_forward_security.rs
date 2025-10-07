use cardano_crypto_class::kes::{CompactSum4Kes, KesAlgorithm, KesError, KesMError, Sum4Kes};

fn message_for_period(label: &[u8], period: u64) -> Vec<u8> {
    let mut message = label.to_vec();
    message.extend_from_slice(&period.to_be_bytes());
    message
}

/// Port of the forward-security regressions from `Test.Crypto.KES` in the
/// Haskell `cardano-crypto-class` suite. Confirms that evolved signing keys
/// preserve historical signature validity while refusing to sign for past
/// periods once evolution has occurred.
fn assert_one_way_evolution<K>(label: &[u8])
where
    K: KesAlgorithm<Context = ()>,
{
    let seed = vec![0x42; K::SEED_SIZE];
    let mut signing_key = K::gen_key_kes_from_seed_bytes(&seed).expect("generate signing key");
    let verification_key =
        K::derive_verification_key(&signing_key).expect("derive verification key");

    let total_periods = K::total_periods();
    assert!(total_periods > 1, "KES must support more than one period");
    let mut stored = Vec::with_capacity(total_periods as usize);

    for period in 0..total_periods {
        let message = message_for_period(label, period);
        let signature =
            K::sign_kes(&(), period, &message, &signing_key).expect("sign at current period");
        K::verify_kes(&(), &verification_key, period, &message, &signature)
            .expect("verify signature for current period");

        let raw_signature = K::raw_serialize_signature_kes(&signature);
        stored.push((period, message, raw_signature));

        let update_result = K::update_kes(&(), signing_key, period).expect("key evolution result");

        match update_result {
            Some(next_key) => {
                for (old_period, old_message, _) in &stored {
                    match K::sign_kes(&(), *old_period, old_message, &next_key) {
                        Ok(signature) => {
                            let verification = K::verify_kes(
                                &(),
                                &verification_key,
                                *old_period,
                                old_message,
                                &signature,
                            );
                            assert!(
                                verification.is_err(),
                                "period {period}: evolved key produced a valid signature for past period {old_period}"
                            );
                        },
                        Err(err) => {
                            assert!(matches!(
                                err,
                                KesMError::Kes(KesError::PeriodOutOfRange { .. })
                                    | KesMError::Kes(KesError::KeyExpired)
                            ));
                        },
                    }
                }

                signing_key = next_key;
            },
            None => {
                assert_eq!(
                    period + 1,
                    total_periods,
                    "key expired before the final supported period"
                );
                break;
            },
        }
    }

    for (period, message, raw_signature) in stored {
        let signature =
            K::raw_deserialize_signature_kes(&raw_signature).expect("deserialize stored signature");
        K::verify_kes(&(), &verification_key, period, &message, &signature)
            .expect("historical signature remains valid");
    }
}

/// Ensure that attempting to evolve a key with a stale period parameter fails,
/// mirroring the `cardano-crypto-class` Haskell regression that enforces
/// monotonic forward evolution.
fn assert_rewind_is_impossible<K>(label: &[u8])
where
    K: KesAlgorithm<Context = ()>,
{
    let seed = vec![0x24; K::SEED_SIZE];
    let mut signing_key = K::gen_key_kes_from_seed_bytes(&seed).expect("generate signing key");
    let verification_key =
        K::derive_verification_key(&signing_key).expect("derive verification key");

    let total_periods = K::total_periods();
    assert!(total_periods > 1, "KES must support more than one period");
    let boundary_period = (total_periods / 2).max(1);

    for period in 0..boundary_period {
        let message = message_for_period(label, period);
        let signature =
            K::sign_kes(&(), period, &message, &signing_key).expect("sign before boundary");
        K::verify_kes(&(), &verification_key, period, &message, &signature)
            .expect("verify signature before boundary");

        signing_key = K::update_kes(&(), signing_key, period)
            .expect("update result")
            .expect("key remains valid before boundary");
    }

    let boundary_message = message_for_period(label, boundary_period);
    let boundary_signature = K::sign_kes(&(), boundary_period, &boundary_message, &signing_key)
        .expect("sign at boundary period");
    K::verify_kes(
        &(),
        &verification_key,
        boundary_period,
        &boundary_message,
        &boundary_signature,
    )
    .expect("verify signature at boundary period");

    let rewind_attempt = K::update_kes(&(), signing_key, boundary_period - 1);
    match rewind_attempt {
        Ok(_) => panic!("evolved key unexpectedly allowed rewind to earlier period"),
        Err(KesMError::Kes(KesError::PeriodOutOfRange { period, .. })) => {
            assert_eq!(period, boundary_period - 1);
        },
        Err(KesMError::Kes(KesError::KeyExpired)) => {},
        Err(other) => panic!("unexpected error while attempting rewind: {other:?}"),
    }
}

#[test]
fn sum_kes_evolution_is_one_way() {
    assert_one_way_evolution::<Sum4Kes>(b"sum-one-way");
    assert_rewind_is_impossible::<Sum4Kes>(b"sum-rewind");
}

#[test]
fn compact_sum_kes_evolution_is_one_way() {
    assert_one_way_evolution::<CompactSum4Kes>(b"compact-one-way");
    assert_rewind_is_impossible::<CompactSum4Kes>(b"compact-rewind");
}

#[test]
fn compact_sum_forward_security_preserves_past_signature_validity() {
    type Kes = CompactSum4Kes;
    let seed = vec![0xA5; Kes::SEED_SIZE];

    let mut signing_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");
    let verification_key =
        Kes::derive_verification_key(&signing_key).expect("compact sum verification key");
    let total_periods = Kes::total_periods();
    let mut stored_signatures = Vec::with_capacity(total_periods as usize);

    for period in 0..total_periods {
        let message = message_for_period(b"forward-security", period);
        let signature =
            Kes::sign_kes(&(), period, &message, &signing_key).expect("compact sum signing");
        let raw_signature = Kes::raw_serialize_signature_kes(&signature);

        let deserialised = Kes::raw_deserialize_signature_kes(&raw_signature)
            .expect("compact sum signature deserialise");
        Kes::verify_kes(&(), &verification_key, period, &message, &deserialised)
            .expect("compact sum verify");

        stored_signatures.push((period, message, raw_signature));

        if period + 1 == total_periods {
            let expired = Kes::update_kes(&(), signing_key, period)
                .expect("final compact sum update succeeds");
            assert!(
                expired.is_none(),
                "compact sum key must expire after final period"
            );
            break;
        }

        let next_key = Kes::update_kes(&(), signing_key, period)
            .expect("compact sum update succeeds")
            .expect("compact sum key should remain valid before final period");

        for (old_period, old_message, _) in &stored_signatures {
            match Kes::sign_kes(&(), *old_period, old_message, &next_key) {
                Ok(signature) => {
                    let verification = Kes::verify_kes(
                        &(),
                        &verification_key,
                        *old_period,
                        old_message,
                        &signature,
                    );
                    assert!(
                        verification.is_err(),
                        "period {period}: evolved key produced a valid signature for past period {old_period}"
                    );
                },
                Err(err) => {
                    assert!(matches!(
                        err,
                        KesMError::Kes(KesError::PeriodOutOfRange { .. })
                            | KesMError::Kes(KesError::KeyExpired)
                    ));
                },
            }
        }

        signing_key = next_key;
    }

    for (period, message, raw_signature) in stored_signatures {
        let signature = Kes::raw_deserialize_signature_kes(&raw_signature)
            .expect("compact sum stored signature");
        Kes::verify_kes(&(), &verification_key, period, &message, &signature)
            .expect("stored signature remains valid");
    }
}
#[test]
fn compact_sum_signatures_are_smaller_than_sum_signatures() {
    assert!(
        CompactSum4Kes::SIGNATURE_SIZE < Sum4Kes::SIGNATURE_SIZE,
        "compact sum signatures should be smaller than sum signatures"
    );
    assert!(
        CompactSum4Kes::VERIFICATION_KEY_SIZE == Sum4Kes::VERIFICATION_KEY_SIZE,
        "verification key size parity expected"
    );
}

#[cfg(feature = "serde")]
mod vector_forward_security {
    use super::*;
    use cardano_crypto_class::kes::{
        CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, CompactSum4Kes, CompactSum5Kes,
        CompactSum6Kes, CompactSum7Kes, Sum1Kes, Sum2Kes, Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes,
        Sum7Kes,
    };
    use cardano_test_vectors::kes;
    use hex::decode;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct PeriodEvolutionVectors {
        levels: Vec<PeriodEvolutionLevel>,
    }

    #[derive(Deserialize)]
    struct PeriodEvolutionLevel {
        level: u8,
        total_periods: u64,
        vectors: Vec<PeriodEvolutionVector>,
    }

    #[derive(Deserialize)]
    struct PeriodEvolutionVector {
        test_name: String,
        seed: String,
        periods: Vec<PeriodVectorEntry>,
    }

    #[derive(Deserialize)]
    struct PeriodVectorEntry {
        period: u64,
        message: String,
        raw_signature: String,
    }

    #[test]
    fn sum_kes_forward_security_matches_period_evolution_vectors() {
        let fixture = kes::get("sum_kes_period_evolution_vectors.json")
            .expect("sum KES period evolution vectors present");
        let parsed: PeriodEvolutionVectors =
            serde_json::from_str(fixture).expect("valid sum KES period evolution JSON");

        for level in &parsed.levels {
            match level.level {
                1 => exercise_forward_security::<Sum1Kes>(level),
                2 => exercise_forward_security::<Sum2Kes>(level),
                3 => exercise_forward_security::<Sum3Kes>(level),
                4 => exercise_forward_security::<Sum4Kes>(level),
                5 => exercise_forward_security::<Sum5Kes>(level),
                6 => exercise_forward_security::<Sum6Kes>(level),
                7 => exercise_forward_security::<Sum7Kes>(level),
                other => panic!("unexpected SumKES evolution level {other}"),
            }
        }
    }

    #[test]
    fn compact_sum_kes_forward_security_matches_period_evolution_vectors() {
        let fixture = kes::get("compact_sum_kes_period_evolution_vectors.json")
            .expect("compact sum KES period evolution vectors present");
        let parsed: PeriodEvolutionVectors =
            serde_json::from_str(fixture).expect("valid compact sum KES period evolution JSON");

        for level in &parsed.levels {
            match level.level {
                1 => exercise_forward_security::<CompactSum1Kes>(level),
                2 => exercise_forward_security::<CompactSum2Kes>(level),
                3 => exercise_forward_security::<CompactSum3Kes>(level),
                4 => exercise_forward_security::<CompactSum4Kes>(level),
                5 => exercise_forward_security::<CompactSum5Kes>(level),
                6 => exercise_forward_security::<CompactSum6Kes>(level),
                7 => exercise_forward_security::<CompactSum7Kes>(level),
                other => panic!("unexpected CompactSumKES evolution level {other}"),
            }
        }
    }

    fn exercise_forward_security<K>(level: &PeriodEvolutionLevel)
    where
        K: KesAlgorithm<Context = ()>,
    {
        assert_eq!(level.total_periods, K::total_periods());

        for vector in &level.vectors {
            let seed_bytes = decode_seed(&vector.seed, K::SEED_SIZE);
            let initial_key =
                K::gen_key_kes_from_seed_bytes(&seed_bytes).expect("generate signing key");
            let verification_key =
                K::derive_verification_key(&initial_key).expect("derive verification key");
            let mut signing_key = Some(initial_key);

            let mut stored = Vec::with_capacity(level.total_periods as usize);

            for (index, entry) in vector.periods.iter().enumerate() {
                let active_key = signing_key
                    .take()
                    .expect("signing key available for current period");
                assert_eq!(entry.period, index as u64, "period ordering mismatch");

                let message = decode_hex(&entry.message);
                let expected_raw_signature = decode_hex(&entry.raw_signature);
                let expected_signature = K::raw_deserialize_signature_kes(&expected_raw_signature)
                    .expect("period signature deserialise");

                K::verify_kes(
                    &(),
                    &verification_key,
                    entry.period,
                    &message,
                    &expected_signature,
                )
                .expect("stored signature verification");

                let produced =
                    K::sign_kes(&(), entry.period, &message, &active_key).expect("kes signing");
                let produced_raw = K::raw_serialize_signature_kes(&produced);
                assert_eq!(
                    expected_raw_signature, produced_raw,
                    "signature mismatch for vector {} period {}",
                    vector.test_name, entry.period
                );

                stored.push((entry.period, message, expected_raw_signature));

                if index + 1 != level.total_periods as usize {
                    let next_key = K::update_kes(&(), active_key, entry.period)
                        .expect("key update result")
                        .expect("key remains valid before final period");

                    // The evolved key must not produce valid signatures for any previous period.
                    for (old_period, old_message, _) in &stored {
                        match K::sign_kes(&(), *old_period, old_message, &next_key) {
                            Ok(signature) => {
                                let verification = K::verify_kes(
                                    &(),
                                    &verification_key,
                                    *old_period,
                                    old_message,
                                    &signature,
                                );
                                assert!(
                                    verification.is_err(),
                                    "level {} vector {} period {}: evolved key produced a valid signature for past period {}",
                                    level.level,
                                    vector.test_name,
                                    entry.period,
                                    old_period
                                );
                            },
                            Err(err) => {
                                assert!(matches!(
                                    err,
                                    KesMError::Kes(KesError::PeriodOutOfRange { .. })
                                        | KesMError::Kes(KesError::KeyExpired)
                                ))
                            },
                        }
                    }

                    signing_key = Some(next_key);
                } else {
                    let expired = K::update_kes(&(), active_key, entry.period)
                        .expect("final key update result");
                    assert!(
                        expired.is_none(),
                        "level {} vector {}: key must expire after final period",
                        level.level,
                        vector.test_name
                    );
                }
            }

            for (period, message, raw_signature) in stored {
                let signature = K::raw_deserialize_signature_kes(&raw_signature)
                    .expect("stored signature deserialise");
                K::verify_kes(&(), &verification_key, period, &message, &signature)
                    .expect("stored signature remains valid");
            }
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
            decode(input).expect("valid hex input")
        }
    }
}
