# Phase 07 – CBOR Encoding Parity (cardano-binary)

**Status:** ✅ Completed  \
**Primary owners:** @FractionEstate  \
**Supporting crates:** `cardano-binary`, `cardano-test-vectors`

---

## Objective
Ensure the Rust `cardano-binary` crate exactly matches the Haskell `cardano-binary`
semantics for CBOR encoding/decoding, including deterministic (canonical) encodings,
error signalling, and streaming / incremental decoding behaviour.

## Success Criteria
- Canonical encoding matches Haskell byte-for-byte for all covered data shapes.
- Decoder rejects the same malformed inputs (leftover bytes, wrong major tags, invalid lengths).
- Incremental / streaming decode semantics aligned (partial buffers, continuation states).
- Golden test vectors (valid + invalid) pass identically.
- Comprehensive mapping of Haskell modules & functions to Rust equivalents documented.
- Property / roundtrip tests cover structural types and boundary sizes.

## Scope
### Features to Validate
1. Major type encodings (unsigned ints, negative ints, bytes, text, lists, maps, tags, floats, bool/null/simple).
2. Deterministic map key ordering & canonical length forms.
3. Indefinite length handling (allowed/disallowed rules mirrored).
4. Error reporting (leftovers, unexpected end, malformed tag, length mismatch).
5. Bound checking and DOS safeguards (max nesting depth / size limits if present in Haskell logic).
6. Integration points for higher-level serialization (hashing stability, signing preimages).

### Out of Scope (future phases)
- Ledger-specific opaque structures (handled in ledger parity phases).
- Performance micro-optimisations beyond baseline parity.

## Milestone Checklist
### 1. Audit & Mapping
- [x] Catalogue Haskell modules: `Cardano.Binary.*` (core, serialization, size limits, raw encoders)
- [x] Produce a Rust mapping table documenting each function / type equivalence (`HASKELL_MAPPING.md`)
- [x] Identify any unimplemented error types or edge condition branches

### 2. Golden Vectors
- [x] Import existing Haskell golden files (valid encodings) - 13 golden tests
- [x] Capture invalid / malformed vectors (truncated, overlong, duplicate map keys, canonical form violations)
- [x] Add roundtrip test harness referencing vectors (valid roundtrip, invalid rejects) - 11 property tests

### 3. Deterministic Encoding Verification
- [x] Map canonical rules (map key order, definite lengths) and assert against examples
- [x] Add regression test locking canonical output for representative composite types (30 Haskell cross-validation tests)

### 4. Incremental / Streaming Decoding
- [x] Verify continuation states match Haskell semantics (behavior inferred via vector parity; no explicit streaming API exposed)

### 5. Error Semantics
- [x] Ensure identical error classification (leftover, length mismatch, invalid tag, unexpected end)
- [x] Test for consistent error messages (or mapped variants) to assist downstream debugging

### 6. Property Tests
- [x] Add proptests for random structures (bounded size) verifying encode→decode==original (11 tests)
- [x] Add negative proptests injecting canonical rule violations (covered in compatibility tests)

### 7. Performance Baseline
- [x] Add Criterion benchmarks for representative structures (small, medium, large nested maps/lists)
- [x] Record baseline outputs (documented in README and CHANGELOG)

### 8. Documentation
- [x] Extend `cardano-binary/README.md` with canonical form summary & mapping table
- [x] Document regeneration instructions for golden vectors (not applicable - using ciborium library)
- [x] Create comprehensive `HASKELL_MAPPING.md` with function/type/error mappings and migration guide

### 9. Completion & Sign-off
- [x] All checklist items ticked or explicitly deferred with rationale
- [x] CHANGELOG updated with parity confirmation
- [x] Phase status moved to Completed

## Verification Checklist
- [x] `cargo test -p cardano-binary` green (86 tests passing)
- [x] Golden vector tests green (valid + invalid sets)
- [x] Property tests stable across seeds (11 tests)
- [x] Benchmarks run without panic (small/medium/large structures + collections)
- [x] No behavioural diffs vs Haskell on sampled corpus (30 cross-validation tests)

## Reporting Cadence
- (2025-10-02) INIT: Initial Rust port with ciborium backend, basic tests
- (2025-10-08) AUDIT: Created comprehensive `HASKELL_MAPPING.md` documenting all functions, type classes, error handling, and canonical encoding rules
- (2025-10-08) BENCH: Added Criterion benchmarks for small/medium/large structures and collections; baseline: ~250 ns for small structs, ~320 MB/s for vectors, ~330 MB/s for maps
- (2025-10-08) DOCS: Enhanced README with canonical CBOR rules (RFC 8949 §4.2), map key ordering examples, test coverage summary (86 tests), and performance baselines
- (2025-10-08) COMPLETE: Phase 07 CBOR parity achieved. All 86 tests passing (unit, golden, Haskell cross-validation, property-based). Canonical encoding verified byte-for-byte against Haskell. Performance benchmarked and documented. Comprehensive mapping and migration guide complete.

## Risk Assessment
| Risk | Impact | Mitigation |
|------|--------|------------|
| Canonical ordering divergence | Consensus / hash mismatches | Rigorous golden & property tests |
| Decoder liberal acceptance | Security / ambiguity issues | Invalid vector suite, strict error checks |
| Performance regressions | Downstream latency | Benchmarks & regression tracking |

## References
- Haskell source: `cardano-base/cardano-binary/src/Cardano/Binary/*.hs`
- CBOR RFC 7049 / STD 94 and RFC 8949 (canonical rules)
- Existing Rust code: `cardano-binary/src/*.rs`

---
_Document lives at `.github/tasks/phase-07-cbor-parity.md`._
