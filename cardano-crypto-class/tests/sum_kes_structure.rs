#![allow(dead_code)]

use cardano_crypto_class::kes::hash::Blake2b256;
use cardano_crypto_class::kes::{
    CompactSum0Kes, CompactSum1Kes, CompactSum2Kes, CompactSum3Kes, Sum0Kes, Sum1Kes, Sum2Kes,
    Sum3Kes, Sum4Kes, Sum5Kes, Sum6Kes, Sum7Kes,
};
use cardano_crypto_class::{KesAlgorithm, KesHashAlgorithm};

#[derive(Debug)]
pub struct ExpectedCompactNode {
    pub vk_bytes: Vec<u8>,
    pub children: Option<(Box<ExpectedCompactNode>, Box<ExpectedCompactNode>)>,
}

pub type ExpectedSumNode = ExpectedCompactNode;

/// Build the expected compact SumKES verification tree for the provided seed.
///
/// # Panics
///
/// Panics if key generation fails for the provided seed material.
#[must_use]
pub fn build_expected_compact_tree(level: usize, seed: &[u8]) -> ExpectedCompactNode {
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

/// Build the expected SumKES verification tree for the provided seed.
///
/// # Panics
///
/// Panics if key generation fails for the provided seed material.
#[must_use]
pub fn build_expected_sum_tree(level: usize, seed: &[u8]) -> ExpectedSumNode {
    if level == 0 {
        let signing_key = Sum0Kes::gen_key_kes_from_seed_bytes(seed).expect("sum leaf signing key");
        let verification_key =
            Sum0Kes::derive_verification_key(&signing_key).expect("sum leaf verification key");
        let vk_bytes = Sum0Kes::raw_serialize_verification_key_kes(&verification_key);
        Sum0Kes::forget_signing_key_kes(signing_key);
        ExpectedSumNode {
            vk_bytes,
            children: None,
        }
    } else {
        let (left_seed, right_seed) = Blake2b256::expand_seed(seed);
        let left_node = build_expected_sum_tree(level - 1, &left_seed);
        let right_node = build_expected_sum_tree(level - 1, &right_seed);
        let combined = Blake2b256::hash_concat(&left_node.vk_bytes, &right_node.vk_bytes);
        ExpectedSumNode {
            vk_bytes: combined,
            children: Some((Box::new(left_node), Box::new(right_node))),
        }
    }
}

const SUM_SIGNATURE_SIZES: [usize; 8] = [
    Sum0Kes::SIGNATURE_SIZE,
    Sum1Kes::SIGNATURE_SIZE,
    Sum2Kes::SIGNATURE_SIZE,
    Sum3Kes::SIGNATURE_SIZE,
    Sum4Kes::SIGNATURE_SIZE,
    Sum5Kes::SIGNATURE_SIZE,
    Sum6Kes::SIGNATURE_SIZE,
    Sum7Kes::SIGNATURE_SIZE,
];

const SUM_VERIFICATION_KEY_SIZES: [usize; 8] = [
    Sum0Kes::VERIFICATION_KEY_SIZE,
    Sum1Kes::VERIFICATION_KEY_SIZE,
    Sum2Kes::VERIFICATION_KEY_SIZE,
    Sum3Kes::VERIFICATION_KEY_SIZE,
    Sum4Kes::VERIFICATION_KEY_SIZE,
    Sum5Kes::VERIFICATION_KEY_SIZE,
    Sum6Kes::VERIFICATION_KEY_SIZE,
    Sum7Kes::VERIFICATION_KEY_SIZE,
];

/// Return the SumKES signature size for a given tree level.
///
/// # Panics
///
/// Panics if `level` is greater than 7.
#[must_use]
pub fn sum_signature_size_for_level(level: usize) -> usize {
    SUM_SIGNATURE_SIZES
        .get(level)
        .copied()
        .expect("SumKES level must be between 0 and 7")
}

/// Return the SumKES verification key size for a given tree level.
///
/// # Panics
///
/// Panics if `level` is greater than 7.
#[must_use]
pub fn sum_verification_key_size_for_level(level: usize) -> usize {
    SUM_VERIFICATION_KEY_SIZES
        .get(level)
        .copied()
        .expect("SumKES level must be between 0 and 7")
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

#[must_use]
pub fn compute_period_path(mut period: u64, levels: usize) -> Vec<Direction> {
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

/// Inspect a SumKES signature against the expected tree structure.
///
/// # Panics
///
/// Panics if the signature layout does not match the expected structure for `level`.
pub fn inspect_sum_signature(
    level: usize,
    signature_bytes: &[u8],
    node: &ExpectedSumNode,
    path: &[Direction],
) {
    assert_eq!(
        signature_bytes.len(),
        sum_signature_size_for_level(level),
        "unexpected SumKES signature size at level {level}",
    );

    if level == 0 {
        assert!(
            path.is_empty(),
            "SumKES leaf level should not have remaining path entries",
        );
        return;
    }

    assert_eq!(
        path.len(),
        level,
        "direction path should have exactly {level} entries for SumKES level {level}",
    );

    let (left_node, right_node) = node
        .children
        .as_ref()
        .map(|(left, right)| (&**left, &**right))
        .expect("non-leaf SumKES node should provide children");

    let child_signature_size = sum_signature_size_for_level(level - 1);
    let child_vk_size = sum_verification_key_size_for_level(level - 1);

    let (child_signature_bytes, remainder) = signature_bytes.split_at(child_signature_size);
    assert_eq!(
        remainder.len(),
        2 * child_vk_size,
        "SumKES signature remainder must contain two child verification keys at level {level}",
    );

    let (vk_left_bytes, vk_right_bytes) = remainder.split_at(child_vk_size);

    assert_eq!(
        vk_left_bytes,
        left_node.vk_bytes.as_slice(),
        "left subtree verification key mismatch at SumKES level {level}",
    );
    assert_eq!(
        vk_right_bytes,
        right_node.vk_bytes.as_slice(),
        "right subtree verification key mismatch at SumKES level {level}",
    );

    let recomputed = Blake2b256::hash_concat(vk_left_bytes, vk_right_bytes);
    assert_eq!(
        recomputed, node.vk_bytes,
        "reconstructed SumKES verification key must match expected node at level {level}",
    );

    let (direction, rest_path) = path
        .split_first()
        .expect("non-leaf SumKES level should have remaining path entries");

    match direction {
        Direction::Left => {
            inspect_sum_signature(level - 1, child_signature_bytes, left_node, rest_path);
        },
        Direction::Right => {
            inspect_sum_signature(level - 1, child_signature_bytes, right_node, rest_path);
        },
    }
}

const COMPACT_SIGNATURE_SIZES: [usize; 4] = [
    CompactSum0Kes::SIGNATURE_SIZE,
    CompactSum1Kes::SIGNATURE_SIZE,
    CompactSum2Kes::SIGNATURE_SIZE,
    CompactSum3Kes::SIGNATURE_SIZE,
];

const COMPACT_VERIFICATION_KEY_SIZES: [usize; 4] = [
    CompactSum0Kes::VERIFICATION_KEY_SIZE,
    CompactSum1Kes::VERIFICATION_KEY_SIZE,
    CompactSum2Kes::VERIFICATION_KEY_SIZE,
    CompactSum3Kes::VERIFICATION_KEY_SIZE,
];

/// Return the compact SumKES signature size for a given tree level.
///
/// # Panics
///
/// Panics if `level` is greater than 3.
#[must_use]
pub fn signature_size_for_level(level: usize) -> usize {
    COMPACT_SIGNATURE_SIZES
        .get(level)
        .copied()
        .expect("Compact SumKES level must be between 0 and 3")
}

/// Return the compact SumKES verification key size for a given tree level.
///
/// # Panics
///
/// Panics if `level` is greater than 3.
#[must_use]
pub fn verification_key_size_for_level(level: usize) -> usize {
    COMPACT_VERIFICATION_KEY_SIZES
        .get(level)
        .copied()
        .expect("Compact SumKES level must be between 0 and 3")
}

/// Inspect a compact SumKES signature against the expected tree structure.
///
/// # Panics
///
/// Panics if the signature layout does not match the expected structure for `level`.
#[must_use]
pub fn inspect_compact_sum_signature(
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
