# Phase 05 – KES Implementation Audit Report

**Date**: October 6, 2025
**Auditor**: AI Agent
**Scope**: Key Evolving Signatures (KES) implementations in `cardano-crypto-class`

---

## Executive Summary

This audit evaluates the current state of KES (Key Evolving Signatures) implementations in the Rust `cardano-crypto-class` crate against the Haskell reference implementation from `cardano-base`. KES is critical for Cardano's consensus protocol (Ouroboros Praos), providing forward-secure signatures that evolve over discrete time periods.

### Key Findings

✅ **Implemented**: All core KES algorithms are present, use mlocked memory, and expose the full Haskell API surface
✅ **Regression Harnesses**: CompactSum levels 1–7 now validate byte-for-byte against regenerated JSON fixtures shared with the Haskell generator
✅ **Boundary Coverage**: `tests/kes_boundary.rs` enforces Single/CompactSingle expiry, CompactSum tamper rejection, and out-of-range signing semantics
⚠️ **Pending Validation**: Dedicated SumKES vector harnesses, forward-security proofs, and cross-language serialization checks remain on the roadmap

### Test Coverage Status

| Component | Unit Tests | Integration Tests | Haskell Vectors | Status |
|-----------|------------|-------------------|-----------------|--------|
| KesAlgorithm trait | ✅ Trait API smoke tests | ✅ Boundary exercises | ⚠️ Pending | Tighten property coverage |
| SingleKes | ✅ Boundary suite | ✅ Tamper/expiry | ⚠️ Pending | Capture Haskell vectors |
| CompactSingleKes | ✅ Boundary suite | ✅ Embedded VK checks | ⚠️ Pending | Capture Haskell vectors |
| SumKes | ⚠️ Smoke | ⚠️ Limited | ❌ None | Build dedicated harness |
| CompactSumKes | ✅ Boundary + vectors | ✅ Evolution walks | ✅ Levels 1–7 | Maintain fixture parity |
| Hash (Blake2b) | ✅ 4 tests | ✅ Complete | N/A | Good |
| Mlocked Memory | ⚠️ Basic | ✅ 4 tests | N/A | Adequate |

---

## Implementation Analysis

### 1. Core KES Trait (`kes/mod.rs` - 306 lines)

**Implemented**:
- ✅ `KesAlgorithm` trait with full API surface
- ✅ `UnsoundKesAlgorithm` trait for test-only operations
- ✅ `SignedKes<A, M>` wrapper type
- ✅ Error types (`KesError`, `KesMError`)
- ✅ Period type alias (`Period = u64`)

**Core Operations**:
- ✅ `total_periods()` - Returns max periods for algorithm
- ✅ `derive_verification_key()` - VK from SK
- ✅ `sign_kes()` - Sign at specific period
- ✅ `verify_kes()` - Verify signature at period
- ✅ `update_kes()` - Evolve key to next period
- ✅ `gen_key_kes()` / `gen_key_kes_from_seed_bytes()` - Key generation
- ✅ Serialization methods for VK, SK (unsound), Signature
- ✅ `forget_signing_key_kes()` - Secure key zeroisation
- ✅ `hash_verification_key_kes<H>()` - VK hashing

**Haskell Parity**:
- ✅ API matches `Cardano.Crypto.KES.Class`
- ✅ Error handling equivalent
- ✅ Signable representation support
- ⚠️ No `ContextKES` type parameter yet (using associated type)

---

### 2. SingleKES (`kes/single.rs` - 116 lines)

**Description**: Basic single-period KES wrapping a DSIGNM algorithm.

**Implementation Status**:
- ✅ Wraps `Ed25519` (via `DsignMAlgorithm`)
- ✅ `total_periods()` returns 1
- ✅ Sign/verify delegate to DSIGN
- ✅ Update returns None (no evolution for single period)
- ✅ Serialization implemented

**Test Coverage**:
- ✅ Key generation from seed (deterministic)
- ✅ Sign and verify round-trip
- ✅ DirectSerialise round-trip
- ✅ Boundary expiry and out-of-range signing via `tests/kes_boundary.rs`
- ⚠️ NO test vectors from Haskell

**Gaps**:
- ⚠️ No test vectors comparing with Haskell `SingleKES Ed25519DSIGN`
- ⚠️ Limited negative coverage (e.g., corrupted signature serialization)

---

### 3. CompactSingleKES (`kes/compact_single.rs` - 262 lines)

**Description**: Single-period KES with embedded verification key in signature (for CompactSum composition).

**Implementation Status**:
- ✅ `OptimizedKesSignature` trait for VK embedding
- ✅ `CompactSingleSig<D>` includes `signature` and `verification_key`
- ✅ Sign/verify with VK embedding
- ✅ Used as base case for CompactSumKes
- ✅ Serialization includes VK in signature

**Key Features**:
```rust
pub struct CompactSingleSig<D: DsignMAlgorithm> {
    pub(crate) signature: D::Signature,
    pub(crate) verification_key: D::VerificationKey,
}
```

**Test Coverage**:
- ✅ Key generation from seed
- ✅ Sign and verify with embedded VK
- ✅ DirectSerialise round-trip
- ✅ Verification key extraction checks (`tests/kes_boundary.rs`)
- ⚠️ NO test vectors from Haskell

**Gaps**:
- ⚠️ No test vectors comparing with Haskell `CompactSingleKES Ed25519DSIGN`
- ⚠️ Missing integration tests that exercise CompactSingle inside all CompactSum levels beyond vector-based coverage

---

### 4. SumKES (`kes/sum.rs` - 507 lines)

**Description**: Binary tree composition for multi-period KES (Sum0 through Sum7).

**Implementation Status**:
- ✅ Recursive binary tree structure
- ✅ Left/right subtree key management
- ✅ Period splitting (first half T, second half T)
- ✅ VK hashing with Blake2b256
- ✅ Key evolution through `update_kes`
- ✅ Type aliases: `Sum0Kes` through `Sum7Kes`

**Key Structure**:
```rust
pub struct SumSigningKey<D, H> {
    pub(crate) sk: D::SigningKey,
    pub(crate) r1_seed: Option<MLockedBytes>,  // Seed for right subtree
    pub(crate) vk0: D::VerificationKey,         // Left VK
    pub(crate) vk1: D::VerificationKey,         // Right VK
    _phantom: PhantomData<H>,
}
```

**Period Support**:
- Sum0: 1 period (= SingleKes)
- Sum1: 2 periods (2^1)
- Sum2: 4 periods (2^2)
- Sum3: 8 periods (2^3)
- Sum4: 16 periods (2^4)
- Sum5: 32 periods (2^5)
- Sum6: 64 periods (2^6)
- Sum7: 128 periods (2^7)

**Test Coverage**:
- ✅ Sum1Kes key generation
- ✅ Sum2Kes key generation
- ✅ Deterministic key generation
- ✅ Sign at different periods (0, 1)
- ✅ Key evolution (period 0 → 1)
- ❌ NO test vectors from Haskell
- ⚠️ Only tested up to Sum2, not Sum5/Sum6/Sum7

**Gaps**:
- ❌ No comprehensive evolution testing (all periods 0 → max)
- ❌ No test vectors for Sum1, Sum2, Sum5 from Haskell
- ❌ No period boundary edge cases (sign at period >= total_periods)
- ❌ No verification after evolution (forward security)
- ⚠️ No tests for signature format compatibility with Haskell

---

### 5. CompactSumKES (`kes/compact_sum.rs` - 397 lines)

**Description**: Optimized sum composition storing only ONE VK per branch node.

**Implementation Status**:
- ✅ Recursive structure like SumKes
- ✅ Only stores "off-side" VK in signature
- ✅ Reconstructs "on-side" VK from signature
- ✅ Smaller signature size than SumKes
- ✅ Type aliases: `CompactSum0Kes` through `CompactSum7Kes`

**Key Optimization**:
```rust
pub struct CompactSumSignature<D, H> {
    pub(crate) sigma: D::Signature,      // Nested signature (includes VK if D is optimized)
    pub(crate) vk_other: D::VerificationKey,  // Only ONE VK, not TWO
    _phantom: PhantomData<H>,
}
```

**Size Comparison**:
```
SumKES signature:     D::SIG_SIZE + 2 * D::VK_SIZE
CompactSumKES signature: D::SIG_SIZE + 1 * D::VK_SIZE
```

**Test Coverage**:
- ✅ Serde-gated regression harness validates levels 1–7 against regenerated
   JSON fixtures (signing, verification, raw serialisation)
- ✅ Boundary suite confirms tamper rejection, embedded verification keys, and
   expiry semantics
- ⚠️ Forward-security and signature-size assertions remain TODOs

**Gaps**:
- ⚠️ Add explicit size regression to document compact signature savings
- ⚠️ Extend coverage to cross-check SumKes and CompactSum outputs over the same
   message set for each level
- ⚠️ Capture fixtures for CompactSum levels beyond 7 once the generator grows

---

### 6. Hash Algorithms (`kes/hash.rs` - 117 lines)

**Description**: Blake2b hashing for KES VK composition.

**Implementation Status**:
- ✅ `KesHashAlgorithm` trait
- ✅ `Blake2b256` (32-byte output)
- ✅ `Blake2b512` (64-byte output)
- ✅ Seed expansion
- ✅ Concatenation hashing

**Test Coverage**:
- ✅ Output size validation
- ✅ Seed expansion
- ✅ Hash concatenation
- ✅ Well-tested

**Status**: ✅ **Good - No issues identified**

---

### 7. Verification Key Hashing (`kes/verify_hash.rs` - 52 lines)

**Description**: Utilities for hashing verification keys in KES compositions.

**Implementation Status**:
- ✅ VK compatibility traits
- ✅ Blake2b256 usage for Sum types

**Test Coverage**:
- ✅ VK compatibility
- ✅ Blake2b256 usage

**Status**: ✅ **Good - Adequate coverage**

---

### 8. Mlocked Memory Security

**Files**: `src/mlocked_bytes.rs`, `src/mlocked_seed.rs`

**Implementation Status**:
- ✅ `MLockedBytes` for secure key storage
- ✅ Memory locking via `mlock()`
- ✅ Zeroisation on drop
- ✅ Clone creates new mlocked region
- ✅ DirectSerialise support

**Test Coverage** (from `mlocked_bytes.rs`):
- ✅ Allocate zeroed memory
- ✅ Copy operations
- ✅ Zero memory
- ✅ Clone copies contents
- ✅ Aligned allocation
- ✅ Dynamic allocate and clone

**Security Tests** (from `kes_direct_serialise.rs`):
- ✅ Multiple keys are independent
- ✅ DirectSerialise security

**Gaps**:
- ⚠️ No explicit testing of zeroisation after forget
- ⚠️ No testing of mlock failure scenarios
- ⚠️ No memory leak tests (valgrind or similar)

---

## Haskell Reference Comparison

### Test Suite Structure

The Haskell test suite in `cardano-crypto-tests` includes:

1. **Property Tests**:
   - `prop_onlyGenSignKeyKES` - Key generation succeeds
   - `prop_onlyGenVerKeyKES` - VK derivation succeeds
   - `prop_oneUpdateSignKeyKES` - Single evolution works
   - `prop_allUpdatesSignKeyKES` - Evolution through all periods
   - `prop_totalPeriodsKES` - Period count matches spec
   - `prop_deriveVerKeyKES` - Same VK from same SK
   - `prop_noErasedBlocksInKey` - No forgotten memory chunks
   - `prop_verifyKES_*` - Various verification tests

2. **Serialization Tests**:
   - Raw serialization round-trips
   - Size validation
   - CBOR encoding/decoding
   - DirectSerialise tests

3. **Negative Tests**:
   - Wrong key verification fails
   - Wrong message verification fails
   - Wrong period verification fails

4. **Unsound Pure API Tests**:
   - Test-only operations
   - Key generation without mlocking
   - Update without IO

### Tested Algorithms in Haskell

```haskell
testKESAlgorithm @(MockKES 7) lock "MockKES"
testKESAlgorithm @(SimpleKES Ed25519DSIGN 7) lock "SimpleKES"
testKESAlgorithm @(SingleKES Ed25519DSIGN) lock "SingleKES"
testKESAlgorithm @(Sum1KES Ed25519DSIGN Blake2b_256) lock "Sum1KES"
testKESAlgorithm @(Sum2KES Ed25519DSIGN Blake2b_256) lock "Sum2KES"
testKESAlgorithm @(Sum5KES Ed25519DSIGN Blake2b_256) lock "Sum5KES"
testKESAlgorithm @(CompactSum1KES Ed25519DSIGN Blake2b_256) lock "CompactSum1KES"
testKESAlgorithm @(CompactSum2KES Ed25519DSIGN Blake2b_256) lock "CompactSum2KES"
testKESAlgorithm @(CompactSum5KES Ed25519DSIGN Blake2b_256) lock "CompactSum5KES"
```

### Not Implemented in Rust

- ❌ `MockKES` - Test-only mock implementation
- ❌ `SimpleKES` - Alternative single period implementation
- ❌ `NeverUsedKES` - Placeholder implementation

---

## Test Vector Requirements

Based on the Haskell test suite, we need test vectors for:

### 1. SingleKES Test Vectors

- Key generation from specific seed
- Signing at period 0
- Verification key derivation
- Signature format
- Serialization (VK, SK, Sig)
- Period boundary enforcement (period > 0 should fail)

### 2. CompactSingleKES Test Vectors

- Same as SingleKES
- Plus: Verification key embedded in signature
- Plus: VK extraction from signature

### 3. SumKES Test Vectors (for Sum1, Sum2, Sum5)

For each level:
- Key generation from seed
- Signing at various periods (0, 1, T-1, T, T+1, 2T-1)
- VK derivation (should be stable across all periods)
- Key evolution sequence (0 → 1 → 2 → ... → max)
- Subtree switching at period T
- Signature format at different periods
- Serialization at different evolution stages

### 4. CompactSumKES Test Vectors (for CompactSum1, CompactSum2, CompactSum5)

- ✅ Levels 1–7 captured in `compact_sum_kes_test_vectors.json` (signatures,
  verification keys, evolution transcripts)
- TODO: Extend fixtures with explicit signature-size assertions
- TODO: Cross-validate CompactSum vs Sum over the same vector sets

### 5. Forward Security Tests

- Old signing keys cannot be recovered after evolution
- Signatures from old periods remain valid after evolution
- Cannot sign for past periods after evolution

### Test Vector Files

Existing:

- `cardano-test-vectors/test_vectors/kes/compact_sum_kes_test_vectors.json`
   (levels 1–7, regeneration script checked in)

Pending:

- `single_kes_test_vectors.json` (~10–20 vectors)
- `compact_single_kes_test_vectors.json` (~10–20 vectors)
- `sum1_kes_test_vectors.json`, `sum2_kes_test_vectors.json`, `sum5_kes_test_vectors.json`
   (20–40 vectors each)
- Additional CompactSum fixtures once new levels or encodings are required

### Test Harness Files

Existing:

- `compact_sum_kes_vectors.rs` (serde-gated vector harness)
- `kes_boundary.rs` (expiry, tamper, and out-of-range checks)

Pending:

- `kes_single_vectors.rs` (~300–400 lines)
- `kes_compact_single_vectors.rs` (~300–400 lines)
- `kes_sum_vectors.rs` (~500–600 lines covering Sum levels)
- `kes_forward_security.rs` (~200–300 lines)
- `kes_integration.rs` (~300–400 lines blending KES + DSIGN + VRF contexts)

### Documentation Files

- `PHASE_05_AUDIT.md` (this document)
- `PHASE_05_TEST_VECTOR_REPORT.md` (new) – summarise generator outputs once
   Single/Sum fixtures land
- `SINGLE_KES_PARITY_COMPLETE.md`, `SUM_KES_PARITY_COMPLETE.md`,
   `COMPACT_SUM_KES_PARITY_COMPLETE.md` – create upon achieving full parity for
   each family, mirroring the VRF parity reports
4. **⚠️ Cross-family parity checks**
   - Impact: CompactSum vs Sum signatures are not exercised over a shared
     corpus
   - Risk: MEDIUM – Divergent verification paths may mask issues
   - Mitigation: Reuse generated vectors to compare both families on identical
     messages and periods

5. **⚠️ Signature size validation**
   - Impact: Compact signature savings are not asserted in tests
   - Risk: LOW – Efficiency rather than correctness, but worth documenting
   - Mitigation: Add regression checks that record and compare signature byte
     lengths across families

6. **⚠️ Serialization interoperability**
   - Impact: No automated round-trips against Haskell encoders/decoders
   - Risk: LOW – Format skew would break network exchanges
   - Mitigation: Add cross-language serde tests once vector extraction
     produces CBOR + raw snapshots

### Nice-to-Have Gaps (Quality)

7. **ℹ️ Negative matrix expansion**
   - Impact: Boundary suite covers tamper cases for CompactSum only
   - Mitigation: Extend wrong-period, wrong-message, and malformed-deserialise
     coverage to SumKES and vector-driven harnesses

8. **ℹ️ Performance benchmarks**
   - Impact: Runtime characteristics remain unmeasured in Rust
   - Mitigation: Capture criterion benchmarks once parity work settles

---

## Recommendations

### Phase 05 Execution Plan

**Completed (October 2025)**
- Regenerated `compact_sum_kes_test_vectors.json` covering levels 1–7 and wired
  serde-gated regression harnesses
- Added `tests/kes_boundary.rs` to assert Single/CompactSingle expiry and
  CompactSum tamper scenarios

**Next Focus**
1. Extend the Haskell generator to emit SingleKES, CompactSingleKES, and SumKES
   vector suites (including raw/CBOR serialisations)
2. Implement dedicated harnesses mirroring the CompactSum structure for those
   families
3. Add forward-security regressions that evolve keys through every period and
   assert failure outside the allowed window
4. Introduce signature-size and cross-family comparison tests to codify compact
   savings
5. Capture serialization round-trips against the Haskell reference once vector
   files include both encodings

### Success Criteria

✅ **Minimum Viable**:
- [ ] SingleKES matches Haskell (10+ test vectors)
- [ ] CompactSingleKES matches Haskell (10+ test vectors)
- [ ] Sum1KES and Sum2KES validated (20+ test vectors each)
- [x] CompactSum1KES and CompactSum2KES validated (20+ test vectors each, levels 1–7 via `compact_sum_kes_test_vectors.json`)
- [ ] Forward security tests passing

✅ **Production Ready**:
- [ ] Sum5KES validated (30+ test vectors, 32 periods tested)
- [ ] CompactSum5KES validated (30+ test vectors)
- [ ] All period evolutions tested
- [ ] Negative test cases comprehensive
- [ ] Mlocked memory security validated
- [ ] Integration tests with cardano-ledger types

✅ **Gold Standard**:
- [ ] All Sum levels tested (Sum0 through Sum7)
- [ ] All CompactSum levels tested
- [ ] Performance benchmarks documented
- [ ] Memory leak tests passing
- [ ] Cross-serialization with Haskell validated

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation Priority |
|------|------------|--------|---------------------|
| Signature incompatibility with Haskell | High | Critical | **P0 - Immediate** |
| Forward security not working | Medium | Critical | **P0 - Immediate** |
| Period evolution bugs | Medium | High | **P1 - Week 1** |
| CompactSum VK reconstruction wrong | Low | Medium | **P2 - Week 3** |
| Mlocked memory leaks | Low | High | **P2 - Week 2** |
| Deep tree (Sum7) untested | Medium | Medium | **P2 - Week 3** |
| Performance issues | Low | Medium | **P3 - Future** |

---

## Files to Create

### Test Vector Files

```
cardano-test-vectors/test_vectors/kes/
├── single_kes_test_vectors.json          (~10-20 vectors)
├── compact_single_kes_test_vectors.json  (~10-20 vectors)
├── sum1_kes_test_vectors.json            (~20-30 vectors)
├── sum2_kes_test_vectors.json            (~20-30 vectors)
├── sum5_kes_test_vectors.json            (~30-40 vectors)
├── compact_sum1_kes_test_vectors.json    (~20-30 vectors)
├── compact_sum2_kes_test_vectors.json    (~20-30 vectors)
└── compact_sum5_kes_test_vectors.json    (~30-40 vectors)
```

### Test Harness Files

```
cardano-crypto-class/tests/
├── kes_single_vectors.rs           (~300-400 lines)
├── kes_compact_single_vectors.rs   (~300-400 lines)
├── kes_sum_vectors.rs              (~500-600 lines, all Sum levels)
├── kes_compact_sum_vectors.rs      (~500-600 lines, all CompactSum levels)
├── kes_forward_security.rs         (~200-300 lines)
└── kes_integration.rs              (~300-400 lines)
```

### Documentation Files

```
PHASE_05_AUDIT.md            (this document)
PHASE_05_TEST_VECTOR_REPORT.md
SINGLE_KES_PARITY_COMPLETE.md
SUM_KES_PARITY_COMPLETE.md
COMPACT_SUM_KES_PARITY_COMPLETE.md
PHASE_05_COMPLETION_REPORT.md
```

---

## Conclusion

The KES implementation in Rust is **functionally complete** but **not yet validated** against the Haskell reference. The core algorithms are present, mlocked memory infrastructure is working, and basic tests pass. However, without comprehensive test vectors from Haskell, we cannot guarantee byte-for-byte compatibility or validate critical forward security properties.

**Estimated Effort**: 3-4 weeks for full validation and parity
**Critical Path**: Test vector extraction → SingleKES → SumKES → CompactSumKES
**Primary Risk**: Signature format incompatibility with cardano-node

**Next Steps**:
1. ✅ Create this audit document
2. ⏭️ Extract SingleKES test vectors from Haskell
3. ⏭️ Implement SingleKES test harness
4. ⏭️ Validate and iterate

---

**Audit Status**: ✅ **COMPLETE**
**Phase 05 Status**: 📋 **READY TO PROCEED**
**Recommendation**: **BEGIN TEST VECTOR EXTRACTION**
