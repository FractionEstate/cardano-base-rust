# Changelog

All notable changes to `cardano-test-vectors` are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the
project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Expanded KES corpus: deterministic Single/CompactSingle/Sum/CompactSum
  fixtures, full period-evolution datasets, and regression harness coverage for
  tracked signatures across the hierarchy.
- BLS12-381 fixture family (`bls_sig_aug`, `ec_operations`, `h2c_large_dst`,
  `pairing`, `serde`) surfaced through `cardano_test_vectors::bls12_381`.
- Comprehensive hash digest catalogue (`hash_test_vectors.json`) plus the
  `generate_hash_vectors` and `compare_hash_vectors` tooling to keep Rust and
  Haskell corpora in sync.
- Feature-gated DSIGN debugging utilities (`ed25519-debug`) alongside a trace
  harness and lightweight performance smoke tests.

### Changed
- Refactored the KES vector generator to reuse shared signing logic and emit
  tracked-period as well as full-evolution datasets in a single pass.
- Regression tests now enforce corpus lengths and cover CompactSumKES levels
  1–7, deterring accidental fixture shrinkage.
- Revalidated the full suite (`cargo test -p cardano-test-vectors`) on
  2025-10-08 after regenerating evolution fixtures to confirm stability across
  VRF, DSIGN, KES, hash, and BLS data sets.
