# cardano-crypto-class (Rust)

A work-in-progress pure-Rust port of the original `cardano-crypto-class`
package. The initial milestone reimplements the deterministic seed utilities
used across the Haskell codebase to seed key generation and pseudo-random
generators.

## Highlights

- `Seed` type mirrors the Haskell newtype, exposing helpers to construct,
  split, expand, and consume deterministic entropy.
- `SeedRng` provides a deterministic RNG implementing `rand_core::RngCore`
  and `CryptoRng`, making it easy to integrate with typical Rust APIs.
- `run_with_seed` executes closures with a seed-backed RNG, mirroring the
  behaviour of `runMonadRandomWithSeed`.
- `read_seed_from_system_entropy` fetches entropy directly from the operating
  system using `OsRng`.
- Utility helpers from `Cardano.Crypto.Util`: hex decoding with length checks,
  natural/word serialisation helpers, `SignableRepresentation`, and the
  `decode_hex_string_or_panic!` macro.
- `PackedBytes<N>` implements the packed-byte optimisations used throughout
  the Haskell library, including XOR helpers and safe packing/unpacking from
  byte slices.
- `PinnedSizedBytes<N>` provides pinned heap allocations with sized pointer
  helpers, mirroring `Cardano.Crypto.PinnedSizedBytes` for safe FFI access.
- Libsodium-style memory helpers expose runtime-sized mlocked buffers,
  allocators, and low-level `zero_mem`/`copy_mem` utilities, matching the
  ergonomics of `Cardano.Crypto.Libsodium.Memory`.
- `MLockedSeed<N>` stores sensitive seeds in `mlock`-backed memory, including
  zeroing helpers, random initialisation, and direct-serialise support.
- Direct serialisation helpers mirror `Cardano.Crypto.DirectSerialise`,
  providing zero-copy traits and size-checked buffer utilities for
  interacting with raw memory.
- Ed25519 DSIGN has been ported with both the pure and mlocked variants,
  including deterministic key generation from seeds, raw and direct
  serialisation helpers, and constant-time pinned or mlocked key storage. The
  Ed25519 harness now ships with RFC 8032 parity tests driven by fixtures in
  [`cardano-test-vectors`](../cardano-test-vectors).

## DSIGN parity progress

| Algorithm | Status | Notes |
|-----------|--------|-------|
| **Ed25519** | ✅ RFC 8032 parity | Harness in `tests/dsign_ed25519_vectors.rs` exercises 11 scenarios, including official RFC vectors and Cardano-specific cases. Signatures and public keys match byte-for-byte. |
| **Ed25519 mlocked** | ✅ Functional parity | Mirrors Ed25519 behaviour with sensitive material kept in `MLockedSeed`. Shares the same serialization and verification logic. |
| **ECDSA secp256k1** | 🟡 Validation pending | Implementation uses `k256`/`ecdsa` with RFC 6979 nonces and low-s normalisation. JSON vectors live in `cardano-test-vectors/test_vectors/ecdsa_secp256k1_test_vectors.json`; harness work is queued in Phase 04. |
| **Schnorr secp256k1** | 🟡 Validation pending | Backed by `k256` Schnorr support. Test vectors (including error cases) are embedded in `cardano-test-vectors/test_vectors/schnorr_secp256k1_test_vectors.json`. |

Key takeaways from the latest DSIGN audit:

- Dedicated JSON fixtures for Ed25519, ECDSA, and Schnorr now live in
  `cardano-test-vectors/test_vectors/`, and the generator script at
  `.github/scripts/generate_dsign_test_vectors.sh` can rebuild them from the
  Haskell reference.
- Pending work focuses on byte-for-byte CBOR parity validation, richer error
  case testing (invalid signatures/keys), and extending the harnesses to cover
  RFC 6979/BIP 340 vectors once the scaffolding is in place.

## Tests

Run the complete suite (unit tests, property tests, and DSIGN harnesses) with:

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

## Usage

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
})?;

println!("deterministic value: {value}");

let bytes = cardano_crypto_class::decode_hex_string_or_panic!("0xdeadbeef", 4);
assert_eq!(bytes, b"\xde\xad\xbe\xef");

let lhs = cardano_crypto_class::pack_bytes::<8>(&[0xff; 8], 0);
let rhs = cardano_crypto_class::pack_bytes::<8>(&[0x0f; 8], 0);
let xored = cardano_crypto_class::xor_packed_bytes(&lhs, &rhs);
assert_eq!(xored.as_slice(), &[0xf0; 8]);
```

> **Note**: Additional cryptographic suites (Ed448, VRF, KES, SECP256k1, etc.)
> are still provided by the original Haskell source tree. They will be ported
> incrementally.
