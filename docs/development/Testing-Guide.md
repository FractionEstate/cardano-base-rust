# Testing Guide

This document describes the testing strategy and how to run tests for the Cardano Base Rust project.

## Test Overview

The project has **148+ passing tests** covering all functionality migrated from Haskell.

### Test Coverage by Package

| Package | Library Tests | Integration Tests | Total |
|---------|--------------|-------------------|-------|
| cardano-vrf-pure | 9 | 0 | 9 |
| cardano-crypto-class | 53 | 2 | 55 |
| cardano-binary | 10 | 0 | 10 |
| cardano-slotting | 11 | 2 | 13 |
| cardano-strict-containers | 19 | 6 | 25 |
| deepseq | 4 | 0 | 4 |
| nothunks | 3 | 0 | 3 |
| heapwords | 7 | 0 | 7 |
| measures | 8 | 0 | 8 |
| Other packages | 14 | 0 | 14 |

## Running Tests

### All Tests
```bash
cargo test --workspace
```

### Specific Package
```bash
cargo test --package cardano-crypto-class
```

### VRF Tests (Critical)
```bash
# Pure Rust VRF internal tests
cargo test --package cardano-vrf-pure

# VRF integration tests
cargo test --package cardano-crypto-class --lib vrf

# VRF test vectors
cargo test --package cardano-crypto-class --test vrf_praos_vectors
```

### With Output
```bash
cargo test --workspace -- --nocapture
```

### Release Mode (Performance)
```bash
cargo test --workspace --release
```

## Test Categories

### 1. Cryptographic Correctness Tests

These verify the cryptographic implementations are correct:

```bash
# VRF prove/verify roundtrip
cargo test --package cardano-vrf-pure prove_verify_roundtrip

# Elligator2 hash-to-curve determinism
cargo test --package cardano-vrf-pure elligator2_deterministic

# Invalid proof rejection
cargo test --package cardano-vrf-pure verify_rejects_invalid
```

### 2. Test Vector Validation

VRF test vectors ensure compatibility:

```bash
# Draft-03 vectors (7 vectors)
cargo test --package cardano-crypto-class praos_vectors_match_reference

# Draft-13 vectors (7 vectors)
cargo test --package cardano-crypto-class praos_batch_vectors_match_reference
```

### 3. Serialization Tests

CBOR and binary serialization:

```bash
cargo test --package cardano-binary
```

### 4. Deep Evaluation Tests

NFData and thunk detection:

```bash
cargo test --package deepseq
cargo test --package nothunks
```

### 5. Container Tests

Strict containers and measures:

```bash
cargo test --package cardano-strict-containers
cargo test --package measures
```

## Continuous Integration

GitHub Actions runs all tests on every push:

```yaml
# .github/workflows/ci.yml
- name: Run tests
  run: cargo test --workspace --all-features
```

## Test Vector Regeneration

If you need to regenerate VRF test vectors (advanced):

1. The test vectors are in `cardano-crypto-class/test_vectors/`
2. Vectors were regenerated using the pure Rust VRF implementation
3. See [VRF Implementation](../migration/VRF-Implementation.md) for details

## Performance Benchmarks

Run benchmarks for performance-critical code:

```bash
cargo bench --package cardano-crypto-class
```

## Code Coverage

Generate code coverage report:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --workspace --out Html
```

## Troubleshooting

### Tests Fail After Update

1. Clean build artifacts:
   ```bash
   cargo clean
   ```

2. Update dependencies:
   ```bash
   cargo update
   ```

3. Rebuild and test:
   ```bash
   cargo build --workspace
   cargo test --workspace
   ```

### VRF Tests Fail

VRF tests are sensitive to cryptographic correctness:

1. Verify curve25519-dalek version: `4.1.x`
2. Check test vectors are not corrupted
3. Ensure no modifications to VRF implementation

### Slow Tests

Some tests are computation-intensive:

```bash
# Run in release mode for speed
cargo test --release --workspace
```

## Adding New Tests

### Unit Tests

Add to the module being tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        // Test code here
    }
}
```

### Integration Tests

Add to `tests/` directory:

```rust
// tests/integration_test.rs
use cardano_crypto_class::vrf::*;

#[test]
fn test_integration() {
    // Integration test code
}
```

### Test Vectors

Add to `test_vectors/` directory following the format:

```
vrf: PraosVRF
ver: ietfdraft03
ciphersuite: ECVRF-ED25519-SHA512-Elligator2
sk: <32-byte hex>
pk: <32-byte hex>
alpha: <message hex>
pi: <80-byte proof hex>
beta: <64-byte output hex>
```

## Test Quality Standards

All tests must:
- ✅ Pass on `cargo test --workspace`
- ✅ Have descriptive names
- ✅ Test one thing clearly
- ✅ Include failure cases
- ✅ Run quickly (< 1s per test)
- ✅ Be deterministic (no flaky tests)

## Related Documentation

- [VRF API](../api/VRF-API.md) - VRF usage examples
- [Migration Summary](../migration/Migration-Summary.md) - Test migration details
- [Contributing Guide](../contributing/CONTRIBUTING.md) - Code review process
