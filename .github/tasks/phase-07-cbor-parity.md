# Phase 07 – CBOR Encoding Parity (cardano-binary)

**Status:** ☐ Not started  \
**Primary owners:** _Unassigned_  \
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
- [ ] Catalogue Haskell modules: `Cardano.Binary.*` (core, serialization, size limits, raw encoders)
- [ ] Produce a Rust mapping table documenting each function / type equivalence.
- [ ] Identify any unimplemented error types or edge condition branches.

### 2. Golden Vectors
- [ ] Import existing Haskell golden files (valid encodings).
- [ ] Capture invalid / malformed vectors (truncated, overlong, duplicate map keys, canonical form violations).
- [ ] Add roundtrip test harness referencing vectors (valid roundtrip, invalid rejects).

### 3. Deterministic Encoding Verification
- [ ] Map canonical rules (map key order, definite lengths) and assert against examples.
- [ ] Add regression test locking canonical output for representative composite types.

### 4. Incremental / Streaming Decoding
- [ ] Introduce chunked decode tests (feed buffer in 1..N byte fragments).
- [ ] Verify continuation states match Haskell semantics (if exposed) or infer matching behaviour via vector parity.

### 5. Error Semantics
- [ ] Ensure identical error classification (leftover, length mismatch, invalid tag, unexpected end).
- [ ] Test for consistent error messages (or mapped variants) to assist downstream debugging.

### 6. Property Tests
- [ ] Add proptests for random structures (bounded size) verifying encode→decode==original.
- [ ] Add negative proptests injecting canonical rule violations.

### 7. Performance Baseline
- [ ] Add Criterion benchmarks for representative structures (small, medium, large nested maps/lists).
- [ ] Record baseline outputs.

### 8. Documentation
- [ ] Extend `cardano-binary/README.md` with canonical form summary & mapping table.
- [ ] Document regeneration instructions for golden vectors.

### 9. Completion & Sign-off
- [ ] All checklist items ticked or explicitly deferred with rationale.
- [ ] CHANGELOG updated with parity confirmation.
- [ ] Phase status moved to Completed.

## Verification Checklist
- [ ] `cargo test -p cardano-binary` green.
- [ ] Golden vector tests green (valid + invalid sets).
- [ ] Property tests stable across seeds.
- [ ] Benchmarks run without panic.
- [ ] No behavioural diffs vs Haskell on sampled corpus.

## Reporting Cadence
- (YYYY-MM-DD) INIT: Phase scaffold created.
- (YYYY-MM-DD) AUDIT: Module mapping complete.
- (YYYY-MM-DD) VECTORS: Golden & malformed vectors integrated.
- (YYYY-MM-DD) PROPS: Property tests stable.
- (YYYY-MM-DD) COMPLETE: Parity validated & documented.

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
