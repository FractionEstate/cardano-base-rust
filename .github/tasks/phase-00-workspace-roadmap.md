# Phase 00 – Workspace Porting Roadmap

**Status:** ☐ Not started / ☑ In progress / ☐ Blocked / ☐ Completed  \
**Primary owners:** _Unassigned_ (add yourself here)  \
**Scope:** Entire Rust workspace (`base-deriving-via`, `cardano-*`, `deepseq`, `heapwords`, `measures`, `nothunks`, `orphans-deriving-via`)

---

## Objective
Provide a holistic, crate-by-crate roadmap that tracks the remaining work needed
to reach feature and behavioural parity with the upstream Haskell
`cardano-base` repository. This phase is an umbrella for coordinating detailed
phase documents (for example, [Phase 03 – Cardano VRF Parity](phase-03-vrf-parity.md))
and ensures no crate is left without an explicit plan or acceptance criteria.

## How to use this document
1. Review the **Global checklist** before touching cross-cutting infrastructure
   (tooling, linting, shared CI).
2. For crate-specific work, locate the corresponding subsection below. Each one
   lists the minimum tasks required to sign off parity.
3. If deeper scoping is required, spin out a dedicated phase file (e.g.
   `phase-04-cardano-binary-cbor.md`) and link it back here under the crate’s
   "References" bullet.
4. Update the status line, tick checkboxes, and append dated notes in
   **Reporting cadence** as milestones are reached.

---

## Global checklist
- [ ] Confirm every crate has an assigned maintainer (@FractionEstate) and a parity checklist in
      this document or a linked phase file.
- [ ] Ensure CI covers all crates (formatting, clippy, unit tests, integration
      tests) and captures `--all-targets` runs.
- [ ] Document the Haskell modules each crate mirrors so contributors can cross
      reference quickly.
- [ ] Maintain a rolling compatibility matrix (consider reviving
      `COMPATIBILITY_MATRIX.md`) that summarises completion status.
- [ ] Validate that shared helper scripts (e.g. `regenerate_vectors.sh`) work on
      a clean checkout without manual tweaking.

---

## Crate-by-crate checklist

### base-deriving-via
- [ ] Audit Haskell `Cardano.Base.DerivingVia` modules to confirm all deriving
      helpers are present.
- [ ] Port any Template Haskell-based instances to macro-free Rust equivalents
      or document the chosen abstraction.
- [ ] Add property tests ensuring the derived behaviours match the Haskell
      expectations (e.g. Semigroup/Monoid laws).
- [ ] Cross-reference usage sites in dependent crates to confirm API coverage.

### cardano-base
- [ ] Catalogue all exported modules from Haskell `Cardano.Base.*` and map them
      to existing Rust modules.
- [ ] Implement missing primitives (e.g. canonical JSON, text utilities) with
      thorough tests.
- [ ] Mirror the error-handling semantics (Either vs Result) and documented edge
      cases from Haskell.
- [ ] Document migration notes for downstream crates that previously depended on
      the Haskell version.

### cardano-binary
- [ ] Align CBOR encoding/decoding semantics with Haskell (including
      canonical/deterministic encoding rules).
- [ ] Import or regenerate golden vectors from the Haskell test suite and make
      sure they pass against the Rust implementation.
- [ ] Ensure streaming, incremental decoding, and error reporting behave like
      the original (e.g. leftover bytes, tag mismatches).
- [ ] Validate integration with `cardano-ledger` style data structures via
      targeted roundtrip tests.

### cardano-crypto-class
- [x] Track progress through [Phase 03 – Cardano VRF Parity](phase-03-vrf-parity.md). ✅ **COMPLETED**
- [x] Track progress through [Phase 04 – DSIGN Algorithm Parity](phase-04-dsign-parity.md). ✅ **COMPLETED**
- [ ] In addition, cover KES and hashing suites for parity against the
      Haskell library (consider additional phase docs as needed).
- [ ] Confirm mlocked memory utilities meet the security guarantees documented
      in Haskell (zeroisation, page locking, error propagation).
- [ ] Provide compatibility tests against known Haskell outputs for KES.

### cardano-git-rev
- [x] Finalise unsafe export handling for Rust 2024 (`#[unsafe(no_mangle)]`).
- [x] Mirror the CLI/build-script behaviour from the Haskell toolchain (e.g.
      fallback logic when Git metadata is missing).
- [x] Add integration tests that simulate build-time stamping under various
      repository states (dirty tree, detached HEAD).
- [x] Document usage expectations for downstream crates/binaries.

### cardano-slotting
- [ ] Port slotting arithmetic (slot length, epochs, `SlottingData`) with unit
      tests derived from Haskell’s property suite.
- [ ] Implement time calculations and conversions (slots ↔ POSIX time) ensuring
      rounding/overflow semantics match.
- [ ] Validate interaction with `cardano-base` types such as `SlotNo`,
      `EpochNo`, `EpochSize`.
- [ ] Add clock-skew and boundary condition tests mirroring Haskell cases.

### cardano-strict-containers
- [ ] Port strict maps, sequences, and helper combinators ensuring structural
      sharing semantics match.
- [ ] Benchmark against Haskell behaviour for large datasets (memory usage,
      laziness vs strictness guarantees).
- [ ] Confirm integration with `nothunks` (no unexpected thunks in stored
      structures).
- [ ] Expand doc comments with usage guidance for downstream crates.

### cardano-vrf-pure
- [x] Coordinated with [Phase 03](phase-03-vrf-parity.md); ensure every low-level
      primitive has matching tests and documentation. ✅ **COMPLETED**
- [x] Add micro-benchmarks to spot regressions versus libsodium where practical.
- [x] Verify `no_std` compatibility if required by downstream components.

### deepseq
- [ ] Mirror Haskell’s `Control.DeepSeq` behaviour, including custom `NFData`
      derivations for Cardano-specific types.
- [ ] Confirm blanket implementations don’t introduce borrow-checker hazards or
      performance cliffs.
- [ ] Add doc examples showing how consumers ensure deep evaluation.

### heapwords
- [ ] Port the heap word counting utilities and ensure they integrate with Rust
      profiling/measurement tools as expected.
- [ ] Write regression tests using representative Cardano data structures.
- [ ] Document how to run heap measurement reports in the Rust toolchain.

### measures
- [ ] Align metric collection APIs (counters, histograms) with the Haskell
      naming and aggregation semantics.
- [ ] Confirm compatibility with the observability stack (e.g. EKG, Prometheus
      exporters) or document migration steps.
- [ ] Add end-to-end tests/integration harness demonstrating event emission.

### nothunks
- [ ] Implement thunk detection mirroring Haskell’s `NoThunks` typeclass.
- [ ] Ensure diagnostic messages match expectations (path, type names).
- [ ] Add property tests covering shallow vs deep thunks, cycles, and large
      structures.
- [ ] Integrate checks across other crates (e.g. `cardano-strict-containers`) as
      part of CI.

### orphans-deriving-via
- [ ] Collect all orphan instances provided in Haskell and port them explicitly,
      documenting any differences required by Rust’s coherence rules.
- [ ] Ensure exporting strategy doesn’t introduce conflicting implementations in
      downstream crates.
- [ ] Add compile-time tests/examples verifying the instances resolve as
      expected.

---

## Verification checklist
- [ ] `cargo fmt`, `cargo clippy --workspace --all-targets`, and
      `cargo test --workspace` green.
- [ ] Cross-language parity evidence captured (links to Haskell comparison
      outputs, scripts, or notebooks).
- [ ] Each crate section either has all boxes checked or links to an active
      phase file detailing remaining work.

## Reporting cadence
- 06-10-2025-time-00:15: cardano-git-rev now reads patched `_cardano_git_rev` bytes,
  mirrors the Haskell fallback order with one-shot warnings, and ships
  integration/unit tests alongside refreshed README notes.
- 06-10-2025-time-04:25: Stabilised `cardano-git-rev` environment mutation tests,
  restored embedded symbol precedence after runtime patching, and
  revalidated the crate-specific test suite.
- 06-10-2025-time-04:30: Added feature-gated VRF debug logging, removed
      unconditional stdout/stderr noise, and kept `cardano-vrf-pure`
      integration tests green.
- 06-10-2025-time-08:10: Routed Montgomery-path diagnostics through the new
      VRF debug helper, documented the feature flag, and added regression tests
      exercising the Elligator fallback branch.
- 06-10-2025-time-12:30: **Phase 03 VRF Parity COMPLETE** ✅ - Fixed critical sign
      bit handling in hash-to-curve operations. All 35 unit tests pass. Official
      test vectors vrf_ver03_standard_10 and vrf_ver03_generated_1 produce
      byte-for-byte identical proofs and VRF outputs. Documented in
      `cardano-vrf-pure/VRF_PARITY_COMPLETE.md`.
- 06-10-2025-time-18:45: **Phase 04 DSIGN Parity COMPLETE** ✅ - Implemented and
      validated comprehensive test harnesses for Ed25519, ECDSA Secp256k1, and
      Schnorr Secp256k1. All 31 tests passing (100%). RFC 8032, RFC 6979, and
      BIP340 compliance validated. Documented in `PHASE_04_COMPLETION_REPORT.md`.
- 06-10-2025-time-21:05: Repaired `.devcontainer/devcontainer.json` by restoring
      the base image and Rust feature wiring so contributors can rebuild the
      container without manual tweaks. Validated JSON structure locally and
      documented the rebuild flow in the root `README.md`.

---

_This roadmap lives at `.github/tasks/phase-00-workspace-roadmap.md`. Update it
whenever you plan or complete work on any crate._
