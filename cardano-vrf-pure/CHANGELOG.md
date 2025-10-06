# Changelog

All notable changes to the `cardano-vrf-pure` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **VRF Parity Achievement** (2025-10-06): Achieved byte-for-byte compatibility with
  Cardano's libsodium VRF implementation
  - Fixed critical sign bit handling in hash-to-curve operations (`r_bytes[31] &= 0x7f`)
  - Corrected suite identifier to `0x04` (ECVRF-ED25519-SHA512-ELL2)
  - Aligned cofactor clearing timing with reference implementation
  - All 35 unit tests pass
  - Official test vectors `vrf_ver03_standard_10` and `vrf_ver03_generated_1` produce
    exact proof and VRF output matches
  - See [VRF_PARITY_COMPLETE.md](VRF_PARITY_COMPLETE.md) for detailed documentation

### Changed
- `cardano_compat::prove::cardano_vrf_prove`: Now clears sign bit before hash-to-curve
- `cardano_compat::verify::cardano_vrf_verify`: Now clears sign bit before hash-to-curve
  and applies cofactor clearing before beta computation
- `cardano_compat::point::hash_to_curve_bigint`: Refactored to apply cofactor clearing
  and let natural point serialization determine final sign bit

### Fixed
- Sign bit handling in Elligator2 hash-to-curve mapping now matches C reference
- VRF proof generation and verification now produce identical outputs to libsodium
- Beta output computation now correctly hashes cofactor-cleared gamma point
- Draft-13 proof hashing now appends the trailing `0x00` byte expected by libsodium,
  restoring parity for official `vrf_ver13_*` beta outputs while keeping draft-03
  vectors unchanged

## [0.1.0] - Initial Implementation

### Added
- Draft-03 VRF implementation (`draft03` module)
- Draft-13 VRF implementation (`draft13` module)
- Cardano-compatible VRF primitives (`cardano_compat` module)
  - `prove.rs`: VRF proof generation
  - `verify.rs`: VRF proof verification
  - `point.rs`: Edwards point operations and cofactor clearing
  - `montgomery.rs`: Montgomery curve operations and Elligator2 mapping
- Pure Rust field arithmetic (no unsafe C bindings)
- Common utilities for both draft versions
- Feature-gated debug logging (`vrf-debug` feature)
- Comprehensive unit and integration tests
- Official test vector validation

[Unreleased]: https://github.com/FractionEstate/cardano-base-rust/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/FractionEstate/cardano-base-rust/releases/tag/v0.1.0
