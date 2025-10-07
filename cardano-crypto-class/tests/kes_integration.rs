use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::hash::KesHashAlgorithm;
use cardano_crypto_class::kes::{
    CompactSingleKes, CompactSum0Kes, CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, KesAlgorithm,
    KesError, KesMError, SingleKes, Sum3Kes, hash::Blake2b256,
};

fn message(label: &[u8], period: u64) -> Vec<u8> {
    let mut payload = label.to_vec();
    payload.extend_from_slice(&period.to_be_bytes());
    payload
}

#[derive(Debug)]
struct ExpectedCompactNode {
    vk_bytes: Vec<u8>,
    children: Option<(Box<ExpectedCompactNode>, Box<ExpectedCompactNode>)>,
}

fn build_expected_compact_tree(level: usize, seed: &[u8]) -> ExpectedCompactNode {
    if level == 0 {
        let signing_key = CompactSum0Kes::gen_key_kes_from_seed_bytes(seed)
            .expect("compact sum leaf signing key");
        let verification_key = CompactSum0Kes::derive_verification_key(&signing_key)
            .expect("compact sum leaf verification key");
        let vk_bytes = CompactSum0Kes::raw_serialize_verification_key_kes(&verification_key);
        CompactSum0Kes::forget_signing_key_kes(signing_key);
        ExpectedCompactNode {
            vk_bytes,
            children: None,
        }
    } else {
        let (left_seed, right_seed) = Blake2b256::expand_seed(seed);
        let left_node = build_expected_compact_tree(level - 1, &left_seed);
        let right_node = build_expected_compact_tree(level - 1, &right_seed);
        let combined = Blake2b256::hash_concat(&left_node.vk_bytes, &right_node.vk_bytes);
        ExpectedCompactNode {
            vk_bytes: combined,
            children: Some((Box::new(left_node), Box::new(right_node))),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

fn signature_size_for_level(level: usize) -> usize {
    match level {
        0 => CompactSum0Kes::SIGNATURE_SIZE,
        1 => CompactSum1Kes::SIGNATURE_SIZE,
        2 => CompactSum2Kes::SIGNATURE_SIZE,
        3 => CompactSum3Kes::SIGNATURE_SIZE,
        _ => panic!("unsupported compact sum level {level}"),
    }
}

fn verification_key_size_for_level(level: usize) -> usize {
    match level {
        0 => CompactSum0Kes::VERIFICATION_KEY_SIZE,
        1 => CompactSum1Kes::VERIFICATION_KEY_SIZE,
        2 => CompactSum2Kes::VERIFICATION_KEY_SIZE,
        3 => CompactSum3Kes::VERIFICATION_KEY_SIZE,
        _ => panic!("unsupported compact sum level {level}"),
    }
}

fn compute_period_path(mut period: u64, levels: usize) -> Vec<Direction> {
    let mut path = Vec::with_capacity(levels);
    for level in (0..levels).rev() {
        let half = 1u64 << level;
        if period < half {
            path.push(Direction::Left);
        } else {
            path.push(Direction::Right);
            period -= half;
        }
    }
    path
}

fn inspect_compact_sum_signature(
    level: usize,
    signature_bytes: &[u8],
    node: &ExpectedCompactNode,
    path: &[Direction],
) -> Vec<u8> {
    assert_eq!(
        signature_bytes.len(),
        signature_size_for_level(level),
        "unexpected signature size at level {level}"
    );

    if level == 0 {
        assert!(
            path.is_empty(),
            "leaf level should not have remaining path entries"
        );
        let vk_size = verification_key_size_for_level(0);
        let signature_len = signature_bytes.len();
        let (_, vk_bytes) = signature_bytes.split_at(signature_len - vk_size);
        assert_eq!(
            vk_bytes,
            node.vk_bytes.as_slice(),
            "leaf verification key bytes must match expected compact sum structure",
        );
        return vk_bytes.to_vec();
    }

    assert_eq!(
        path.len(),
        level,
        "direction path should have exactly {level} entries for level {level}",
    );
    let (direction, rest_path) = path
        .split_first()
        .expect("non-leaf level should have remaining path entries");

    let (left_node, right_node) = node
        .children
        .as_ref()
        .map(|(left, right)| (&**left, &**right))
        .expect("non-leaf node should provide children");

    let child_signature_size = signature_size_for_level(level - 1);
    let child_vk_size = verification_key_size_for_level(level - 1);
    let (child_signature_bytes, vk_other_bytes) = signature_bytes.split_at(child_signature_size);
    assert_eq!(
        vk_other_bytes.len(),
        child_vk_size,
        "embedded verification key length mismatch at level {level}",
    );

    match direction {
        Direction::Left => {
            assert_eq!(
                vk_other_bytes,
                right_node.vk_bytes.as_slice(),
                "right subtree verification key must be embedded when traversing left at level {level}",
            );
            let active_bytes = inspect_compact_sum_signature(
                level - 1,
                child_signature_bytes,
                left_node,
                rest_path,
            );
            let recomputed = Blake2b256::hash_concat(&active_bytes, vk_other_bytes);
            assert_eq!(
                recomputed, node.vk_bytes,
                "reconstructed verification key must match expected node at level {level}",
            );
            recomputed
        },
        Direction::Right => {
            assert_eq!(
                vk_other_bytes,
                left_node.vk_bytes.as_slice(),
                "left subtree verification key must be embedded when traversing right at level {level}",
            );
            let active_bytes = inspect_compact_sum_signature(
                level - 1,
                child_signature_bytes,
                right_node,
                rest_path,
            );
            let recomputed = Blake2b256::hash_concat(vk_other_bytes, &active_bytes);
            assert_eq!(
                recomputed, node.vk_bytes,
                "reconstructed verification key must match expected node at level {level}",
            );
            recomputed
        },
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
