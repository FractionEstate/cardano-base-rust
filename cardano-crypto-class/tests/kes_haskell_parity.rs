//! KES parity harness: revalidates Rust KES signatures against embedded test vectors
//! originally derived from / generated to match the Haskell implementation.
//! This ensures deterministic reproduction of verification keys & signatures across
//! Single, CompactSingle, Sum (levels 1-7) and CompactSum variants.
//!
//! Fixture layout (as embedded by cardano-test-vectors):
//! * single / compact_single: {
//!       "vectors": [ { seed (hex), message (hex), period, expected { verification_key / derived_verification_key, signature, raw_signature } } ]
//!   }
//! * sum / compact_sum: {
//!       "levels": [ { level, total_periods, vectors: [ { verification_key, seed, tracked_periods: [ { period, message (hex), signature, raw_signature } ] } ] } ]
//!   }
//!
//! Raw signatures for compact variants concatenate the signature bytes with the embedded
//! verification key (the expected layout enforced in Compact* implementations). We validate
//! both the signature portion and the embedded VK bytes for completeness.

use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::*;
use cardano_test_vectors::kes;
use serde::Deserialize;

// ------------------------------ Utility ----------------------------------

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(hex.len() / 2);
    let bytes = hex.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = bytes[i];
        let lo = bytes[i + 1];
        let val = (hex_val(hi) << 4) | hex_val(lo);
        out.push(val);
        i += 2;
    }
    out
}

fn hex_val(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => panic!("invalid hex char"),
    }
}

// --------------------------- Single / Compact -----------------------------

#[derive(Debug, Deserialize)]
struct SingleExpected {
    #[serde(default)]
    verification_key: Option<String>,
    #[serde(default)]
    derived_verification_key: Option<String>,
    #[serde(default)]
    embedded_verification_key: Option<String>,
    signature: String,
    raw_signature: String,
}

#[derive(Debug, Deserialize)]
struct SingleVectorEntry {
    test_name: String,
    seed: String,
    message: String,
    period: u64,
    expected: SingleExpected,
}

#[derive(Debug, Deserialize)]
struct SingleRoot {
    vectors: Vec<SingleVectorEntry>,
}

fn run_single_like<A>(file: &str, algo_name: &str)
where
    A: KesAlgorithm<Context = ()> + 'static,
    A::VerificationKey: Clone,
{
    let raw = kes::get(file).expect("vector present");
    let root: SingleRoot = serde_json::from_str(raw).expect("valid single root JSON");
    for v in root.vectors {
        assert!(
            v.period < A::total_periods(),
            "period out of range in fixture"
        );
        let seed = hex_to_bytes(&v.seed);
        let msg = hex_to_bytes(&v.message);
        let sk = A::gen_key_kes_from_seed_bytes(&seed).expect("signing key");
        let vk = A::derive_verification_key(&sk).expect("derive vk");
        let sig = A::sign_kes(&(), v.period, &msg, &sk).expect("sign");
        A::verify_kes(&(), &vk, v.period, &msg, &sig).expect("verify");

        // Expected VK hex (depending on variant naming in fixture)
        let expected_vk_hex = v
            .expected
            .verification_key
            .as_ref()
            .or(v.expected.derived_verification_key.as_ref())
            .expect("expected vk hex present");
        let expected_vk = hex_to_bytes(expected_vk_hex);
        let vk_ser = A::raw_serialize_verification_key_kes(&vk);
        assert_eq!(
            vk_ser, expected_vk,
            "vk mismatch for {} / {}",
            algo_name, v.test_name
        );

        // Expected signature hex (not including embedded VK for compact variants)
        let expected_sig = hex_to_bytes(&v.expected.signature);
        let sig_ser_full = A::raw_serialize_signature_kes(&sig);
        // For CompactSingle the serialized signature is DSIGN_SIG || VK. Our expected.signature
        // field contains only the DSIGN portion (legacy parity). Trim before comparing.
        let sig_ser = if algo_name.starts_with("CompactSingle") {
            sig_ser_full[..expected_sig.len()].to_vec()
        } else {
            sig_ser_full.clone()
        };
        assert_eq!(
            sig_ser, expected_sig,
            "signature mismatch for {} / {}",
            algo_name, v.test_name
        );

        // Raw signature (compact variants: signature || embedded vk)
        if let Some(embed_vk_hex) = v.expected.embedded_verification_key.as_ref() {
            // Ensure raw_signature splits correctly.
            let raw_bytes = hex_to_bytes(&v.expected.raw_signature);
            let sig_len = expected_sig.len(); // DSIGN signature length
            assert!(
                raw_bytes.len() == sig_len + expected_vk.len(),
                "raw signature length mismatch (compact single)"
            );
            let (sig_part, vk_part) = raw_bytes.split_at(sig_len);
            assert_eq!(sig_part, expected_sig.as_slice(), "raw sig prefix mismatch");
            assert_eq!(vk_part, hex_to_bytes(embed_vk_hex), "embedded vk mismatch");
        } else {
            // Non-compact raw_signature should equal signature
            assert_eq!(
                v.expected.raw_signature, v.expected.signature,
                "raw_signature should equal signature for non-compact"
            );
        }
    }
}

// --------------------------- Sum / CompactSum -----------------------------

#[derive(Debug, Deserialize)]
struct TrackedPeriod {
    period: u64,
    message: String,
    signature: String,
    raw_signature: String,
}

#[derive(Debug, Deserialize)]
struct SumVectorEntry {
    test_name: String,
    seed: String,
    description: String,
    verification_key: String,
    tracked_periods: Vec<TrackedPeriod>,
}

#[derive(Debug, Deserialize)]
struct SumLevel {
    level: u64,
    total_periods: u64,
    vectors: Vec<SumVectorEntry>,
}

#[derive(Debug, Deserialize)]
struct SumRoot {
    levels: Vec<SumLevel>,
}

fn run_sum_like<A>(file: &str, algo_name: &str, expected_level: u64)
where
    A: KesAlgorithm<Context = ()> + 'static,
    A::VerificationKey: Clone,
{
    let raw = kes::get(file).expect("vector present");
    let root: SumRoot = serde_json::from_str(raw).expect("valid sum root JSON");
    let level_entry = root
        .levels
        .iter()
        .find(|l| l.level == expected_level)
        .expect("requested level present in vectors");
    assert_eq!(
        A::total_periods(),
        level_entry.total_periods,
        "total periods mismatch for level {}",
        expected_level
    );

    for vec_entry in &level_entry.vectors {
        let seed = hex_to_bytes(&vec_entry.seed);
        let description = vec_entry.description.as_str();
        assert!(
            !description.trim().is_empty(),
            "missing description for {} / {}",
            algo_name,
            vec_entry.test_name
        );
        let mut sk = A::gen_key_kes_from_seed_bytes(&seed).expect("signing key");
        let vk = A::derive_verification_key(&sk).expect("derive vk");
        let expected_vk = hex_to_bytes(&vec_entry.verification_key);
        let vk_ser = A::raw_serialize_verification_key_kes(&vk);
        assert_eq!(
            vk_ser, expected_vk,
            "vk mismatch {} / {}",
            algo_name, vec_entry.test_name
        );

        let mut tracked_iter = vec_entry.tracked_periods.iter().peekable();
        let total = A::total_periods();
        for period in 0..total {
            if let Some(tp) = tracked_iter.peek() {
                if tp.period == period {
                    let msg = hex_to_bytes(&tp.message);
                    let sig = A::sign_kes(&(), period, &msg, &sk).expect("sign");
                    A::verify_kes(&(), &vk, period, &msg, &sig).expect("verify");
                    let sig_ser_full = A::raw_serialize_signature_kes(&sig);
                    let expected_sig = hex_to_bytes(&tp.signature);
                    let expected_raw_sig = hex_to_bytes(&tp.raw_signature);

                    assert_eq!(
                        sig_ser_full,
                        expected_raw_sig.as_slice(),
                        "raw_signature mismatch {} / {} period {} (description: {})",
                        algo_name,
                        vec_entry.test_name,
                        period,
                        description
                    );
                    // For compact variants expected signature is prefix of full raw signature
                    let cmp_slice = if algo_name.starts_with("CompactSum") {
                        &sig_ser_full[..expected_sig.len()]
                    } else {
                        &sig_ser_full[..]
                    };
                    assert_eq!(
                        cmp_slice,
                        expected_sig.as_slice(),
                        "signature mismatch {} / {} period {}",
                        algo_name,
                        vec_entry.test_name,
                        period
                    );
                    tracked_iter.next();
                }
            }
            if period + 1 == total {
                break;
            }
            sk = A::update_kes(&(), sk, period)
                .expect("update")
                .expect("key valid");
        }
        assert!(
            tracked_iter.next().is_none(),
            "unused tracked periods remain"
        );
    }
}

// ------------------------------ Tests -------------------------------------

#[test]
fn kes_single_parity() {
    run_single_like::<SingleKes<Ed25519>>("single_kes_test_vectors.json", "SingleKes");
    run_single_like::<CompactSingleKes<Ed25519>>(
        "compact_single_kes_test_vectors.json",
        "CompactSingleKes",
    );
}

#[test]
fn kes_sum_parity() {
    // Level numbers map directly to type alias depth (Sum1Kes has level 1, etc.)
    run_sum_like::<Sum1Kes>("sum_kes_test_vectors.json", "Sum1Kes", 1);
    run_sum_like::<Sum2Kes>("sum_kes_test_vectors.json", "Sum2Kes", 2);
    run_sum_like::<Sum3Kes>("sum_kes_test_vectors.json", "Sum3Kes", 3);
    run_sum_like::<Sum4Kes>("sum_kes_test_vectors.json", "Sum4Kes", 4);
    run_sum_like::<Sum5Kes>("sum_kes_test_vectors.json", "Sum5Kes", 5);
    run_sum_like::<Sum6Kes>("sum_kes_test_vectors.json", "Sum6Kes", 6);
    run_sum_like::<Sum7Kes>("sum_kes_test_vectors.json", "Sum7Kes", 7);
}

#[test]
fn kes_compact_sum_parity() {
    run_sum_like::<CompactSum1Kes>("compact_sum_kes_test_vectors.json", "CompactSum1Kes", 1);
    run_sum_like::<CompactSum2Kes>("compact_sum_kes_test_vectors.json", "CompactSum2Kes", 2);
    run_sum_like::<CompactSum3Kes>("compact_sum_kes_test_vectors.json", "CompactSum3Kes", 3);
    run_sum_like::<CompactSum4Kes>("compact_sum_kes_test_vectors.json", "CompactSum4Kes", 4);
    run_sum_like::<CompactSum5Kes>("compact_sum_kes_test_vectors.json", "CompactSum5Kes", 5);
    run_sum_like::<CompactSum6Kes>("compact_sum_kes_test_vectors.json", "CompactSum6Kes", 6);
    run_sum_like::<CompactSum7Kes>("compact_sum_kes_test_vectors.json", "CompactSum7Kes", 7);
}
