# Changelog

All notable changes to `cardano-crypto-class` are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the crate
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `hash::blake2b224` plus expanded vector coverage and the
  `compare_hash_vectors` CLI to mirror `Cardano.Crypto.Hash.Blake2b_224`.
- Criterion benchmark suites: `hash_bench` (SHA-2/3, Keccak, RIPEMD160,
  Hash160, Blake2b) and `kes_bench` (keygen/sign/verify/evolve + serialized
  size reporting).
- Comprehensive hash regression harness (`tests/hash_vectors.rs`) powered by
  JSON fixtures from `cardano-test-vectors`.
- `hash::constant_time_eq` with regression coverage for equal/unequal paths.
- DSIGN harnesses for Ed25519 (serde-gated cross-compat), ECDSA secp256k1, and
  Schnorr secp256k1, each backed by regenerated JSON fixtures.
- Serde-gated KES harnesses covering Single, CompactSingle, Sum0–7, and
  CompactSum0–7 plus unified Haskell parity validation.
- Forward-security, structural, and integration tests for every KES family,
  including compact signature decomposition (`sum_kes_structure.rs`).
- Feature-gated diagnostics: `mlocked-metrics` and `kes-metrics` relaxed atomic
  counters for secure memory and KES workloads.
- Secure memory primitives (`mlocked_bytes`, `MLockedSeed`), packed byte
  helpers, and deterministic entropy infrastructure (`Seed`, `SeedRng`,
  `run_with_seed`).
- Documentation refresh describing module mapping, canonical encoding
  responsibilities, regeneration workflows, and parity status across DSIGN,
  KES, VRF, and hashing modules.

### Changed
- Replaced pointer-based `DirectSerialise`/`DirectDeserialise` callbacks with
  slice-based closures to eliminate remaining `unsafe` paths while matching the
  Haskell interface.
- Tightened KES parity harnesses to assert raw signature dumps and populated
  metadata, expanding fixture fidelity.
- Hardened hash unit tests with Blake2b output-length assertions and 1 MiB
  stress inputs to guarantee deterministic behaviour.
- Extended KES benchmarks with serialized size reporting and improved baseline
  documentation.
- Enabled serde-gated Ed25519 regressions by default, silencing unused
  metadata warnings and keeping behaviour aligned with `Cardano.Crypto.DSIGN`.
- Expanded integration suites to reject malformed signatures/verification keys
  and to prove Sum0 remains byte-for-byte with SingleKES.
- Extracted shared helpers for Sum/CompactSum structural checks to improve test
  reuse and clarity.

## [2.2.3.2]

- No Rust-specific notes carried over from upstream tag.

## [2.2.3.1]

- Add package bound on pkg-config lib blst in upstream Haskell sources (#544).

## [2.2.3.0]

- Add `blsMSM` to the BLS12_381 interface.
- Drop GHC ≤ 9.4 support.

## [2.2.2.1]

- Upstream patch release (no additional notes).

## [2.2.2.0]

- Add `SHA512` and `SHA3_512` algorithms.

## [2.2.1.0]

- Add `NoThunks` constraint on `UnsoundPureSignKeyKES` (missed in prior KES
  changes).

## [2.2.0.0]

- Require `HashAlgorithm` constraint for `Hash` serialisation.
- Add `MemPack` for `Hash`/`PackedBytes`.
- Introduce memory locking and secure forgetting functionality (#255, #404).
- Adopt memlocking within KES implementations (#255, #404).
- Introduce `DSIGNM` using the new memlocking utilities (#404).
- Add `blst` bindings for BLS12-381 operations (#266).
- Introduce `DirectSerialise` / `DirectDeserialise` APIs (#404).
- Restructure libsodium bindings and related APIs (#404).
- Re-introduce non-mlocked KES implementations to ease migration (#504).
- Expose constructors of the BLS12-381 internals (#509).

## [2.1.0.2]

- Deserialisation performance improvements.
- GHC-9.6 compatibility updates.

## [2.1.0.1]

- Remove `development` Cabal flag (#372).

## [2.1.0.0]

- Rename `encodedSignKeyDESIGNSizeExpr` → `encodedSignKeyDSIGNSizeExpr`.
- Add `IsString` instance for `Code Q (Hash h a)` to support `$$"deadbeaf"`
  with GHC 9.2.

## [2.0.0.1]

- Initial release.
