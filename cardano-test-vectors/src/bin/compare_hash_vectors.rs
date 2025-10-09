// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Compare two hash vector JSON corpora and report any mismatches.
//!
//! This helper is intended to validate the Rust-generated
//! `hash_test_vectors.json` against the output produced by the Haskell
//! `HashVectors.hs` script or any other reference implementation. Usage:
//!
//! ```text
//! cargo run -p cardano-test-vectors --bin compare_hash_vectors \
//!     /path/to/hash_vectors_haskell.json
//! ```
//!
//! The optional second argument overrides the Rust corpus path (defaults to the
//! repository copy embedded in this crate). Differences are reported with the
//! offending vector name and digest key. Exit status is non-zero on mismatch.

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Metadata {
    #[allow(dead_code)]
    description: Option<String>,
    #[allow(dead_code)]
    version: Option<u64>,
    #[allow(dead_code)]
    note: Option<String>,
    #[allow(dead_code)]
    generator: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
struct HashVectorsFile {
    #[allow(dead_code)]
    metadata: Metadata,
    vectors: Vec<HashVector>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let reference_path = args
        .next()
        .ok_or("Usage: compare_hash_vectors <reference.json> [candidate.json]")?;
    let candidate_path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(default_candidate_path);

    let reference = load_vectors(Path::new(&reference_path))?;
    let candidate = load_vectors(&candidate_path)?;

    let mismatches = compare(&reference, &candidate);

    if mismatches.is_empty() {
        println!(
            "✅ hash vector comparison succeeded ({} entries)",
            reference.vectors.len()
        );
        println!("Reference: {}", Path::new(&reference_path).display());
        println!("Candidate: {}", candidate_path.display());
        Ok(())
    } else {
        eprintln!("❌ hash vector comparison found mismatches:");
        for mismatch in mismatches {
            eprintln!("  - {}", mismatch);
        }
        Err("hash vector mismatch".into())
    }
}

fn load_vectors(path: &Path) -> Result<HashVectorsFile, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let vectors: HashVectorsFile = serde_json::from_str(&data)?;
    Ok(vectors)
}

fn default_candidate_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("test_vectors")
        .join("hash_test_vectors.json")
}

fn compare(reference: &HashVectorsFile, candidate: &HashVectorsFile) -> Vec<String> {
    let mut mismatches = Vec::new();

    let reference_map: BTreeMap<_, _> = reference
        .vectors
        .iter()
        .map(|vector| (vector.name.as_str(), vector))
        .collect();
    let candidate_map: BTreeMap<_, _> = candidate
        .vectors
        .iter()
        .map(|vector| (vector.name.as_str(), vector))
        .collect();

    for (name, reference_vector) in &reference_map {
        match candidate_map.get(name) {
            Some(candidate_vector) => {
                compare_vector(name, reference_vector, candidate_vector, &mut mismatches);
            },
            None => mismatches.push(format!(
                "candidate missing vector '{}', expected from reference",
                name
            )),
        }
    }

    for name in candidate_map.keys() {
        if !reference_map.contains_key(name) {
            mismatches.push(format!(
                "candidate has extra vector '{}' not present in reference",
                name
            ));
        }
    }

    mismatches
}

fn compare_vector(
    name: &str,
    reference: &HashVector,
    candidate: &HashVector,
    mismatches: &mut Vec<String>,
) {
    macro_rules! check_field {
        ($field:ident, $label:expr) => {
            if reference.$field != candidate.$field {
                mismatches.push(format!(
                    "{}: {} mismatch (reference={}, candidate={})",
                    name, $label, reference.$field, candidate.$field,
                ));
            }
        };
    }

    check_field!(input_hex, "input_hex");
    check_field!(sha256, "sha256 digest");
    check_field!(sha256d, "sha256d digest");
    check_field!(sha512, "sha512 digest");
    check_field!(sha3_256, "sha3_256 digest");
    check_field!(sha3_512, "sha3_512 digest");
    check_field!(keccak256, "keccak256 digest");
    check_field!(ripemd160, "ripemd160 digest");
    check_field!(hash160, "hash160 digest");
    check_field!(blake2b224, "blake2b224 digest");
    check_field!(blake2b256, "blake2b256 digest");
    check_field!(blake2b512, "blake2b512 digest");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_identical_vectors_succeeds() {
        let path = default_candidate_path();
        let vectors = load_vectors(&path).expect("load canonical vectors");
        let mismatches = compare(&vectors, &vectors);
        assert!(mismatches.is_empty());
    }

    #[test]
    fn compare_detects_mismatch() {
        let path = default_candidate_path();
        let reference = load_vectors(&path).expect("load canonical vectors");
        let mut candidate = reference.clone();
        // Corrupt one digest.
        candidate.vectors[0].sha256.push('0');

        let mismatches = compare(&reference, &candidate);
        assert!(
            mismatches
                .iter()
                .any(|entry| entry.contains("sha256 digest mismatch"))
        );
    }
}
