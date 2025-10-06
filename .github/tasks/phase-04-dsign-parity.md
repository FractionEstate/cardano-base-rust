# Phase 04 – DSIGN Algorithm Parity

**Status:** ☐ Not started / ☐ In progress / ☐ Blocked / ☐ Completed  \
**Primary owners:** _Unassigned_ (claim by editing this file)  \
**Supporting crates:** `cardano-crypto-class`

---

## Objective

Bring the Rust DSIGN (Digital Signature) implementations to 100% functional parity with the reference Haskell implementations. This includes Ed25519, Ed25519 Extended, ECDSA Secp256k1, and Schnorr Secp256k1 signature schemes used in Cardano.

## Success Criteria

- `cardano-crypto-class` DSIGN tests pass for all supported algorithms
- Rust-generated signatures match byte-for-byte against Haskell reference for test vectors
- Key generation, signing, and verification match reference implementations
- Proper error handling for invalid keys and signatures
- Documentation clearly states verified compatibility guarantees

## Scope

### DSIGN Algorithms to Cover

1. **Ed25519** - Standard Ed25519 signatures (payment keys)
2. **Ed25519 Extended** - BIP32-HD extended Ed25519 (stake keys)
3. **ECDSA Secp256k1** - ECDSA signatures over secp256k1 curve
4. **Schnorr Secp256k1** - Schnorr signatures over secp256k1 curve
5. **Mock DSIGN** - Test-only implementation for development

### Out of Scope (Future Phases)

- KES (Key Evolving Signatures) - Phase 05
- Multi-signature schemes - Future consideration
- Hardware wallet integration - Future consideration

---

## Milestone Checklist

### 1. Audit and Analysis

- [ ] Compare Rust DSIGN implementations against Haskell reference
  - `Cardano.Crypto.DSIGN.Class` - Core DSIGN typeclass
  - `Cardano.Crypto.DSIGN.Ed25519` - Ed25519 implementation
  - `Cardano.Crypto.DSIGN.Ed25519Extended` - Extended Ed25519
  - `Cardano.Crypto.DSIGN.EcdsaSecp256k1` - ECDSA implementation
  - `Cardano.Crypto.DSIGN.SchnorrSecp256k1` - Schnorr implementation

- [ ] Document differences and missing features
- [ ] Identify test vectors for each algorithm
- [ ] Review security considerations and edge cases

### 2. Ed25519 Parity

- [ ] Verify key generation matches reference
  - Seed to secret key expansion
  - Public key derivation
  - Key serialization format

- [ ] Validate signing operations
  - Message hashing and nonce generation
  - Signature computation
  - Deterministic signatures (RFC 8032)

- [ ] Confirm verification logic
  - Signature validation
  - Batch verification if applicable
  - Error cases (invalid signatures, malformed keys)

- [ ] Test with official vectors
  - RFC 8032 test vectors
  - Cardano-specific test vectors
  - Edge cases (empty messages, max-length messages)

### 3. Ed25519 Extended Parity

- [ ] Implement BIP32-HD key derivation
  - Chain code handling
  - Parent to child key derivation
  - Hardened vs non-hardened derivation

- [ ] Extended key serialization
  - XPrv format (64-byte secret + 32-byte chain code)
  - XPub format (32-byte public + 32-byte chain code)

- [ ] Signing with extended keys
  - Proper scalar usage
  - Chain code in derivation paths

- [ ] Verification with extended keys
  - Public key extraction
  - Signature validation

### 4. ECDSA Secp256k1 Parity

- [ ] Key generation over secp256k1
  - Random scalar generation
  - Point multiplication for public keys
  - Compressed/uncompressed point formats

- [ ] ECDSA signing
  - RFC 6979 deterministic k generation
  - r, s signature components
  - Low-s normalization if required

- [ ] ECDSA verification
  - Signature validation algorithm
  - Public key recovery (if needed)
  - DER encoding/decoding

- [ ] Test vectors
  - Bitcoin/Ethereum test vectors (where compatible)
  - Cardano-specific vectors

### 5. Schnorr Secp256k1 Parity

- [ ] Schnorr signature generation
  - BIP 340 compliance (if applicable)
  - Nonce generation
  - Challenge hash computation

- [ ] Schnorr verification
  - Signature validation
  - Batch verification support

- [ ] Key aggregation (if in scope)
  - MuSig protocol support
  - Multi-signature verification

- [ ] Test vectors
  - BIP 340 test vectors
  - Cardano-specific vectors

### 6. Test Coverage and Validation

- [ ] Unit tests for each algorithm
  - Key generation tests
  - Signing tests
  - Verification tests
  - Roundtrip tests

- [ ] Integration tests
  - Cross-algorithm compatibility
  - Serialization/deserialization
  - Error handling

- [ ] Property-based tests
  - Sign/verify roundtrip always succeeds for valid keys
  - Invalid signatures always fail verification
  - Key derivation is deterministic

- [ ] Performance benchmarks
  - Key generation speed
  - Signing throughput
  - Verification throughput
  - Compare against reference implementations

### 7. Documentation and Release

- [ ] API documentation for all DSIGN types
- [ ] Migration guide from Haskell implementations
- [ ] Security considerations documented
- [ ] Example usage code
- [ ] Performance characteristics documented
- [ ] Update CHANGELOG with DSIGN parity milestone

---

## Verification Checklist

- [ ] `cargo fmt && cargo clippy --workspace --all-targets`
- [ ] `cargo test --workspace` - All tests passing
- [ ] `cargo test -p cardano-crypto-class dsign` - DSIGN-specific tests
- [ ] Cross-validation with Haskell outputs for each algorithm
- [ ] Performance benchmarks documented
- [ ] Security review completed

---

## Dependencies & References

### Haskell Source
- `cardano-base/cardano-crypto-class/src/Cardano/Crypto/DSIGN/*.hs`
- Test vectors: `cardano-crypto-tests/test_vectors/`

### Specifications
- **Ed25519**: RFC 8032 - Edwards-Curve Digital Signature Algorithm (EdDSA)
- **ECDSA**: SEC 1 v2.0 - Elliptic Curve Cryptography
- **Schnorr**: BIP 340 - Schnorr Signatures for secp256k1
- **BIP32**: Hierarchical Deterministic Wallets

### Rust Crates
- `ed25519-dalek` - Ed25519 signatures
- `k256` - secp256k1 curve operations
- `ecdsa` - ECDSA signatures
- `schnorr-fun` or similar - Schnorr signatures

---

## Risk Assessment

### High Priority Risks

1. **Key Derivation Differences**: BIP32-HD derivation must match Haskell exactly
   - Mitigation: Extensive cross-validation with test vectors

2. **Signature Determinism**: Must produce identical signatures for same inputs
   - Mitigation: Use RFC 6979 / RFC 8032 deterministic nonce generation

3. **Secp256k1 Point Encoding**: Compressed vs uncompressed formats
   - Mitigation: Clear documentation and tests for both formats

### Medium Priority Risks

1. **Performance**: Rust implementation should be competitive
   - Mitigation: Benchmark against reference, optimize hot paths

2. **Edge Cases**: Invalid keys, malformed signatures must be handled correctly
   - Mitigation: Comprehensive error testing

---

## Estimated Effort

- **Audit & Planning**: 1-2 days
- **Ed25519 Parity**: 2-3 days
- **Ed25519 Extended**: 3-4 days
- **ECDSA Secp256k1**: 2-3 days
- **Schnorr Secp256k1**: 2-3 days
- **Testing & Documentation**: 2-3 days
- **Total**: 12-18 days (approximately 2-3 weeks)

---

## Reporting Cadence

- Update the **Status** line and tick checkboxes as work progresses.
- Provide short status notes (date + bullet) under this section:
  - _TBD_: Phase 04 planning complete, ready to begin implementation.

---

## Related Work

- **Completed**: Phase 03 - VRF Parity (100% complete)
- **Upcoming**: Phase 05 - KES Implementation
- **Future**: Phase 06 - Multi-signature schemes

---

_This document lives at `.github/tasks/phase-04-dsign-parity.md`. Update it after every meaningful change._
