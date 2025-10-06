# Changelog

All notable changes to `cardano-test-vectors` are documented in this file.

The format roughly follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- SumKES JSON fixtures (`sum_kes_test_vectors.json`) mirrored from the Haskell
  reference generator (`Cardano.Crypto.KES.Sum`).
- CompactSumKES level-1 JSON fixtures (`compact_sum_kes_test_vectors.json`)
  generated alongside the existing KES datasets to unblock compact tree
  regression tests.
- Hierarchical SumKES coverage in `tests/kes_vectors.rs`, exercising levels 1–6
  and asserting tracked period signatures as described in
  `Cardano.Crypto.KES.Sum`.
- Feature-gated DSIGN debug helpers (`ed25519-debug`) with supporting
  integration tests and CLI hints.
- Lightweight Ed25519 performance smoke test and trace harness.
- BLS12-381 fixtures (`bls_sig_aug_test_vectors`, `ec_operations_test_vectors`,
  `h2c_large_dst`, `pairing_test_vectors`, `serde_test_vectors`) embedded from
  `cardano-crypto-tests/bls12-381-test-vectors` with lookup helpers in
  `cardano_test_vectors::bls12_381`.

### Changed
- `generate_kes_vectors.rs` now emits SumKES vectors alongside existing Single
  and CompactSingle outputs, mirroring the logic from
  `Cardano.Crypto.KES.Sum`.
- `compact_sum_kes_test_vectors.json` now covers CompactSumKES levels 1–7, and
  the regression harness exercises every level to match the regenerated
  fixtures.
- `generate_ed25519_outputs.rs` surfaces optional debugging guidance when the
  new feature flag is enabled.
- Library module exports extended to expose the sum KES fixtures at compile
  time.
