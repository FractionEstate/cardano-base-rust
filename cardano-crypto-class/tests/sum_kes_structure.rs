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

pub fn sum_signature_size_for_level(level: usize) -> usize {
    match level {
        0 => Sum0Kes::SIGNATURE_SIZE,
        1 => Sum1Kes::SIGNATURE_SIZE,
        2 => Sum2Kes::SIGNATURE_SIZE,
        3 => Sum3Kes::SIGNATURE_SIZE,
        4 => Sum4Kes::SIGNATURE_SIZE,
        5 => Sum5Kes::SIGNATURE_SIZE,
        6 => Sum6Kes::SIGNATURE_SIZE,
        7 => Sum7Kes::SIGNATURE_SIZE,
        _ => panic!("unsupported sum level {level}"),
    }
}

pub fn sum_verification_key_size_for_level(level: usize) -> usize {
    match level {
        0 => Sum0Kes::VERIFICATION_KEY_SIZE,
        1 => Sum1Kes::VERIFICATION_KEY_SIZE,
        2 => Sum2Kes::VERIFICATION_KEY_SIZE,
        3 => Sum3Kes::VERIFICATION_KEY_SIZE,
        4 => Sum4Kes::VERIFICATION_KEY_SIZE,
        5 => Sum5Kes::VERIFICATION_KEY_SIZE,
        6 => Sum6Kes::VERIFICATION_KEY_SIZE,
        7 => Sum7Kes::VERIFICATION_KEY_SIZE,
        _ => panic!("unsupported sum level {level}"),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
}

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

pub fn signature_size_for_level(level: usize) -> usize {
    match level {
        0 => CompactSum0Kes::SIGNATURE_SIZE,
        1 => CompactSum1Kes::SIGNATURE_SIZE,
        2 => CompactSum2Kes::SIGNATURE_SIZE,
        3 => CompactSum3Kes::SIGNATURE_SIZE,
        _ => panic!("unsupported compact sum level {level}"),
    }
}

pub fn verification_key_size_for_level(level: usize) -> usize {
    match level {
        0 => CompactSum0Kes::VERIFICATION_KEY_SIZE,
        1 => CompactSum1Kes::VERIFICATION_KEY_SIZE,
        2 => CompactSum2Kes::VERIFICATION_KEY_SIZE,
        3 => CompactSum3Kes::VERIFICATION_KEY_SIZE,
        _ => panic!("unsupported compact sum level {level}"),
    }
}

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
