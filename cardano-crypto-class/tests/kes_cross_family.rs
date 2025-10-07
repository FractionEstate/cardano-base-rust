use cardano_crypto_class::kes::{
    CompactSum0Kes, CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, CompactSum4Kes, KesAlgorithm,
    Sum0Kes, Sum1Kes, Sum2Kes, Sum3Kes, Sum4Kes,
};

fn exercise_sum_compact_pair<S, C>(label: &str, seed_byte: u8)
where
    S: KesAlgorithm<Context = ()>,
    C: KesAlgorithm<Context = ()>,
    S::VerificationKey: Clone + PartialEq + std::fmt::Debug,
    C::VerificationKey: Clone + PartialEq + std::fmt::Debug,
{
    assert_eq!(
        S::SEED_SIZE,
        C::SEED_SIZE,
        "{label}: seed size mismatch between Sum and CompactSum"
    );

    let seed = vec![seed_byte; S::SEED_SIZE];
    let mut sum_key = Some(S::gen_key_kes_from_seed_bytes(&seed).expect("sum signing key"));
    let mut compact_key =
        Some(C::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key"));

    let sum_vk = S::derive_verification_key(sum_key.as_ref().expect("sum key present"))
        .expect("sum verification key");
    let compact_vk = C::derive_verification_key(compact_key.as_ref().expect("compact key present"))
        .expect("compact verification key");

    let sum_vk_bytes = S::raw_serialize_verification_key_kes(&sum_vk);
    let compact_vk_bytes = C::raw_serialize_verification_key_kes(&compact_vk);
    assert_eq!(
        sum_vk_bytes, compact_vk_bytes,
        "{label}: verification key bytes must match between Sum and CompactSum",
    );

    let compact_vk_from_sum = C::raw_deserialize_verification_key_kes(&sum_vk_bytes)
        .expect("compact sum accepts sum verification key bytes");
    let sum_vk_from_compact = S::raw_deserialize_verification_key_kes(&compact_vk_bytes)
        .expect("sum accepts compact verification key bytes");

    assert_eq!(
        compact_vk, compact_vk_from_sum,
        "{label}: CompactSum verification key mismatch when reconstructed from Sum bytes",
    );
    assert_eq!(
        sum_vk, sum_vk_from_compact,
        "{label}: Sum verification key mismatch when reconstructed from CompactSum bytes",
    );

    let total_periods = S::total_periods();
    assert_eq!(
        total_periods,
        C::total_periods(),
        "{label}: total periods must align",
    );

    for period in 0..total_periods {
        let sum_active = sum_key.take().expect("sum key available for period");
        let compact_active = compact_key
            .take()
            .expect("compact key available for period");
        let message = format!("{label}-period-{period}").into_bytes();

        let sum_signature =
            S::sign_kes(&(), period, &message, &sum_active).expect("sum signing succeeds");
        let compact_signature =
            C::sign_kes(&(), period, &message, &compact_active).expect("compact signing succeeds");

        S::verify_kes(&(), &sum_vk, period, &message, &sum_signature)
            .expect("sum verification succeeds");
        C::verify_kes(&(), &compact_vk, period, &message, &compact_signature)
            .expect("compact verification succeeds");

        S::verify_kes(&(), &sum_vk_from_compact, period, &message, &sum_signature)
            .expect("sum verification stable via compact bytes");
        C::verify_kes(
            &(),
            &compact_vk_from_sum,
            period,
            &message,
            &compact_signature,
        )
        .expect("compact verification stable via sum bytes");

        let sum_next = S::update_kes(&(), sum_active, period).expect("sum update succeeds");
        let compact_next =
            C::update_kes(&(), compact_active, period).expect("compact update succeeds");

        if period + 1 == total_periods {
            assert!(
                sum_next.is_none(),
                "{label}: sum key should expire after the final period",
            );
            assert!(
                compact_next.is_none(),
                "{label}: compact sum key should expire after the final period",
            );
        } else {
            let next_sum_key = sum_next.expect("sum key remains valid before final period");
            let next_compact_key =
                compact_next.expect("compact key remains valid before final period");

            let next_sum_vk =
                S::derive_verification_key(&next_sum_key).expect("sum verification key stable");
            let next_compact_vk = C::derive_verification_key(&next_compact_key)
                .expect("compact verification key stable");

            assert_eq!(
                sum_vk, next_sum_vk,
                "{label}: sum verification key must remain stable after evolution",
            );
            assert_eq!(
                compact_vk, next_compact_vk,
                "{label}: compact verification key must remain stable after evolution",
            );

            sum_key = Some(next_sum_key);
            compact_key = Some(next_compact_key);
        }
    }
}

#[test]
fn sum_and_compact_sum_cross_family_parity() {
    exercise_sum_compact_pair::<Sum0Kes, CompactSum0Kes>("level-0", 0x31);
    exercise_sum_compact_pair::<Sum1Kes, CompactSum1Kes>("level-1", 0x32);
    exercise_sum_compact_pair::<Sum2Kes, CompactSum2Kes>("level-2", 0x33);
    exercise_sum_compact_pair::<Sum3Kes, CompactSum3Kes>("level-3", 0x34);
    exercise_sum_compact_pair::<Sum4Kes, CompactSum4Kes>("level-4", 0x35);
}
