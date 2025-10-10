# cardano-crypto-class (Rust)

Pure-Rust implementations of the cryptographic building blocks that the Haskell
[`cardano-crypto-class`](https://github.com/IntersectMBO/cardano-base/tree/master/cardano-crypto-class)
package exports: deterministic seeds, DSIGN, KES, Praos VRFs, secure memory, and
the canonical Cardano hash suite. The crate keeps the public API aligned with the
original modules so downstream consumers can swap languages without losing
behavioural parity.

## Parity & provenance

- Mirrors the upstream `Cardano.Crypto.*` hierarchy one module at a time.
- Shares fixtures and generators with [`cardano-test-vectors`](../cardano-test-vectors/README.md)
  for byte-for-byte validation against Haskell outputs.
- Avoids `unsafe` code paths: direct-serialise helpers expose slice-based
  callbacks, secure memory uses Rust allocations with explicit zeroisation, and
  all algorithms run on stable Rust.

## Module map

| Rust module | Responsibility | Haskell references |
| --- | --- | --- |
| `seed` | Deterministic entropy (`Seed`, `SeedRng`, split/expand helpers) | `Cardano.Crypto.Seed`, `Cardano.Crypto.Random` |
| `packed_bytes`, `pinned_sized_bytes`, `mlocked_bytes`, `mlocked_seed` | Packed byte abstractions, pinned/locked buffers, libsodium-style allocators | `Cardano.Crypto.PackedBytes`, `Cardano.Crypto.PinnedSizedBytes`, `Cardano.Crypto.Libsodium.Memory`, `Cardano.Crypto.MLockedSeed` |
| `direct_serialise` | Zero-copy serialisation traits with size checking | `Cardano.Crypto.DirectSerialise` |
| `dsign` (`ed25519`, `ecdsa_secp256k1`, `schnorr_secp256k1`, …) | DSIGN algorithms, deterministic keygen/sign/verify, mlocked variants | `Cardano.Crypto.DSIGN.*` |
| `kes` (`single`, `compact_single`, `sum`, `compact_sum`) | Key Evolving Signatures and shared helpers | `Cardano.Crypto.KES.*` |
| `vrf` | Praos VRF certificate plumbing | `Cardano.Crypto.VRF.Praos` |
| `hash` | Blake2b, SHA-2/3, Keccak, RIPEMD160, Hash160 wrappers | `Cardano.Crypto.Hash`, `Cardano.Crypto.Hash.Keccak`, `Cardano.Crypto.Hash.RIPEMD160` |
| `util` | Helper utilities: hex decoding, randomness, CBOR-friendly slicing | `Cardano.Crypto.Util` |
| `ffi` | Sized pointer wrappers used by legacy C bindings | `Cardano.Crypto.FFI` |
| `mlocked_metrics`, `kes::metrics` (feature gated) | Diagnostics counters for secure memory and KES workloads | Haskell parity work tracked in Phase 05 notes |

## Key capabilities

### Deterministic entropy & utilities

- `Seed`, `SeedRng`, `expand_seed`, and `run_with_seed` reproduce Haskell’s
  deterministic RNG semantics. Nested splits and byte extraction behave
  identically across languages.
- `PackedBytes`, `PinnedSizedBytes`, and `MLockedSeed` guarantee alignment,
  zeroisation, and direct-serialise support for sensitive byte material.
- `util` exposes the familiar hex parsing helpers, big integer conversions, and
  `slice/splits_at` combinators.

### Hash suite

- Covers Blake2b-224/256/512, SHA-256/512, SHA3-256/512, Keccak-256,
  RIPEMD160, Hash160, and helper compositions. Unit tests lock digest sizes and
  stress every algorithm with 1 MiB patterned payloads.
- Regression harness `tests/hash_vectors.rs` consumes
  `cardano-test-vectors/test_vectors/hash_test_vectors.json`, which exercises
  boundary conditions (SHA-2 block 63/64/65, SHA3 rate 136/137, multi-block
  inputs) plus real-world composites (Bitcoin genesis header, Ethereum legacy
  transaction). Regenerate vectors via:

  ```bash
  cargo run -p cardano-test-vectors --bin generate_hash_vectors
  ```

- Distinguishes SHA3 vs Keccak padding (`0x06` vs `0x01`) and records the
  behaviour in tests so algorithms never alias.
- `hash::constant_time_eq` wraps `subtle::ConstantTimeEq` for side-channel-safe
  digest comparisons; mismatched lengths short-circuit with an error.

### DSIGN implementations

| Algorithm | Status | Notes |
| --- | --- | --- |
| Ed25519 | ✅ RFC 8032 parity harness (`tests/dsign_ed25519_vectors.rs`) exercises RFC vectors and Cardano fixtures. | Mirrors `Cardano.Crypto.DSIGN.Ed25519` including mlocked key support. |
| Ed25519 (mlocked) | ✅ Functional parity using `MLockedSeed`, sharing the same serialisation and verification paths. | |
| ECDSA secp256k1 | 🟡 Harness passing; cross-language review pending. | Deterministic RFC6979 nonces + low-`s` normalisation via `k256`; JSON fixtures in `cardano-test-vectors`. |
| Schnorr secp256k1 | 🟡 Harness passing; cross-language review pending. | BIP340-compatible; tests cover invalid encoding, tamper cases, and deterministic signing. |

All DSIGN modules surface `DsignAlgorithm` / `DsignMAlgorithm` traits, direct
serialise helpers, and sized key/signature introspection matching the Haskell
APIs.

### KES families

- `SingleKes`, `CompactSingleKes`, `Sum0–Sum7Kes`, and `CompactSum0–CompactSum7Kes`
  recreate the tree structures from `Cardano.Crypto.KES`. Shared helpers rebuild
  verification keys on demand, zeroise stale secrets, and enforce forward
  security (signing for old periods fails deterministically).
- Harness coverage:
  - `tests/kes_sum_vectors.rs`, `tests/compact_sum_kes_vectors.rs`, and
    `tests/kes_single_vectors.rs` validate serde-gated fixtures level-by-level.
  - `tests/kes_haskell_parity.rs` walks the hierarchical fixtures for byte-level
    parity with Haskell outputs.
  - `tests/kes_boundary.rs` and `tests/kes_forward_security.rs` lock expiry,
    tamper detection, and period evolution semantics.
- Performance benchmarks (`benches/kes_bench.rs`) track keygen/sign/verify
  throughput and serialized sizes for representative algorithms.

### Praos VRF plumbing

`vrf` wraps the Praos VRF primitives (certificate generation, verification,
output extraction) so consensus and networking layers can consume them without
FFI shims. Fixtures live alongside DSIGN / KES vectors in
`cardano-test-vectors`.

### Direct serialise & secure memory

- `direct_serialise` exposes safe buffers with compile-time size checks,
  mirroring `Cardano.Crypto.DirectSerialise`. No raw pointers are exposed.
- `mlocked_bytes` implements libsodium-style allocators (aligned, zeroed,
  fallible) together with `MLockedBytes`, `MLockedSizedBytes`, and helper
  functions (`copy_mem`, `zero_mem`). Feature `mlocked-metrics` tallies secure
  allocation counters.

### Feature-gated diagnostics

| Feature flag | Counters | Purpose |
| --- | --- | --- |
| `mlocked-metrics` | allocations, allocation_bytes, zeroizations, failed_locks | Observe secure memory lifecycles without leaking sensitive pointers. |
| `kes-metrics` | signing_keys, signing_key_bytes, signatures, signature_bytes, updates | Characterise KES workloads during benchmarks or stress tests. |

Features are off by default; snapshots return zeros when disabled.

## Usage examples

### Deterministic entropy

```rust
use cardano_crypto_class::{expand_seed, mk_seed_from_bytes, run_with_seed};
use sha2::Sha256;

let seed = mk_seed_from_bytes([0u8; 32]);
let (left, _right) = expand_seed::<Sha256>(&seed);

let value = run_with_seed(left, |rng| {
  let mut buf = [0u8; 4];
  rng.fill_bytes_checked(&mut buf)?;
  Ok(u32::from_le_bytes(buf))
}).expect("seeded RNG execution");

assert_eq!(value, 0u32); // Deterministic for the chosen seed
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

    assert!(matches!(
        SingleKes::<Ed25519>::sign_kes(&(), 1, message, &signing_key),
        Err(KesMError::Kes(KesError::PeriodOutOfRange { .. }))
    ));

    SingleKes::<Ed25519>::forget_signing_key_kes(signing_key);
    Ok(())
}

demo_single_kes().expect("single KES lifecycle");
```

> Tip: enable the `serde` feature when consuming JSON fixtures from
> `cardano-test-vectors` for cross-language parity checks.

## Testing

Run everything (unit tests, property tests, DSIGN/KES harnesses):

```bash
cargo test -p cardano-crypto-class
```

Targeted suites:

```bash
cargo test -p cardano-crypto-class --test hash_vectors
cargo test -p cardano-crypto-class --test kes_boundary
cargo test -p cardano-crypto-class --test kes_forward_security
cargo test -p cardano-crypto-class --features serde --test kes_haskell_parity
cargo test -p cardano-crypto-class --features serde --test dsign_ed25519_vectors
cargo test -p cardano-crypto-class --features serde --test dsign_ecdsa_secp256k1_vectors
cargo test -p cardano-crypto-class --features serde --test dsign_schnorr_secp256k1_vectors
```

Vector regeneration helpers in `cardano-test-vectors` keep fixtures fresh:

```bash
cargo run -p cardano-test-vectors --bin generate_hash_vectors
cargo run -p cardano-test-vectors --bin generate_kes_vectors
cargo run -p cardano-test-vectors --bin generate_dsign_test_vectors
```

When comparing to the Haskell reference, generate parity dumps from the
upstream repository and feed them into
`cargo run -p cardano-test-vectors --bin compare_hash_vectors <haskell.json>`.

## Benchmarks

Hash throughput (MB/s across payload sizes):

```bash
cargo bench -p cardano-crypto-class --bench hash_bench
```

KES evolution measurements and signature sizing:

```bash
cargo bench -p cardano-crypto-class --bench kes_bench
```

Criterion stores HTML/JSON reports under `target/criterion/`. Archive notable
runs in release notes or the workspace roadmap to track regressions.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE](./LICENSE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE](./LICENSE) or <http://opensource.org/licenses/MIT>)

at your option.
