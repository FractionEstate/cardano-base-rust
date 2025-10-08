# Changelog

All notable changes to `cardano-test-vectors` are documented in this file.

The format roughly follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added
- Period evolution JSON fixtures for SumKES and CompactSumKES
  (`sum_kes_period_evolution_vectors.json`,
  `compact_sum_kes_period_evolution_vectors.json`) plus regression tests that
  assert every signature across the full hierarchy.
- Deterministic seed/message helpers in `generate_kes_vectors.rs` that expand
  the SingleKES and CompactSingleKES corpora to cover twelve vectors, ensuring
  broader parity with the Haskell reference inputs.
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
- Negative-path regression tests that mutate fixture messages and confirm the
  SingleKES, CompactSingleKES, SumKES, and CompactSumKES signatures reject the
  tampered payloads.
- Hash parity fixture (`hash_test_vectors.json`) with SHA-2/SHA3/Keccak/
  RIPEMD160/Hash160/Blake2b digests across empty, short, block-boundary, and
  multi-block inputs plus a dedicated Rust generator
  (`generate_hash_vectors.rs`) to keep the corpus aligned with
  `cardano_crypto_class::hash`.
- Blake2b-224 digests for every hash vector, covering the verification-key
  hashing path used during address derivation so the Rust fixtures match the
  Haskell `Cardano.Crypto.Hash.Blake2b_224` outputs.
- `compare_hash_vectors` CLI to diff a Haskell-produced JSON corpus against the
  committed Rust vectors, reporting any digest or input mismatches to close the
  verification loop.
- Added Bitcoin/Ethereum composite fixtures (genesis block header, genesis
  public key, and legacy RLP transaction) to `hash_test_vectors.json`,
  extending coverage for `sha256d` and `hash160` against widely referenced
  real-world inputs.

### Changed
- Refactored the KES vector generator to reuse shared period-signing logic and
  emit both tracked-period and full-evolution datasets in one pass.
- `generate_kes_vectors.rs` now emits SumKES vectors alongside existing Single
  and CompactSingle outputs, mirroring the logic from
  `Cardano.Crypto.KES.Sum`.
- `compact_sum_kes_test_vectors.json` now covers CompactSumKES levels 1–7, and
  the regression harness exercises every level to match the regenerated
  fixtures.
- Expanded SumKES/CompactSumKES tracked vector coverage to 32 deterministic
  seeds per level and limited the full-period evolution suites to six
  representative seeds for manageable fixture sizes.
- `tests/kes_vectors.rs` now enforces minimum corpus lengths for
  Single/CompactSingle/Sum/CompactSum fixtures so future regressions detect
  accidental vector shrinkage.
- `generate_ed25519_outputs.rs` surfaces optional debugging guidance when the
  new feature flag is enabled.
- Library module exports extended to expose the sum KES fixtures at compile
  time.
- Revalidated the full regression suite (`cargo test -p cardano-test-vectors`)
  on 2025-10-07 after regenerating evolution fixtures, confirming fixture
  integrity and internal harness stability (cross-language parity spot checks
  for expanded KES structure tests remain pending).
