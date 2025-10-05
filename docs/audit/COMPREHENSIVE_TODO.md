# Comprehensive Gap Closure Todo List

**Date:** October 4, 2025
**Total Estimated Effort:** 11-16 days
**Status:** Not Started

This document provides an exhaustive, actionable todo list for closing all gaps between the Rust and Haskell cardano-base implementations.

---

## 🔴 Phase 1: CBOR Serialization (CRITICAL) - Days 1-4

### 1.1 KES CBOR Serialization - SingleKes

- [ ] **1.1.1** Add `Serialize` impl for `SingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Pattern: Serialize as CBOR bytes wrapping `raw_serialize_verification_key_kes`
  - Time: 30 min

- [ ] **1.1.2** Add `Deserialize` impl for `SingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Pattern: Deserialize bytes, call `raw_deserialize_verification_key_kes`
  - Time: 30 min

- [ ] **1.1.3** Add `Serialize` impl for `SingleKesSignature`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Pattern: Same as verification key
  - Time: 30 min

- [ ] **1.1.4** Add `Deserialize` impl for `SingleKesSignature`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Time: 30 min

- [ ] **1.1.5** Define `UnsoundPureSingleKesSigningKey` type
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Wrapper around DSIGN signing key (non-mlocked)
  - Time: 1 hour

- [ ] **1.1.6** Add `Serialize` impl for `UnsoundPureSingleKesSigningKey`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - WARNING comment: Only for testing, violates mlocking
  - Time: 30 min

- [ ] **1.1.7** Add `Deserialize` impl for `UnsoundPureSingleKesSigningKey`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Time: 30 min

- [ ] **1.1.8** Add roundtrip test for SingleKes verification key CBOR
  - File: `cardano-crypto-class/tests/kes_cbor.rs` (new)
  - Test: Generate key, serialize to CBOR, deserialize, compare
  - Time: 30 min

- [ ] **1.1.9** Add roundtrip test for SingleKes signature CBOR
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

- [ ] **1.1.10** Add roundtrip test for UnsoundPure signing key CBOR
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

**Subtotal: 5.5 hours (0.7 days)**

---

### 1.2 KES CBOR Serialization - SumKes (Sum1Kes through Sum7Kes)

- [ ] **1.2.1** Add `Serialize` impl for `SumKesVerificationKey<D, H>`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Generic over child algorithm D and hash H
  - Time: 1 hour

- [ ] **1.2.2** Add `Deserialize` impl for `SumKesVerificationKey<D, H>`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Time: 1 hour

- [ ] **1.2.3** Add `Serialize` impl for `SumKesSignature<D, H>`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Time: 1 hour

- [ ] **1.2.4** Add `Deserialize` impl for `SumKesSignature<D, H>`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Time: 1 hour

- [ ] **1.2.5** Define `UnsoundPureSumKesSigningKey<D, H>` struct
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Fields: child SK, seed, VK0, VK1 (matching Haskell)
  - Time: 1.5 hours

- [ ] **1.2.6** Add `Serialize` impl for `UnsoundPureSumKesSigningKey<D, H>`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Time: 1 hour

- [ ] **1.2.7** Add `Deserialize` impl for `UnsoundPureSumKesSigningKey<D, H>`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Time: 1 hour

- [ ] **1.2.8** Add CBOR roundtrip tests for Sum1Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Test VK, Sig, UnsoundPure SK
  - Time: 1 hour

- [ ] **1.2.9** Add CBOR roundtrip tests for Sum2Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

- [ ] **1.2.10** Add CBOR roundtrip tests for Sum3Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

- [ ] **1.2.11** Add CBOR roundtrip tests for Sum4Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

- [ ] **1.2.12** Add CBOR roundtrip tests for Sum5Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

- [ ] **1.2.13** Add CBOR roundtrip tests for Sum6Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

- [ ] **1.2.14** Add CBOR roundtrip tests for Sum7Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 30 min

**Subtotal: 11.5 hours (1.4 days)**

---

### 1.3 KES CBOR Serialization - CompactSingleKes

- [ ] **1.3.1** Add `Serialize` impl for `CompactSingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 30 min

- [ ] **1.3.2** Add `Deserialize` impl for `CompactSingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 30 min

- [ ] **1.3.3** Add `Serialize` impl for `CompactSingleKesSignature`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 30 min

- [ ] **1.3.4** Add `Deserialize` impl for `CompactSingleKesSignature`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 30 min

- [ ] **1.3.5** Define `UnsoundPureCompactSingleKesSigningKey`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 1 hour

- [ ] **1.3.6** Add `Serialize` impl for `UnsoundPureCompactSingleKesSigningKey`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 30 min

- [ ] **1.3.7** Add `Deserialize` impl for `UnsoundPureCompactSingleKesSigningKey`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 30 min

- [ ] **1.3.8** Add CBOR roundtrip tests for CompactSingleKes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 1 hour

**Subtotal: 5 hours (0.6 days)**

---

### 1.4 KES CBOR Serialization - CompactSumKes

- [ ] **1.4.1** Add `Serialize` impl for `CompactSumKesVerificationKey<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 1 hour

- [ ] **1.4.2** Add `Deserialize` impl for `CompactSumKesVerificationKey<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 1 hour

- [ ] **1.4.3** Add `Serialize` impl for `CompactSumKesSignature<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 1 hour

- [ ] **1.4.4** Add `Deserialize` impl for `CompactSumKesSignature<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 1 hour

- [ ] **1.4.5** Define `UnsoundPureCompactSumKesSigningKey<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 1.5 hours

- [ ] **1.4.6** Add `Serialize` impl for `UnsoundPureCompactSumKesSigningKey<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 1 hour

- [ ] **1.4.7** Add `Deserialize` impl for `UnsoundPureCompactSumKesSigningKey<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 1 hour

- [ ] **1.4.8** Add CBOR roundtrip tests for CompactSum1Kes through CompactSum7Kes
  - File: `cardano-crypto-class/tests/kes_cbor.rs`
  - Time: 2 hours

**Subtotal: 9.5 hours (1.2 days)**

---

### 1.5 VRF CBOR Serialization

- [ ] **1.5.1** Add `Serialize` impl for `PraosVerificationKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 30 min

- [ ] **1.5.2** Add `Deserialize` impl for `PraosVerificationKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 30 min

- [ ] **1.5.3** Add `Serialize` impl for `PraosSigningKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 30 min

- [ ] **1.5.4** Add `Deserialize` impl for `PraosSigningKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 30 min

- [ ] **1.5.5** Add `Serialize` impl for `PraosProof`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 30 min

- [ ] **1.5.6** Add `Deserialize` impl for `PraosProof`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 30 min

- [ ] **1.5.7** Add `Serialize` impl for `PraosBatchCompatVerificationKey`
  - File: `cardano-crypto-class/src/vrf/praos_batch.rs`
  - Time: 30 min

- [ ] **1.5.8** Add `Deserialize` impl for `PraosBatchCompatVerificationKey`
  - File: `cardano-crypto-class/src/vrf/praos_batch.rs`
  - Time: 30 min

- [ ] **1.5.9** Add `Serialize` impl for `PraosBatchCompatSigningKey`
  - File: `cardano-crypto-class/src/vrf/praos_batch.rs`
  - Time: 30 min

- [ ] **1.5.10** Add `Deserialize` impl for `PraosBatchCompatSigningKey`
  - File: `cardano-crypto-class/src/vrf/praos_batch.rs`
  - Time: 30 min

- [ ] **1.5.11** Add `Serialize` impl for `PraosBatchCompatProof`
  - File: `cardano-crypto-class/src/vrf/praos_batch.rs`
  - Time: 30 min

- [ ] **1.5.12** Add `Deserialize` impl for `PraosBatchCompatProof`
  - File: `cardano-crypto-class/src/vrf/praos_batch.rs`
  - Time: 30 min

- [ ] **1.5.13** Add CBOR roundtrip tests for all Praos VRF types
  - File: `cardano-crypto-class/tests/vrf_cbor.rs` (new)
  - Time: 2 hours

- [ ] **1.5.14** Add CBOR roundtrip tests for all PraosBatchCompat types
  - File: `cardano-crypto-class/tests/vrf_cbor.rs`
  - Time: 1 hour

- [ ] **1.5.15** Add `Serialize`/`Deserialize` for SimpleVRF types
  - File: `cardano-crypto-class/src/vrf/simple.rs`
  - Time: 1.5 hours

- [ ] **1.5.16** Add `Serialize`/`Deserialize` for MockVRF types
  - File: `cardano-crypto-class/src/vrf/mock.rs`
  - Time: 1.5 hours

- [ ] **1.5.17** Add CBOR tests for SimpleVRF and MockVRF
  - File: `cardano-crypto-class/tests/vrf_cbor.rs`
  - Time: 1 hour

**Subtotal: 12 hours (1.5 days)**

---

### 1.6 DSIGN CBOR Serialization

- [ ] **1.6.1** Add `Serialize` impl for `Ed25519VerificationKey`
  - File: `cardano-crypto-class/src/dsign/ed25519.rs`
  - Time: 30 min

- [ ] **1.6.2** Add `Deserialize` impl for `Ed25519VerificationKey`
  - File: `cardano-crypto-class/src/dsign/ed25519.rs`
  - Time: 30 min

- [ ] **1.6.3** Add `Serialize` impl for `Ed25519SigningKey` (non-mlocked)
  - File: `cardano-crypto-class/src/dsign/ed25519.rs`
  - Time: 30 min

- [ ] **1.6.4** Add `Deserialize` impl for `Ed25519SigningKey` (non-mlocked)
  - File: `cardano-crypto-class/src/dsign/ed25519.rs`
  - Time: 30 min

- [ ] **1.6.5** Add `Serialize` impl for `Ed25519Signature`
  - File: `cardano-crypto-class/src/dsign/ed25519.rs`
  - Time: 30 min

- [ ] **1.6.6** Add `Deserialize` impl for `Ed25519Signature`
  - File: `cardano-crypto-class/src/dsign/ed25519.rs`
  - Time: 30 min

- [ ] **1.6.7** Add `Serialize` impl for `Ed25519MLockedVerificationKey`
  - File: `cardano-crypto-class/src/dsign/ed25519_mlocked.rs`
  - Time: 30 min

- [ ] **1.6.8** Add `Deserialize` impl for `Ed25519MLockedVerificationKey`
  - File: `cardano-crypto-class/src/dsign/ed25519_mlocked.rs`
  - Time: 30 min

- [ ] **1.6.9** Add `Serialize` impl for `Ed25519MLockedSignature`
  - File: `cardano-crypto-class/src/dsign/ed25519_mlocked.rs`
  - Time: 30 min

- [ ] **1.6.10** Add `Deserialize` impl for `Ed25519MLockedSignature`
  - File: `cardano-crypto-class/src/dsign/ed25519_mlocked.rs`
  - Time: 30 min

- [ ] **1.6.11** Add CBOR roundtrip tests for Ed25519
  - File: `cardano-crypto-class/tests/dsign_cbor.rs` (new)
  - Time: 1 hour

- [ ] **1.6.12** Add CBOR roundtrip tests for Ed25519MLocked
  - File: `cardano-crypto-class/tests/dsign_cbor.rs`
  - Time: 1 hour

**Subtotal: 7 hours (0.9 days)**

---

**Phase 1 Total: 50.5 hours (6.3 days)**

---

## ⚠️ Phase 2: Comprehensive Testing (HIGH PRIORITY) - Days 5-12

### 2.1 Basic KES Tests - Positive Tests

- [ ] **2.1.1** Create test file structure
  - File: `cardano-crypto-class/tests/kes_basic.rs` (new)
  - Setup test helpers and common fixtures
  - Time: 1 hour

- [ ] **2.1.2** Test: Sign and verify at period 0 (SingleKes)
  - Generate key, sign message at period 0, verify
  - Time: 30 min

- [ ] **2.1.3** Test: Sign and verify at period 0 (Sum1Kes)
  - Time: 30 min

- [ ] **2.1.4** Test: Sign and verify at period 0 (Sum7Kes)
  - Time: 30 min

- [ ] **2.1.5** Test: Key evolution through multiple periods (Sum7Kes)
  - Update key from period 0→1→2→3, verify each
  - Time: 1 hour

- [ ] **2.1.6** Test: Verification key stays constant after updates
  - Update signing key multiple times, verify VK unchanged
  - Time: 1 hour

- [ ] **2.1.7** Test: Sign at different periods (Sum7Kes)
  - Sign at periods 0, 1, 63, 127, verify all
  - Time: 1 hour

- [ ] **2.1.8** Test: CompactSingleKes basic sign/verify
  - Time: 30 min

- [ ] **2.1.9** Test: CompactSum7Kes evolution and sign
  - Time: 1 hour

- [ ] **2.1.10** Test: All KES variants at max period
  - Test signing at maximum allowed period for each variant
  - Time: 1.5 hours

**Subtotal: 8.5 hours (1.1 days)**

---

### 2.2 Basic KES Tests - Negative Tests

- [ ] **2.2.1** Test: Verify with wrong verification key
  - Sign with key A, verify with key B, should fail
  - Test all KES variants
  - Time: 1 hour

- [ ] **2.2.2** Test: Verify with wrong message
  - Sign message A, verify with message B, should fail
  - Test all KES variants
  - Time: 1 hour

- [ ] **2.2.3** Test: Verify with wrong period
  - Sign at period 0, verify at period 1, should fail
  - Test all KES variants
  - Time: 1 hour

- [ ] **2.2.4** Test: Sign at invalid period
  - Try to sign at period > max, should error
  - Test all KES variants
  - Time: 1 hour

- [ ] **2.2.5** Test: Update beyond max period
  - Try to update at period >= max, should fail
  - Test all KES variants
  - Time: 1 hour

- [ ] **2.2.6** Test: Corrupted signature verification
  - Corrupt signature bytes, verify should fail
  - Time: 1 hour

- [ ] **2.2.7** Test: Corrupted verification key
  - Corrupt VK bytes, operations should fail
  - Time: 1 hour

**Subtotal: 7 hours (0.9 days)**

---

### 2.3 KES Serialization Tests

- [ ] **2.3.1** Test: Raw serialization roundtrip (all KES types)
  - Serialize VK→bytes→deserialize, verify equality
  - Time: 2 hours

- [ ] **2.3.2** Test: Raw serialization size matches constants
  - Verify serialized size == SIZE constant
  - Time: 1 hour

- [ ] **2.3.3** Test: CBOR serialization matches raw + wrapping
  - Verify CBOR(x) == encode_bytes(raw_serialize(x))
  - Time: 2 hours

- [ ] **2.3.4** Test: Signature serialization roundtrip (all types)
  - Time: 1.5 hours

- [ ] **2.3.5** Test: UnsoundPure signing key serialization (all types)
  - Time: 1.5 hours

**Subtotal: 8 hours (1 day)**

---

### 2.4 Cross-Compatibility Tests with Haskell

- [ ] **2.4.1** Create Haskell test vector generator script
  - Script: `test-vectors/generate_kes_vectors.hs` (new)
  - Generate vectors for all KES algorithms
  - Time: 4 hours

- [ ] **2.4.2** Generate test vectors: SingleKes
  - Include: seed, VK, period, message, signature
  - Save as JSON
  - Time: 1 hour

- [ ] **2.4.3** Generate test vectors: Sum1Kes through Sum7Kes
  - Time: 2 hours

- [ ] **2.4.4** Generate test vectors: CompactSingleKes
  - Time: 1 hour

- [ ] **2.4.5** Generate test vectors: CompactSum1Kes through CompactSum7Kes
  - Time: 2 hours

- [ ] **2.4.6** Create Rust test loader for Haskell vectors
  - File: `cardano-crypto-class/tests/kes_cross_compat.rs` (new)
  - Parse JSON, run verification tests
  - Time: 3 hours

- [ ] **2.4.7** Test: Load and verify all Haskell-generated signatures
  - Verify all test vectors pass
  - Time: 2 hours

- [ ] **2.4.8** Test: Generate Rust signatures, verify in Haskell
  - Export Rust-generated vectors
  - Create Haskell verification script
  - Time: 4 hours

- [ ] **2.4.9** Test: CBOR encoding matches Haskell byte-for-byte
  - Compare CBOR encoding of same key/signature
  - Time: 3 hours

**Subtotal: 22 hours (2.8 days)**

---

### 2.5 UnsoundPureKESAlgorithm Trait Implementation

- [ ] **2.5.1** Define `UnsoundPureKesAlgorithm` trait
  - File: `cardano-crypto-class/src/kes/mod.rs`
  - Associated type: `UnsoundPureSigningKey`
  - Methods: gen, sign, update, derive_vk, conversions
  - Time: 2 hours

- [ ] **2.5.2** Add comprehensive trait documentation
  - Document all UNSOUND warnings
  - Explain why each method is unsafe
  - Provide usage examples
  - Time: 1.5 hours

- [ ] **2.5.3** Implement `UnsoundPureKesAlgorithm` for `SingleKes`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - UnsoundPure key = DSIGN key directly
  - Time: 2 hours

- [ ] **2.5.4** Implement `UnsoundPureKesAlgorithm` for `SumKes<D, H>`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Complex: child SK, seed, VK0, VK1 storage
  - Time: 4 hours

- [ ] **2.5.5** Implement `UnsoundPureKesAlgorithm` for `CompactSingleKes`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 2 hours

- [ ] **2.5.6** Implement `UnsoundPureKesAlgorithm` for `CompactSumKes<D, H>`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 4 hours

- [ ] **2.5.7** Add `to_unsound_pure_signing_key` for all KES types
  - Convert from mlocked SK to pure SK
  - Time: 2 hours

- [ ] **2.5.8** Add `from_unsound_pure_signing_key` for all KES types
  - Convert from pure SK to mlocked SK
  - Time: 2 hours

- [ ] **2.5.9** Add unit tests for UnsoundPure operations
  - Test gen, sign, update, derive for all types
  - Time: 3 hours

- [ ] **2.5.10** Add tests for SK conversions
  - Test to/from unsound pure for all types
  - Time: 2 hours

**Subtotal: 24.5 hours (3.1 days)**

---

### 2.6 Property-Based Tests

- [ ] **2.6.1** Add `proptest` dependency to workspace
  - File: `Cargo.toml`
  - Time: 15 min

- [ ] **2.6.2** Create property test file
  - File: `cardano-crypto-class/tests/kes_properties.rs` (new)
  - Setup proptest framework
  - Time: 1 hour

- [ ] **2.6.3** Property: Sign/verify roundtrip always succeeds
  - For all valid (seed, period, message) inputs
  - Test all KES variants
  - Time: 2 hours

- [ ] **2.6.4** Property: Verification key constant after updates
  - Update SK multiple times, VK unchanged
  - Time: 1.5 hours

- [ ] **2.6.5** Property: Wrong key always fails verification
  - Generate two keys, sign with A, verify with B fails
  - Time: 1.5 hours

- [ ] **2.6.6** Property: Wrong message always fails verification
  - Sign message A, verify message B fails
  - Time: 1.5 hours

- [ ] **2.6.7** Property: Wrong period always fails verification
  - Sign at period P, verify at period Q≠P fails
  - Time: 1.5 hours

- [ ] **2.6.8** Property: Serialization is deterministic
  - Serialize same key twice, get same bytes
  - Time: 1 hour

- [ ] **2.6.9** Property: Serialization roundtrip is identity
  - serialize→deserialize→serialize gives same bytes
  - Time: 1.5 hours

- [ ] **2.6.10** Property: Period must be in valid range
  - Operations outside [0, max_period) fail
  - Time: 1 hour

- [ ] **2.6.11** Property: Update is monotonic
  - Can't update from period P to Q < P
  - Time: 1 hour

**Subtotal: 13.75 hours (1.7 days)**

---

**Phase 2 Total: 83.75 hours (10.5 days)**

---

## 📊 Phase 3: Performance Optimization (MEDIUM PRIORITY) - Days 13-16

### 3.1 DirectSerialise for KES Types

- [ ] **3.1.1** Implement `DirectSerialise` for `SingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Zero-copy serialization from internal buffer
  - Time: 1 hour

- [ ] **3.1.2** Implement `DirectDeserialise` for `SingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/single.rs`
  - Time: 1 hour

- [ ] **3.1.3** Implement `DirectSerialise` for `Sum1KesVerificationKey` through `Sum7KesVerificationKey`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Time: 2 hours

- [ ] **3.1.4** Implement `DirectDeserialise` for `Sum1-7 KesVerificationKey`
  - File: `cardano-crypto-class/src/kes/sum.rs`
  - Time: 2 hours

- [ ] **3.1.5** Implement `DirectSerialise` for `CompactSingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 1 hour

- [ ] **3.1.6** Implement `DirectDeserialise` for `CompactSingleKesVerificationKey`
  - File: `cardano-crypto-class/src/kes/compact_single.rs`
  - Time: 1 hour

- [ ] **3.1.7** Implement `DirectSerialise` for `CompactSum1-7 KesVerificationKey`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 2 hours

- [ ] **3.1.8** Implement `DirectDeserialise` for `CompactSum1-7 KesVerificationKey`
  - File: `cardano-crypto-class/src/kes/compact_sum.rs`
  - Time: 2 hours

- [ ] **3.1.9** Implement `DirectSerialise` for all KES signature types
  - Files: single.rs, sum.rs, compact_single.rs, compact_sum.rs
  - Time: 3 hours

- [ ] **3.1.10** Implement `DirectDeserialise` for all KES signature types
  - Time: 3 hours

- [ ] **3.1.11** Add DirectSerialise roundtrip tests
  - Test all KES types
  - Time: 2 hours

**Subtotal: 20 hours (2.5 days)**

---

### 3.2 DirectSerialise for VRF Types

- [ ] **3.2.1** Implement `DirectSerialise` for `PraosVerificationKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 1 hour

- [ ] **3.2.2** Implement `DirectDeserialise` for `PraosVerificationKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 1 hour

- [ ] **3.2.3** Implement `DirectSerialise` for `PraosSigningKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 1 hour

- [ ] **3.2.4** Implement `DirectDeserialise` for `PraosSigningKey`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 1 hour

- [ ] **3.2.5** Implement `DirectSerialise` for `PraosProof`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 1 hour

- [ ] **3.2.6** Implement `DirectDeserialise` for `PraosProof`
  - File: `cardano-crypto-class/src/vrf/praos.rs`
  - Time: 1 hour

- [ ] **3.2.7** Implement DirectSerialise for PraosBatchCompat types
  - File: `cardano-crypto-class/src/vrf/praos_batch.rs`
  - Time: 2 hours

- [ ] **3.2.8** Implement DirectSerialise for SimpleVRF types
  - File: `cardano-crypto-class/src/vrf/simple.rs`
  - Time: 1.5 hours

- [ ] **3.2.9** Implement DirectSerialise for MockVRF types
  - File: `cardano-crypto-class/src/vrf/mock.rs`
  - Time: 1.5 hours

- [ ] **3.2.10** Add DirectSerialise roundtrip tests for VRF
  - File: `cardano-crypto-class/tests/vrf_direct.rs` (new)
  - Time: 2 hours

**Subtotal: 13 hours (1.6 days)**

---

### 3.3 Performance Benchmarking

- [ ] **3.3.1** Create benchmark suite
  - File: `cardano-crypto-class/benches/serialization.rs` (new)
  - Setup criterion framework
  - Time: 1 hour

- [ ] **3.3.2** Benchmark: Raw serialization vs DirectSerialise
  - Measure speed improvement
  - Time: 2 hours

- [ ] **3.3.3** Benchmark: CBOR serialization performance
  - Measure encoding/decoding speed
  - Time: 1 hour

- [ ] **3.3.4** Benchmark: KES sign/verify operations
  - Measure core crypto performance
  - Time: 2 hours

- [ ] **3.3.5** Benchmark: VRF operations
  - Time: 1 hour

- [ ] **3.3.6** Create performance comparison report
  - Document: Rust vs Haskell performance
  - Time: 2 hours

**Subtotal: 9 hours (1.1 days)**

---

**Phase 3 Total: 42 hours (5.3 days)**

---

## 🔵 Phase 4: Additional Features and Polish - Days 17-18

### 4.1 Additional Test Coverage

- [ ] **4.1.1** Add VRF basic positive tests
  - File: `cardano-crypto-class/tests/vrf_basic.rs` (new)
  - Test prove/verify operations
  - Time: 2 hours

- [ ] **4.1.2** Add VRF negative tests
  - Test wrong key, wrong message scenarios
  - Time: 2 hours

- [ ] **4.1.3** Add DSIGN basic tests
  - File: `cardano-crypto-class/tests/dsign_basic.rs` (new)
  - Test sign/verify operations
  - Time: 2 hours

- [ ] **4.1.4** Add DSIGN negative tests
  - Time: 2 hours

- [ ] **4.1.5** Add edge case tests (empty messages, max sizes, etc.)
  - Time: 2 hours

**Subtotal: 10 hours (1.3 days)**

---

### 4.2 Documentation Improvements

- [ ] **4.2.1** Add module-level documentation for kes/mod.rs
  - Explain KES concepts, usage examples
  - Time: 2 hours

- [ ] **4.2.2** Add module-level documentation for vrf/mod.rs
  - Time: 2 hours

- [ ] **4.2.3** Add module-level documentation for dsign/mod.rs
  - Time: 2 hours

- [ ] **4.2.4** Document all public APIs with examples
  - Ensure all public functions have doc comments
  - Time: 4 hours

- [ ] **4.2.5** Create usage guide
  - File: `docs/usage/CRYPTO_GUIDE.md` (new)
  - How to use KES, VRF, DSIGN in applications
  - Time: 3 hours

- [ ] **4.2.6** Create migration guide from Haskell
  - File: `docs/migration/HASKELL_TO_RUST_CRYPTO.md` (new)
  - API differences, porting tips
  - Time: 3 hours

**Subtotal: 16 hours (2 days)**

---

### 4.3 Code Quality and Cleanup

- [ ] **4.3.1** Run clippy on all crypto code, fix warnings
  - Time: 2 hours

- [ ] **4.3.2** Run rustfmt on all crypto code
  - Time: 30 min

- [ ] **4.3.3** Check for code duplication, refactor
  - Time: 3 hours

- [ ] **4.3.4** Review error messages, improve clarity
  - Time: 2 hours

- [ ] **4.3.5** Add #[must_use] attributes where appropriate
  - Time: 1 hour

- [ ] **4.3.6** Audit unsafe code usage
  - Verify all unsafe blocks are necessary and documented
  - Time: 2 hours

**Subtotal: 10.5 hours (1.3 days)**

---

**Phase 4 Total: 36.5 hours (4.6 days)**

---

## 📝 Final Verification and Documentation - Day 19

### 5.1 Final Testing

- [ ] **5.1.1** Run full test suite
  - `cargo test --workspace`
  - Verify all tests pass
  - Time: 30 min

- [ ] **5.1.2** Run property tests with high iteration count
  - `cargo test --release -- --ignored`
  - Time: 1 hour

- [ ] **5.1.3** Run benchmarks and verify performance
  - `cargo bench`
  - Time: 1 hour

- [ ] **5.1.4** Test with Haskell cardano-node (if available)
  - Integration test with real node
  - Time: 3 hours

**Subtotal: 5.5 hours**

---

### 5.2 Documentation Updates

- [ ] **5.2.1** Update GAPS_ANALYSIS.md to mark gaps as closed
  - Time: 1 hour

- [ ] **5.2.2** Update README.md with new features
  - Document CBOR support, comprehensive tests
  - Time: 1 hour

- [ ] **5.2.3** Update CHANGELOG.md
  - Document all changes made
  - Time: 1 hour

- [ ] **5.2.4** Create COMPLETION_REPORT.md
  - Document what was accomplished
  - Before/after comparison
  - Time: 2 hours

**Subtotal: 5 hours**

---

### 5.3 Final Cleanup

- [ ] **5.3.1** Review all commit messages
  - Time: 30 min

- [ ] **5.3.2** Squash/organize commits if needed
  - Time: 1 hour

- [ ] **5.3.3** Update version numbers
  - Bump to 0.2.0 (breaking changes)
  - Time: 30 min

- [ ] **5.3.4** Create GitHub release notes
  - Time: 1 hour

**Subtotal: 3 hours**

---

**Phase 5 Total: 13.5 hours (1.7 days)**

---

## 📊 Grand Total Summary

| Phase | Description | Hours | Days |
|-------|-------------|-------|------|
| **Phase 1** | CBOR Serialization (CRITICAL) | 50.5 | 6.3 |
| **Phase 2** | Comprehensive Testing (HIGH) | 83.75 | 10.5 |
| **Phase 3** | Performance Optimization (MEDIUM) | 42 | 5.3 |
| **Phase 4** | Additional Features & Polish | 36.5 | 4.6 |
| **Phase 5** | Final Verification | 13.5 | 1.7 |
| **TOTAL** | **All Gaps Closed** | **226.25** | **28.3** |

**Note:** Original estimate was 11-16 days for just Phases 1-3 (critical + high priority).
This comprehensive todo includes all gaps + polish + documentation, coming to ~28 days.

---

## 📋 Progress Tracking

### By Priority Level

**🔴 Critical (Must Have):**

- [ ] Phase 1: CBOR Serialization - 0/90 tasks complete

**⚠️ High Priority (Should Have):**

- [ ] Phase 2: Comprehensive Testing - 0/72 tasks complete

**📊 Medium Priority (Nice to Have):**

- [ ] Phase 3: Performance - 0/31 tasks complete

**🔵 Low Priority (Polish):**

- [ ] Phase 4: Additional Features - 0/20 tasks complete
- [ ] Phase 5: Final Verification - 0/12 tasks complete

### By Module

**KES Module:**

- [ ] 0/120 tasks complete

**VRF Module:**

- [ ] 0/35 tasks complete

**DSIGN Module:**

- [ ] 0/20 tasks complete

**Testing:**

- [ ] 0/50 tasks complete

**Documentation:**

- [ ] 0/15 tasks complete

---

## 🎯 Recommended Execution Strategy

### Option A: Phased Delivery (Recommended)

1. **Sprint 1 (Week 1):** Complete Phase 1 - Delivers Cardano node compatibility
2. **Sprint 2 (Weeks 2-3):** Complete Phase 2 - Delivers production confidence
3. **Sprint 3 (Week 4):** Complete Phase 3 - Delivers performance optimization
4. **Sprint 4 (Week 5):** Complete Phases 4-5 - Delivers polish and completion

### Option B: Module-by-Module

1. Complete all KES gaps (all phases)
2. Complete all VRF gaps (all phases)
3. Complete all DSIGN gaps (all phases)
4. Final integration and testing

### Option C: Parallel Teams

1. Team A: CBOR implementation
2. Team B: Test suite development
3. Team C: DirectSerialise implementation
4. Merge and integrate at end

---

## ✅ Success Criteria

### Phase 1 Success

- [ ] All crypto types serialize/deserialize via CBOR
- [ ] Can integrate with Cardano node
- [ ] CBOR encoding matches Haskell byte-for-byte

### Phase 2 Success

- [ ] Test coverage > 80%
- [ ] All Haskell test vectors pass
- [ ] Property tests find no issues
- [ ] UnsoundPure trait fully functional

### Phase 3 Success

- [ ] DirectSerialise faster than regular serialization
- [ ] Benchmarks show performance gains
- [ ] Zero-copy working correctly

### Overall Success

- [ ] All 225 tasks completed
- [ ] All gaps with Haskell closed
- [ ] Production-ready code
- [ ] Comprehensive documentation
- [ ] Ready for crates.io publication

---

**Last Updated:** October 4, 2025
**Total Tasks:** 225
**Completed:** 0
**Remaining:** 225
**Progress:** 0%
