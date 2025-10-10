# Changelog

All notable changes to `cardano-binary` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Distribution-grade README describing the module map, canonical encoding
  responsibilities, nested CBOR workflows, and error-handling guidance with
  links back to the originating Haskell modules.

### Changed
- Updated the changelog structure to Keep a Changelog conventions and
  documented the parity-focused documentation work.

## [0.1.0] - 2025-10-02

### Added
- Replaced the legacy Haskell package with a Rust crate powered by `ciborium`.
- Added helper functions for serialisation (`serialize`, `serialize_into_writer`,
  `serialize_into_vec`, `serialize_with_capacity`, `encode_nested_cbor`) and
  deserialisation (`decode_full`, `decode_nested_cbor`).
- Introduced the `BinaryError` type capturing trailing bytes, nested tag
  mismatches, and underlying CBOR errors.
- Landed `HASKELL_MAPPING.md` to document function/module parity, canonical
  encoding rules, and migration notes.

### Benchmarks
- Added Criterion-based benchmarks (`benches/cbor_bench.rs`) measuring
  serialisation/deserialisation throughput for small, medium, and large
  structures (e.g. ~250 ns for small structs, ~320 MB/s for integer vectors,
  ~330 MB/s for maps). HTML reports emit under `target/criterion/`.
