---
layout: page
title: Audit: Rust Implementation vs Original Haskell cardano-base
permalink: /audit/audit-comparison/
---

# Audit: Rust Implementation vs Original Haskell cardano-base

**Date**: October 3, 2025
**Status**: 🔍 In Progress
**Original Repository**: <https://github.com/IntersectMBO/cardano-base>
**Rust Implementation**: FractionEstate/cardano-base-rust

---

## Executive Summary

This audit compares the Rust implementation against the original Haskell `cardano-base` repository to verify:

1. **Completeness**: All critical functionality is ported
2. **Correctness**: Implementations match expected behavior
3. **Security**: Cryptographic operations are secure
4. **Compatibility**: APIs provide equivalent functionality

---

## Package Comparison

### Packages in Original (Haskell)

| Package | Purpose | Status in Rust |
|---------|---------|----------------|
| base-deriving-via | Generic deriving helpers | ✅ Ported |
| cardano-base | Base types and utilities | ✅ Ported |
| cardano-binary | CBOR serialization | ✅ Ported |
| cardano-crypto-class | Main crypto library | ✅ Ported |
| **cardano-crypto-praos** | Praos-specific crypto | ⚠️ **Not ported** |
| **cardano-crypto-tests** | Test utilities | ⚠️ **Not ported** |
| cardano-git-rev | Git revision tracking | ✅ Ported |
| cardano-slotting | Time/slot management | ✅ Ported |
| cardano-strict-containers | Strict containers | ✅ Ported |
| heapwords | Heap allocation tracking | ✅ Ported |
| measures | Measurement abstractions | ✅ Ported |
| orphans-deriving-via | Orphan instance helpers | ✅ Ported |

### Additional Packages in Rust (Not in Original)

| Package | Purpose | Justification |
|---------|---------|---------------|
| cardano-vrf-pure | Pure Rust VRF | ✅ Replaces C bindings with pure Rust |
| deepseq | Deep evaluation | ✅ Rust equivalent of Haskell deepseq |
| nothunks | Thunk detection | ✅ Rust equivalent of Haskell no-thunks |

---

## Critical Findings

### 🔴 HIGH PRIORITY: Missing Packages

#### 1. cardano-crypto-praos (Not Ported)

**Original Purpose**: Praos-specific cryptographic operations

- **Contains**: KES (Key Evolving Signatures), VRF implementations
- **Status**: ⚠️ Partially replaced by cardano-vrf-pure
- **Risk**: May be missing KES functionality

**Action Required**:

- [ ] Verify KES is implemented elsewhere or not needed
- [ ] Check if Praos VRF is fully covered by cardano-vrf-pure
- [ ] Document intentional omission if appropriate

#### 2. cardano-crypto-tests (Not Ported)

**Original Purpose**: Shared test utilities for crypto packages

- **Contains**: Test vectors, property tests, golden tests
- **Status**: ⚠️ Tests may be inline in Rust packages
- **Risk**: May miss cross-package test coverage

**Action Required**:

- [ ] Verify all test vectors are ported
- [ ] Check cross-package test coverage
- [ ] Consider creating equivalent test utilities

---

## Package-by-Package Audit

### ✅ cardano-crypto-class

**Original (Haskell)**: Main cryptographic library with C FFI bindings
**Rust Implementation**: Pure Rust with some FFI for compatibility

#### Structure Comparison

**Original modules**:

- Crypto.Hash (Blake2b, SHA256, etc.)
- Crypto.DSIGN (Ed25519, etc.)
- Crypto.VRF (VRF implementations)
- Crypto.KES (Key Evolving Signatures)

**Rust modules**:

- `src/dsign/` - Digital signatures ✅
- `src/vrf/` - VRF implementations ✅
- `src/mlocked_bytes.rs` - Memory-locked allocations ✅
- `src/seed.rs` - Seed handling ✅
- `src/ffi.rs` - FFI compatibility layer ✅

#### Key Differences

1. **VRF Implementation**:
   - Original: C library bindings via FFI
   - Rust: Pure Rust in `cardano-vrf-pure` + FFI wrapper
   - **Status**: ✅ **IMPROVED** - Pure Rust is safer

2. **Memory Management**:
   - Original: Haskell GC + C allocations
   - Rust: `mlocked_bytes.rs` with explicit memory locking
   - **Status**: ✅ Equivalent security

3. **Missing Features**:
   - ⚠️ KES (Key Evolving Signatures) - Need to verify if ported
   - ⚠️ Some hash functions may not be exposed

#### Action Items

- [ ] Verify KES implementation or document why not needed
- [ ] Compare hash function APIs
- [ ] Check Ed25519 implementation matches original
- [ ] Verify VRF test vectors match

---

### ✅ cardano-binary

**Original (Haskell)**: CBOR serialization using `cborg` library
**Rust Implementation**: Uses `serde_cbor` (deprecated, needs migration)

#### API Comparison

**Original Haskell functions** (approx):

```haskell
serialize :: ToCBOR a => a -> ByteString
deserialize :: FromCBOR a => ByteString -> Either DecoderError a

```

**Rust equivalents**:

```rust
pub fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>, BinaryError>
pub fn decode_full<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, BinaryError>

```

**Status**: ✅ API equivalent, ⚠️ uses deprecated `serde_cbor`

#### Known Issues

1. ⚠️ **serde_cbor deprecated** - Migration to ciborium planned
2. ✅ Error handling improved in Rust version
3. ✅ Documentation added (13 functions now documented)

---

### ✅ cardano-vrf-pure

**Original**: Part of `cardano-crypto-praos` (C bindings)
**Rust**: Pure Rust implementation

#### VRF Variants

**Implemented**:

- ✅ IETF VRF Draft-03 (ECVRF-ED25519-SHA512-ELL2)
- ✅ IETF VRF Draft-13 (ECVRF-ED25519-SHA512-TAI)

**Test Coverage**:

- ✅ 9 standard test vectors
- ✅ Property tests
- ✅ Batch verification tests

**Status**: ✅ **IMPROVED** - Pure Rust eliminates C dependencies

---

### ✅ cardano-slotting

**Original (Haskell)**: Time and slot arithmetic
**Rust**: Direct port with equivalent functionality

**Key Functions**:

- Slot arithmetic ✅
- Time conversions ✅
- Epoch calculations ✅

**Status**: ✅ Complete port

---

### ⚠️ cardano-base

**Original (Haskell)**: Base types and utilities
**Rust**: Appears minimal - needs deeper review

#### Concerns

- Original has extensive type-level programming
- Rust version may not replicate all type safety
- Need to compare core types

#### Action Items

- [ ] Compare core type definitions
- [ ] Verify feature flags match
- [ ] Check if type-level guarantees are preserved

---

## Security Audit

### Memory Safety

| Aspect | Original (Haskell) | Rust | Status |
|--------|-------------------|------|--------|
| Memory leaks | GC prevents | Ownership prevents | ✅ Equivalent |
| Use-after-free | Impossible | Impossible | ✅ Equivalent |
| Buffer overflows | Runtime checks | Compile-time + runtime | ✅ **IMPROVED** |
| Secret wiping | Manual C code | `mlocked_bytes.rs` | ✅ Equivalent |

### Cryptographic Operations

| Operation | Original | Rust | Verified |
|-----------|----------|------|----------|
| Ed25519 signing | C library | `ed25519-dalek` | ⚠️ **TODO** |
| Blake2b hashing | C library | `blake2` crate | ⚠️ **TODO** |
| VRF Draft-03 | C library | Pure Rust | ✅ |
| VRF Draft-13 | C library | Pure Rust | ✅ |
| KES | C library | ❓ **UNKNOWN** | ❌ **TODO** |

---

## Test Coverage Comparison

### Original (Haskell)

- Unit tests in each package
- cardano-crypto-tests shared utilities
- Golden tests for serialization
- Property tests with QuickCheck

### Rust Implementation

- ✅ 148 unit tests passing
- ✅ VRF test vectors (9 tests)
- ⚠️ No shared test utilities package
- ⚠️ Property testing not evident

### Gaps

1. ❌ **No property tests** (Rust has `proptest` - not used)
2. ❌ **No golden tests** for serialization stability
3. ⚠️ **Test coverage lower** than original

---

## API Compatibility

### Serialization Format

**CRITICAL**: CBOR format must be byte-identical

- [ ] **TODO**: Compare serialization output byte-for-byte
- [ ] **TODO**: Verify deserialize(serialize(x)) == x for all types
- [ ] **TODO**: Test against Haskell-generated CBOR

### Cryptographic Signatures

**CRITICAL**: Signatures must be verifiable by Haskell code

- [ ] **TODO**: Cross-verify Ed25519 signatures
- [ ] **TODO**: Cross-verify VRF proofs
- [ ] **TODO**: Test with real Cardano network data

---

## Migration Quality Assessment

### Strengths ✅

1. **Pure Rust VRF**: Eliminates C dependencies, improves safety
2. **Memory safety**: Rust ownership model prevents entire classes of bugs
3. **Documentation**: Added comprehensive error docs to cardano-binary
4. **Testing**: 148 tests, all passing
5. **Build system**: Modern Cargo workspace
6. **CI/CD**: Comprehensive GitHub Actions workflow

### Weaknesses ⚠️

1. **Missing KES**: Key Evolving Signatures not verified
2. **Deprecated deps**: serde_cbor needs migration
3. **Test coverage**: Lower than original, no property tests
4. **No cross-validation**: Haven't verified CBOR/crypto compatibility
5. **Missing crypto-praos**: Entire package not ported
6. **Type safety**: May lack some of Haskell's type-level guarantees

### Risks 🔴

1. **CBOR compatibility**: If formats differ, breaks network compatibility
2. **Crypto compatibility**: If signatures differ, breaks consensus
3. **KES missing**: If needed for Cardano, major functionality gap
4. **Untested against production**: No evidence of Cardano network testing

---

## Recommendations

### Immediate Actions (Critical)

1. **Verify KES status** 🔴
   - Determine if KES is needed
   - Implement if required for Cardano
   - Document if intentionally omitted

2. **CBOR compatibility testing** 🔴
   - Create test suite comparing Haskell ↔ Rust serialization
   - Test with real Cardano data
   - Ensure byte-for-byte compatibility

3. **Crypto cross-validation** 🔴
   - Verify signatures are compatible
   - Test VRF proofs work with Haskell code
   - Validate against Cardano test vectors

### Short-term (High Priority)

4. **Migrate serde_cbor** 🟡
   - Replace with `ciborium`
   - Already planned, needs execution

5. **Add property tests** 🟡
   - Use `proptest` crate
   - Port QuickCheck tests from original

6. **Create golden tests** 🟡
   - Serialize known-good data
   - Verify future changes don't break format

### Long-term (Nice to Have)

7. **Shared test utilities** 🟢
   - Consider creating test-utils crate
   - Share fixtures across packages

8. **Formal verification** 🟢
   - Consider formal methods for crypto code
   - Use Rust's type system more extensively

---

## Verification Checklist

### Functional Completeness

- [ ] All Haskell functions have Rust equivalents
- [ ] KES either ported or documented as unnecessary
- [ ] All hash algorithms available
- [ ] VRF implementations complete

### Correctness

- [ ] CBOR serialization byte-compatible
- [ ] Ed25519 signatures cross-verify
- [ ] VRF proofs cross-verify
- [ ] All test vectors pass

### Security

- [ ] No unsafe code without SAFETY comments (✅ DONE)
- [ ] Memory-locked secrets properly handled (✅ DONE)
- [ ] No panic! in production code (⚠️ TODO: verify)
- [ ] Dependency vulnerabilities scanned (✅ CI configured)

### Quality

- [ ] Test coverage ≥ original
- [ ] Property tests implemented
- [ ] Golden tests for serialization
- [ ] Documentation complete
- [ ] CI/CD comprehensive (✅ DONE)

---

## Next Steps

1. **Investigate cardano-crypto-praos**
   - Review what functionality it provides
   - Determine if KES is critical
   - Make go/no-go decision on porting

2. **Set up cross-validation tests**
   - Create Haskell ↔ Rust test harness
   - Test CBOR serialization compatibility
   - Test cryptographic operations compatibility

3. **Engage with Cardano team**
   - Get feedback on this audit
   - Clarify requirements for production use
   - Identify any missing critical features

4. **Complete serde_cbor migration**
   - Already planned
   - Critical for long-term maintenance

---

## Conclusion

**Overall Assessment**: 🟡 **Good progress with critical gaps**

The Rust implementation is **high quality** with significant improvements over the original (pure Rust, better memory safety, good documentation). However, there are **critical unknowns**:

1. ❓ **KES status** - May be mission-critical for Cardano
2. ❓ **CBOR compatibility** - Must be byte-perfect
3. ❓ **Crypto compatibility** - Must work with Cardano network

**Recommendation**: Before production use, must complete cross-validation testing and verify KES requirements.

**Estimated completion for production-ready**:

- High priority items: 2-4 weeks
- Full compatibility testing: 4-6 weeks
- Formal security audit: 6-8 weeks

---

**Audit Status**: 🔍 Phase 1 Complete - Needs practical testing
**Next Phase**: Cross-validation testing and KES investigation
**Updated**: October 3, 2025
