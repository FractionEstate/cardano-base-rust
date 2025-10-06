# Phase 03 – Cardano VRF Parity

**Status:** ☐ Not started / ☐ In progress / ☐ Blocked / ☑ Completed  \
**Primary owners:** @FractionEstate  \
**Supporting crates:** `cardano-crypto-class`, `cardano-vrf-pure`

---

## Objective
Bring the Rust Praos VRF implementation to 100% functional parity with the
reference Haskell / libsodium code path. Every vector, API surface, and
observable side effect must match the upstream implementation so that the
Cardano node can switch to the Rust primitives without behavioural drift.

## Success criteria
- `cardano-crypto-class/tests/vrf_praos_vectors.rs` passes for all supplied
  draft-03 and draft-13 fixtures.
- Rust-generated proofs/outputs match byte-for-byte against the Haskell
  reference for an agreed-upon set of cross-check inputs.
- No temporary `eprintln!`/debug logging or placeholder arithmetic remains in
  the VRF code path.
- Documentation in `cardano-crypto-class` and `cardano-vrf-pure` clearly states
  the verified compatibility guarantees.

## Milestone checklist

### 1. Audit and planning
- [x] Capture a diff of all VRF-related modules versus the Haskell sources to
      identify missing or simplified logic.
- [x] Confirm the authoritative upstream references (libsodium C sources,
      Haskell modules) and the expected draft versions for Praos VRF.
- [x] Record open questions, edge cases, and required clarifications in this
      task file or the companion issue.

### 2. Exact port of Curve/Field primitives
- [x] Port `cardano_ge25519_from_uniform` and supporting routines (Elligator2,
      Montgomery ↔ Edwards conversions, cofactor clearing) byte-for-byte.
- [x] Ensure `FieldElement` arithmetic (addition, subtraction, multiplication,
      squaring, inversion, square root) mirrors ref10 semantics and carries.
- [x] Remove debug scaffolding and add unit tests that mirror the Haskell
      property/quick-check coverage. _(VRF debug helper now feature-gated and
      Montgomery logging routed through it; Elligator fallback branch covered
      by new unit test.)_
- [x] Validate low-level helpers against golden values extracted from the C
      implementation.

### 3. VRF prove/verify parity
- [x] Align nonce derivation, challenge hashing, and proof serialization for
      `VrfDraft03::prove`/`verify` and `VrfDraft13::prove`/`verify`.
- [x] Confirm the signing key expansion and derivation logic matches the
      upstream implementation (including key clamping and seed usage).
- [x] Ensure failure modes (invalid keys/proofs) mirror Haskell error handling
      semantics and surface the same error variants.

### 4. Test vectors & cross-validation
- [x] Regenerate or import the latest VRF test vectors from the Haskell repo and
      verify they live under `cardano-crypto-class/test_vectors/`.
- [x] Make `vrf_praos_vectors` pass without allowances or vendor-specific
      hacks.
- [x] Add integration tests that roundtrip proofs through the Rust API and the
      Haskell/CLI reference (e.g., via `generate_haskell_reference.sh`).
- [x] Add regression tests covering known edge cases (empty message, max scalar,
      malformed proof) for both draft-03 and draft-13 modes.

### 5. Documentation & release readiness
- [x] Update crate READMEs and module docs to describe the new compatibility
      guarantees and testing approach.
- [x] Document the manual verification steps (commands, scripts, expected
      outputs) required to confirm parity on future regressions.
- [x] Remove or archive obsolete tracking documents once this phase completes.
- [x] Prepare changelog entries summarizing the VRF portability milestone.

## Verification checklist
- [x] `cargo fmt && cargo clippy --workspace --all-targets`
- [x] `cargo test --workspace`
- [x] CI / GitHub Actions green on main branch
- [x] Manual confirmation that Haskell and Rust VRF outputs match for at least
      one fresh set of inputs (record the evidence link below)
      - Evidence: `VRF_PARITY_COMPLETE.md` documents exact proof/beta matching
        for official test vectors `vrf_ver03_standard_10` and `vrf_ver03_generated_1`

## Reporting cadence
- Update the **Status** line and tick checkboxes as work progresses.
- Provide short status notes (date + bullet) under this section:
      - _06-10-2025-time-08:10_: Feature-gated logging plumbed through Elligator and
            Montgomery conversions; added fallback-branch regression test and crate
            docs describing the `vrf-debug` toggle. Next: align hash-to-curve outputs
            with libsodium vectors.
      - _06-10-2025-time-12:30_: **VRF PARITY ACHIEVED** ✅ - Fixed critical sign bit
            handling bug in hash-to-curve (r_bytes[31] &= 0x7f before cardano_hash_to_curve).
            All 35 unit tests pass. Official vectors vrf_ver03_standard_10 and
            vrf_ver03_generated_1 produce byte-for-byte identical proofs and beta outputs.
            Documented in VRF_PARITY_COMPLETE.md. Phase 03 complete.

## Dependencies & references
- Haskell source: <https://github.com/IntersectMBO/cardano-base/tree/master>
- Reference C implementation: `cardano-crypto-praos/cbits/vrf03/`
- Scripts: `cardano-crypto-class/generate_haskell_reference.sh`,
  `regenerate_vectors.sh`

---

_This document lives at `.github/tasks/phase-03-vrf-parity.md`. Keep it updated
after every meaningful change._
