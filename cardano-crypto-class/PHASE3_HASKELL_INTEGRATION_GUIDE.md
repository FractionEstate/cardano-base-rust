# Phase 3: Haskell Integration Guide

**Status:** 🟡 In Progress - Infrastructure Ready, Awaiting Haskell Reference Values
**Created:** October 4, 2025
**Phase 2 Completion:** October 4, 2025

## Overview

Phase 3 establishes byte-for-byte compatibility verification between the Rust `cardano-crypto-class` implementation and the Haskell `cardano-base` library. This ensures that CBOR-encoded cryptographic primitives are interoperable across both implementations, which is **critical** for Cardano blockchain applications.

## Prerequisites

✅ **Completed:**

- Phase 1: CBOR serde implementations for 18 types
- Phase 2: 30 Rust test vectors generated with deterministic CBOR encodings

⏳ **Required:**

- Haskell cardano-base reference implementation
- Haskell CBOR values for identical test vectors
- Method to generate or obtain Haskell test data

## Phase 3 Objectives

### Primary Goals

1. **Obtain Haskell Reference Values**
   - Generate CBOR encodings from Haskell cardano-base for all 30 test vectors
   - Ensure identical inputs (same seeds, messages, periods)
   - Document Haskell version and dependencies used

2. **Implement Comparison Framework**
   - Byte-for-byte comparison of Rust vs Haskell CBOR
   - Detailed diff reporting for any discrepancies
   - Automated test suite for regression prevention

3. **Enable CI Integration**
   - Add Haskell comparison tests to CI pipeline
   - Document compatibility status in README
   - Create versioned compatibility matrix

### Success Criteria

- ✅ All 30 test vectors match Haskell byte-for-byte (100% compatibility)
- ✅ Automated tests prevent regressions
- ✅ Documented process for adding new test vectors
- ✅ CI pipeline validates every commit

## Test Vector Inventory

### Phase 2 Complete: 30 Rust Test Vectors

| Algorithm | Vectors | File | Status |
|-----------|---------|------|--------|
| Ed25519 | 5 | `ed25519_vectors.json` | ✅ Ready |
| PraosVRF | 5 | `praos_vrf_vectors.json` | ✅ Ready |
| SimpleVRF | 5 | `simple_vrf_vectors.json` | ✅ Ready |
| MockVRF | 5 | `mock_vrf_vectors.json` | ✅ Ready |
| SingleKes | 5 | `single_kes_vectors.json` | ✅ Ready |
| CompactSingleKes | 5 | `compact_single_kes_vectors.json` | ✅ Ready |

### Required: Haskell Reference Values

For each test vector, we need Haskell to generate:

**Input (from our JSON files):**

- `seed` - Hex-encoded seed bytes
- `message` - Hex-encoded message bytes
- `period` - Integer period value (KES only)

**Expected Output (Haskell CBOR):**

- Verification Key CBOR (hex-encoded)
- Signature/Proof CBOR (hex-encoded)

## Implementation Strategy

### Option 1: Direct Haskell Coordination (Recommended)

**Contact:** Haskell cardano-base maintainers at IntersectMBO

**Request:**

```
Subject: Rust Port CBOR Compatibility Testing - Reference Values Needed

Hi cardano-base team,

We're completing a pure Rust port of cardano-base cryptographic primitives
and need Haskell reference CBOR values to verify byte-for-byte compatibility.

We have 30 test vectors ready (Ed25519, VRF, KES) with deterministic inputs.
Could you generate corresponding CBOR outputs using cardano-base?

Test vectors: [link to our JSON files]
Format needed: Hex-encoded CBOR for verification keys and signatures

This ensures Rust<->Haskell interoperability for Cardano applications.

Thank you!
```

**Resources:**

- Haskell cardano-base: <https://github.com/IntersectMBO/cardano-base>
- Contact: File issue or reach out to maintainers

### Option 2: Self-Generate Using Haskell Tooling

**Requirements:**

- Install Haskell Stack or Cabal
- Clone cardano-base repository
- Build cardano-crypto-class Haskell package
- Create Haskell test harness

**Steps:**

1. **Setup Haskell Environment**

```bash
# Install Stack
curl -sSL https://get.haskellstack.org/ | sh

# Clone cardano-base
git clone https://github.com/IntersectMBO/cardano-base.git
cd cardano-base

# Build cardano-crypto-class
stack build cardano-crypto-class
```

2. **Create Test Harness**

Create `GenerateTestVectors.hs`:

```haskell
{-# LANGUAGE DataKinds #-}
{-# LANGUAGE TypeApplications #-}

module Main where

import qualified Data.ByteString.Base16 as B16
import qualified Data.ByteString as BS
import Cardano.Crypto.DSIGN.Ed25519 (Ed25519DSIGN)
import Cardano.Crypto.KES.Single (SingleKES)
import Cardano.Crypto.VRF.Praos (PraosVRF)
import Cardano.Binary (serialize')
import qualified Cardano.Crypto.DSIGN as DSIGN
import qualified Cardano.Crypto.KES as KES
import qualified Cardano.Crypto.VRF as VRF

-- Generate CBOR for Ed25519 test vector
generateEd25519 :: BS.ByteString -> BS.ByteString -> IO ()
generateEd25519 seed msg = do
  let sk = DSIGN.genKeyDSIGN @Ed25519DSIGN seed
      vk = DSIGN.deriveVerKeyDSIGN sk
      sig = DSIGN.signDSIGN () msg sk

  putStrLn $ "VK CBOR:  " ++ B16.encode (serialize' vk)
  putStrLn $ "Sig CBOR: " ++ B16.encode (serialize' sig)

-- Similar functions for VRF, KES...

main :: IO ()
main = do
  -- Read test vectors from JSON
  -- Generate CBOR for each
  -- Output in format matching our JSON structure
  putStrLn "Generating Haskell reference values..."
```

3. **Run and Collect**

```bash
stack runghc GenerateTestVectors.hs > haskell_reference_values.json
```

### Option 3: Extract from Haskell Tests

**Approach:** Examine existing Haskell cardano-base tests

**Files to check:**

- `cardano-crypto-class/test/Test/Crypto/*.hs`
- Look for CBOR serialization tests
- Extract test vectors and expected values

**Advantage:** Values already validated in Haskell test suite
**Disadvantage:** May not match our exact test cases

## Test Implementation Plan

### Step 1: Add Haskell Reference Values to JSON

Extend each JSON file with Haskell values:

```json
{
  "description": "Ed25519 DSIGN cross-compatibility test vectors",
  "algorithm": "Ed25519",
  "vectors": [
    {
      "name": "all_zeros_seed",
      "seed": "0000000000000000000000000000000000000000000000000000000000000000",
      "message": "48656c6c6f2c20576f726c6421",
      "rust_cbor": {
        "vk": "58203b6a27bcceb6a42d62a3a8d02a6f0d73653215771de243a63ac048a18b59da29",
        "sig": "5840bb1a53a55494a41cd7f98302d1a156cf9a1ac046eced43b2b565e2debe4a336c..."
      },
      "haskell_cbor": {
        "vk": "TO_BE_FILLED",
        "sig": "TO_BE_FILLED"
      },
      "compatible": null
    }
  ]
}
```

### Step 2: Create Comparison Tests

Add to `tests/cross_compat.rs`:

```rust
#[test]
fn test_ed25519_rust_vs_haskell_cbor() {
    let vectors: TestVectors = load_test_vectors("ed25519_vectors.json");

    for vector in vectors.vectors {
        // Compare Rust vs Haskell CBOR byte-for-byte
        assert_eq!(
            vector.rust_cbor.vk,
            vector.haskell_cbor.vk,
            "VK mismatch for {}: Rust != Haskell",
            vector.name
        );

        assert_eq!(
            vector.rust_cbor.sig,
            vector.haskell_cbor.sig,
            "Signature mismatch for {}: Rust != Haskell",
            vector.name
        );
    }
}
```

### Step 3: Implement Detailed Diff Reporting

For any mismatches, provide detailed analysis:

```rust
fn compare_cbor_detailed(rust: &str, haskell: &str, label: &str) {
    if rust != haskell {
        println!("\n=== CBOR MISMATCH: {} ===", label);
        println!("Rust length:    {} bytes", rust.len() / 2);
        println!("Haskell length: {} bytes", haskell.len() / 2);

        // Hex diff
        for (i, (r, h)) in rust.chars().zip(haskell.chars()).enumerate() {
            if r != h {
                println!("First diff at byte {}: Rust={} Haskell={}", i/2, r, h);
                break;
            }
        }

        // CBOR structure analysis
        analyze_cbor_structure(rust, "Rust");
        analyze_cbor_structure(haskell, "Haskell");
    }
}
```

### Step 4: Enable in CI

Update `.github/workflows/test.yml`:

```yaml
- name: Run cross-compatibility tests
  run: cargo test --test cross_compat --features serde,haskell-compat

- name: Verify Haskell compatibility
  run: cargo test test_.*_rust_vs_haskell_cbor --features haskell-compat
```

## Known Considerations

### Potential Compatibility Issues

1. **CBOR Canonicalization**
   - Haskell may use different ordering for maps
   - Ensure both use canonical CBOR (RFC 7049)

2. **Endianness**
   - Should not be an issue (CBOR is big-endian)
   - Verify on different architectures

3. **Padding/Alignment**
   - Raw crypto bytes should match exactly
   - CBOR encoding should be identical

4. **Version Differences**
   - Document exact Haskell cardano-base version used
   - Track compatibility across versions

### Debugging Workflow

If mismatches occur:

1. **Verify Input Consistency**
   - Confirm exact same seed/message/period used
   - Check hex encoding is correct

2. **Isolate the Issue**
   - Does VK match but signature differ?
   - Does raw crypto match but CBOR encoding differ?

3. **CBOR Deep Dive**
   - Decode CBOR to diagnostic notation
   - Compare structure element by element
   - Check major types, lengths, tags

4. **Consult Haskell Implementation**
   - Review Haskell CBOR instances
   - Check for custom encoding logic
   - Look for version-specific changes

## Timeline & Milestones

### Milestone 1: Infrastructure Complete (Current)

- ✅ 30 Rust test vectors generated
- ✅ JSON structure defined
- ✅ Test framework ready
- ✅ Documentation complete

### Milestone 2: Haskell Values Obtained

- ⏳ Contact Haskell maintainers OR
- ⏳ Generate using Haskell tooling
- ⏳ Validate Haskell values independently

### Milestone 3: Comparison Tests Implemented

- ⏳ Add Haskell values to JSON files
- ⏳ Implement comparison test functions
- ⏳ Run and debug any mismatches

### Milestone 4: CI Integration

- ⏳ Enable in continuous integration
- ⏳ Document compatibility status
- ⏳ Create versioned compatibility matrix

## Resources & References

### Haskell cardano-base

- **Repository:** <https://github.com/IntersectMBO/cardano-base>
- **Package:** cardano-crypto-class
- **Docs:** <https://cardano-crypto-class.readthedocs.io/>

### CBOR Specifications

- **RFC 7049:** CBOR - Concise Binary Object Representation
- **RFC 8949:** CBOR - Updated specification (2020)
- **Canonical CBOR:** Section 3.9 of RFC 7049

### Rust Implementation

- **Our Code:** `/workspaces/cardano-base-rust/cardano-crypto-class/`
- **Test Vectors:** `/workspaces/cardano-base-rust/cardano-crypto-class/tests/test_vectors/`
- **Test Suite:** `/workspaces/cardano-base-rust/cardano-crypto-class/tests/cross_compat.rs`

### Related Documentation

- **Phase 2 Report:** `CROSS_COMPAT_PHASE2_COMPLETE.md`
- **CBOR Implementation:** `CBOR_IMPLEMENTATION_REPORT.md`
- **Test Vector README:** `tests/test_vectors/README.md`

## Next Actions

### Immediate (Phase 3.1)

1. **Contact Haskell Team**
   - File issue on cardano-base repository
   - Request collaboration on test vectors
   - Share our test vector JSON files

2. **OR Setup Haskell Environment**
   - Install Stack/Cabal
   - Clone cardano-base
   - Build and test Haskell implementation

3. **Document Process**
   - Track which approach we use
   - Document any issues encountered
   - Share learnings with community

### Short-term (Phase 3.2)

1. **Implement Comparison Tests**
   - Add test functions for each algorithm
   - Implement detailed diff reporting
   - Create helper utilities

2. **Validate and Debug**
   - Run comparison tests
   - Investigate any mismatches
   - Fix or document differences

3. **CI Integration**
   - Add to GitHub Actions workflow
   - Document compatibility badge
   - Setup automatic validation

### Long-term

1. **Expand Test Coverage**
   - Add edge cases
   - Test boundary conditions
   - Stress test large inputs

2. **Version Compatibility**
   - Test across Haskell versions
   - Track breaking changes
   - Maintain compatibility matrix

3. **Performance Benchmarking**
   - Compare Rust vs Haskell performance
   - Optimize bottlenecks
   - Document performance characteristics

## Conclusion

Phase 3 infrastructure is now in place. The primary blocker is obtaining Haskell reference CBOR values. Once obtained, the comparison framework is ready for immediate use.

This phase is **critical** for establishing trust in the Rust implementation and ensuring seamless interoperability with existing Cardano Haskell infrastructure.

---

**Status Updates:**

- **October 4, 2025:** Phase 3 infrastructure and documentation complete
- **Next:** Awaiting Haskell reference values (Option 1 or 2)
