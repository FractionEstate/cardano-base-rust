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
- [x] Document deriving helpers, macro usage, and Haskell mapping for the Rust
      port.
- [ ] Audit Haskell `Cardano.Base.DerivingVia` modules to confirm all deriving
      helpers are present.
- [ ] Port any Template Haskell-based instances to macro-free Rust equivalents
      or document the chosen abstraction.
- [ ] Add property tests ensuring the derived behaviours match the Haskell
      expectations (e.g. Semigroup/Monoid laws).
- [ ] Cross-reference usage sites in dependent crates to confirm API coverage.

### cardano-base
- [x] Document feature flag primitives, README/CHANGELOG parity, and Haskell
      module mapping for the Rust port.
- [ ] Catalogue all exported modules from Haskell `Cardano.Base.*` and map them
      to existing Rust modules.
- [ ] Implement missing primitives (e.g. canonical JSON, text utilities) with
      thorough tests.
- [ ] Mirror the error-handling semantics (Either vs Result) and documented edge
      cases from Haskell.
- [x] Document migration notes for downstream crates that previously depended on
      the Haskell version.

### cardano-binary
- [x] Document canonical CBOR helpers, README/CHANGELOG parity, and Haskell
      module mapping for the Rust port.
- [x] Align CBOR encoding/decoding semantics with Haskell (including
      canonical/deterministic encoding rules).
- [x] Import or regenerate golden vectors from the Haskell test suite and make
      sure they pass against the Rust implementation (86 tests passing).
- [x] Ensure streaming, incremental decoding, and error reporting behave like
      the original (e.g. leftover bytes, tag mismatches).
- [x] Validate integration with `cardano-ledger` style data structures via
      targeted roundtrip tests.
- [x] Track structured work via [Phase 07 – CBOR Encoding Parity](phase-07-cbor-parity.md). ✅ **COMPLETED**

### cardano-crypto-class
- [x] Document cryptographic primitives, README/CHANGELOG parity notes, and
      regeneration workflows for DSIGN/KES/VRF/hash modules.
- [x] Track progress through [Phase 03 – Cardano VRF Parity](phase-03-vrf-parity.md). ✅ **COMPLETED**
- [x] Track progress through [Phase 04 – DSIGN Algorithm Parity](phase-04-dsign-parity.md). ✅ **COMPLETED**
- [x] Track progress through [Phase 05 – KES Algorithm Parity](phase-05-kes-parity.md). ✅ **COMPLETED**
  - [x] KES parity (Single, CompactSingle, Sum0–7, CompactSum0–7) achieved with unified hierarchical
        fixture harness.
  - [x] Compatibility tests against Haskell outputs (verification keys & signatures) incorporated
        into parity harness; byte-for-byte parity validated for all tracked periods.
  - [x] Zeroisation & secure memory instrumentation in place (feature `mlocked-metrics` counts
        allocations, bytes, zeroizations, failed_locks). Hooks for error propagation implemented.
  - [ ] OS-level page locking behaviour (swap avoidance / induced `mlock` failure) deferred to future
        security-focused phase.
  - [ ] Memory leak checks (valgrind) and formal security review deferred.
- [x] Track progress through [Phase 06 – Hash Algorithm Parity](phase-06-hash-parity.md). ✅ **COMPLETED**

### cardano-git-rev
- [x] Finalise unsafe export handling for Rust 2024 (`#[unsafe(no_mangle)]`).
- [x] Mirror the CLI/build-script behaviour from the Haskell toolchain (e.g.
      fallback logic when Git metadata is missing).
- [x] Add integration tests that simulate build-time stamping under various
      repository states (dirty tree, detached HEAD).
- [x] Document usage expectations for downstream crates/binaries.

### cardano-slotting
- [x] Document slotting primitives, Haskell mapping, and integration guidance
      for the Rust port.
- [ ] Port slotting arithmetic (slot length, epochs, `SlottingData`) with unit
      tests derived from Haskell’s property suite.
- [ ] Implement time calculations and conversions (slots ↔ POSIX time) ensuring
      rounding/overflow semantics match.
- [ ] Validate interaction with `cardano-base` types such as `SlotNo`,
      `EpochNo`, `EpochSize`.
- [ ] Add clock-skew and boundary condition tests mirroring Haskell cases.

### cardano-strict-containers
- [x] Document strict container usage, Haskell mapping, and finger tree
      guidance for downstream crates.
- [ ] Port strict maps, sequences, and helper combinators ensuring structural
      sharing semantics match.
- [ ] Benchmark against Haskell behaviour for large datasets (memory usage,
      laziness vs strictness guarantees).
- [ ] Confirm integration with `nothunks` (no unexpected thunks in stored
      structures).
- [ ] Expand doc comments with usage guidance for downstream crates.

### cardano-vrf-pure
- [x] Document VRF highlights, diagnostics wiring, and Haskell mapping in the
      Rust README/CHANGELOG so downstream crates can cross-reference parity evidence.
- [x] Coordinated with [Phase 03](phase-03-vrf-parity.md); ensure every low-level
      primitive has matching tests and documentation. ✅ **COMPLETED**
- [x] Add micro-benchmarks to spot regressions versus libsodium where practical.
- [x] Verify `no_std` compatibility if required by downstream components.

### deepseq
- [x] Document NFData traits, deriving helpers, and Haskell mapping for the
      Rust port; include guidance on pairing with `base-deriving-via`/`nothunks`.
- [ ] Mirror Haskell’s `Control.DeepSeq` behaviour, including custom `NFData`
      derivations for Cardano-specific types.
- [ ] Confirm blanket implementations don’t introduce borrow-checker hazards or
      performance cliffs.
- [ ] Add doc examples showing how consumers ensure deep evaluation.

### heapwords
- [x] Document HeapWords trait usage, helper combinators, and Haskell mapping
      for the Rust port; include guidance on reporting helpers and compile-time
      constraints.
- [ ] Port the heap word counting utilities and ensure they integrate with Rust
      profiling/measurement tools as expected.
- [ ] Write regression tests using representative Cardano data structures.
- [x] Document how to run heap measurement reports in the Rust toolchain.

### measures
- [x] Document highlights, crate layout, Haskell mapping, and validation steps
      in the README/CHANGELOG to keep distribution guidance in sync with the
      latest Rust port.
- [x] Document measurement APIs, Haskell mapping, and usage notes for the Rust
      port.
- [ ] Align metric collection APIs (counters, histograms) with the Haskell
      naming and aggregation semantics.
- [ ] Confirm compatibility with the observability stack (e.g. EKG, Prometheus
      exporters) or document migration steps.
- [ ] Add end-to-end tests/integration harness demonstrating event emission.

### nothunks
- [x] Document NoThunks usage, diagnostics, and Haskell mapping for the Rust
      port; include guidance on generic deriving and integration across the
      workspace.
- [ ] Implement thunk detection mirroring Haskell’s `NoThunks` typeclass.
- [ ] Ensure diagnostic messages match expectations (path, type names).
- [ ] Add property tests covering shallow vs deep thunks, cycles, and large
      structures.
- [ ] Integrate checks across other crates (e.g. `cardano-strict-containers`) as
      part of CI.

### cardano-test-vectors
- [x] Document fixture layout, regeneration tooling, and Haskell mapping for the
      Rust port; highlight CLI tooling and feature-gated diagnostics.
- [ ] Audit fixture parity against the latest Haskell generators after each
      upstream release and capture deltas in the changelog.
- [ ] Expand cross-language comparison scripts to cover streaming hash cases
      and additional KES evolution scenarios.
- [ ] Automate regeneration tooling within CI to guard against drift (follow-up
      task once parity pipeline is finalised).

### orphans-deriving-via
- [x] Document crate purpose, re-export surface, and Haskell mapping for
      downstream users.
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
- 08-10-2025-time-02:30: **Phase 05 KES Parity COMPLETE** ✅ - KES parity harness
      landed (Single / CompactSingle / Sum1–7 / CompactSum1–7) with hierarchical fixtures;
      forward security tests and serialized size guard added. Added `mlocked-metrics`
      instrumentation (allocations / bytes / zeroizations / failed_locks) and comprehensive
      documentation (forward security narrative, period evolution guide, Haskell→Rust mapping).
      All 415 tests passing. Deferred items: induced `mlock` failure test, OS swap validation,
      Haskell vs Rust perf comparison, formal security review.
- 08-10-2025-time-02:35: **Phase 06 Hash Parity COMPLETE** ✅ - Implemented Blake2b-224/256/512,
      SHA-2/3, Keccak, RIPEMD160, Hash160 with comprehensive test vectors, Criterion benchmarks,
      constant-time comparison helpers, and cross-language validation tooling. All hash tests
      passing (219 total workspace tests). Documentation includes Haskell→Rust mapping and
      regeneration workflow.
- 08-10-2025-time-04:00: **Phase 07 CBOR Parity COMPLETE** ✅ - Comprehensive CBOR encoding/decoding
      parity achieved with 86 tests passing (unit, golden, Haskell cross-validation, property-based).
      Created `HASKELL_MAPPING.md` with complete function/type/error mappings and migration guide.
      Added Criterion benchmarks (baseline: ~250 ns small structs, ~320 MB/s vectors, ~330 MB/s maps).
      Enhanced README with canonical encoding rules (RFC 8949 §4.2), map key ordering examples, and
      verification strategy. All CBOR types, error handling, and deterministic encoding validated
      byte-for-byte against Haskell reference.
- 08-10-2025-time-06:15: Authored `heapwords` README/CHANGELOG documenting usage, helper mapping,
      and test commands. Marked roadmap documentation task complete while leaving parity/test items
      open for future work.
- 09-10-2025-time-03:05: Refreshed `measures` README/CHANGELOG with Haskell mapping, iterator helper
      guidance, and testing notes; added roadmap checkbox to track remaining parity work separately.
- 09-10-2025-time-03:55: Updated `cardano-strict-containers` README/CHANGELOG with strict finger tree
      examples, serde integration guidance, and Haskell lookup table; marked roadmap documentation
      checkbox while keeping remaining parity tasks open.
- 09-10-2025-time-04:20: Added README/CHANGELOG for `orphans-deriving-via`, documenting re-exports and
      Haskell parity; flagged roadmap checklist to reflect completed documentation while parity
      tasks remain outstanding.
- 09-10-2025-time-04:45: Refreshed `base-deriving-via` README/CHANGELOG with highlights, custom
      derivation examples, and Haskell lookup table; marked documentation checkbox while keeping
      parity test items open for future work.
- 10-10-2025-time-01:10: Updated `cardano-slotting` README/CHANGELOG with epoch/time examples,
      Haskell mapping, and testing guidance; marked roadmap documentation task while parity-focused
      work (arithmetic proofs, boundary tests) remains open.
- 10-10-2025-time-03:40: Documented `cardano-base` feature flag primitives, refreshed README with
      Haskell mapping and parsing guidance, converted the changelog to Keep a Changelog format, and
      recorded remaining parity tasks for future implementation.
- 10-10-2025-time-05:05: Refactored `cardano-binary` README with module mapping, canonical encoding
      contract, error-handling guidance, and nested CBOR usage; aligned changelog with Keep a Changelog
      structure and checked off roadmap documentation tasks.
- 10-10-2025-time-07:10: Refreshed `cardano-crypto-class` README with module map, parity summaries
      for DSIGN/KES/VRF/hash, regeneration workflows, and diagnostics guidance; converted the changelog
      to Keep a Changelog format and marked roadmap documentation items complete.
- 10-10-2025-time-08:30: Updated `cardano-git-rev` README with Haskell module mapping, build-script
      workflow, troubleshooting guidance, and testing instructions; adopted Keep a Changelog format and
      captured the work in the crate changelog to keep the documentation campaign consistent.
- 10-10-2025-time-09:15: Expanded `deepseq` README with highlights, crate layout, generic deriving
      patterns, and integration guidance; converted the changelog to Keep a Changelog format and checked
      off the documentation task in the roadmap.
- 10-10-2025-time-10:00: Refreshed `nothunks` README with highlights, generic deriving patterns,
      integration notes, and crate layout; migrated the changelog to Keep a Changelog format and marked
      the roadmap documentation task complete.
- 10-10-2025-time-10:45: Updated `heapwords` README with highlights, integration guidance, crate layout,
      and refined testing instructions; recorded the work in the changelog and marked the roadmap
      documentation item complete.
- 10-10-2025-time-11:30: Refreshed `cardano-test-vectors` README with highlights, fixture layout table,
      Haskell↔Rust mapping, and expanded regeneration instructions; adopted Keep a Changelog structure,
      and marked the roadmap documentation checkbox while leaving parity and automation tasks open.
- 10-10-2025-time-12:15: Reworked `cardano-vrf-pure` README with highlights, layout table, Haskell mapping,
      and diagnostics guidance; noted the changes in the changelog and recorded the documentation
      checkbox while existing performance/parity tasks remain complete.
- 10-10-2025-time-12:45: Refreshed `measures` README with highlights, crate layout, Haskell mapping,
      integration notes, and validation guidance; updated the changelog and roadmap documentation entry.
- 10-10-2025-time-13:05: Added migration notes to `cardano-base` README, logged the change in the
      changelog, and marked the roadmap checklist item so downstream crates have parity guidance when
      swapping from the Haskell feature-flag helpers.

---

_This roadmap lives at `.github/tasks/phase-00-workspace-roadmap.md`. Update it
whenever you plan or complete work on any crate._
