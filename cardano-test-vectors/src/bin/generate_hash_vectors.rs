// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Helper binary to regenerate `hash_test_vectors.json`.
//!
//! Usage:
//!
//! ```bash
//! cargo run -p cardano-test-vectors --bin generate_hash_vectors
//! ```
//!
//! This will overwrite `cardano-test-vectors/test_vectors/hash_test_vectors.json`
//! with freshly computed values using the Rust hashing implementations.

use std::fs;
use std::path::PathBuf;

use cardano_crypto_class::hash::{
    Blake2b256, Blake2b512, blake2b224, hash160, keccak256, ripemd160, sha3_256, sha3_512, sha256,
    sha256d, sha512,
};
use cardano_crypto_class::kes::hash::KesHashAlgorithm;
use serde::Serialize;

#[derive(Debug)]
struct Case {
    name: &'static str,
    description: Option<&'static str>,
    bytes: Vec<u8>,
}

#[derive(Debug, Serialize)]
struct Metadata<'a> {
    description: &'a str,
    version: u64,
    note: &'a str,
    generator: &'a str,
}

#[derive(Debug, Serialize)]
struct HashVector {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Debug, Serialize)]
struct Output<'a> {
    metadata: Metadata<'a>,
    vectors: Vec<HashVector>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cases = build_cases();

    let vectors = cases.into_iter().map(to_hash_vector).collect::<Vec<_>>();

    let output = Output {
        metadata: Metadata {
            description: "Baseline hash test vectors (boundary + composite coverage)",
            version: 4,
            note: "Extend with future cross-language confirmations, streaming edge cases, and address-key hashing",
            generator: "cargo run -p cardano-test-vectors --bin generate_hash_vectors",
        },
        vectors,
    };

    let json = serde_json::to_string_pretty(&output)? + "\n";
    let out_path = output_path();
    fs::write(&out_path, &json)?;

    println!(
        "Regenerated {} hash vectors at {}",
        output.vectors.len(),
        out_path.display()
    );

    Ok(())
}

fn build_cases() -> Vec<Case> {
    vec![
        Case {
            name: "empty",
            description: None,
            bytes: vec![],
        },
        Case {
            name: "hello_ascii",
            description: Some("UTF-8 string 'hello world'"),
            bytes: b"hello world".to_vec(),
        },
        Case {
            name: "short_sequence",
            description: Some("Bytes 0x00 through 0x09"),
            bytes: (0u8..=9).collect(),
        },
        Case {
            name: "single_byte_ff",
            description: Some("Single byte 0xff"),
            bytes: vec![0xff],
        },
        Case {
            name: "sha2_block_minus_one",
            description: Some("Sequential bytes 0x00..0x3e (63 bytes, SHA-2 block size minus one)"),
            bytes: sequential_bytes(63),
        },
        Case {
            name: "sha2_block_exact",
            description: Some("Sequential bytes 0x00..0x3f (64 bytes, SHA-2 block size)"),
            bytes: sequential_bytes(64),
        },
        Case {
            name: "sha2_block_plus_one",
            description: Some(
                "Sequential bytes 0x00..0x40 (65 bytes, crosses SHA-2 block boundary)",
            ),
            bytes: sequential_bytes(65),
        },
        Case {
            name: "sha3_rate_block",
            description: Some("Sequential bytes 0x00..0x87 (136 bytes, SHA3-256 rate)"),
            bytes: sequential_bytes(136),
        },
        Case {
            name: "sha3_rate_plus_one",
            description: Some("Sequential bytes 0x00..0x88 (137 bytes, crosses SHA3-256 rate)"),
            bytes: sequential_bytes(137),
        },
        Case {
            name: "multi_block_1024",
            description: Some("1024 bytes repeating 0x00..0xff pattern"),
            bytes: repeating_pattern(1024),
        },
        Case {
            name: "bitcoin_genesis_pubkey",
            description: Some(
                "Uncompressed secp256k1 pubkey from Bitcoin genesis coinbase (P2PKH 1A1zP1e...)",
            ),
            bytes: hex_bytes(
                "04678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5f",
            ),
        },
        Case {
            name: "bitcoin_genesis_block_header",
            description: Some("80-byte block header (little-endian fields) for Bitcoin block 0"),
            bytes: hex_bytes(
                "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c",
            ),
        },
        Case {
            name: "ethereum_legacy_tx_009",
            description: Some(
                "RLP-encoded legacy transaction (nonce=9, 20 gwei gasPrice, to=0x3535...) used in go-ethereum docs",
            ),
            bytes: hex_bytes(
                "f86c098504a817c800825208943535353535353535353535353535353535353535880de0b6b3a7640000801ba05e1d3a76fbf824220e631d3f0c70d0e1cf5b1af1b0b44d1c45e6a64d4c6865b1a0045a915e4d06b7a76efc8696fa7aafc0fb8fdcbccc23e74abf5c75f9ebf1b0f3",
            ),
        },
    ]
}

fn sequential_bytes(len: usize) -> Vec<u8> {
    (0..len).map(|i| i as u8).collect()
}

fn repeating_pattern(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i % 256) as u8).collect()
}

fn hex_bytes(s: &str) -> Vec<u8> {
    hex::decode(s).expect("valid hex literal")
}

fn to_hash_vector(case: Case) -> HashVector {
    let input_hex = hex::encode(&case.bytes);

    let sha256 = hex::encode(sha256(&case.bytes));
    let sha256d = hex::encode(sha256d(&case.bytes));
    let sha512 = hex::encode(sha512(&case.bytes));
    let sha3_256 = hex::encode(sha3_256(&case.bytes));
    let sha3_512 = hex::encode(sha3_512(&case.bytes));
    let keccak256 = hex::encode(keccak256(&case.bytes));
    let ripemd160 = hex::encode(ripemd160(&case.bytes));
    let hash160 = hex::encode(hash160(&case.bytes));
    let blake2b224 = hex::encode(blake2b224(&case.bytes));

    let blake2b256 = hex::encode(Blake2b256::hash(&case.bytes));
    let blake2b512 = hex::encode(Blake2b512::hash(&case.bytes));

    HashVector {
        name: case.name.to_owned(),
        description: case.description.map(|s| s.to_owned()),
        input_hex,
        sha256,
        sha256d,
        sha512,
        sha3_256,
        sha3_512,
        keccak256,
        ripemd160,
        hash160,
        blake2b224,
        blake2b256,
        blake2b512,
    }
}

fn output_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_vectors")
        .join("hash_test_vectors.json")
}
