# Changelog for `cardano-binary`

## Unreleased

### Added
- **Haskell Mapping Documentation** (`HASKELL_MAPPING.md`): Complete function/module mapping,
  type class translations, canonical encoding rules, and migration guide with examples.
- **Performance Benchmarks**: Criterion-based benchmarks (`benches/cbor_bench.rs`) measuring
  serialization/deserialization throughput for small/medium/large structures and collections.
  Baseline: ~250 ns for small structs, ~320 MB/s for integer vectors, ~330 MB/s for maps.
- **Enhanced README**: Documented canonical CBOR encoding rules (RFC 8949 §4.2), map key ordering
  examples, verification strategy, and performance baselines. Added test coverage summary (86 tests).

### Documentation
- Clarified canonical encoding guarantees and verification approach.
- Documented benchmark usage and interpretation.
- Added Haskell→Rust migration examples.

## 0.1.0 (2025-10-02)

- Replaced the legacy Haskell package with a Rust crate powered by
  `ciborium`.
- Added helper functions for serialisation (`serialize`,
  `serialize_into_writer`, `serialize_into_vec`, `serialize_with_capacity`,
  `encode_nested_cbor`) and deserialisation
  (`decode_full`, `deserialise_decoder`, `decode_nested_cbor`).
- Introduced the `BinaryError` type capturing trailing bytes, nested tag
  mismatches, and underlying CBOR errors.
