# Missing Features and Action Items

This document tracks the features identified in the audit that are missing or incomplete in the Rust implementation compared to the official Haskell cardano-base.

## Priority Classification

- **P0 (Critical)**: Required for production blockchain operations
- **P1 (High)**: Important for completeness and some use cases  
- **P2 (Medium)**: Nice to have, improves feature parity
- **P3 (Low)**: Optional, mainly for testing or edge cases

---

## P0: Critical Missing Features

### 1. VRF Libsodium Compatibility Validation

**Status:** ⚠️ NEEDS VALIDATION  
**Issue:** The Rust implementation uses pure Rust curve25519-dalek instead of libsodium like Haskell  
**Impact:** Could cause consensus failures if outputs differ  

**Action Items:**
- [ ] Extract comprehensive test vectors from Haskell cardano-crypto-praos
- [ ] Generate known-good VRF outputs using Haskell implementation
- [ ] Compare Rust outputs byte-for-byte with Haskell
- [ ] Document any intentional differences
- [ ] Add continuous validation tests

**Files to modify:**
- `cardano-vrf-pure/tests/haskell_vrf_cross_validation.rs` - Add comprehensive test vectors
- `cardano-crypto-class/tests/cross_compat.rs` - Add VRF validation

**Estimate:** 3-5 days

---

## P1: High Priority Missing Features

### 2. BLS12-381 Elliptic Curve Support

**Status:** ❌ NOT IMPLEMENTED  
**Haskell Modules:** 
- `Cardano/Crypto/EllipticCurve/BLS12_381.hs`
- `Cardano/Crypto/EllipticCurve/BLS12_381/Internal.hs`

**Impact:** Required for pairing-based cryptography, may be needed for future Cardano features

**Investigation needed:**
- [ ] Determine if BLS12-381 is actively used in current Cardano
- [ ] Check if planned for future protocol upgrades
- [ ] Review Conway era requirements

**If needed, implementation plan:**
- [ ] Choose Rust crate: blst (fast, C-based) vs arkworks (pure Rust)
- [ ] Create `cardano-crypto-class/src/elliptic_curve/` module
- [ ] Implement `bls12_381.rs` with G1, G2, GT operations
- [ ] Add pairing operations
- [ ] Create comprehensive test suite
- [ ] Add cross-validation with Haskell

**Estimate:** 2-3 weeks (if required)

### 3. cardano-crypto-praos Package

**Status:** ❌ NOT PORTED  
**Haskell Modules:**
- `Cardano/Crypto/VRF/Praos.hs`
- `Cardano/Crypto/VRF/PraosBatchCompat.hs`
- `Cardano/Crypto/RandomBytes.hs`

**Impact:** Official Praos VRF implementation with libsodium bindings

**Action Items:**
- [ ] Analyze differences between crypto-praos and current Rust implementation
- [ ] Port RandomBytes functionality or document Rust equivalent
- [ ] Ensure Praos VRF compatibility
- [ ] Create new `cardano-crypto-praos` crate if significant differences found

**Estimate:** 1-2 weeks

### 4. Ed448 Digital Signature Algorithm

**Status:** ❌ NOT IMPLEMENTED  
**Haskell Module:** `Cardano/Crypto/DSIGN/Ed448.hs`

**Impact:** May be required for some protocol features

**Action Items:**
- [ ] Research Ed448 usage in Cardano ecosystem
- [ ] Check if used in any current or planned features
- [ ] If needed, implement using `ed448-goldilocks` crate
- [ ] Create `cardano-crypto-class/src/dsign/ed448.rs`
- [ ] Add test vectors from RFC 8032
- [ ] Cross-validate with Haskell implementation

**Estimate:** 1 week (if required)

### 5. Simple KES Algorithm

**Status:** ❌ NOT IMPLEMENTED  
**Haskell Module:** `Cardano/Crypto/KES/Simple.hs`

**Impact:** May be used in some KES configurations

**Action Items:**
- [ ] Review Haskell Simple KES implementation
- [ ] Determine if used in mainnet or testnets
- [ ] Implement `cardano-crypto-class/src/kes/simple.rs`
- [ ] Add comprehensive tests
- [ ] Validate against Haskell with test vectors

**Estimate:** 1 week

---

## P2: Medium Priority Missing Features

### 6. Short Hash Algorithm

**Status:** ❌ NOT IMPLEMENTED  
**Haskell Module:** `Cardano/Crypto/Hash/Short.hs`

**Impact:** Used for compact identifiers

**Action Items:**
- [ ] Review Haskell Short hash implementation
- [ ] Understand use cases in Cardano
- [ ] Implement in `cardano-crypto-class/src/hash.rs`
- [ ] Add test vectors
- [ ] Document when to use Short vs full hash

**Estimate:** 2-3 days

### 7. Libsodium Initialization

**Status:** ⚠️ PARTIAL (pure Rust approach)  
**Haskell Modules:**
- `Cardano/Crypto/Libsodium/Init.hs`
- `Cardano/Crypto/Libsodium/Memory.hs`

**Impact:** Library initialization and memory utilities

**Action Items:**
- [ ] Document that Rust implementation avoids libsodium dependency
- [ ] Verify all libsodium functionality has Rust equivalent
- [ ] Add initialization code if needed for compatibility
- [ ] Document design decision in architecture docs

**Estimate:** 3-5 days

### 8. Complete Strict Containers

**Status:** ⚠️ PARTIAL  
**Missing:** Complete Set/Map implementations with all operations

**Impact:** Performance in lazy evaluation scenarios

**Action Items:**
- [ ] Review Haskell cardano-strict-containers package
- [ ] Implement missing Set operations
- [ ] Implement missing Map operations  
- [ ] Add conversion utilities to/from standard library
- [ ] Add comprehensive tests
- [ ] Document performance characteristics

**Estimate:** 1 week

### 9. cardano-crypto-tests Equivalent

**Status:** ❌ NOT PORTED  
**Haskell Package:** `cardano-crypto-tests`

**Impact:** Comprehensive test vector suite

**Action Items:**
- [ ] Review Haskell cardano-crypto-tests package structure
- [ ] Extract all test vectors
- [ ] Create `cardano-crypto-tests` crate or `test_vectors` directory
- [ ] Organize test vectors by algorithm
- [ ] Add automated test generation from Haskell
- [ ] Document test vector format

**Estimate:** 2 weeks

---

## P3: Low Priority Missing Features

### 10. Mock and NeverUsed Types

**Status:** ❌ NOT IMPLEMENTED  
**Haskell Modules:**
- `DSIGN/Mock.hs`
- `DSIGN/NeverUsed.hs`
- `KES/Mock.hs`
- `KES/NeverUsed.hs`
- `VRF/NeverUsed.hs`
- `Hash/NeverUsed.hs`

**Impact:** Testing utilities and placeholder types

**Action Items:**
- [ ] Determine if Mock types are needed for testing
- [ ] Implement if required for test compatibility
- [ ] NeverUsed types can likely remain unimplemented

**Estimate:** 2-3 days

### 11. FFI Utilities

**Status:** ❌ NOT IMPLEMENTED  
**Haskell Modules:**
- `Cardano/Foreign.hs`
- `Cardano/Crypto/Libsodium/C.hs`
- `Cardano/Crypto/SECP256K1/C.hs`

**Impact:** Foreign function interface helpers

**Note:** Rust uses native crates (secp256k1, etc.), so direct FFI may not be needed

**Action Items:**
- [ ] Document that Rust uses native crate ecosystem
- [ ] Verify no FFI compatibility issues
- [ ] Add FFI utilities only if needed for specific integrations

**Estimate:** N/A (may not be needed)

---

## Additional Quality Improvements

### 12. Enhanced Documentation

**Status:** 🔄 IN PROGRESS

**Action Items:**
- [ ] Add per-algorithm accuracy guarantees to docs
- [ ] Create compatibility matrix with Haskell versions
- [ ] Document design decisions (pure Rust vs libsodium)
- [ ] Add migration guide from Haskell
- [ ] Document which Haskell features are intentionally omitted
- [ ] Add troubleshooting guide

**Files:**
- `docs/cryptography.md` - Add accuracy guarantees
- `docs/compatibility.md` - NEW: Compatibility matrix
- `docs/migration.md` - NEW: Migration guide
- `docs/design-decisions.md` - NEW: Architectural decisions

**Estimate:** 1 week

### 13. Performance Benchmarking

**Status:** ❌ NOT IMPLEMENTED

**Action Items:**
- [ ] Create benchmark suite in `benches/`
- [ ] Compare Rust performance vs Haskell
- [ ] Identify performance bottlenecks
- [ ] Document performance characteristics
- [ ] Add CI performance regression tests

**Estimate:** 1 week

### 14. Integration Testing

**Status:** ❌ NOT IMPLEMENTED

**Action Items:**
- [ ] Test with actual cardano-node components (if available)
- [ ] Verify blockchain consensus compatibility
- [ ] Test with real mainnet data
- [ ] Add end-to-end test scenarios

**Estimate:** 2-3 weeks (requires Cardano infrastructure)

---

## Summary of Estimates

| Priority | Tasks | Estimated Time |
|----------|-------|----------------|
| P0 | 1 | 3-5 days |
| P1 | 5 | 5-9 weeks (if all needed) |
| P2 | 4 | 4-5 weeks |
| P3 | 2 | 2-3 days |
| Quality | 3 | 4 weeks |
| **Total** | **15** | **12-19 weeks** |

**Note:** Not all tasks may be necessary. Priority should be based on:
1. Actual Cardano usage requirements
2. Production deployment needs
3. Community feedback
4. Protocol roadmap

---

## Recommended Next Steps

### Immediate (This Week)
1. ✅ Complete initial audit
2. 🔄 Validate VRF implementation with test vectors
3. 🔄 Investigate BLS12-381 requirements

### Short-term (Next Month)
1. Implement missing features based on priority
2. Enhanced documentation
3. Complete strict containers

### Medium-term (2-3 Months)
1. cardano-crypto-tests equivalent
2. Performance benchmarking
3. Integration testing

### Long-term (3+ Months)
1. BLS12-381 if required
2. Advanced features as needed
3. Continuous maintenance and updates

---

**Last Updated:** October 2025  
**Maintained by:** Development Team  
**Review Frequency:** Monthly
