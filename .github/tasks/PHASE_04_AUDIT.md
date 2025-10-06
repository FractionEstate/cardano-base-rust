# Phase 04 - DSIGN Implementation Audit

**Date:** October 6, 2025
**Status:** In Progress
**Auditor:** @FractionEstate

---

## Executive Summary

This document audits the current state of DSIGN (Digital Signature) implementations in `cardano-crypto-class` against the Haskell reference implementation from `cardano-base`. The goal is to identify gaps, validate existing implementations, and plan parity work.

---

## Current Implementation Status

### Implemented Algorithms

| Algorithm | Rust Module | Status | Notes |
|-----------|-------------|--------|-------|
| Ed25519 | `dsign/ed25519.rs` | ✅ Implemented | Uses `ed25519-dalek` |
| Ed25519 MLocked | `dsign/ed25519_mlocked.rs` | ✅ Implemented | Secure memory variant |
| ECDSA Secp256k1 | `dsign/ecdsa_secp256k1.rs` | ✅ Implemented | Uses `k256`/`ecdsa` crates |
| Schnorr Secp256k1 | `dsign/schnorr_secp256k1.rs` | ✅ Implemented | Uses `k256` |

### Missing Implementations

| Algorithm | Haskell Module | Priority | Notes |
|-----------|----------------|----------|-------|
| Ed25519 Extended | N/A in Haskell base | Medium | BIP32-HD derivation needed for stake keys |
| Mock DSIGN | `DSIGN/Mock.hs` | Low | Test-only implementation |
| Ed448 | `DSIGN/Ed448.hs` | Low | Not used in current Cardano |
| NeverUsed | `DSIGN/NeverUsed.hs` | N/A | Placeholder type |

---

## Detailed Algorithm Analysis

### 1. Ed25519 Standard

**Haskell Reference:** `Cardano/Crypto/DSIGN/Ed25519.hs`
**Rust Implementation:** `cardano-crypto-class/src/dsign/ed25519.rs`

#### Current State
```rust
// Key Sizes
pub const SEED_BYTES: usize = 32;
pub const VERIFICATION_KEY_BYTES: usize = 32;
pub const SIGNATURE_BYTES: usize = 64;
pub const SECRET_COMPOUND_BYTES: usize = 64;

// Uses ed25519-dalek 2.x
- SigningKey (seed expansion)
- VerifyingKey (public key)
- Signature (64-byte signatures)
```

#### Implementation Details
- ✅ RFC 8032 compliant via `ed25519-dalek`
- ✅ Deterministic signatures
- ✅ Compound secret key (32-byte seed + 32-byte public key)
- ✅ CBOR serialization support
- ✅ Direct serialization for FFI
- ✅ Pinned memory for keys

#### Test Coverage
- ✅ CBOR serialization tests (`tests/cbor_serialization.rs`)
- ✅ Cross-compatibility tests (`tests/cross_compat.rs`)
- ✅ KES integration tests (`tests/kes_gen_key_from_seed.rs`)
- ⚠️ Missing: Specific Ed25519 test vector validation
- ⚠️ Missing: RFC 8032 test vectors

#### Gaps Identified
1. **Test Vectors:** No dedicated Ed25519 test vector file
2. **RFC 8032 Compliance:** Not explicitly tested against RFC vectors
3. **Batch Verification:** Not implemented (low priority)
4. **Error Cases:** Limited testing of invalid signatures

---

### 2. Ed25519 MLocked

**Haskell Reference:** `Cardano/Crypto/DSIGN/Ed25519ML.hs`
**Rust Implementation:** `cardano-crypto-class/src/dsign/ed25519_mlocked.rs`

#### Current State
- Secure memory variant using `MLockedSeed`
- Prevents key material from being swapped to disk
- Uses same cryptographic primitives as standard Ed25519

#### Implementation Details
- ✅ MLocked memory for seed storage
- ✅ Zeroization on drop
- ✅ Same signing/verification logic as standard Ed25519
- ✅ CBOR serialization (verification keys only)

#### Test Coverage
- ✅ Basic functionality tests
- ✅ MLocked memory tests
- ⚠️ Missing: Performance comparison with standard Ed25519

#### Gaps Identified
1. **Documentation:** Limited usage examples
2. **Performance:** No benchmarks comparing mlocked vs standard
3. **Security Audit:** Needs formal review of memory handling

---

### 3. ECDSA Secp256k1

**Haskell Reference:** `Cardano/Crypto/DSIGN/EcdsaSecp256k1.hs`
**Rust Implementation:** `cardano-crypto-class/src/dsign/ecdsa_secp256k1.rs`

#### Current State
```rust
// Uses k256 and ecdsa crates from RustCrypto
- SigningKey (secp256k1 scalar)
- VerifyingKey (secp256k1 point)
- Signature (r, s components, 64 bytes)
```

#### Implementation Details
- ✅ RFC 6979 deterministic k generation
- ✅ Low-s normalization
- ✅ Compressed point serialization
- ⚠️ DER encoding support unclear

#### Test Coverage
- ✅ Basic sign/verify tests
- ⚠️ Missing: Bitcoin/Ethereum cross-compatibility vectors
- ⚠️ Missing: Edge case testing (invalid points, etc.)

#### Gaps Identified
1. **Test Vectors:** No ECDSA-specific test vector file
2. **DER Encoding:** Unclear if DER encoding/decoding is supported
3. **Public Key Recovery:** Not implemented (may not be needed)
4. **Cross-Compatibility:** No tests against Bitcoin/Ethereum implementations

---

### 4. Schnorr Secp256k1

**Haskell Reference:** `Cardano/Crypto/DSIGN/SchnorrSecp256k1.hs`
**Rust Implementation:** `cardano-crypto-class/src/dsign/schnorr_secp256k1.rs`

#### Current State
```rust
// Uses k256 crate with Schnorr support
- Similar key structure to ECDSA
- 64-byte signatures (R point + s scalar)
```

#### Implementation Details
- ✅ Schnorr signature scheme over secp256k1
- ⚠️ BIP 340 compliance unclear
- ⚠️ Batch verification support unclear

#### Test Coverage
- ✅ Basic sign/verify tests
- ⚠️ Missing: BIP 340 test vectors
- ⚠️ Missing: Batch verification tests

#### Gaps Identified
1. **BIP 340 Compliance:** Not explicitly tested
2. **Test Vectors:** No Schnorr-specific test vector file
3. **Batch Verification:** Implementation status unclear
4. **MuSig Support:** Not in scope but worth noting

---

## Cross-Cutting Concerns

### CBOR Serialization
- ✅ Implemented for verification keys and signatures
- ✅ Tests validate CBOR structure
- ✅ Matches Haskell encoding format
- ⚠️ Need more comprehensive cross-compatibility tests

### Direct Serialization (FFI)
- ✅ Implemented for all types
- ✅ Used for C FFI boundaries
- ⚠️ Limited testing of FFI serialization

### Error Handling
- ✅ `DsignError` type with variants
- ✅ Proper error propagation
- ⚠️ Some errors use String instead of typed variants

### Security Considerations
- ✅ Pinned memory for key material
- ✅ Zeroization on drop (via PinnedSizedBytes)
- ⚠️ Need formal security audit
- ⚠️ Side-channel resistance not explicitly documented

---

## Test Vector Status

### Available Test Vectors

| Algorithm | File | Status | Vectors Count |
|-----------|------|--------|---------------|
| Ed25519 | N/A | ❌ Missing | 0 |
| ECDSA | N/A | ❌ Missing | 0 |
| Schnorr | N/A | ❌ Missing | 0 |
| Mock VRF | `mock_vrf_vectors.json` | ✅ Exists | Multiple |
| Praos VRF | `praos_vrf_vectors.json` | ✅ Exists | Multiple |

### Needed Test Vectors

1. **Ed25519:**
   - RFC 8032 test vectors (official)
   - Cardano-specific test vectors from Haskell
   - Edge cases: empty messages, max-length messages
   - Invalid signature tests

2. **ECDSA Secp256k1:**
   - RFC 6979 deterministic k vectors
   - Bitcoin test vectors (where applicable)
   - Invalid signature tests
   - Point validation tests

3. **Schnorr Secp256k1:**
   - BIP 340 test vectors (if applicable)
   - Cardano-specific vectors from Haskell
   - Batch verification vectors
   - Invalid signature tests

---

## Haskell Parity Checklist

### Core Functionality

- [x] Ed25519 key generation
- [x] Ed25519 signing
- [x] Ed25519 verification
- [x] Ed25519 MLocked variant
- [x] ECDSA Secp256k1 key generation
- [x] ECDSA Secp256k1 signing
- [x] ECDSA Secp256k1 verification
- [x] Schnorr Secp256k1 key generation
- [x] Schnorr Secp256k1 signing
- [x] Schnorr Secp256k1 verification

### Serialization

- [x] CBOR serialization for all types
- [x] Direct serialization for all types
- [ ] Validate CBOR matches Haskell byte-for-byte
- [ ] FFI serialization tests

### Test Coverage

- [ ] RFC 8032 Ed25519 test vectors
- [ ] Cardano Ed25519 test vectors
- [ ] ECDSA test vectors
- [ ] Schnorr test vectors
- [ ] Cross-compatibility tests with Haskell outputs
- [ ] Error case testing

### Documentation

- [x] API documentation
- [ ] Usage examples for each algorithm
- [ ] Security considerations documented
- [ ] Performance characteristics documented

---

## Priority Ranking

### High Priority (P0)
1. **Ed25519 Test Vectors** - Add RFC 8032 and Cardano-specific vectors
2. **CBOR Cross-Compatibility** - Validate byte-for-byte matching with Haskell
3. **Error Case Testing** - Test invalid signatures, malformed keys

### Medium Priority (P1)
4. **ECDSA Test Vectors** - Add comprehensive test coverage
5. **Schnorr Test Vectors** - Add BIP 340 and Cardano vectors
6. **Documentation** - Add usage examples and guides

### Low Priority (P2)
7. **Performance Benchmarks** - Compare with Haskell/C implementations
8. **Batch Verification** - Implement for Ed25519 and Schnorr
9. **Mock DSIGN** - Implement test-only variant

---

## Recommended Action Plan

### Phase 1: Test Vector Collection (1-2 days)
1. Extract Ed25519 test vectors from Haskell tests
2. Create JSON test vector files matching VRF format
3. Add RFC 8032 official vectors
4. Create test harness for loading and running vectors

### Phase 2: Ed25519 Validation (2-3 days)
1. Run all test vectors against current implementation
2. Fix any discrepancies found
3. Add error case tests
4. Document any Haskell differences

### Phase 3: ECDSA/Schnorr Validation (2-3 days)
1. Collect ECDSA and Schnorr test vectors
2. Run vectors against implementations
3. Validate BIP 340 compliance for Schnorr
4. Fix any issues found

### Phase 4: Cross-Compatibility (1-2 days)
1. Generate test outputs from Haskell
2. Compare Rust outputs byte-for-byte
3. Fix CBOR serialization discrepancies
4. Document compatibility guarantees

### Phase 5: Documentation & Benchmarks (1-2 days)
1. Add usage examples
2. Document security considerations
3. Add performance benchmarks
4. Update phase tracking documents

**Total Estimated Effort:** 7-12 days (1.5-2.5 weeks)

---

## Dependencies

### Rust Crates
- `ed25519-dalek` 2.x - Ed25519 implementation
- `k256` - secp256k1 curve operations
- `ecdsa` - ECDSA signatures
- `schnorr` or similar - Schnorr signatures

### Haskell Reference
- `cardano-crypto-class` - Reference implementations
- `cardano-crypto-tests` - Test vectors

### Tools
- `hex` - Hex encoding/decoding for test vectors
- `ciborium` - CBOR serialization
- `criterion` - Performance benchmarking (optional)

---

## Risk Assessment

### Technical Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Test vector mismatch | High | Medium | Careful validation, byte-by-byte comparison |
| CBOR encoding differences | Medium | Low | Existing tests should catch this |
| Performance regression | Low | Low | Benchmark before/after changes |
| Security vulnerability | High | Very Low | Using well-audited crates |

### Schedule Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Missing test vectors | Medium | Medium | Extract from Haskell if needed |
| Complex BIP32 derivation | Medium | Low | Out of scope for Phase 04 |
| Haskell reference unclear | Low | Medium | Ask Cardano team for clarification |

---

## Success Criteria

Phase 04 is complete when:

1. ✅ All test vectors pass with exact matches
2. ✅ CBOR serialization matches Haskell byte-for-byte
3. ✅ Error cases properly tested and handled
4. ✅ Documentation complete with examples
5. ✅ Performance benchmarks documented
6. ✅ All checklist items in phase-04-dsign-parity.md marked complete

---

## Appendix: File Locations

### Rust Implementation
```
cardano-crypto-class/src/dsign/
├── mod.rs                    # Core trait definitions
├── ed25519.rs               # Ed25519 implementation
├── ed25519_mlocked.rs       # MLocked Ed25519
├── ecdsa_secp256k1.rs       # ECDSA implementation
└── schnorr_secp256k1.rs     # Schnorr implementation
```

### Haskell Reference
```
reference-cardano-base/cardano-crypto-class/src/Cardano/Crypto/DSIGN/
├── Class.hs                 # Core typeclass
├── Ed25519.hs              # Ed25519 implementation
├── Ed25519ML.hs            # MLocked variant
├── EcdsaSecp256k1.hs       # ECDSA implementation
├── SchnorrSecp256k1.hs     # Schnorr implementation
└── Mock.hs                 # Test implementation
```

### Tests
```
cardano-test-vectors/
└── test_vectors/            # All test vector files
    ├── ed25519_test_vectors.json
    ├── ecdsa_secp256k1_test_vectors.json
    ├── schnorr_secp256k1_test_vectors.json
    ├── vrf_ver03_generated_*
    └── vrf_ver13_generated_*

cardano-crypto-class/tests/
├── cross_compat.rs          # Cross-compatibility tests
├── cbor_serialization.rs    # CBOR tests
└── direct_serialise_impls.rs # FFI serialization
```

---

**Audit Complete:** October 6, 2025
**Next Step:** Begin test vector collection and validation
