// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Hash vector parity tests (initial scaffold).
//!
//! Loads `hash_test_vectors.json` from the `cardano-test-vectors` crate and verifies
//! that all listed digests match the outputs produced by the hashing helpers in
//! `cardano_crypto_class::hash`.
//!
//! Extended / large / multi-block cases and cross-language confirmation will be
//! added in subsequent commits (Phase 06 – Hash Algorithm Parity).

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Metadata {
    description: Option<String>,
    version: Option<u64>,
    note: Option<String>,
    generator: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HashVector {
    name: String,
    #[allow(dead_code)]
    description: Option<String>,
    input_hex: String,
    sha256: String,
    sha256d: String,
    sha512: String,
    sha3_256: String,
    sha3_512: String,
    keccak256: String,
    ripemd160: String,
    hash160: String,
    blake2b224: String,
    blake2b256: String,
    blake2b512: String,
}

#[derive(Debug, Deserialize)]
struct HashVectorsFile {
    #[allow(dead_code)]
    metadata: Metadata,
    vectors: Vec<HashVector>,
}

fn hex_to_bytes(s: &str) -> Vec<u8> {
    if s.is_empty() {
        return vec![];
    }
    hex::decode(s).expect("invalid hex in test vector")
}

#[test]
fn hash_vectors_match() {
    // Locate the test vectors via env var emitted by build scripts or fallback relative path.
    // For now we assume workspace relative path.
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..") // up from cardano-crypto-class
        .join("cardano-test-vectors")
        .join("test_vectors")
        .join("hash_test_vectors.json");
    let data = std::fs::read(&path)
        .expect("hash_test_vectors.json not found; ensure test vectors crate present");
    let parsed: HashVectorsFile = serde_json::from_slice(&data).expect("valid hash vectors json");

    use cardano_crypto_class::hash::*;
    use cardano_crypto_class::kes::hash::KesHashAlgorithm;

    for v in parsed.vectors.iter() {
        let input = hex_to_bytes(&v.input_hex);
        assert_eq!(
            hex::encode(sha256(&input)),
            v.sha256,
            "sha256 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(sha256d(&input)),
            v.sha256d,
            "sha256d mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(sha512(&input)),
            v.sha512,
            "sha512 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(sha3_256(&input)),
            v.sha3_256,
            "sha3_256 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(sha3_512(&input)),
            v.sha3_512,
            "sha3_512 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(keccak256(&input)),
            v.keccak256,
            "keccak256 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(ripemd160(&input)),
            v.ripemd160,
            "ripemd160 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(hash160(&input)),
            v.hash160,
            "hash160 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(blake2b224(&input)),
            v.blake2b224,
            "blake2b224 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(Blake2b256::hash(&input)),
            v.blake2b256,
            "blake2b256 mismatch for {}",
            v.name
        );
        assert_eq!(
            hex::encode(Blake2b512::hash(&input)),
            v.blake2b512,
            "blake2b512 mismatch for {}",
            v.name
        );
    }

    let names: std::collections::HashSet<_> =
        parsed.vectors.iter().map(|v| v.name.as_str()).collect();
    for required in [
        "bitcoin_genesis_pubkey",
        "bitcoin_genesis_block_header",
        "ethereum_legacy_tx_009",
    ] {
        assert!(
            names.contains(required),
            "missing required composite vector: {}",
            required
        );
    }
}
