use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{
    CompactSingleKes, CompactSum3Kes, KesAlgorithm, KesError, KesMError, SingleKes, Sum0Kes,
    Sum1Kes, Sum2Kes, Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes, Sum7Kes,
};

#[path = "sum_kes_structure.rs"]
mod sum_kes_structure;

use sum_kes_structure::{
    build_expected_compact_tree, build_expected_sum_tree, compute_period_path,
    inspect_compact_sum_signature, inspect_sum_signature, signature_size_for_level,
    sum_signature_size_for_level, sum_verification_key_size_for_level,
};

fn message(label: &[u8], period: u64) -> Vec<u8> {
    let mut payload = label.to_vec();
    payload.extend_from_slice(&period.to_be_bytes());
    payload
}

fn assert_sum_signature_components<Kes>(levels: usize, seed_byte: u8, label: &[u8])
where
    Kes: KesAlgorithm<Context = ()>,
{
    assert!(
        (0..=7).contains(&levels),
        "SumKES structural helper only supports levels 0 through 7",
    );

    let expected_periods = 1u64 << levels;
    assert_eq!(
        Kes::total_periods(),
        expected_periods,
        "SumKES total periods mismatch for level {levels}",
    );

    let expected_signature_len = sum_signature_size_for_level(levels);
    assert_eq!(
        Kes::SIGNATURE_SIZE,
        expected_signature_len,
        "SumKES signature size constant mismatch for level {levels}",
    );

    let expected_verification_key_len = sum_verification_key_size_for_level(levels);
    assert_eq!(
        Kes::VERIFICATION_KEY_SIZE,
        expected_verification_key_len,
        "SumKES verification key size constant mismatch for level {levels}",
    );

    let seed = vec![seed_byte; Kes::SEED_SIZE];
    let expected_tree = build_expected_sum_tree(levels, &seed);

    let signing_key_initial = Kes::gen_key_kes_from_seed_bytes(&seed).expect("sum signing key");
    let verification_key = Kes::derive_verification_key(&signing_key_initial)
        .expect("sum verification key derivation");
    let expected_root_bytes = expected_tree.vk_bytes.clone();
    assert_eq!(
        Kes::raw_serialize_verification_key_kes(&verification_key),
        expected_root_bytes,
        "derived verification key must match expected sum structure",
    );

    let total_periods = Kes::total_periods();
    let mut signing_key = Some(signing_key_initial);

    for period in 0..total_periods {
        let payload = message(label, period);
        let current_key = signing_key
            .take()
            .expect("sum signing key should be available for this period");
        let signature = Kes::sign_kes(&(), period, &payload, &current_key).expect("sum signing");
        Kes::verify_kes(&(), &verification_key, period, &payload, &signature)
            .expect("sum verification");

        let raw_signature = Kes::raw_serialize_signature_kes(&signature);
        assert_eq!(
            raw_signature.len(),
            expected_signature_len,
            "raw signature length must match SumKES size for level {levels}",
        );

        let path = compute_period_path(period, levels);
        inspect_sum_signature(levels, &raw_signature, &expected_tree, &path);

        let update_result =
            Kes::update_kes(&(), current_key, period).expect("sum update result should be ok");
        if period + 1 == total_periods {
            assert!(
                update_result.is_none(),
                "SumKES key must expire after final period",
            );
            break;
        }

        let next_key = update_result.expect("sum key should remain valid before final period");
        signing_key = Some(next_key);
    }
}

fn run_sum_signature_components(level: usize, seed_byte: u8, label: &[u8]) {
    match level {
        0 => {
            assert_eq!(Sum0Kes::total_periods(), 1, "Sum0 total periods must be 1");
            assert!(
                seed_byte == 0,
                "Sum0 seed byte should be 0 for deterministic single period coverage",
            );
            assert_sum_signature_components::<Sum0Kes>(level, seed_byte, label);
        },
        1 => assert_sum_signature_components::<Sum1Kes>(level, seed_byte, label),
        2 => assert_sum_signature_components::<Sum2Kes>(level, seed_byte, label),
        3 => assert_sum_signature_components::<Sum3Kes>(level, seed_byte, label),
        4 => assert_sum_signature_components::<Sum4Kes>(level, seed_byte, label),
        5 => assert_sum_signature_components::<Sum5Kes>(level, seed_byte, label),
        6 => assert_sum_signature_components::<Sum6Kes>(level, seed_byte, label),
        7 => assert_sum_signature_components::<Sum7Kes>(level, seed_byte, label),
        other => panic!("unsupported sum level {other}"),
    }
}
#[test]
fn single_kes_end_to_end_workflow_and_errors() {
    type Kes = SingleKes<Ed25519>;

    let seed = vec![0xA5; Kes::SEED_SIZE];
    let message = b"phase-05-single";

    let signing_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("single signing key");
    let verification_key =
        Kes::derive_verification_key(&signing_key).expect("single verification key derivation");

    let signature = Kes::sign_kes(&(), 0, message, &signing_key).expect("single signing");
    Kes::verify_kes(&(), &verification_key, 0, message, &signature).expect("single verification");

    let wrong_period_err = Kes::verify_kes(&(), &verification_key, 1, message, &signature)
        .expect_err("verification should fail for wrong period");
    assert!(matches!(
        wrong_period_err,
        KesError::PeriodOutOfRange {
            period: 1,
            max_period: 1
        }
    ));

    let mut tampered = message.to_vec();
    tampered[0] ^= 0x01;
    let tampered_err = Kes::verify_kes(&(), &verification_key, 0, &tampered, &signature)
        .expect_err("verification should fail for tampered message");
    assert!(matches!(tampered_err, KesError::VerificationFailed));

    let sign_err = Kes::sign_kes(&(), 1, message, &signing_key)
        .err()
        .expect("signing beyond period 0 must fail");
    assert!(matches!(
        sign_err,
        KesMError::Kes(KesError::PeriodOutOfRange {
            period: 1,
            max_period: 1
        })
    ));

    let expired = Kes::update_kes(&(), signing_key, 0).expect("single update succeeds");
    assert!(expired.is_none(), "SingleKES must expire after period 0");

    let raw_signature = Kes::raw_serialize_signature_kes(&signature);
    assert!(
        !raw_signature.is_empty(),
        "serialized signature must not be empty"
    );
    let mut truncated_signature = raw_signature.clone();
    truncated_signature.pop();
    assert!(
        Kes::raw_deserialize_signature_kes(&truncated_signature).is_none(),
        "truncated signature must be rejected"
    );

    let mut extended_signature = raw_signature.clone();
    extended_signature.push(0u8);
    assert!(
        Kes::raw_deserialize_signature_kes(&extended_signature).is_none(),
        "extended signature must be rejected"
    );

    let raw_verification_key = Kes::raw_serialize_verification_key_kes(&verification_key);
    assert!(
        !raw_verification_key.is_empty(),
        "serialized verification key must not be empty"
    );
    let mut truncated_vk = raw_verification_key.clone();
    truncated_vk.pop();
    assert!(
        Kes::raw_deserialize_verification_key_kes(&truncated_vk).is_none(),
        "truncated verification key must be rejected"
    );

    let mut extended_vk = raw_verification_key.clone();
    extended_vk.push(0u8);
    assert!(
        Kes::raw_deserialize_verification_key_kes(&extended_vk).is_none(),
        "extended verification key must be rejected"
    );
}

#[test]
fn sum3_kes_end_to_end_workflow_and_errors() {
    type Kes = Sum3Kes;

    let seed = vec![0x3C; Kes::SEED_SIZE];
    let total_periods = Kes::total_periods();
    assert!(total_periods > 1);

    let mut signing_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("sum signing key");
    let verification_key =
        Kes::derive_verification_key(&signing_key).expect("sum verification key derivation");

    let mut stored_signatures = Vec::with_capacity(total_periods as usize);

    for period in 0..total_periods {
        let payload = message(b"phase-05-sum", period);
        let signature = Kes::sign_kes(&(), period, &payload, &signing_key).expect("sum signing");
        Kes::verify_kes(&(), &verification_key, period, &payload, &signature)
            .expect("sum verification");

        let raw_signature = Kes::raw_serialize_signature_kes(&signature);
        stored_signatures.push((period, payload.clone(), raw_signature));

        if period + 1 == total_periods {
            let expired = Kes::update_kes(&(), signing_key, period)
                .expect("final update result should be ok");
            assert!(
                expired.is_none(),
                "SumKES key must expire after final period"
            );
            break;
        }

        signing_key = Kes::update_kes(&(), signing_key, period)
            .expect("sum key update succeeds")
            .expect("sum key should remain valid before final period");
    }

    for (period, payload, raw_signature) in &stored_signatures {
        let signature = Kes::raw_deserialize_signature_kes(raw_signature)
            .expect("stored signature should deserialize");
        Kes::verify_kes(&(), &verification_key, *period, payload, &signature)
            .expect("stored signature must remain valid");
    }

    let fresh_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("fresh sum signing key");
    let fresh_verification_key =
        Kes::derive_verification_key(&fresh_key).expect("fresh sum verification key derivation");
    let fresh_message = message(b"phase-05-sum", 0);
    let fresh_signature =
        Kes::sign_kes(&(), 0, &fresh_message, &fresh_key).expect("sum signing at period 0");
    Kes::verify_kes(
        &(),
        &fresh_verification_key,
        0,
        &fresh_message,
        &fresh_signature,
    )
    .expect("fresh signature should verify");

    let mut mismatched_vk = fresh_verification_key.clone();
    mismatched_vk[0] ^= 0xFF;
    let mismatch_err = Kes::verify_kes(&(), &mismatched_vk, 0, &fresh_message, &fresh_signature)
        .expect_err("verification should fail when verification key hash mismatches");
    assert!(matches!(mismatch_err, KesError::VerificationFailed));

    let out_of_range_message = message(b"phase-05-sum", total_periods);
    let out_of_range_err = Kes::sign_kes(&(), total_periods, &out_of_range_message, &fresh_key)
        .expect_err("signing beyond final period must fail");
    match out_of_range_err {
        KesMError::Kes(KesError::PeriodOutOfRange { .. }) => {},
        other => panic!("unexpected error when signing out of range: {other:?}"),
    }

    let raw_signature_example = stored_signatures
        .first()
        .map(|(_, _, sig)| sig.clone())
        .expect("at least one signature should be stored");
    assert!(
        !raw_signature_example.is_empty(),
        "serialized signature must not be empty"
    );

    let mut truncated_signature = raw_signature_example.clone();
    truncated_signature.pop();
    assert!(
        Kes::raw_deserialize_signature_kes(&truncated_signature).is_none(),
        "truncated SumKES signature must be rejected"
    );

    let mut extended_signature = raw_signature_example.clone();
    extended_signature.push(0u8);
    assert!(
        Kes::raw_deserialize_signature_kes(&extended_signature).is_none(),
        "extended SumKES signature must be rejected"
    );

    let raw_verification_key = Kes::raw_serialize_verification_key_kes(&verification_key);
    assert!(
        !raw_verification_key.is_empty(),
        "serialized verification key must not be empty"
    );

    let mut truncated_vk = raw_verification_key.clone();
    truncated_vk.pop();
    assert!(
        Kes::raw_deserialize_verification_key_kes(&truncated_vk).is_none(),
        "truncated verification key hash must be rejected"
    );

    let mut extended_vk = raw_verification_key.clone();
    extended_vk.push(0u8);
    assert!(
        Kes::raw_deserialize_verification_key_kes(&extended_vk).is_none(),
        "extended verification key hash must be rejected"
    );

    Kes::forget_signing_key_kes(fresh_key);
}

#[test]
fn sum3_kes_signature_components() {
    run_sum_signature_components(3, 0x3C, b"phase-05-sum-structure");
}

#[test]
fn sum7_kes_signature_components() {
    run_sum_signature_components(7, 0x7A, b"phase-05-sum7-structure");
}

#[test]
fn sum_kes_signature_components_levels() {
    let scenarios = [
        (0usize, 0x00u8, b"phase-05-sum0-structure".as_slice()),
        (1, 0x11, b"phase-05-sum1-structure".as_slice()),
        (2, 0x22, b"phase-05-sum2-structure".as_slice()),
        (4, 0x44, b"phase-05-sum4-structure".as_slice()),
        (5, 0x55, b"phase-05-sum5-structure".as_slice()),
        (6, 0x66, b"phase-05-sum6-structure".as_slice()),
    ];

    for (level, seed_byte, label) in scenarios {
        run_sum_signature_components(level, seed_byte, label);
    }
}

#[test]
fn sum0_kes_matches_singlekes_base_case() {
    type Sum0 = Sum0Kes;
    type Single = SingleKes<Ed25519>;

    assert_eq!(
        Sum0::SEED_SIZE,
        Single::SEED_SIZE,
        "Sum0 seed length must equal SingleKES"
    );
    assert_eq!(
        Sum0::VERIFICATION_KEY_SIZE,
        Single::VERIFICATION_KEY_SIZE,
        "Sum0 verification key size must equal SingleKES",
    );
    assert_eq!(
        Sum0::SIGNATURE_SIZE,
        Single::SIGNATURE_SIZE,
        "Sum0 signature size must equal SingleKES",
    );
    assert_eq!(
        Sum0::total_periods(),
        Single::total_periods(),
        "Sum0 total periods must match SingleKES",
    );

    let seed = vec![0x5Au8; Sum0::SEED_SIZE];
    let sum_signing = Sum0::gen_key_kes_from_seed_bytes(&seed).expect("Sum0 signing key");
    let single_signing = Single::gen_key_kes_from_seed_bytes(&seed).expect("SingleKES signing key");

    let sum_vk =
        Sum0::derive_verification_key(&sum_signing).expect("Sum0 verification key derivation");
    let single_vk = Single::derive_verification_key(&single_signing)
        .expect("SingleKES verification key derivation");

    let sum_vk_bytes = Sum0::raw_serialize_verification_key_kes(&sum_vk);
    let single_vk_bytes = Single::raw_serialize_verification_key_kes(&single_vk);
    assert_eq!(
        sum_vk_bytes, single_vk_bytes,
        "Sum0 verification key bytes must match SingleKES base case",
    );

    let message = b"phase-05-sum0-single-parity";
    let sum_signature = Sum0::sign_kes(&(), 0, message, &sum_signing).expect("Sum0 signing");
    let single_signature =
        Single::sign_kes(&(), 0, message, &single_signing).expect("SingleKES signing");

    let sum_raw_signature = Sum0::raw_serialize_signature_kes(&sum_signature);
    let single_raw_signature = Single::raw_serialize_signature_kes(&single_signature);
    assert_eq!(
        sum_raw_signature, single_raw_signature,
        "Sum0 signature bytes must align with SingleKES",
    );

    let single_from_sum = Single::raw_deserialize_signature_kes(&sum_raw_signature)
        .expect("Sum0 signature should decode via SingleKES");
    Single::verify_kes(&(), &single_vk, 0, message, &single_from_sum)
        .expect("SingleKES verification of Sum0 signature");

    let sum_from_single = Sum0::raw_deserialize_signature_kes(&single_raw_signature)
        .expect("SingleKES signature should decode via Sum0");
    Sum0::verify_kes(&(), &sum_vk, 0, message, &sum_from_single)
        .expect("Sum0 verification of SingleKES signature");

    let sum_expired = Sum0::update_kes(&(), sum_signing, 0).expect("Sum0 update succeeds");
    assert!(sum_expired.is_none(), "Sum0 must expire after period 0");

    let single_expired =
        Single::update_kes(&(), single_signing, 0).expect("SingleKES update succeeds");
    assert!(
        single_expired.is_none(),
        "SingleKES must expire after period 0",
    );
}

#[test]
fn compact_single_kes_end_to_end_workflow_and_errors() {
    type Kes = CompactSingleKes<Ed25519>;

    let seed = vec![0xB7; Kes::SEED_SIZE];
    let message = b"phase-05-compact-single";

    let signing_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact single signing key");
    let verification_key = Kes::derive_verification_key(&signing_key)
        .expect("compact single verification key derivation");

    let signature = Kes::sign_kes(&(), 0, message, &signing_key).expect("compact single signing");
    Kes::verify_kes(&(), &verification_key, 0, message, &signature)
        .expect("compact single verification");

    let mut wrong_vk_bytes = Kes::raw_serialize_verification_key_kes(&verification_key);
    wrong_vk_bytes[0] ^= 0x01;
    let wrong_vk = Kes::raw_deserialize_verification_key_kes(&wrong_vk_bytes)
        .expect("mutated verification key bytes should deserialize");
    Kes::verify_kes(&(), &wrong_vk, 0, message, &signature)
        .expect("embedded verification key should drive verification");

    let wrong_period_err = Kes::verify_kes(&(), &verification_key, 1, message, &signature)
        .expect_err("verification should fail for wrong period");
    assert!(matches!(
        wrong_period_err,
        KesError::PeriodOutOfRange {
            period: 1,
            max_period: 1
        }
    ));

    let mut tampered_message = message.to_vec();
    tampered_message[0] ^= 0x01;
    let tampered_err = Kes::verify_kes(&(), &verification_key, 0, &tampered_message, &signature)
        .expect_err("verification should fail for tampered message");
    assert!(matches!(tampered_err, KesError::VerificationFailed));

    let sign_err = Kes::sign_kes(&(), 1, message, &signing_key)
        .expect_err("signing beyond period 0 must fail");
    assert!(matches!(
        sign_err,
        KesMError::Kes(KesError::PeriodOutOfRange {
            period: 1,
            max_period: 1
        })
    ));

    let expired = Kes::update_kes(&(), signing_key, 0).expect("compact single update succeeds");
    assert!(
        expired.is_none(),
        "CompactSingleKES must expire after period 0"
    );

    let raw_signature = Kes::raw_serialize_signature_kes(&signature);
    assert!(
        !raw_signature.is_empty(),
        "serialized compact single signature must not be empty"
    );

    let mut truncated_signature = raw_signature.clone();
    truncated_signature.pop();
    assert!(
        Kes::raw_deserialize_signature_kes(&truncated_signature).is_none(),
        "truncated compact single signature must be rejected"
    );

    let mut extended_signature = raw_signature.clone();
    extended_signature.push(0u8);
    assert!(
        Kes::raw_deserialize_signature_kes(&extended_signature).is_none(),
        "extended compact single signature must be rejected"
    );

    let mut corrupted_signature = raw_signature.clone();
    let last_index = corrupted_signature.len() - 1;
    corrupted_signature[last_index] ^= 0x80;
    let corrupted_signature = Kes::raw_deserialize_signature_kes(&corrupted_signature)
        .expect("mutated compact single signature still decodes");
    let corrupted_err = Kes::verify_kes(&(), &verification_key, 0, message, &corrupted_signature)
        .expect_err("verification should fail when embedded verification key is corrupted");
    assert!(matches!(corrupted_err, KesError::VerificationFailed));

    let raw_verification_key = Kes::raw_serialize_verification_key_kes(&verification_key);
    assert!(
        !raw_verification_key.is_empty(),
        "serialized compact single verification key must not be empty"
    );

    let mut truncated_vk = raw_verification_key.clone();
    truncated_vk.pop();
    assert!(
        Kes::raw_deserialize_verification_key_kes(&truncated_vk).is_none(),
        "truncated compact single verification key must be rejected"
    );

    let mut extended_vk = raw_verification_key.clone();
    extended_vk.push(0u8);
    assert!(
        Kes::raw_deserialize_verification_key_kes(&extended_vk).is_none(),
        "extended compact single verification key must be rejected"
    );
}

#[test]
fn compact_sum3_kes_end_to_end_workflow_and_errors() {
    type Kes = CompactSum3Kes;

    let seed = vec![0x5F; Kes::SEED_SIZE];
    let total_periods = Kes::total_periods();
    assert!(total_periods > 1);

    let mut signing_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");
    let verification_key = Kes::derive_verification_key(&signing_key)
        .expect("compact sum verification key derivation");

    let mut stored_signatures = Vec::with_capacity(total_periods as usize);

    for period in 0..total_periods {
        let payload = message(b"phase-05-compact-sum", period);
        let signature =
            Kes::sign_kes(&(), period, &payload, &signing_key).expect("compact sum signing");
        Kes::verify_kes(&(), &verification_key, period, &payload, &signature)
            .expect("compact sum verification");

        let raw_signature = Kes::raw_serialize_signature_kes(&signature);
        stored_signatures.push((period, payload.clone(), raw_signature));

        if period + 1 == total_periods {
            let expired = Kes::update_kes(&(), signing_key, period)
                .expect("compact sum final update result should be ok");
            assert!(
                expired.is_none(),
                "CompactSumKES key must expire after final period"
            );
            break;
        }

        signing_key = Kes::update_kes(&(), signing_key, period)
            .expect("compact sum key update succeeds")
            .expect("compact sum key should remain valid before final period");
    }

    for (period, payload, raw_signature) in &stored_signatures {
        let signature = Kes::raw_deserialize_signature_kes(raw_signature)
            .expect("stored compact sum signature should deserialize");
        Kes::verify_kes(&(), &verification_key, *period, payload, &signature)
            .expect("stored compact sum signature must remain valid");
    }

    let fresh_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("fresh compact sum signing key");
    let fresh_verification_key = Kes::derive_verification_key(&fresh_key)
        .expect("fresh compact sum verification key derivation");
    let fresh_message = message(b"phase-05-compact-sum", 0);
    let fresh_signature =
        Kes::sign_kes(&(), 0, &fresh_message, &fresh_key).expect("compact sum signing at period 0");
    Kes::verify_kes(
        &(),
        &fresh_verification_key,
        0,
        &fresh_message,
        &fresh_signature,
    )
    .expect("fresh compact sum signature should verify");

    let mut mismatched_vk = fresh_verification_key.clone();
    mismatched_vk[0] ^= 0xFF;
    let mismatch_err = Kes::verify_kes(&(), &mismatched_vk, 0, &fresh_message, &fresh_signature)
        .expect_err("verification should fail when verification key hash mismatches");
    assert!(matches!(mismatch_err, KesError::VerificationFailed));

    let out_of_range_message = message(b"phase-05-compact-sum", total_periods);
    let out_of_range_err = Kes::sign_kes(&(), total_periods, &out_of_range_message, &fresh_key)
        .err()
        .expect("signing beyond final compact sum period must fail");
    match out_of_range_err {
        KesMError::Kes(KesError::PeriodOutOfRange { .. }) => {},
        other => panic!("unexpected error when signing out of range: {other:?}"),
    }

    let raw_signature_example = stored_signatures
        .first()
        .map(|(_, _, sig)| sig.clone())
        .expect("at least one compact sum signature should be stored");
    assert!(
        !raw_signature_example.is_empty(),
        "serialized compact sum signature must not be empty"
    );

    let mut truncated_signature = raw_signature_example.clone();
    truncated_signature.pop();
    assert!(
        Kes::raw_deserialize_signature_kes(&truncated_signature).is_none(),
        "truncated compact sum signature must be rejected"
    );

    let mut extended_signature = raw_signature_example.clone();
    extended_signature.push(0u8);
    assert!(
        Kes::raw_deserialize_signature_kes(&extended_signature).is_none(),
        "extended compact sum signature must be rejected"
    );

    let mut corrupted_signature_bytes = raw_signature_example.clone();
    let last_index = corrupted_signature_bytes.len() - 1;
    corrupted_signature_bytes[last_index] ^= 0x80;
    let corrupted_signature = Kes::raw_deserialize_signature_kes(&corrupted_signature_bytes)
        .expect("mutated compact sum signature still decodes");
    let corrupted_err = Kes::verify_kes(
        &(),
        &verification_key,
        stored_signatures[0].0,
        &stored_signatures[0].1,
        &corrupted_signature,
    )
    .expect_err("verification should fail when embedded verification data is corrupted");
    assert!(matches!(corrupted_err, KesError::VerificationFailed));

    let raw_verification_key = Kes::raw_serialize_verification_key_kes(&verification_key);
    assert!(
        !raw_verification_key.is_empty(),
        "serialized compact sum verification key must not be empty"
    );

    let mut truncated_vk = raw_verification_key.clone();
    truncated_vk.pop();
    assert!(
        Kes::raw_deserialize_verification_key_kes(&truncated_vk).is_none(),
        "truncated compact sum verification key must be rejected"
    );

    let mut extended_vk = raw_verification_key.clone();
    extended_vk.push(0u8);
    assert!(
        Kes::raw_deserialize_verification_key_kes(&extended_vk).is_none(),
        "extended compact sum verification key must be rejected"
    );

    let mut corrupted_vk_bytes = raw_verification_key.clone();
    corrupted_vk_bytes[0] ^= 0x01;
    let corrupted_vk = Kes::raw_deserialize_verification_key_kes(&corrupted_vk_bytes)
        .expect("mutated verification key hash should deserialize");
    let corrupted_vk_err = Kes::verify_kes(&(), &corrupted_vk, 0, &fresh_message, &fresh_signature)
        .expect_err("verification should fail for corrupted verification key hash");
    assert!(matches!(corrupted_vk_err, KesError::VerificationFailed));

    Kes::forget_signing_key_kes(fresh_key);
}

#[test]
fn compact_sum3_kes_signature_components() {
    type Kes = CompactSum3Kes;
    const LEVELS: usize = 3;

    let seed = vec![0x5F; Kes::SEED_SIZE];
    let expected_tree = build_expected_compact_tree(LEVELS, &seed);

    let signing_key_initial =
        Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");
    let verification_key = Kes::derive_verification_key(&signing_key_initial)
        .expect("compact sum verification key derivation");
    let expected_root_bytes = expected_tree.vk_bytes.clone();
    assert_eq!(
        Kes::raw_serialize_verification_key_kes(&verification_key),
        expected_root_bytes,
        "derived verification key must match expected compact sum structure",
    );

    let total_periods = Kes::total_periods();
    let expected_signature_len = signature_size_for_level(LEVELS);
    let mut signing_key = Some(signing_key_initial);

    for period in 0..total_periods {
        let payload = message(b"phase-05-compact-sum-structure", period);
        let current_key = signing_key
            .take()
            .expect("compact sum signing key should be available for this period");
        let signature =
            Kes::sign_kes(&(), period, &payload, &current_key).expect("compact sum signing");
        Kes::verify_kes(&(), &verification_key, period, &payload, &signature)
            .expect("compact sum verification");

        let raw_signature = Kes::raw_serialize_signature_kes(&signature);
        assert_eq!(
            raw_signature.len(),
            expected_signature_len,
            "raw signature length must match CompactSum3 size",
        );

        let path = compute_period_path(period, LEVELS);
        let derived_vk_bytes =
            inspect_compact_sum_signature(LEVELS, &raw_signature, &expected_tree, &path);
        assert_eq!(
            derived_vk_bytes, expected_root_bytes,
            "root verification key bytes mismatch for period {period}"
        );

        let update_result =
            Kes::update_kes(&(), current_key, period).expect("final update result should be ok");
        if period + 1 == total_periods {
            assert!(
                update_result.is_none(),
                "CompactSumKES key must expire after final period"
            );
            break;
        }

        let next_key =
            update_result.expect("compact sum key should remain valid before final period");
        signing_key = Some(next_key);
    }
}
