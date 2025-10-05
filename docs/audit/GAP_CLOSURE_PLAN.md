# Gap Closure Implementation Plan

**Date:** October 4, 2025
**Goal:** Close all gaps with Haskell cardano-base repository
**Total Estimated Effort:** 11-16 days

---

## Executive Summary

Based on comprehensive gap analysis and review of the Haskell cardano-base repository, this document provides a detailed implementation plan to close all identified gaps and achieve full parity with the Haskell implementation.

**Current Status:**

- ✅ Core algorithms: Complete and correct
- ✅ Binary compatibility: Achieved (Blake2b-256 fix)
- ❌ CBOR serialization: Missing (all crypto modules)
- ❌ Comprehensive tests: Minimal
- ❌ DirectSerialise optimization: Incomplete
- ❌ UnsoundPure trait: Not implemented

---

## Implementation Phases

### Phase 1: CBOR Serialization (Days 1-4) 🔴 CRITICAL

**Goal:** Enable Cardano node integration

#### Task 1.1: Add CBOR for KES Types (2 days)

**Files to Modify:**

- `cardano-crypto-class/src/kes/single.rs`
- `cardano-crypto-class/src/kes/sum.rs`
- `cardano-crypto-class/src/kes/compact_single.rs`
- `cardano-crypto-class/src/kes/compact_sum.rs`

**Implementation Pattern** (from Haskell):

```haskell
-- Haskell pattern
instance ToCBOR (VerKeyKES v) where
  toCBOR = encodeBytes . rawSerialiseVerKeyKES

instance FromCBOR (VerKeyKES v) where
  fromCBOR = do
    bs <- decodeBytes
    case rawDeserialiseVerKeyKES bs of
      Nothing -> fail "invalid verification key"
      Just vk -> return vk
```

**Rust Implementation:**

```rust
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Error as DeError;

// For verification keys
impl<D, H> Serialize for SumKesVerificationKey<D, H>
where
    D: KesAlgorithm,
    H: KesHashAlgorithm,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as CBOR bytes wrapping raw serialization
        let bytes = SumKes::<D, H>::raw_serialize_verification_key_kes(self);
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de, D, H> Deserialize<'de> for SumKesVerificationKey<D, H>
where
    D: KesAlgorithm,
    H: KesHashAlgorithm,
{
    fn deserialize<DE>(deserializer: DE) -> Result<Self, DE::Error>
    where
        DE: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        SumKes::<D, H>::raw_deserialize_verification_key_kes(&bytes)
            .ok_or_else(|| DE::Error::custom("invalid KES verification key"))
    }
}

// Similar for signatures
impl<D, H> Serialize for SumKesSignature<D, H> { ... }
impl<'de, D, H> Deserialize<'de> for SumKesSignature<D, H> { ... }

// Note: Signing keys should NOT have ToCBOR/FromCBOR in production
// (violates mlocking). Only UnsoundPureSignKey should be serializable.
```

**Types Needing CBOR:**

1. **SingleKes:**
   - `SingleKesVerificationKey`
   - `SingleKesSignature`
   - `UnsoundPureSingleKesSigningKey` (for testing only)

2. **Sum1Kes through Sum7Kes:**
   - `SumKesVerificationKey<D, H>`
   - `SumKesSignature<D, H>`
   - `UnsoundPureSumKesSigningKey<D, H>` (for testing)

3. **CompactSingleKes:**
   - `CompactSingleKesVerificationKey`
   - `CompactSingleKesSignature`
   - `UnsoundPureCompactSingleKesSigningKey`

4. **CompactSum1Kes through CompactSum7Kes:**
   - `CompactSumKesVerificationKey<D, H>`
   - `CompactSumKesSignature<D, H>`
   - `UnsoundPureCompactSumKesSigningKey<D, H>`

**Tests to Add:**

```rust
#[cfg(test)]
mod cbor_tests {
    use super::*;
    use ciborium::{ser, de};

    #[test]
    fn test_verification_key_cbor_roundtrip() {
        // Generate key
        let seed = [0u8; 32];
        let sk = Sum1Kes::gen_key_kes(&seed).unwrap();
        let vk = Sum1Kes::derive_verification_key_kes(&sk);

        // Serialize to CBOR
        let mut cbor_bytes = Vec::new();
        ser::into_writer(&vk, &mut cbor_bytes).unwrap();

        // Deserialize from CBOR
        let vk2: Sum1KesVerificationKey = de::from_reader(&cbor_bytes[..]).unwrap();

        // Verify equality
        assert_eq!(
            Sum1Kes::raw_serialize_verification_key_kes(&vk),
            Sum1Kes::raw_serialize_verification_key_kes(&vk2)
        );
    }

    #[test]
    fn test_signature_cbor_roundtrip() {
        // Similar pattern for signatures
    }
}
```

**Acceptance Criteria:**

- ✅ All KES verification key types serialize/deserialize via CBOR
- ✅ All KES signature types serialize/deserialize via CBOR
- ✅ UnsoundPure signing keys serialize (for testing only)
- ✅ Roundtrip tests pass for all types
- ✅ CBOR encoding matches Haskell byte-for-byte (verify with test vectors)

---

#### Task 1.2: Add CBOR for VRF Types (1 day)

**Files to Modify:**

- `cardano-crypto-class/src/vrf/praos.rs`
- `cardano-crypto-class/src/vrf/simple.rs`
- `cardano-crypto-class/src/vrf/mock.rs`

**Pattern:** Same as KES - serialize as CBOR bytes wrapping raw serialization

**Types Needing CBOR:**

- `PraosVerificationKey`
- `PraosSigningKey`
- `PraosProof`
- Similar for Simple, Mock variants

---

#### Task 1.3: Add CBOR for DSIGN Types (1 day)

**Files to Modify:**

- `cardano-crypto-class/src/dsign/ed25519.rs`
- `cardano-crypto-class/src/dsign/ed25519_mlocked.rs`

**Types Needing CBOR:**

- `Ed25519VerificationKey`
- `Ed25519SigningKey` (if not mlocked)
- `Ed25519Signature`

---

### Phase 2: Test Suite Expansion (Days 5-12) ⚠️ HIGH

**Goal:** Comprehensive test coverage matching Haskell

#### Task 2.1: Basic KES Tests (2 days)

**File to Create:** `cardano-crypto-class/tests/kes_basic.rs`

**Tests to Implement:**

1. **Positive Tests:**

```rust
#[test]
fn test_sign_verify_period_0() {
    let seed = [0u8; 32];
    let sk = Sum1Kes::gen_key_kes(&seed).unwrap();
    let vk = Sum1Kes::derive_verification_key_kes(&sk);

    let message = b"test message";
    let signature = Sum1Kes::sign_kes((), 0, message, &sk).unwrap();

    assert!(Sum1Kes::verify_kes((), &vk, 0, message, &signature).is_ok());
}

#[test]
fn test_key_evolution() {
    let seed = [0u8; 32];
    let sk0 = Sum7Kes::gen_key_kes(&seed).unwrap();

    // Evolve through multiple periods
    let sk1 = Sum7Kes::update_kes(&sk0, 0).unwrap();
    let sk2 = Sum7Kes::update_kes(&sk1, 1).unwrap();

    // Verify each period still signs correctly
    let vk = Sum7Kes::derive_verification_key_kes(&sk2);
    let msg = b"period 2 message";
    let sig = Sum7Kes::sign_kes((), 2, msg, &sk2).unwrap();

    assert!(Sum7Kes::verify_kes((), &vk, 2, msg, &sig).is_ok());
}
```

2. **Negative Tests:**

```rust
#[test]
fn test_verify_wrong_key() {
    let sk1 = Sum1Kes::gen_key_kes(&[0u8; 32]).unwrap();
    let sk2 = Sum1Kes::gen_key_kes(&[1u8; 32]).unwrap();

    let vk2 = Sum1Kes::derive_verification_key_kes(&sk2);

    let msg = b"test";
    let sig = Sum1Kes::sign_kes((), 0, msg, &sk1).unwrap();

    // Should fail - wrong key
    assert!(Sum1Kes::verify_kes((), &vk2, 0, msg, &sig).is_err());
}

#[test]
fn test_verify_wrong_message() {
    let sk = Sum1Kes::gen_key_kes(&[0u8; 32]).unwrap();
    let vk = Sum1Kes::derive_verification_key_kes(&sk);

    let sig = Sum1Kes::sign_kes((), 0, b"original", &sk).unwrap();

    // Should fail - wrong message
    assert!(Sum1Kes::verify_kes((), &vk, 0, b"different", &sig).is_err());
}

#[test]
fn test_verify_wrong_period() {
    let sk = Sum1Kes::gen_key_kes(&[0u8; 32]).unwrap();
    let vk = Sum1Kes::derive_verification_key_kes(&sk);

    let msg = b"test";
    let sig = Sum1Kes::sign_kes((), 0, msg, &sk).unwrap();

    // Should fail - wrong period
    assert!(Sum1Kes::verify_kes((), &vk, 1, msg, &sig).is_err());
}
```

3. **Serialization Roundtrip Tests:**

```rust
#[test]
fn test_verification_key_serialization_roundtrip() {
    for algorithm in ALL_KES_ALGORITHMS {
        // Test raw serialization
        // Test CBOR serialization
        // Verify byte-for-byte equality
    }
}
```

---

#### Task 2.2: Cross-Compatibility Tests (3 days)

**Requires:** Golden test vectors from Haskell implementation

**File to Create:** `cardano-crypto-class/tests/kes_cross_compat.rs`

**Approach:**

1. Generate test vectors in Haskell
2. Save as JSON or binary files
3. Load and verify in Rust

**Test Vector Format:**

```json
{
  "algorithm": "Sum1Kes",
  "seed": "0000...0000",
  "period": 0,
  "message": "test message",
  "verification_key": "abcd...",
  "signature": "1234...",
  "expected_result": "valid"
}
```

---

#### Task 2.3: UnsoundPureKESAlgorithm Implementation (3 days)

**File to Modify:** `cardano-crypto-class/src/kes/mod.rs`

**Trait Definition:**

```rust
/// Pure (non-monadic) KES operations for testing.
///
/// # WARNING: UNSOUND FOR PRODUCTION
///
/// These operations are:
/// - NOT constant-time (may leak timing information)
/// - NOT memory-safe for secrets (no mlocking)
/// - NOT suitable for production use
///
/// Use ONLY for:
/// - Property-based testing
/// - Test vector generation
/// - Development and debugging
pub trait UnsoundPureKesAlgorithm: KesAlgorithm {
    /// Pure signing key type (not mlocked, can be cloned/compared)
    type UnsoundPureSigningKey: Clone + Eq + std::fmt::Debug;

    /// Generate a key from a seed (pure, deterministic, not constant-time)
    ///
    /// # Warning
    /// This violates constant-time guarantees. Use only for testing.
    fn unsound_pure_gen_key_kes(seed: &[u8]) -> Self::UnsoundPureSigningKey;

    /// Sign without IO (pure, deterministic, not constant-time)
    ///
    /// # Warning
    /// This violates constant-time and mlocking guarantees.
    fn unsound_pure_sign_kes(
        context: Self::Context,
        period: u32,
        message: &[u8],
        signing_key: &Self::UnsoundPureSigningKey,
    ) -> Result<Self::Signature, KesError>;

    /// Update key without IO (pure, consumes old key)
    fn unsound_pure_update_kes(
        signing_key: Self::UnsoundPureSigningKey,
        period: u32,
    ) -> Option<Self::UnsoundPureSigningKey>;

    /// Derive verification key (pure)
    fn unsound_pure_derive_ver_key_kes(
        signing_key: &Self::UnsoundPureSigningKey,
    ) -> Self::VerificationKey;

    /// Convert from safe (mlocked) to unsafe (pure) signing key
    ///
    /// # Warning
    /// This defeats mlocking protections. Use only for testing.
    fn to_unsound_pure_signing_key(
        signing_key: &Self::SigningKey,
    ) -> Result<Self::UnsoundPureSigningKey, KesError>;

    /// Convert from unsafe (pure) to safe (mlocked) signing key
    fn from_unsound_pure_signing_key(
        unsound_key: Self::UnsoundPureSigningKey,
    ) -> Result<Self::SigningKey, KesError>;
}
```

**Implementation for SingleKes:**

```rust
impl<D> UnsoundPureKesAlgorithm for SingleKes<D>
where
    D: DsignAlgorithm,
{
    type UnsoundPureSigningKey = D::SigningKey; // Just use DSIGN key directly

    fn unsound_pure_gen_key_kes(seed: &[u8]) -> Self::UnsoundPureSigningKey {
        // Generate DSIGN key from seed
        D::gen_key_dsign(seed)
    }

    fn unsound_pure_sign_kes(
        context: (),
        period: u32,
        message: &[u8],
        signing_key: &Self::UnsoundPureSigningKey,
    ) -> Result<Self::Signature, KesError> {
        if period != 0 {
            return Err(KesError::PeriodOutOfRange);
        }

        let sig = D::sign_dsign(context, message, signing_key)?;
        Ok(SingleKesSignature(sig))
    }

    // ... other methods
}
```

**Implementation for SumKes:**

```rust
impl<D, H> UnsoundPureKesAlgorithm for SumKes<D, H>
where
    D: UnsoundPureKesAlgorithm,
    H: KesHashAlgorithm,
{
    type UnsoundPureSigningKey = UnsoundPureSumKesSigningKey<D, H>;

    // Implementation similar to Haskell's UnsoundPureSumKES
    // Store child key, seed, and both verification keys
}
```

**Acceptance Criteria:**

- ✅ Trait defined with clear UNSOUND warnings
- ✅ Implemented for all KES variants
- ✅ Property tests can use this trait
- ✅ Serialize/Deserialize implemented for UnsoundPure keys

---

#### Task 2.4: Property-Based Tests (2 days)

**File to Create:** `cardano-crypto-class/tests/kes_properties.rs`

**Using `proptest`:**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_sign_verify_roundtrip(
        seed in any::<[u8; 32]>(),
        period in 0u32..128,
        message in any::<Vec<u8>>(),
    ) {
        let sk = Sum7Kes::unsound_pure_gen_key_kes(&seed);

        // Sign
        if let Ok(signature) = Sum7Kes::unsound_pure_sign_kes((), period, &message, &sk) {
            // Verify should succeed
            let vk = Sum7Kes::unsound_pure_derive_ver_key_kes(&sk);
            prop_assert!(Sum7Kes::verify_kes((), &vk, period, &message, &signature).is_ok());
        }
    }

    #[test]
    fn prop_update_preserves_verification_key(
        seed in any::<[u8; 32]>(),
        period in 0u32..127,
    ) {
        let sk0 = Sum7Kes::unsound_pure_gen_key_kes(&seed);
        let vk0 = Sum7Kes::unsound_pure_derive_ver_key_kes(&sk0);

        if let Some(sk1) = Sum7Kes::unsound_pure_update_kes(sk0, period) {
            let vk1 = Sum7Kes::unsound_pure_derive_ver_key_kes(&sk1);

            // Verification key should not change after update
            prop_assert_eq!(
                Sum7Kes::raw_serialize_verification_key_kes(&vk0),
                Sum7Kes::raw_serialize_verification_key_kes(&vk1)
            );
        }
    }
}
```

---

### Phase 3: Performance Optimization (Days 13-16) 📊 MEDIUM

**Goal:** Zero-copy serialization for hot paths

#### Task 3.1: DirectSerialise for KES (2 days)

**Files to Modify:**

- `cardano-crypto-class/src/kes/single.rs`
- `cardano-crypto-class/src/kes/sum.rs`
- etc.

**Pattern** (following Ed25519):

```rust
use crate::direct_serialise::{DirectSerialise, DirectDeserialise, DirectResult};

impl<D, H> DirectSerialise for SumKesVerificationKey<D, H>
where
    D: KesAlgorithm,
    H: KesHashAlgorithm,
{
    fn direct_serialise(
        &self,
        f: &mut dyn FnMut(*const u8, usize) -> DirectResult<()>,
    ) -> DirectResult<()> {
        // Serialize directly from internal buffer without copying
        let bytes = self.as_bytes(); // Assuming you have this method
        f(bytes.as_ptr(), bytes.len())
    }
}

impl<D, H> DirectDeserialise for SumKesVerificationKey<D, H>
where
    D: KesAlgorithm,
    H: KesHashAlgorithm,
{
    fn direct_deserialise(
        f: &mut dyn FnMut(*mut u8, usize) -> DirectResult<()>,
    ) -> DirectResult<Self> {
        let mut buffer = vec![0u8; Self::SIZE];
        f(buffer.as_mut_ptr(), buffer.len())?;

        Self::from_bytes(&buffer)
            .ok_or_else(|| SizeCheckError {
                expected_size: Self::SIZE,
                actual_size: buffer.len(),
            })
    }
}
```

---

#### Task 3.2: DirectSerialise for VRF (2 days)

Similar pattern for VRF types.

---

## Implementation Guidelines

### Code Style

1. **Follow Existing Patterns:**
   - Look at how Ed25519 implements DirectSerialise
   - Follow VRF test vector pattern for KES tests
   - Use same error handling as existing code

2. **Safety First:**
   - Mark UnsoundPure operations with clear warnings
   - Never serialize mlocked signing keys (except UnsoundPure)
   - Use proper zeroization

3. **Documentation:**
   - Document why each operation is unsound
   - Provide examples in doc comments
   - Link to Haskell source for reference

### Testing Strategy

1. **Unit Tests:** Test each component individually
2. **Integration Tests:** Test full workflows
3. **Property Tests:** Test mathematical properties
4. **Cross-Compat Tests:** Verify Haskell compatibility
5. **Benchmark Tests:** Measure performance improvements

### Git Workflow

1. **One PR per Phase:** Don't try to do everything at once
2. **Incremental Commits:** Small, focused commits with clear messages
3. **Test at Each Step:** Run tests after each change
4. **Review Carefully:** Get code review before merging

---

## Acceptance Criteria (Overall)

### Phase 1 Complete When

- ✅ All crypto types have working CBOR serialization
- ✅ Roundtrip tests pass for all types
- ✅ Can serialize/deserialize keys and signatures
- ✅ CBOR encoding matches Haskell (verified with test vectors)

### Phase 2 Complete When

- ✅ Comprehensive test suite matches Haskell coverage
- ✅ All positive/negative tests pass
- ✅ Cross-compatibility with Haskell verified
- ✅ UnsoundPure trait implemented and tested
- ✅ Property tests running and passing

### Phase 3 Complete When

- ✅ DirectSerialise implemented for all types
- ✅ Benchmarks show performance improvement
- ✅ Zero-copy serialization working correctly

---

## Risk Mitigation

### Risks

1. **Time Estimation:** 11-16 days is aggressive
   - **Mitigation:** Break into smaller chunks, prioritize Phase 1

2. **Haskell Compatibility:** Byte-for-byte encoding might differ
   - **Mitigation:** Use official test vectors, verify early

3. **UnsoundPure Complexity:** Trait is complex to implement correctly
   - **Mitigation:** Start with SimpleKes, then extend to Sum types

4. **Test Coverage:** Hard to know when "comprehensive" is enough
   - **Mitigation:** Port specific Haskell tests, aim for 80%+ coverage

---

## Next Steps

**Immediate (Today):**

1. Set up development environment
2. Review Haskell source code in detail
3. Create test vector generation script (in Haskell)

**Short-term (This Week):**

1. Start Phase 1 implementation
2. Implement CBOR for Sum1Kes first (proof of concept)
3. Write roundtrip tests

**Medium-term (Next 2 Weeks):**

1. Complete all of Phase 1
2. Start Phase 2 basic tests
3. Generate comprehensive test vectors

---

## Resources

**Haskell Source Files to Reference:**

- `cardano-crypto-class/src/Cardano/Crypto/KES/Class.hs` - Core KES trait
- `cardano-crypto-class/src/Cardano/Crypto/KES/Sum.hs` - Sum implementation
- `cardano-crypto-class/src/Cardano/Crypto/KES/CompactSum.hs` - Compact variant
- `cardano-crypto-tests/src/Test/Crypto/KES.hs` - Test suite

**Existing Rust Code to Reference:**

- `cardano-crypto-class/src/dsign/ed25519.rs` - DirectSerialise example
- `cardano-binary/tests/proptest_roundtrip.rs` - Property test example
- `cardano-crypto-class/tests/vrf_praos_vectors.rs` - Test vector example

---

**Total Estimated Timeline:** 11-16 days (focused full-time development)

**Priority Order:**

1. Phase 1 (CRITICAL) - 4 days
2. Phase 2.1-2.2 (HIGH) - 5 days
3. Phase 2.3-2.4 (MEDIUM) - 5 days
4. Phase 3 (MEDIUM) - 4 days

**Recommended Approach:** Start with Phase 1, deliver incrementally, gather feedback before proceeding to next phase.
