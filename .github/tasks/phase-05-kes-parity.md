# Phase 05 – KES Algorithm Parity

**Status:** ☐ Not started / ☑ In progress / ☐ Blocked / ☐ Completed  \
**Primary owners:** @FractionEstate  \
**Supporting crates:** `cardano-crypto-class`

---

## Objective

Validate and achieve 100% functional parity between the Rust KES (Key Evolving Signature) implementations and the reference Haskell implementations. KES is critical for Cardano's blockchain consensus protocol (Ouroboros Praos), providing forward-secure signatures that evolve over time periods.

## Success Criteria

- All KES implementations (SingleKES, SumKES, CompactSingleKES, CompactSumKES) pass comprehensive tests
- Test vectors extracted from Haskell reference and validated
- Key evolution, signing, and verification operations match Haskell byte-for-byte
- mlocked memory utilities function correctly (secure key storage, zeroisation)
- Period boundaries and expiration handling work correctly
- Forward security properties validated
- Documentation complete with usage examples

## Scope

### KES Algorithms to Cover

1. **SingleKES** - Basic KES with single period (wraps DSIGN)
2. **CompactSingleKES** - Single KES with embedded verification key
3. **SumKES** - Binary tree composition (Sum0 through Sum7)
4. **CompactSumKES** - Optimized sum with reduced verification keys (CompactSum0 through CompactSum7)

### Key Features

- **Forward Security**: Old keys cannot be reconstructed after evolution
- **Time Periods**: Keys evolve through discrete time periods
- **Merkle Tree Structure**: SumKES uses binary tree for key composition
- **Mlocked Memory**: Signing keys stored in secure memory (protected from swapping)
- **Key Evolution**: `update_kes` advances to next time period
- **Signature Verification**: Includes period validation

### Out of Scope (Not Implemented Yet)

- MockKES - Test-only implementation
- NeverUsedKES - Placeholder implementation
- SimpleKES - Alternative single period implementation

---

## Milestone Checklist

### 1. Audit and Analysis

- [X] Compare Rust KES implementations against Haskell reference
  - `Cardano.Crypto.KES.Class` - Core KES typeclass
  - `Cardano.Crypto.KES.Single` - Single period implementation
  - `Cardano.Crypto.KES.CompactSingle` - Compact single with embedded VK
  - `Cardano.Crypto.KES.Sum` - Standard sum composition
  - `Cardano.Crypto.KES.CompactSum` - Optimized compact sum

- [X] Document differences and missing features
- [X] Identify test vectors for each algorithm
- [X] Review forward security guarantees and mlocked memory usage

### 2. Test Vector Extraction

- [ ] Extract KES test vectors from Haskell test suite
  - `cardano-crypto-tests/src/Test/Crypto/KES.hs`
  - [x] Single/CompactSingle sign/verify vectors (deterministic Rust reproduction aligned with Haskell coverage)
  - [x] Sum hierarchical vectors (levels 1-6 captured via `sum_kes_test_vectors.json`)
  - [x] CompactSum hierarchical vectors *(levels 1–7 regenerated after recursive verification key reconstruction landed)*
  - [ ] Period evolution sequences

- [ ] Create JSON test vector files
  - [x] `single_kes_test_vectors.json`
  - [x] `compact_single_kes_test_vectors.json`
  - [x] `sum_kes_test_vectors.json`
  - [x] `compact_sum_kes_test_vectors.json` *(levels 1–7 regenerated from the Rust implementation after recursive vk wiring)*

- [x] Migrate vectors to `cardano-test-vectors` crate

### 3. SingleKES Parity

- [ ] Verify key generation from seeds
  - Deterministic key derivation
  - Verification key derivation from signing key

- [ ] Validate signing operations
  - Period 0 signing
  - Message signing with DSIGN backend

- [ ] Confirm verification logic
  - Signature validation
  - Period checking
  - Error cases (wrong period, expired key)

- [ ] Test serialization
  - Signing key serialization (mlocked)
  - Verification key serialization
  - Signature serialization

### 4. CompactSingleKES Parity

- [ ] Embedded verification key in signature
  - Signature includes VK for Merkle tree reconstruction
  - OptimizedKesSignature trait implementation

- [ ] Integration with CompactSumKES
  - Base case for compact binary tree
  - VK extraction from signature

- [ ] Test vectors
  - Sign/verify with embedded VK
  - Serialization roundtrip

### 5. SumKES Parity

- [ ] Binary tree composition
  - Left/right subtree key management
  - Period splitting (first half vs second half)
  - VK hashing for branch nodes

- [ ] Key evolution through periods
  - Update from period t to t+1
  - Subtree switching at period boundaries
  - Key expiration at max period

- [ ] Hierarchical verification
  - Verification with period routing
  - VK reconstruction from signature path

- [ ] Test all Sum levels
  - Sum0 (= SingleKES, 1 period)
  - Sum1 (2 periods)
  - Sum2 (4 periods)
  - Sum3 (8 periods)
  - Sum4 (16 periods)
  - Sum5 (32 periods)
  - Sum6 (64 periods)
  - Sum7 (128 periods)

### 6. CompactSumKES Parity

- [ ] Optimized VK storage
  - Only one VK per branch node (not two)
  - "Off-side" VK in signature, "on-side" reconstructed

- [ ] Signature structure
  - Nested signatures with embedded VKs
  - Smaller total size than SumKES

- [ ] Key evolution
  - Same period boundaries as SumKES
  - Efficient subtree switching

- [ ] Test all CompactSum levels
  - CompactSum0 through CompactSum7
  - Verify size reduction vs standard Sum

### 7. Mlocked Memory Security

- [ ] Verify memory locking
  - Signing keys stored in mlocked memory
  - mlock/munlock system calls succeed
  - Memory not swappable to disk

- [ ] Zeroisation on drop
  - Keys cleared when dropped
  - No residual key material in memory

- [ ] Error propagation
  - mlock failures detected
  - Graceful degradation if locking unavailable

### 8. Forward Security Validation

- [ ] Key evolution is one-way
  - Cannot reconstruct previous keys from evolved key
  - Old signing keys are zeroised after evolution

- [ ] Period boundaries enforced
  - Cannot sign with expired key
  - Cannot sign for past periods

- [ ] Signature immutability
  - Old signatures remain valid after key evolution
  - Verification works for all historical periods

### 9. Integration Tests

- [ ] End-to-end KES workflow
  - Generate → Sign → Evolve → Sign → Verify all

- [ ] Cross-level compatibility
  - Sum compositions work correctly
  - CompactSum matches Sum verification

- [ ] Error handling
  - Period out of range
  - Key expired
  - Invalid signatures
  - Serialization errors

### 10. Performance Benchmarks

- [ ] Key generation speed
- [ ] Signing throughput
- [ ] Verification throughput
- [ ] Evolution speed
- [ ] Memory usage (mlocked bytes)
- [ ] Compare against Haskell reference

### 11. Documentation

- [ ] API documentation for all KES types
- [ ] Forward security explanation
- [ ] Period evolution guide
- [ ] Mlocked memory usage notes
- [ ] Migration guide from Haskell
- [ ] Example usage code
- [ ] Update CHANGELOG

---

## Verification Checklist

- [ ] `cargo fmt && cargo clippy --workspace --all-targets`
- [ ] `cargo test --workspace` - All tests passing
- [ ] `cargo test -p cardano-crypto-class kes` - KES-specific tests
- [ ] Cross-validation with Haskell outputs for each algorithm
- [ ] Memory leak checks (valgrind or similar)
- [ ] Forward security properties validated
- [ ] Security review completed

---

## Dependencies & References

### Haskell Source
- `cardano-base/cardano-crypto-class/src/Cardano/Crypto/KES/*.hs`
- Test vectors: `cardano-crypto-tests/test_vectors/`
- Property tests: `cardano-crypto-tests/src/Test/Crypto/KES.hs`

### Specifications
- **MMM Paper**: "Composition and Efficiency Tradeoffs for Forward-Secure Digital Signatures"
  by Tal Malkin, Daniele Micciancio, and Sara Miner
  https://eprint.iacr.org/2001/034
- **Ouroboros Praos**: KES usage in Cardano consensus

### Rust Implementation
- `cardano-crypto-class/src/kes/mod.rs` - Core KES trait
- `cardano-crypto-class/src/kes/single.rs` - SingleKES
- `cardano-crypto-class/src/kes/compact_single.rs` - CompactSingleKES
- `cardano-crypto-class/src/kes/sum.rs` - SumKES family
- `cardano-crypto-class/src/kes/compact_sum.rs` - CompactSumKES family
- `cardano-crypto-class/src/kes/hash.rs` - Blake2b hashing for KES
- `cardano-crypto-class/src/mlocked_bytes.rs` - Secure memory management

### Existing Tests
- `cardano-crypto-class/tests/kes_gen_key_from_seed.rs` - Key generation
- `cardano-crypto-class/tests/kes_direct_serialise.rs` - Serialization
- `cardano-crypto-class/tests/kes_exports.rs` - Public API exports
- `cardano-crypto-class/tests/sum_kes_unblocked.rs` - Sum KES operations

---

## Risk Assessment

### High Priority Risks

1. **Mlocked Memory Portability**: mlock behavior varies across platforms
   - Mitigation: Test on Linux, macOS, Windows; graceful degradation

2. **Forward Security**: Must ensure old keys cannot be recovered
   - Mitigation: Comprehensive zeroisation tests, security audit

3. **Period Evolution Correctness**: Off-by-one errors could break consensus
   - Mitigation: Extensive test coverage, cross-validation with Haskell

### Medium Priority Risks

1. **Verification Key Reconstruction**: CompactSum VK derivation is complex
   - Mitigation: Detailed test vectors, step-by-step validation

2. **Performance**: KES operations are on consensus critical path
   - Mitigation: Benchmark and optimize hot paths

3. **Memory Leaks**: Mlocked memory must be freed correctly
   - Mitigation: Use valgrind, check for leaks in CI

---

## Estimated Effort

- **Audit & Planning**: 1 day
- **Test Vector Extraction**: 1-2 days
- **SingleKES/CompactSingleKES Parity**: 1-2 days
- **SumKES Parity**: 2-3 days
- **CompactSumKES Parity**: 2-3 days
- **Mlocked Memory Security**: 1-2 days
- **Forward Security Validation**: 1 day
- **Integration Tests**: 1-2 days
- **Performance Benchmarks**: 1 day
- **Documentation**: 1-2 days
- **Total**: 12-20 days (approximately 2-4 weeks)

---

## Reporting Cadence

- Update the **Status** line and tick checkboxes as work progresses.
- Provide short status notes (date + bullet) under this section:
  - **2025-10-06**: Phase 05 initiated after DSIGN parity completion. Status: In progress, Owner: @FractionEstate
  - **2025-10-07**: Generated deterministic SingleKES and CompactSingleKES vectors, embedded them in `cardano-test-vectors`, and added verification tests. Next up: SumKES vector extraction.
  - **2025-10-08**: Added SumKES generator coverage in `cardano-test-vectors`, produced `sum_kes_test_vectors.json`, and wired regression tests for levels 1–6. CompactSum fixtures remain outstanding.
  - **2025-10-06-time-21:45**: Ported CompactSumKES level-1 vectors into `cardano-test-vectors`, added regression hooks in both crates, and documented remaining work for deeper compact levels.
  - **2025-10-09**: Reworked CompactSumKES verification to recover recursive branch keys, regenerated fixtures for levels 1–7, expanded regression tests, and refreshed docs/CHANGELOG entries. Next: continue parity validation for evolution edge cases.
  - **2025-10-10**: Regenerated CompactSum fixtures after recursive vk changes, confirmed serde-gated regression in `cardano-crypto-class` exercises levels 1–7, and ran `cardano-crypto-class`/`cardano-test-vectors` test suites. Next: focus on evolution boundary edge cases and Single/CompactSingle parity sign-off.
  - **2025-10-10**: Locked down `SingleKES`/`CompactSingleKES` expiry semantics, added boundary and tamper regression tests for CompactSum, and refreshed crate docs to reflect the new coverage. Next: expand evolution edge-case validation for SumKES variants.

---

## Related Work

- **Completed**: Phase 03 - VRF Parity (100% complete)
- **Completed**: Phase 04 - DSIGN Parity (100% complete)
- **Upcoming**: Phase 06 - Hash Algorithm Parity
- **Future**: Phase 07 - CBOR Serialization

---

## Current Implementation Status

### ✅ Implemented
- `KesAlgorithm` trait
- `SingleKes` - basic single period KES
- `CompactSingleKes` - with embedded VK
- `SumKes` family (Sum0-Sum7)
- `CompactSumKes` family (CompactSum0-CompactSum7)
- `Blake2b256` and `Blake2b512` hash functions
- Mlocked memory utilities
- Serialization/deserialization

### ⏳ Needs Validation
- Test vectors against Haskell reference
- Forward security guarantees
- Period evolution correctness
- Signature compatibility
- Error handling edge cases

### ❓ Unknown Status
- Full parity with all Haskell KES implementations
- CompactSum optimization correctness
- Mlocked memory security across platforms

---

_This document lives at `.github/tasks/phase-05-kes-parity.md`. Update it after every meaningful change._
