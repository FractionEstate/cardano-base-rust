# Cryptography Reference

This guide documents the cryptographic functionality that currently exists in the
workspace. Every section links to the exact Rust modules that implement the behaviour so
updates can stay grounded in code.

## Deterministic Seeds and Secure Memory

- [`seed.rs`](../cardano-crypto-class/src/seed.rs) defines the `Seed` abstraction used to
  deterministically derive key material. Helpers like `split_seed`, `expand_seed`, and
  `run_with_seed` are thin wrappers around byte slices with explicit error handling.
- [`mlocked_bytes.rs`](../cardano-crypto-class/src/mlocked_bytes.rs) and
  [`mlocked_seed.rs`](../cardano-crypto-class/src/mlocked_seed.rs) allocate sensitive
  buffers using the `mlock`-backed allocator and zero them on drop. All exported APIs
  return `MLockedBytes`/`MLockedSeed` wrappers instead of bare `Vec<u8>`.

Whenever you add a new signing key type that should participate in secure memory, ensure
the implementation mirrors these existing patterns.

## DSIGN Algorithms

The DSIGN family is defined in [`dsign/mod.rs`](../cardano-crypto-class/src/dsign/mod.rs).
It provides the `DsignAlgorithm` trait and helper functions such as `signed_dsign` and
`verify_signed_dsign` that operate on `SignableRepresentation` values from
[`util.rs`](../cardano-crypto-class/src/util.rs).

### Implementations

| Algorithm | Module | Notes |
|-----------|--------|-------|
| Ed25519 | [`dsign/ed25519.rs`](../cardano-crypto-class/src/dsign/ed25519.rs) | Uses `ed25519-dalek`, offers pinned byte wrappers plus direct-serialise support. |
| Ed25519 (mlocked) | [`dsign/ed25519_mlocked.rs`](../cardano-crypto-class/src/dsign/ed25519_mlocked.rs) | Stores signing keys inside `MLockedBytes` to avoid copying secrets. |
| ECDSA secp256k1 | [`dsign/ecdsa_secp256k1.rs`](../cardano-crypto-class/src/dsign/ecdsa_secp256k1.rs) | Wraps the upstream `secp256k1` crate; messages longer than 32 bytes are hashed with SHA-256 before signing. |
| Schnorr secp256k1 (BIP340) | [`dsign/schnorr_secp256k1.rs`](../cardano-crypto-class/src/dsign/schnorr_secp256k1.rs) | Provides x-only public keys and 64 byte signatures matching Taproot. |

Integration tests under [`cardano-crypto-class/tests`](../cardano-crypto-class/tests)
exercise serialisation, deterministic key generation, and cross-algorithm workflows across
these implementations.

## Key Evolving Signatures (KES)

The `KesAlgorithm` trait lives in [`kes/mod.rs`](../cardano-crypto-class/src/kes/mod.rs)
and exposes `sign_kes`, `verify_kes`, `update_kes`, and conveniences for calculating
expected byte sizes. Today the codebase supports:

- `SingleKes`: defined in [`kes/single.rs`](../cardano-crypto-class/src/kes/single.rs), a
  basic KES tree with two signatures.
- `SumKes` families: [`kes/sum.rs`](../cardano-crypto-class/src/kes/sum.rs) implements
  tiers `Sum0` through `Sum7` using Blake2b-256 for hashing.
- `CompactSumKes` variants: [`kes/compact_sum.rs`](../cardano-crypto-class/src/kes/compact_sum.rs)
  produce smaller signatures by folding verification keys.
- `CompactSingleKes`: [`kes/compact_single.rs`](../cardano-crypto-class/src/kes/compact_single.rs)
  provides an alternative representation optimised for storage.
- Hash helpers such as `Blake2b256`, `Blake2b512`, and
  `KesHashAlgorithm` live in [`kes/hash.rs`](../cardano-crypto-class/src/kes/hash.rs).

Tests worth consulting:

- [`kes_direct_serialise.rs`](../cardano-crypto-class/tests/kes_direct_serialise.rs) covers
  serialisation and deterministic key generation.
- [`sum_kes_unblocked.rs`](../cardano-crypto-class/tests/sum_kes_unblocked.rs) validates
  the tree evolution logic.

## Verifiable Random Functions (VRF)

Two layers implement VRF support:

1. **Low-level primitives** in the `cardano-vrf-pure` crate implement the actual
   curve25519 math.
   - [`draft03.rs`](../cardano-vrf-pure/src/draft03.rs) implements ECVRF-ED25519-SHA512-
     Elligator2 with 80-byte proofs.
   - [`draft13.rs`](../cardano-vrf-pure/src/draft13.rs) implements the batch-compatible
     ECVRF-ED25519-SHA512-TAI variant with 128-byte proofs.
   - Tests in [`cardano-vrf-pure/src`](../cardano-vrf-pure/src/lib.rs) verify proof
     determinism, invalid proof rejection, and cofactor clearing.
2. **High-level wrappers** in `cardano-crypto-class/vrf` provide the Praos-specific API
   expected by Cardano consensus. [`vrf/mod.rs`](../cardano-crypto-class/src/vrf/mod.rs)
   defines the `VRFAlgorithm` trait and helpers like `eval_certified`. Praos-specific
   types are in [`vrf/praos.rs`](../cardano-crypto-class/src/vrf/praos.rs) and the batch
   compatible layer sits in [`vrf/praos_batch.rs`](../cardano-crypto-class/src/vrf/praos_batch.rs).

Golden tests under [`vrf_praos_vectors.rs`](../cardano-crypto-class/tests/vrf_praos_vectors.rs)
compare outputs against the reference vectors stored in [`test_vectors/`](../test_vectors).

## Hash Functions

[`hash.rs`](../cardano-crypto-class/src/hash.rs) centralises the non-Blake2 hashing
support available to clients today:

- SHA-256 and SHA-512 (via `sha2`)
- Double SHA-256 (Bitcoin-style)
- SHA3-256, SHA3-512, Keccak-256 (via `sha3`)
- RIPEMD-160 and the composite `hash160`

Each helper returns a fixed-size array and has dedicated unit tests inside the same module
so that expected vectors stay inline with widely published constants.

## Direct Serialisation Helpers

`direct_serialise.rs` exposes `DirectSerialise`/`DirectDeserialise` traits used by key and
signature types to avoid intermediate allocations. Unit tests in
[`cardano-crypto-class/tests/direct_serialise_impls.rs`](../cardano-crypto-class/tests/direct_serialise_impls.rs)
exercise Ed25519 and Praos types to guarantee round-trips succeed.

## Adding New Algorithms

When introducing a new algorithm:

1. Start with the trait interfaces (`DsignAlgorithm`, `KesAlgorithm`, or `VRFAlgorithm`).
2. Provide a dedicated module mirroring the naming scheme above.
3. Add unit tests alongside the module and extend the relevant integration tests to cover
   cross-algorithm behaviour.
4. Update this document with links to the new module and a concise description of the API.
