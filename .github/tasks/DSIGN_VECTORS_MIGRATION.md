# DSIGN Test Vectors - Location Update

**Date:** October 6, 2025
**Action:** Moved DSIGN test vectors to dedicated test-vectors crate

---

## Summary

All DSIGN test vector JSON files have been moved from `cardano-crypto-class/test_vectors/` to the dedicated `cardano-test-vectors` crate, consolidating all test fixtures in a single location alongside the existing VRF vectors.

---

## Changes Made

### Files Moved

| File | From | To | Size |
|------|------|-----|------|
| `ed25519_test_vectors.json` | `cardano-crypto-class/test_vectors/` | `cardano-test-vectors/test_vectors/` | 1.3K |
| `ecdsa_secp256k1_test_vectors.json` | `cardano-crypto-class/test_vectors/` | `cardano-test-vectors/test_vectors/` | 4.9K |
| `schnorr_secp256k1_test_vectors.json` | `cardano-crypto-class/test_vectors/` | `cardano-test-vectors/test_vectors/` | 2.7K |

### Code Updates

**1. cardano-test-vectors/src/lib.rs**
- Added new `dsign` module parallel to existing `vrf` module
- Embedded all 3 DSIGN test vector JSON files at compile time
- Provided `get()` and `names()` convenience functions
- Total: ~50 new lines of code

**2. .github/scripts/generate_dsign_test_vectors.sh**
- Updated `OUTPUT_DIR` from `cardano-crypto-class/test_vectors` to `cardano-test-vectors/test_vectors`

**3. Documentation Updates**
- Updated `.github/tasks/PHASE_04_AUDIT.md` - file location references
- Updated `.github/tasks/PHASE_04_TEST_VECTOR_REPORT.md` - all 12 occurrences of old path
- Updated `.github/tasks/phase-04-dsign-parity.md` - reporting cadence notes

---

## New Directory Structure

```
cardano-test-vectors/
├── Cargo.toml
├── src/
│   └── lib.rs                                    # ← Updated with dsign module
└── test_vectors/
    ├── ed25519_test_vectors.json                # ← New
    ├── ecdsa_secp256k1_test_vectors.json        # ← New
    ├── schnorr_secp256k1_test_vectors.json      # ← New
    ├── vrf_ver03_generated_1                    # Existing VRF vectors
    ├── vrf_ver03_generated_2
    ├── vrf_ver03_generated_3
    ├── vrf_ver03_generated_4
    ├── vrf_ver03_standard_10
    ├── vrf_ver03_standard_11
    ├── vrf_ver03_standard_12
    ├── vrf_ver13_generated_1
    ├── vrf_ver13_generated_2
    ├── vrf_ver13_generated_3
    ├── vrf_ver13_generated_4
    ├── vrf_ver13_standard_10
    ├── vrf_ver13_standard_11
    └── vrf_ver13_standard_12
```

---

## Usage

### Accessing DSIGN Test Vectors

```rust
use cardano_test_vectors::dsign;

// Get all DSIGN test vector names
for name in dsign::names() {
    println!("Vector: {}", name);
}

// Get a specific test vector
if let Some(contents) = dsign::get("ed25519_test_vectors.json") {
    // Parse JSON and run tests
    let vectors: Ed25519TestVectors = serde_json::from_str(contents)?;
    // ... use vectors
}

// Iterate over all vectors
for vector in dsign::ALL {
    println!("Testing with: {}", vector.name);
    // Parse and test
}
```

### Accessing VRF Test Vectors (Existing)

```rust
use cardano_test_vectors::vrf;

// Same API as dsign module
for name in vrf::names() {
    println!("VRF Vector: {}", name);
}

if let Some(contents) = vrf::get("vrf_ver03_standard_10") {
    // Parse and use
}
```

---

## Benefits

1. **Centralized Test Fixtures**: All test vectors now in one crate
2. **Compile-time Embedding**: No runtime I/O needed to load test vectors
3. **Consistent API**: Same interface for VRF and DSIGN vectors
4. **Easier Maintenance**: Single location for all test vector updates
5. **Workspace Sharing**: Any crate can depend on `cardano-test-vectors` for fixtures
6. **No Duplication**: Test vectors aren't copied across multiple packages

---

## Verification

Build successful:
```bash
cd /workspaces/cardano-base-rust
cargo build -p cardano-test-vectors
# ✅ Compiling cardano-test-vectors v0.1.0
# ✅ Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
```

All files present:
```bash
ls -lh cardano-test-vectors/test_vectors/*.json
# ✅ ecdsa_secp256k1_test_vectors.json (4.9K)
# ✅ ed25519_test_vectors.json (1.3K)
# ✅ schnorr_secp256k1_test_vectors.json (2.7K)
```

---

## Next Steps

When implementing the DSIGN test harness, use the new location:

```rust
// In cardano-crypto-class/tests/dsign_test_vectors.rs

use cardano_test_vectors::dsign;
use serde_json;

#[test]
fn test_ed25519_vectors() {
    let vectors_json = dsign::get("ed25519_test_vectors.json")
        .expect("Ed25519 test vectors should exist");

    let vectors: Ed25519TestVectors = serde_json::from_str(vectors_json)
        .expect("Should parse Ed25519 test vectors");

    for vector in vectors.vectors {
        // Run test with vector
        test_ed25519_sign_verify(&vector);
    }
}
```

---

## Summary of Changes

- ✅ 3 JSON files moved to `cardano-test-vectors/test_vectors/`
- ✅ `cardano-test-vectors/src/lib.rs` updated with `dsign` module
- ✅ Test vector generation script updated
- ✅ All documentation updated (3 files)
- ✅ Build verified successful
- ✅ Ready for test harness implementation

---

**Migration Complete:** October 6, 2025
**Verification Status:** ✅ All checks passed
**Ready for:** Test harness implementation (Task #3)
