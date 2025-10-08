# Phase 06 – Hash Algorithm Parity

**Status:** ☑ In progress  \
**Primary owners:** _Unassigned_  \
**Supporting crates:** `cardano-crypto-class`, `cardano-vrf-pure`

---

## Objective
Achieve byte-for-byte behavioural parity between the Rust hashing implementations and the
Haskell `cardano-base` hashing modules. This includes all general-purpose and
Cardano-specific hash functions used across signatures, addresses, KES, VRF, and
serialization (e.g. Blake2b256/512, SHA256, SHA512, SHA3/Keccak variants, RIPEMD160,
composite / staged hashing helpers, and hash-to-fixed-length utilities).

## Success Criteria
- Deterministic output equality with Haskell reference for every supported hash
  (golden vectors regenerated or imported).
- Length / domain separation semantics match exactly (no accidental truncation or padding differences).
- Composite helpers (double SHA256, hash160-style combinations) validated against reference cases.
- Constant-time equality semantics preserved (where applicable) and tested.
- Clear Haskell→Rust module/function mapping documented.
- Benchmarks established for per-hash throughput to detect regressions.
- Documentation updated (README + module docs if needed) summarising mapping and guarantees.

## Scope
### Hash Functions & Helpers
1. Blake2b256 / Blake2b512
2. SHA256 / SHA512
3. SHA3-256 / SHA3-512 vs Keccak256 distinction (confirm semantics & naming parity)
4. Keccak256 (legacy / Ethereum compatibility)
5. RIPEMD160
6. Hash160 / combined hash constructions (e.g. SHA256 then RIPEMD160)
7. Double SHA256 (sha256d)
8. Address / verification key hashing helpers (as used in KES / DSIGN / VRF modules)
9. Packed / direct serialisation hash helpers (if any per Haskell `Cardano.Crypto.Hash.*`)

### Out of Scope (future phases)
- Merkle tree / incremental hashing for large structures (unless required by upstream parity checklist).
- Ledger-specific script/data hash forms (reserved for ledger-focused phase).

## Milestone Checklist
### 1. Audit & Mapping
- [ ] Enumerate Haskell modules:
  - `Cardano.Crypto.Hash`
  - `Cardano.Crypto.Hash.Class`
  - `Cardano.Crypto.Hash.Blake2b`
  - `Cardano.Crypto.Hash.SHA256`
  - `Cardano.Crypto.Hash.SHA512`
  - `Cardano.Crypto.Hash.Keccak`
  - `Cardano.Crypto.Hash.RIPEMD160`
- [ ] Produce Rust mapping table (README / doc comment) referencing each exported item.
- [ ] Identify any missing algorithms or parameterisations.

#### Current Rust Coverage Snapshot (initial audit)
| Haskell Module / Concept | Rust Location / Function | Status |
|--------------------------|--------------------------|--------|
| SHA256 (`Hash.SHA256`) | `hash::sha256` | Implemented |
| Double SHA256 (Bitcoin) | `hash::sha256d` | Implemented |
| SHA512 (`Hash.SHA512`) | `hash::sha512` | Implemented |
| SHA3-256 | `hash::sha3_256` | Implemented |
| SHA3-512 | `hash::sha3_512` | Implemented |
| Keccak-256 | `hash::keccak256` | Implemented |
| RIPEMD160 | `hash::ripemd160` | Implemented |
| Hash160 (RIPEMD160(SHA256)) | `hash::hash160` | Implemented |
| Blake2b-256 | (present in KES module `kes/hash.rs` – integrate mapping) | Implemented (scoped) |
| Blake2b-512 | (present in KES module `kes/hash.rs`) | Implemented (scoped) |
| Domain-separated / composite helpers | (Some in KES / DSIGN contexts) | Partial / needs enumeration |
| Incremental streaming API parity | (Digest traits via external crates) | Pending evaluation |

> NOTE: Blake2b helpers reside in `cardano-crypto-class/src/kes/hash.rs`; decide whether to re-export or document path.
> UPDATE: Blake2b256 / Blake2b512 now re-exported via `hash.rs` for a unified API; added explicit known-answer tests
> (empty string and "hello world") to lock digest stability before adding larger vector sets.

### 2. Test Vector Acquisition
- [x] Gather or regenerate vectors for each algorithm (empty, short, multi-block, long messages).
- [x] Include edge cases: 0-length, 1-byte, block-size−1, block-size, block-size+1, large streaming input.
- [x] Add composite vectors (hash160, double SHA256) with known Bitcoin / Ethereum style examples.

### 3. Implementation Validation
- [ ] Cross-check output lengths & constant definitions.
- [ ] Verify truncation logic for fixed-length outputs (Blake2b256 from Blake2b512 domain, etc.).
- [ ] Confirm difference between Keccak256 and standardized SHA3-256 (padding rules) with explicit tests.
- [ ] Ensure no unintended panics on empty input or large input.

### 4. Performance Benchmarks
- [ ] Criterion benchmarks per hash (small, medium, large buffers) capturing MB/s.
- [ ] Baseline JSON/CSV export for future comparisons.
- [ ] Add regression threshold / trend note (manual for now; automation later).

### 5. Security & Constant-Time Behaviour
- [ ] Verify constant-time equality (timing smoke test / statistical sampling or code review of primitives used).
- [ ] Document side-channel considerations (no branching on secret data in wrappers).

### 6. Integration Points
- [ ] Re-run KES / DSIGN / VRF tests with any updated hashing internals (should remain green).
- [ ] Add targeted test ensuring address / verification key hashing path unchanged (if applicable).

### 7. Documentation
- [ ] Add mapping table (Haskell→Rust) to root or crate README (avoid duplicate if already present; extend existing mapping).
- [ ] Describe differences (if any) between Keccak and SHA3 naming and usage.
- [ ] Note composite helper usage contexts (address construction, proof formatting).

### 8. Parity Evidence
- [x] Store golden test vectors in `cardano-test-vectors` (hash subdirectory).
- [ ] Provide script or instructions for regenerating vectors from Haskell.

### 9. Completion & Sign-off
- [ ] All checklist items ticked or consciously deferred with rationale.
- [ ] CHANGELOG entries summarising added tests / benchmarks / docs.
- [ ] Phase status moved to Completed.

## Verification Checklist
- [ ] `cargo test -p cardano-crypto-class --features serde` green.
- [ ] New hash vector tests pass in `cardano-test-vectors`.
- [ ] Criterion benchmarks run without panic.
- [ ] Outputs match Haskell-produced vectors (cross-language script logged / referenced).
- [ ] No added unsafe code or external crypto dependencies.

## Reporting Cadence
- (YYYY-MM-DD) INIT: Phase scaffold created.
- (YYYY-MM-DD) AUDIT: Haskell mapping complete, missing items enumerated.
- (2025-10-08) VECTORS: Expanded hash corpus in `cardano-test-vectors` with boundary/multi-block coverage, added Rust generator + parity harness updates (`tests/hash_vectors.rs`), docs & changelog refreshed; Blake2b now included alongside SHA/Keccak/RIPEMD outputs.
- (2025-10-08) COMPOSITES: Added Bitcoin genesis header/public key and canonical Ethereum legacy transaction fixtures to `hash_test_vectors.json`; regenerated parity harness, updated docs/CHANGELOG, and validated `sha256d`/`hash160` pathways over real-world inputs.
- (2025-10-08) WARNINGS: Eliminated lingering `kes_haskell_parity` dead-field warnings by asserting raw signature hex dumps and enforcing populated descriptions in the JSON fixtures; full `cargo test -p cardano-crypto-class` run is clean.
- (YYYY-MM-DD) BENCH: Baseline benchmarks captured.
- (YYYY-MM-DD) COMPLETE: Parity confirmed, docs & CHANGELOG updated.

## Risk Assessment
| Risk | Impact | Mitigation |
|------|--------|------------|
| Divergent Keccak vs SHA3 semantics | Incorrect consensus / address hashes | Explicit tests & doc notes |
| Incomplete vector coverage | Silent truncation bug | Exhaustive size & boundary cases |
| Performance regression | Slower consensus path | Benchmarks with trend tracking |

## References
- Haskell source: `cardano-base/cardano-crypto-class/src/Cardano/Crypto/Hash/*`
- Specification: FIPS 202 (SHA3), BLAKE2 RFC 7693, RIPEMD160 (ISO/IEC 10118-3 reference).
- Existing Rust modules: `cardano-crypto-class/src/hash.rs` (and submodules as relevant).

---
_Document lives at `.github/tasks/phase-06-hash-parity.md`._
