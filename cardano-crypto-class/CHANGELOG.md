# Changelog for `cardano-crypto-class`

## Unreleased

* Added `hash::blake2b224` and the accompanying `Blake2b224` helper to mirror
  `Cardano.Crypto.Hash.Blake2b_224`, expanded the hash vector corpus (and
  regression tests) with 224-bit digests, wired the new variant into the
  Criterion benchmark suite, and documented a `compare_hash_vectors` CLI so
  parity checks against Haskell outputs are automated.
* Added `hash_bench` Criterion harness measuring throughput for SHA-2/3, Keccak, RIPEMD160,
  Hash160, and Blake2b helpers over 32 B, 1 KiB, 64 KiB, and 1 MiB patterned payloads with
  MB/s reports (HTML/JSON) stored under `target/criterion` for regression tracking, plus manual
  baseline capture instructions in the README.
* Documented the `Cardano.Crypto.Hash` → Rust module mapping in the crate README, clarified
  the Keccak vs SHA3 parameterisation (padding/domain separation), and explained the composite
  helper layering plus vector regeneration workflow to keep Phase 06 parity reviews traceable.
* Exposed `hash::constant_time_eq` so callers can compare digests via `subtle::ConstantTimeEq`,
  added regression tests to cover equal/unequal paths, and documented the side-channel guidance in
  the README.
* Added `tests/hash_vectors.rs` regression coverage for SHA-256/SHA3/Keccak/RIPEMD160/Hash160/Blake2b helpers, backed by the regenerated `cardano-test-vectors/test_vectors/hash_test_vectors.json` corpus (boundary, rate, multi-block, and Bitcoin/Ethereum composite inputs) and a new Rust-side fixture generator (`generate_hash_vectors.rs`).
* Tightened the KES parity harness (`tests/kes_haskell_parity.rs`) to assert raw signature hex dumps and require populated description metadata, eliminating lingering dead-field warnings while increasing fixture fidelity.
* Introduced KES performance benchmarks (`benches/kes_bench.rs`) measuring key generation,
  signing, verification, and bounded evolution cycles for `SingleKes`, `Sum4Kes`, and
  `CompactSum4Kes`. Added `criterion` (dev-dependency) with HTML reports to establish a
  reproducible baseline before optimisation or memory layout changes. Benchmarks cap
  sampled periods to keep CI runtime acceptable.
* Hardened `hash` unit tests with Blake2b output-length assertions, explicit confirmation that the 256-bit variant is not a simple truncation of the 512-bit digest, and 1 MiB stress cases across all algorithms to lock digest sizes and prove no panics on large inputs.
* Extended KES benchmarks with serialized size reporting (`serialized_sizes`) emitting the raw
  byte lengths of verification keys and signatures plus total period count for each algorithm.
  (Signing key raw serialization intentionally omitted—would require an unsound testing trait
  not implemented for production benchmarking.) Forms the initial memory footprint baseline
  (no OS-level RSS sampling yet).
* Added ECDSA secp256k1 DSIGN vector harness (`tests/dsign_ecdsa_secp256k1_vectors.rs`) exercising key generation from seeds, deterministic RFC6979 signing, low-s normalisation, verify-only cases, and error-path vectors (malformed keys / signatures / negative-s rejection).
* Added Schnorr secp256k1 DSIGN vector harness (`tests/dsign_schnorr_secp256k1_vectors.rs`) covering deterministic signing, verification, and negative-path scenarios (invalid R/S encodings, altered messages, malformed public keys).
* Enabled the serde-gated Ed25519 cross-compatibility regression (`tests/cross_compat.rs`) to
  run by default with the bundled `ed25519_vectors.json` fixtures, keeping behaviour aligned
  with `Cardano.Crypto.DSIGN` while silencing unused metadata warnings from the JSON loader.
* Start Rust port with `Seed` abstraction, deterministic `SeedRng`, packed
  byte utilities (`PackedBytes`), helper functions mirroring
  `Cardano.Crypto.Util`, pinned memory helpers (`PinnedSizedBytes`), mlocked
  seed storage (`MLockedSeed`), and zero-copy direct serialisation helpers
  mirroring `Cardano.Crypto.DirectSerialise`.
* Add DSIGN infrastructure (`dsign` module) together with a full Ed25519
  implementation, covering deterministic key generation, signing and
  verification, raw/direct serialisation, and mlocked key support.
* Introduce libsodium-style memory utilities, including runtime-length
  mlocked buffers, allocators, and low-level `zero_mem`/`copy_mem` helpers
  mirroring `Cardano.Crypto.Libsodium.Memory`.
* Added a serde-gated CompactSumKES regression harness covering levels 1–7 via
  shared fixtures, keeping the compact tree implementation in sync with
  regenerated vectors and signature formats.
* Extended CompactSumKES verification to reconstruct recursive verification
  keys, enabling parity for levels 1–7 alongside the SumKES hierarchy and the
  regenerated shared fixtures.
* Aligned `SingleKES`/`CompactSingleKES` key evolution with the Haskell
  reference and added boundary/tamper regression tests to lock down compact
  tree verification behaviour.
* Allow `PackedBytes` serde deserialisation to accept human-readable byte
  sequences as well as base64 strings, matching Haskell JSON fixtures.
* Added serde-gated regression harnesses for `SingleKes`, `CompactSingleKes`,
  and `Sum{1-7}Kes` that consume the embedded JSON fixtures and assert
  signature parity, verification, and key evolution across tracked periods.
* Strengthened `tests/kes_forward_security.rs` with Sum/CompactSum per-period
  evolution sweeps, mirroring `Test.Crypto.KES` by re-verifying historical
  signatures, rejecting stale-period signing after each update, and asserting
  explicit rewind attempts fail with the expected errors.
* Added `tests/kes_integration.rs` to exercise end-to-end SingleKES and SumKES
  workflows, covering deterministic signing/verification across every period,
  key expiry, verification-key mismatches, and period-out-of-range failures.
* Extended the KES integration regression to reject truncated or extended
  serialized signatures and verification keys, locking down the remaining
  serialization error-path checklist items.
* Expanded the integration suite with CompactSingleKES and CompactSumKES
  workflows, validating embedded verification key behaviour, tamper
  detection, and serialization failure handling alongside the standard
  SumKES coverage.
* Added `compact_sum3_kes_signature_components` regression that walks every
  period, recursively decoding compact signatures to prove each embedded
  subtree verification key matches the reconstructed CompactSum tree from
  `Cardano.Crypto.KES.CompactSum`.
* Added `sum3_kes_signature_components` regression validating SumKES signature
  layering against the reconstructed binary tree from
  `Cardano.Crypto.KES.Sum`, ensuring both child verification keys and recomputed
  branch hashes align for every period.
* Extended the SumKES structural regression to level 7 via
  `sum7_kes_signature_components`, exercising all 128 periods and proving every
  nested signature embeds the correct branch verification keys and produces the
  expected Blake2b256 root.
* Added `sum_kes_signature_components_levels`, sweeping Sum0–Sum6 to ensure
  every intermediate tree level mirrors the expected verification-key wiring
  and signature sizing described in `Cardano.Crypto.KES.Sum`, while the shared
  helper now asserts total-period and constant-size invariants for each level.
* Hardened the Sum0 path by asserting the single-period invariant and locking
  the deterministic seed selection before dispatching into the shared SumKES
  structural helper, keeping the regression aligned with the Haskell base
  case in `Cardano.Crypto.KES.Sum`.
* Added `sum0_kes_matches_singlekes_base_case` to prove the Sum0 alias stays
  byte-for-byte with `SingleKES` for key derivation, signatures, and expiry,
  preventing accidental divergence from the Haskell reference base case.
* Extracted the Sum/CompactSum structural helpers into `tests/sum_kes_structure.rs`
  for reuse across integration and fixture suites, and extended the
  `sum_kes_test_vectors` regression to decompose signatures with the shared
  helper so every tracked JSON vector now revalidates its verification-key
  path against `Cardano.Crypto.KES.Sum`.
* Added consolidated `tests/kes_haskell_parity.rs` harness which consumes the
  embedded JSON fixtures for Single, CompactSingle, Sum (levels 1–7) and
  CompactSum (levels 1–7) to assert byte-for-byte verification key and signature
  parity while exercising deterministic period evolution. This replaces the
  earlier (unused) flat-array prototype and aligns the Rust port with the
  hierarchical fixture structure used by the Haskell reference generator.
* Introduced feature-gated `mlocked-metrics` instrumentation (`src/mlocked_metrics.rs`) with
  relaxed atomic counters for allocations, allocation_bytes, zeroizations, and failed_locks.
  Instrumented `MLockedRegion` allocate/drop paths and added unit tests (including alignment
  rounding and zero-sized edge cases). Provides visibility into secure memory lifecycle without
  exposing secret contents; disabled by default for production parity builds.
* Added forward security & period evolution narrative to `kes/mod.rs` and README, including a
  concise Haskell→Rust module mapping table to aid audit/parity reviews. Documentation checklist
  in phase task file updated to reflect completion; no functional changes.

## 2.2.3.2

*

## 2.2.3.1

* Add package bound on pkg-config lib blst in #544

## 2.2.3.0

* Add `blsMSM` to the BLS12_381 interface
* Drop GHC <= 9.4 support

## 2.2.2.1

*

## 2.2.2.0

* Add `SHA512` and `SHA3_512` algorithms.

## 2.2.1.0

* Add `NoThunks` constraint on `UnsoundPureSignKeyKES` that was missed during KES changes

## 2.2.0.0

* Add required `HashAlgorithm` constraint to `Hash` serialization.
* Add `MemPack` instance for `Hash` and `PackedBytes`
* Introduce memory locking and secure forgetting functionality:
  [#255](https://github.com/input-output-hk/cardano-base/pull/255)
  [#404](https://github.com/input-output-hk/cardano-base/pull/404)
* KES started using the new memlocking functionality:
  [#255](https://github.com/input-output-hk/cardano-base/pull/255)
  [#404](https://github.com/input-output-hk/cardano-base/pull/404)
* Introduction of `DSIGNM` that uses the new memlocking functionality:
  [#404](https://github.com/input-output-hk/cardano-base/pull/404)
* Included bindings to `blst` library to enable operations over curve BLS12-381
  [#266](https://github.com/input-output-hk/cardano-base/pull/266)
* Introduction of `DirectSerialise` / `DirectDeserialise` APIs, providing
  direct access to mlocked keys in RAM:
  [#404](https://github.com/input-output-hk/cardano-base/pull/404)
* Restructuring of libsodium bindings and related APIs:
  [#404](https://github.com/input-output-hk/cardano-base/pull/404)
* Re-introduction of non-mlocked KES implementations to support a smoother
  migration path:
  [#504](https://github.com/IntersectMBO/cardano-base/pull/504)
* Exposing constructors of the BLS12-381 internals: [#509](https://github.com/IntersectMBO/cardano-base/pull/509)

## 2.1.0.2

* Deserialization performance improvements
* GHC-9.6 compatibility

## 2.1.0.1

* Remove `development` flag: #372

## 2.1.0.0

* Fixed the name `encodedSignKeyDESIGNSizeExpr` -> `encodedSignKeyDSIGNSizeExpr`
* Add `IsString` instance for `Code Q (Hash h a)`, so `$$"deadbeaf"` would work with GHC-9.2

## 2.0.0.1

* Initial release
