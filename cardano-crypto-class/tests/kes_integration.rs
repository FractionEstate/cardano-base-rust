use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::hash::KesHashAlgorithm;
use cardano_crypto_class::kes::{
    CompactSingleKes, CompactSum2Kes, CompactSum3Kes, KesAlgorithm, KesError, KesMError, SingleKes,
    Sum3Kes, hash::Blake2b256,
};

fn message(label: &[u8], period: u64) -> Vec<u8> {
    let mut payload = label.to_vec();
    payload.extend_from_slice(&period.to_be_bytes());
    payload
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
    type Child = CompactSum2Kes;

    let seed = vec![0x5F; Kes::SEED_SIZE];
    let (left_seed, right_seed) = Blake2b256::expand_seed(&seed);

    let left_signing_key =
        Child::gen_key_kes_from_seed_bytes(&left_seed).expect("left subtree signing key");
    let left_verification_key =
        Child::derive_verification_key(&left_signing_key).expect("left subtree verification key");
    Child::forget_signing_key_kes(left_signing_key);

    let right_signing_key =
        Child::gen_key_kes_from_seed_bytes(&right_seed).expect("right subtree signing key");
    let right_verification_key =
        Child::derive_verification_key(&right_signing_key).expect("right subtree verification key");
    Child::forget_signing_key_kes(right_signing_key);

    let mut signing_key = Kes::gen_key_kes_from_seed_bytes(&seed).expect("compact sum signing key");

    let payload_left = message(b"phase-05-compact-sum-structure", 0);
    let signature_left = Kes::sign_kes(&(), 0, &payload_left, &signing_key)
        .expect("compact sum signing at period 0");

    let raw_signature_left = Kes::raw_serialize_signature_kes(&signature_left);
    let (left_sigma_bytes, left_other_bytes) = raw_signature_left.split_at(Child::SIGNATURE_SIZE);
    let extracted_left_signature =
        Child::raw_deserialize_signature_kes(left_sigma_bytes).expect("left subtree signature");
    let extracted_left_vk_other = Child::raw_deserialize_verification_key_kes(left_other_bytes)
        .expect("right subtree verification key in left signature");

    Child::verify_kes(
        &(),
        &left_verification_key,
        0,
        &payload_left,
        &extracted_left_signature,
    )
    .expect("embedded compact sum signature must verify with left subtree key");
    assert_eq!(
        extracted_left_vk_other, right_verification_key,
        "left period must carry right subtree verification key"
    );

    let t_half = Kes::total_periods() / 2;
    for period in 0..t_half {
        signing_key = Kes::update_kes(&(), signing_key, period)
            .expect("compact sum update succeeds")
            .expect("key must remain valid before final period");
    }

    let payload_right = message(b"phase-05-compact-sum-structure", t_half);
    let signature_right = Kes::sign_kes(&(), t_half, &payload_right, &signing_key)
        .expect("compact sum signing at right subtree period");

    let raw_signature_right = Kes::raw_serialize_signature_kes(&signature_right);
    let (right_sigma_bytes, right_other_bytes) =
        raw_signature_right.split_at(Child::SIGNATURE_SIZE);
    let extracted_right_signature =
        Child::raw_deserialize_signature_kes(right_sigma_bytes).expect("right subtree signature");
    let extracted_right_vk_other = Child::raw_deserialize_verification_key_kes(right_other_bytes)
        .expect("left subtree verification key in right signature");

    Child::verify_kes(
        &(),
        &right_verification_key,
        0,
        &payload_right,
        &extracted_right_signature,
    )
    .expect("embedded compact sum signature must verify with right subtree key");
    assert_eq!(
        extracted_right_vk_other, left_verification_key,
        "right period must carry left subtree verification key"
    );

    Kes::forget_signing_key_kes(signing_key);
}
