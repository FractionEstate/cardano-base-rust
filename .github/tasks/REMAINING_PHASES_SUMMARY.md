# Remaining Crates: Phase Planning Summary

**Date:** 2025-01-08
**Status:** Planning complete, ready to execute

---

## Overview

This document provides a high-level summary of the remaining work to achieve full Haskell parity across all `cardano-base-rust` workspace crates. Phases 03-07 (crypto and CBOR) are complete. This planning covers the remaining infrastructure, container, and utility crates.

## Completed Phases (Reference)

| Phase | Crate | Status | Tests | Notes |
|-------|-------|--------|-------|-------|
| 03 | `cardano-vrf-pure` | ✅ Complete | 35 | Byte-for-byte parity with Haskell |
| 04 | `cardano-crypto-class` (DSIGN) | ✅ Complete | 31 | Ed25519, ECDSA, Schnorr validated |
| 05 | `cardano-crypto-class` (KES) | ✅ Complete | 415 | All KES variants, mlocked-metrics |
| 06 | `cardano-crypto-class` (Hash) | ✅ Complete | 219 | Blake2b, SHA-2/3, Keccak, RIPEMD |
| 07 | `cardano-binary` | ✅ Complete | 86 | CBOR parity, Criterion benchmarks |

**Total Completed Tests:** 415 workspace-wide (as of Phase 05)

---

## Remaining Phases

### Phase 08 – Cardano Slotting Parity
**Document:** `.github/tasks/phase-08-slotting-parity.md`
**Crate:** `cardano-slotting`
**Status:** ☐ Not started
**Estimated Effort:** 6-9 days (1-2 weeks)

#### Scope
- Slot/epoch arithmetic (SlotNo, EpochNo, EpochSize)
- Time conversions (slots ↔ POSIX timestamps)
- Epoch information providers (fixed and variable epoch sizes)
- Clock skew handling
- Overflow/underflow protection

#### Key Deliverables
- Core types with checked arithmetic
- Time conversion functions (SystemStart, SlotLength)
- FixedEpochInfo and ExtendedEpochInfo providers
- Property-based tests (associativity, monotonicity)
- Haskell cross-validation with mainnet parameters
- Performance benchmarks

#### Dependencies
- `cardano-binary` (CBOR serialization) ✅
- None blocking

---

### Phase 09 – Cardano Strict Containers Parity
**Document:** `.github/tasks/phase-09-strict-containers-parity.md`
**Crate:** `cardano-strict-containers`
**Status:** ☐ Not started
**Estimated Effort:** 10-12 days (2 weeks)

#### Scope
- StrictSeq - strict sequence container
- StrictMap - strict ordered map
- StrictMaybe - strict optional value
- Integration with `nothunks` validation
- CBOR serialization with canonical ordering

#### Key Deliverables
- StrictSeq with efficient operations (push, pop, split, append)
- StrictMap with BTreeMap-like ordering for CBOR
- StrictMaybe with forced evaluation
- NoThunks trait implementations
- Property tests (container laws)
- Haskell cross-validation
- Performance benchmarks vs standard containers

#### Dependencies
- `nothunks` crate (Phase 10) - can be developed in parallel
- `cardano-binary` (CBOR) ✅

---

### Phase 10 – Helper Crates Parity
**Document:** `.github/tasks/phase-10-helper-crates-parity.md`
**Crates:** `deepseq`, `heapwords`, `measures`, `nothunks`
**Status:** ☐ Not started
**Estimated Effort:** 12-17 days (2-3 weeks)

#### Scope
Each helper crate provides debugging/profiling utilities:

1. **deepseq** (1-2 days)
   - Deep strictness evaluation
   - DeepSeq trait (likely minimal in Rust)
   - Documentation of Rust differences

2. **heapwords** (2-3 days)
   - Memory footprint calculation
   - HeapSize trait for types
   - Recursive size calculation for containers

3. **measures** (3-4 days)
   - Performance metrics (time, memory)
   - Stopwatch, histograms, percentiles
   - JSON/Prometheus export

4. **nothunks** (2-3 days)
   - Thunk detection (validate strictness)
   - NoThunks trait with derive macros
   - Integration with strict containers
   - Debug-mode assertions

#### Key Deliverables
- Per-crate trait definitions and implementations
- Integration with Cardano types (crypto, containers)
- Feature-gated debug checks (zero overhead in release)
- Comprehensive documentation

#### Dependencies
- None blocking (can develop in parallel with Phase 08-09)
- `nothunks` needed by Phase 09, but can coordinate

---

### Phase 11 – Deriving-Via Crates Parity
**Document:** `.github/tasks/phase-11-deriving-via-parity.md`
**Crates:** `base-deriving-via`, `orphans-deriving-via`
**Status:** ☐ Not started
**Estimated Effort:** 12-14 days (2-3 weeks)

#### Scope

1. **base-deriving-via** (6-7 days)
   - Semigroup/Monoid traits
   - Newtype wrappers (Sum, Product, First, Last)
   - InstantiatedAt pattern (phantom type tagging)
   - Derive macros for common traits

2. **orphans-deriving-via** (3 days)
   - Newtype wrappers for foreign types
   - Workarounds for Rust orphan rules
   - Re-export strategy
   - Type aliases

#### Key Deliverables
- Semigroup/Monoid trait definitions
- Derive macros for automatic implementations
- InstantiatedAt<T, Tag> wrapper
- Integration with Cardano types
- Property tests (monoid/semigroup laws)
- Examples demonstrating patterns

#### Dependencies
- Should integrate with Phase 08-10 crates
- Can develop core traits/macros independently

---

### Phase 12 – Cardano Base Crate Parity
**Document:** `.github/tasks/phase-12-cardano-base-parity.md`
**Crate:** `cardano-base`
**Status:** ☐ Not started (blocks on Phase 08-11)
**Estimated Effort:** 10-11 days (2 weeks)

#### Scope
- Unified re-exports of all sub-crates
- Prelude module for common imports
- Unified error type (CardanoBaseError)
- Helper utilities and type aliases
- Integration tests across crates
- Examples demonstrating full stack
- Comprehensive documentation

#### Key Deliverables
- Module hierarchy mirroring Haskell
- `cardano_base::prelude` for ergonomic imports
- Cross-crate integration tests
- Usage examples (crypto, CBOR, slotting, containers)
- Full-stack performance benchmarks
- Haskell → Rust migration guide
- Release preparation (versions, CI, docs)

#### Dependencies
- **BLOCKS on completion of Phase 08-11**
- All sub-crates must have stable APIs

---

## Execution Strategy

### Recommended Order

1. **Phase 08 (Slotting)** - Start immediately
   - Critical for blockchain operations
   - Relatively self-contained
   - No dependencies on other remaining phases

2. **Phase 10 (Helper Crates)** - Start in parallel with Phase 08
   - Provides utilities needed by Phase 09
   - Can develop `nothunks` first to unblock Phase 09
   - `deepseq`, `heapwords`, `measures` can follow

3. **Phase 09 (Strict Containers)** - After `nothunks` ready
   - Needs `nothunks` for validation
   - Can start once Phase 10 `nothunks` is functional
   - Important for memory-safe long-running processes

4. **Phase 11 (Deriving-Via)** - Start after Phase 08-09 in progress
   - Provides ergonomic APIs for Phase 08-10 crates
   - Can integrate incrementally as other phases complete
   - Derive macros take time to stabilize

5. **Phase 12 (Cardano Base)** - Final integration phase
   - Wait for Phase 08-11 to complete
   - Focus on clean APIs and documentation
   - Prepare for release

### Parallel Development

Can run in parallel:
- Phase 08 (Slotting) + Phase 10 (Helper Crates)
- Phase 09 (Containers) after Phase 10 `nothunks` + Phase 11 (Deriving-Via)

Critical path:
```
Phase 08 ──┐
           ├──> Phase 09 ──┐
Phase 10 ──┘               ├──> Phase 12
                           │
Phase 11 ──────────────────┘
```

### Estimated Total Time

- **Sequential:** ~50-63 days (10-13 weeks, 2.5-3 months)
- **With parallelization:** ~35-45 days (7-9 weeks, 1.5-2 months)

---

## Risk Mitigation

### High-Risk Areas

1. **Strict Containers (Phase 09)**
   - Risk: Strictness semantics differ from Haskell
   - Mitigation: Extensive property tests, `nothunks` validation

2. **Derive Macros (Phase 11)**
   - Risk: Macro complexity, compilation errors
   - Mitigation: Start with hand-written impls, expand gradually

3. **Integration (Phase 12)**
   - Risk: API changes during integration
   - Mitigation: Establish stable APIs early, semantic versioning

### Medium-Risk Areas

1. **Time Conversions (Phase 08)**
   - Risk: Rounding inconsistencies, overflow
   - Mitigation: Property tests, Haskell cross-validation

2. **Orphan Rules (Phase 11)**
   - Risk: Cannot implement needed traits
   - Mitigation: Newtype wrappers, re-export strategy

3. **Performance (All Phases)**
   - Risk: Regression vs Haskell
   - Mitigation: Criterion benchmarks, early profiling

---

## Success Criteria (All Phases)

### Functional
- [ ] All workspace tests pass (`cargo test --workspace`)
- [ ] Haskell cross-validation tests green for all crates
- [ ] Property tests validate mathematical laws
- [ ] No panics on edge cases (overflow, invalid input)

### Quality
- [ ] `cargo clippy --workspace` clean
- [ ] `cargo fmt --all` passes
- [ ] Documentation complete (READMEs, CHANGELOGs, API docs)
- [ ] Examples compile and run

### Performance
- [ ] Benchmarks establish baselines
- [ ] No significant regressions vs Haskell
- [ ] Memory usage within acceptable bounds

### Integration
- [ ] Cross-crate tests validate interactions
- [ ] Feature flags work correctly
- [ ] CI covers all targets

---

## Tracking Progress

### Phase Documents
Each phase has a dedicated document:
- `.github/tasks/phase-08-slotting-parity.md`
- `.github/tasks/phase-09-strict-containers-parity.md`
- `.github/tasks/phase-10-helper-crates-parity.md`
- `.github/tasks/phase-11-deriving-via-parity.md`
- `.github/tasks/phase-12-cardano-base-parity.md`

### Master Roadmap
- `.github/tasks/phase-00-workspace-roadmap.md`
- Update status and checkboxes as work progresses
- Add dated reporting cadence entries

### Git Commits
- Commit after completing major milestones
- Reference phase documents in commit messages
- Keep commits focused and well-documented

---

## Questions & Clarifications

If you encounter any of the following, document in the relevant phase document:

1. **Haskell semantics unclear** - Note in "Audit & Analysis" section
2. **Rust patterns differ significantly** - Document in phase doc, update roadmap
3. **Performance concerns** - Add benchmarks, document in phase doc
4. **API design questions** - Discuss in phase doc, link to related issues
5. **Testing gaps** - Add to verification checklist

---

## Next Steps

1. ✅ Phase planning complete (this document)
2. 🔄 Review and approve phase documents
3. ☐ Begin Phase 08 (Slotting) implementation
4. ☐ Begin Phase 10 (Nothunks) implementation in parallel
5. ☐ Track progress in phase documents and Phase 00 roadmap

---

_This summary lives at `.github/tasks/REMAINING_PHASES_SUMMARY.md`. Update after significant progress or plan changes._
