# Changelog for `cardano-crypto-class`

## Unreleased

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
* Added `compact_sum3_kes_signature_components` regression to assert the
  embedded subtree verification keys stored in compact signatures match the
  derived CompactSum structure from `Cardano.Crypto.KES.CompactSum`.

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
