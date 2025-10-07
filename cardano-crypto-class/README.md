# cardano-crypto-class (Rust)

A pure-Rust port of the original `cardano-crypto-class` package. The crate now
covers deterministic seed handling, Ed25519 DSIGN (including the mlocked
variant), secure memory helpers, VRF plumbing, and the family of
Cardano-specific Key Evolving Signatures (KES).

## Highlights

- **Deterministic entropy**: `Seed`, `SeedRng`, and `run_with_seed` mirror the
  Haskell APIs for reproducible randomness, with helpers to split, expand, and
  consume seed material (including mlocked variants).
- **Secure memory and packing**: `PackedBytes`, `PinnedSizedBytes`,
  `MLockedSeed`, and libsodium-style allocators provide safe, zeroed buffers
  for sensitive material, together with zero-copy direct-serialise traits.
- **DSIGN**: Ed25519 (pure + mlocked) parity harnesses exercise RFC 8032 and
  Cardano fixtures housed in [`cardano-test-vectors`](../cardano-test-vectors).
- **KES**: `SingleKes`, `Sum{0-7}Kes`, and `CompactSum{0-7}Kes` share
  recursive verification-key reconstruction routines. Serde-gated fixtures keep
  CompactSum levels 1–7 in lock-step with the Haskell reference, and dedicated
  boundary tests assert expiry and tamper behaviour.
- **VRF plumbing**: The crate wires through Praos VRF primitives so higher
  layers can embed them without crossing FFI boundaries.

## DSIGN parity progress

| Algorithm | Status | Notes |
|-----------|--------|-------|
| **Ed25519** | ✅ RFC 8032 parity | Harness in `tests/dsign_ed25519_vectors.rs` exercises 11 scenarios, including official RFC vectors and Cardano-specific cases. Signatures and public keys match byte-for-byte. |
| **Ed25519 mlocked** | ✅ Functional parity | Mirrors Ed25519 behaviour with sensitive material kept in `MLockedSeed`. Shares the same serialization and verification logic. |
| **ECDSA secp256k1** | 🟡 Validation pending | Implementation uses `k256`/`ecdsa` with RFC 6979 nonces and low-s normalisation. JSON vectors live in `cardano-test-vectors/test_vectors/ecdsa_secp256k1_test_vectors.json`; harness work is queued in Phase 04. |
| **Schnorr secp256k1** | 🟡 Validation pending | Backed by `k256` Schnorr support. Test vectors (including error cases) are embedded in `cardano-test-vectors/test_vectors/schnorr_secp256k1_test_vectors.json`. |

## KES status snapshot

| Algorithm | Status | Notes |
|-----------|--------|-------|
| **SingleKes** | ✅ Vector + boundary | `tests/kes_single_vectors.rs` consumes JSON fixtures (serde-gated) while `tests/kes_boundary.rs` enforces expiry and out-of-range errors. Cross-language vectors are still to be captured. |
| **CompactSingleKes** | ✅ Vector + boundary | `tests/kes_compact_single_vectors.rs` validates embedded verification keys alongside boundary checks. |
| **Sum{0-7}Kes** | ✅ Vector harness | `tests/kes_sum_vectors.rs` walks Rust-generated fixtures to verify signing, verification, and evolution across all tracked periods. |
| **CompactSum{1-7}Kes** | ✅ Vector parity | Serde-gated fixtures in `tests/compact_sum_kes_vectors.rs` assert byte-for-byte signatures for levels 1–7, including evolution and tamper checks. |
| **Forward security** | ✅ Regression in place | `tests/kes_forward_security.rs` now walks every period for `Sum4Kes` and `CompactSum4Kes`, re-verifies historic signatures, rejects stale-period signing after each evolution, and asserts that rewind attempts fail with the expected errors. |

Key takeaways from the latest DSIGN audit:

- Dedicated JSON fixtures for Ed25519, ECDSA, and Schnorr now live in
  `cardano-test-vectors/test_vectors/`, and the generator script at
  `.github/scripts/generate_dsign_test_vectors.sh` can rebuild them from the
  Haskell reference.
- Pending work focuses on byte-for-byte CBOR parity validation, richer error
  case testing (invalid signatures/keys), and extending the harnesses to cover
  RFC 6979/BIP 340 vectors once the scaffolding is in place.

## Tests

Run the complete suite (unit tests, property tests, KES boundary checks, and DSIGN harnesses) with:

```bash
cargo test -p cardano-crypto-class
```

The Ed25519 harness (`tests/dsign_ed25519_vectors.rs`) exercises:

- Four Cardano reference vectors migrated into `cardano-test-vectors`
- Three RFC 8032 canonical vectors (empty, single-byte, multi-byte messages)
- Failure scenarios for mismatched messages and verification keys
- Serialization round-trips and large-message signing

Additional DSIGN harnesses for ECDSA and Schnorr will live alongside the
Ed25519 suite as Phase 04 progresses.

Serde-gated vector harnesses consume the shared JSON fixtures:

```bash
cargo test -p cardano-crypto-class --features serde --test kes_single_vectors
cargo test -p cardano-crypto-class --features serde --test kes_compact_single_vectors
cargo test -p cardano-crypto-class --features serde --test kes_sum_vectors
cargo test -p cardano-crypto-class --features serde --test compact_sum_kes_vectors
```

`tests/kes_boundary.rs` focuses on evolution edge cases: Single/CompactSingle
expiry, CompactSum period rollovers, tamper detection for verification key
reconstruction, and out-of-range signing behaviour. To run only the boundary
suite:

```bash
cargo test -p cardano-crypto-class --test kes_boundary
```

Forward-security behaviour is covered by:

```bash
cargo test -p cardano-crypto-class --test kes_forward_security
```

## Usage

### Deterministic entropy helpers

```rust
use cardano_crypto_class::{
  expand_seed,
  mk_seed_from_bytes,
  run_with_seed,
};
use sha2::Sha256;

let seed = mk_seed_from_bytes([0u8; 32]);
let (left, right) = expand_seed::<Sha256>(&seed);

let value = run_with_seed(left, |rng| {
  let mut buf = [0u8; 4];
  rng.fill_bytes_checked(&mut buf)?;
  Ok(u32::from_le_bytes(buf))
})
.expect("seeded RNG execution");

println!("deterministic value: {value}");

let bytes = cardano_crypto_class::decode_hex_string_or_panic!("0xdeadbeef", 4);
assert_eq!(bytes, b"\xde\xad\xbe\xef");

let lhs = cardano_crypto_class::pack_bytes::<8>(&[0xff; 8], 0);
let rhs = cardano_crypto_class::pack_bytes::<8>(&[0x0f; 8], 0);
let xored = cardano_crypto_class::xor_packed_bytes(&lhs, &rhs);
assert_eq!(xored.as_slice(), &[0xf0; 8]);
```

### Single-period KES lifecycle

```rust
use cardano_crypto_class::dsign::ed25519::Ed25519;
use cardano_crypto_class::kes::{KesAlgorithm, KesError, KesMError, SingleKes};

fn demo_single_kes() -> Result<(), KesMError> {
  let seed = vec![0u8; SingleKes::<Ed25519>::SEED_SIZE];
  let signing_key = SingleKes::<Ed25519>::gen_key_kes_from_seed_bytes(&seed)?;
  let verification_key = SingleKes::<Ed25519>::derive_verification_key(&signing_key)?;

  let message = b"boundary-check";
  let signature = SingleKes::<Ed25519>::sign_kes(&(), 0, message, &signing_key)?;
  SingleKes::<Ed25519>::verify_kes(&(), &verification_key, 0, message, &signature)?;

  // Attempting to sign beyond the single allowed period fails.
  assert!(matches!(
    SingleKes::<Ed25519>::sign_kes(&(), 1, message, &signing_key),
    Err(KesMError::Kes(KesError::PeriodOutOfRange { .. }))
  ));

  SingleKes::<Ed25519>::forget_signing_key_kes(signing_key);
  Ok(())
}

demo_single_kes().expect("single KES lifecycle");
```

> **Tip**: Enable the `serde` feature to access JSON vector helpers and fixtures
> shared with `cardano-test-vectors` when performing cross-language parity
> checks.
