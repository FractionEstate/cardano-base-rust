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
- **Hash utilities**: `hash::{blake2b224, blake2b256, sha256, sha3_256, keccak256, hash160, …}`
  expose the same primitives as `Cardano.Crypto.Hash`, now backed by an
  expanded JSON vector suite in `cardano-test-vectors` to lock byte-for-byte
  parity across boundary and multi-block inputs.

## Hash parity coverage

The `hash` module mirrors the helpers from `Cardano.Crypto.Hash` and is now
(`"hello world"`), SHA-2 block boundaries (63/64/65 bytes), SHA3-256 rate
backed by JSON fixtures that cover empty inputs, classic sanity checks

boundaries (136/137 bytes), single-byte edge cases, a 1024-byte
multi-block pattern, and real-world composites such as the Bitcoin genesis
block header/public key and a canonical go-ethereum legacy transaction. Run
the parity harness with:
```bash
cargo test -p cardano-crypto-class --test hash_vectors
```

`hash_vectors.rs` loads `cardano-test-vectors/test_vectors/hash_test_vectors.json`
and asserts SHA-256, double SHA-256, SHA-512, SHA3-256, SHA3-512, Keccak-256,
RIPEMD-160, Hash160, Blake2b-256, and Blake2b-512 digests for every case. The
fixture itself is regenerated via the companion helper in
`cardano-test-vectors`:

```bash
cargo run -p cardano-test-vectors --bin generate_hash_vectors
```

Future Phase 06 work will fold in cross-language confirmations from the
Haskell generator and streaming edge cases once the scaffolding lands.

Unit tests inside `hash.rs` also lock the compile-time digest sizes (including
the Blake2b re-exports), assert that the 256-bit variant is **not** a simple
truncation of the 512-bit digest (matching the Haskell parameterisation), and
exercise every algorithm against a 1 MiB patterned input to guarantee
deterministic behaviour without panics on large buffers.

`blake2b224` fills the final gap in the Cardano hashing surface: it mirrors
`Cardano.Crypto.Hash.Blake2b_224` and is used when hashing verification keys
for address derivation. Vector coverage now includes the 224-bit digest, and
unit tests assert that it is a distinct parameterisation rather than a
truncation of the 256/512-bit variants. This keeps the address pipeline (and
KES verification-key hashing helpers) aligned with the Haskell reference.

#### Keccak vs SHA3 parameterisation

- `Sha3_256` / `Sha3_512` use the NIST-standard domain separation suffix (`0x06`)
  and sponge capacity, matching `Cardano.Crypto.Hash.SHA3_*`.
- `Keccak256` intentionally preserves the legacy Ethereum-style padding (`0x01`)
  so hashes align with `Cardano.Crypto.Hash.Keccak_256`.
- Regression tests assert the digest mismatch between the pair for canonical
  messages (empty string, `"abc"`, and the Bitcoin transaction payload) to
  prove we are not aliasing the algorithms.
- The JSON fixtures exercise both variants across empty, boundary-sized, and
  multi-block inputs to keep the parameterisation locked in.

Regenerate the vectors with `cargo run -p cardano-test-vectors --bin generate_hash_vectors`
any time the digest backends change; the helper recomputes every digest with
the Rust implementations so the Keccak/SHA3 separation stays byte-for-byte
stable with the rest of the suite.

#### Composite helpers

`hash::double_sha256` and `hash::hash160` layer the primitive algorithms in the
same order as `Cardano.Crypto.Hash` (`SHA256(SHA256(x))` and
`RIPEMD160(SHA256(x))`). The fixtures stress these helpers against Bitcoin
headers, addresses, and Cardano inputs so downstream consumers can rely on the
exact byte layout without rederiving the composition logic.

#### Haskell reference regeneration

The original Haskell `cardano-crypto-class` exposes the same hashes via
`Cardano.Crypto.Hash`. To regenerate the JSON vectors from the reference code:

1. Clone `https://github.com/IntersectMBO/cardano-base` and enter the repo.
2. Save the following helper as `scripts/HashVectors.hs` (adjust the case list
   if new fixtures are added on the Rust side):

   ```haskell
   {-# LANGUAGE DataKinds #-}
   {-# LANGUAGE OverloadedStrings #-}
   {-# LANGUAGE ScopedTypeVariables #-}
   {-# LANGUAGE TypeApplications #-}

   module Main where

   import Cardano.Crypto.Hash
     ( Blake2b_256, Blake2b_512, HashAlgorithm, SHA256, SHA512
     , SHA3_256, SHA3_512, hashToBytes, hashWith
     )
   import Cardano.Crypto.Hash.Keccak (Keccak_256)
   import Cardano.Crypto.Hash.RIPEMD160 (RIPEMD160)
   import Data.Proxy (Proxy (..))
   import qualified Data.Aeson as Aeson
   import qualified Data.ByteString as BS
   import qualified Data.ByteString.Base16 as B16
   import qualified Data.ByteString.Lazy as LBS
   import qualified Data.ByteString.Char8 as BSC
   import qualified Data.Text as T
   import qualified Data.Text.Encoding as TE

   cases :: [(String, BS.ByteString)]
   cases =
     [ ("empty", BS.empty)
     , ("hello_ascii", BSC.pack "hello world")
     , ("short_sequence", BS.pack [0x00 .. 0x09])
     , ("single_byte_ff", BS.pack [0xff])
     , ("sha2_block_minus_one", BS.pack [0x00 .. 0x3e])
     , ("sha2_block_exact", BS.pack [0x00 .. 0x3f])
     , ("sha2_block_plus_one", BS.pack [0x00 .. 0x40])
     , ("sha3_rate_block", BS.pack [0x00 .. 0x87])
     , ("sha3_rate_plus_one", BS.pack [0x00 .. 0x88])
     , ("multi_block_1024", BS.pack (take 1024 (cycle [0x00 .. 0xff])))
     ]

   encodeHex :: BS.ByteString -> Aeson.Value
   encodeHex = Aeson.String . TE.decodeUtf8 . B16.encode

   digest :: forall h. HashAlgorithm h => Proxy h -> BS.ByteString -> BS.ByteString
   digest _ bytes = hashToBytes (hashWith @h id bytes)

   entry :: (String, BS.ByteString) -> Aeson.Value
   entry (name, bytes) =
     Aeson.object
       [ "name" Aeson..= name
       , "input_hex" Aeson..= encodeHex bytes
       , "sha256" Aeson..= encodeHex (digest (Proxy @SHA256) bytes)
       , "sha256d" Aeson..= encodeHex (digest (Proxy @SHA256) (digest (Proxy @SHA256) bytes))
       , "sha512" Aeson..= encodeHex (digest (Proxy @SHA512) bytes)
       , "sha3_256" Aeson..= encodeHex (digest (Proxy @SHA3_256) bytes)
       , "sha3_512" Aeson..= encodeHex (digest (Proxy @SHA3_512) bytes)
       , "keccak256" Aeson..= encodeHex (digest (Proxy @Keccak_256) bytes)
       , "ripemd160" Aeson..= encodeHex (digest (Proxy @RIPEMD160) bytes)
       , "hash160" Aeson..= encodeHex (digest (Proxy @RIPEMD160) (digest (Proxy @SHA256) bytes))
       , "blake2b256" Aeson..= encodeHex (digest (Proxy @Blake2b_256) bytes)
       , "blake2b512" Aeson..= encodeHex (digest (Proxy @Blake2b_512) bytes)
       ]

   metadata :: Aeson.Value
   metadata = Aeson.object
     [ "description" Aeson..= ("Haskell hash parity dump" :: String)
     , "generator" Aeson..= ("scripts/HashVectors.hs" :: String)
     , "note" Aeson..= ("Compare with Rust cardano-test-vectors" :: String)
     ]

   main :: IO () = do
     let payload = Aeson.object
           [ "metadata" Aeson..= metadata
           , "vectors" Aeson..= map entry cases
           ]
     LBS.putStr (Aeson.encode payload)
   ```
3. Run with `stack runghc scripts/HashVectors.hs > hash_vectors_haskell.json` (or Cabal
   equivalent), then compare the resulting JSON/hex digests with the Rust-generated
   `cardano-test-vectors/test_vectors/hash_test_vectors.json` using `jq` or a diff tool.
4. Run `cargo run -p cardano-test-vectors --bin compare_hash_vectors \
  hash_vectors_haskell.json` to produce a machine-readable diff; the command exits
  with a non-zero status when digests diverge.
5. Copy new digests into the Rust vectors if the Haskell reference reports updates.

This keeps the regeneration workflow traceable even before full automation lands.

#### Constant-time comparisons

All hash helpers surface raw byte arrays. Use `hash::constant_time_eq(lhs, rhs)` when comparing
digests derived from secret material so the check leverages `subtle::ConstantTimeEq` instead of a
branchy byte-by-byte loop. The helper rejects mismatched lengths up front and otherwise executes in
constant time, keeping the equality semantics aligned with the Haskell reference implementations.

### Haskell → Rust mapping (hashes)

| Haskell Symbol | Rust Equivalent |
|----------------|-----------------|
| `Cardano.Crypto.Hash` (module) | `hash` module in `cardano_crypto_class::hash` |
| `HashAlgorithm` | `hash::HashAlgorithm` trait |
| `Blake2b_224` | `hash::Blake2b224` / `hash::blake2b224` |
| `Blake2b_256`, `Blake2b_512` | `hash::Blake2b256`, `hash::Blake2b512` |
| `SHA256`, `SHA512` | `hash::Sha256`, `hash::Sha512` |
| `SHA3_256`, `SHA3_512` | `hash::Sha3_256`, `hash::Sha3_512` |
| `Keccak_256` | `hash::Keccak256` |
| `RIPEMD160` | `hash::Ripemd160` |
| `hashRaw`, `hashToBytes`, `hashFromBytes` | `hash::hash_raw`, `hash::hash_to_bytes`, `hash::hash_from_bytes` |
| Composite helpers (`doubleSHA256`, `hash160`) | `hash::double_sha256`, `hash::hash160` |

> The table mirrors the minimal surface needed for parity audits; additions should
> preserve the original naming to keep cross-referencing straightforward.

## DSIGN parity progress

| Algorithm | Status | Notes |
|-----------|--------|-------|
| **Ed25519** | ✅ RFC 8032 parity | Harness in `tests/dsign_ed25519_vectors.rs` exercises 11 scenarios, including official RFC vectors and Cardano-specific cases. Signatures and public keys match byte-for-byte. |
| **Ed25519 mlocked** | ✅ Functional parity | Mirrors Ed25519 behaviour with sensitive material kept in `MLockedSeed`. Shares the same serialization and verification logic. |
| **ECDSA secp256k1** | 🟡 Harness passing* | RFC 6979 deterministic nonces + low-s normalisation via `k256`. Vector harness (`tests/dsign_ecdsa_secp256k1_vectors.rs`) loads JSON fixtures and all tests pass locally. Formal cross-language parity review still pending. |
| **Schnorr secp256k1** | 🟡 Harness passing* | BIP340-style Schnorr implementation backed by `k256`. Vector harness (`tests/dsign_schnorr_secp256k1_vectors.rs`) exercises sign/verify and error cases; all tests pass locally. Formal cross-language parity review still pending. |

*Harness passing indicates the Rust implementation deterministically reproduces and validates the embedded fixtures; a separate cross-language audit against the Haskell reference (and/or upstream spec vectors) will promote the status to full parity (✅).

## KES status snapshot

| Algorithm | Status | Notes |
|-----------|--------|-------|
| **SingleKes** | ✅ Vector + boundary | `tests/kes_single_vectors.rs` consumes JSON fixtures (serde-gated) while `tests/kes_boundary.rs` enforces expiry and out-of-range errors. Cross-language vectors are still to be captured. |
| **CompactSingleKes** | ✅ Vector + boundary | `tests/kes_compact_single_vectors.rs` validates embedded verification keys alongside boundary checks. |
| **Sum{0-7}Kes** | ✅ Vector harness | `tests/kes_sum_vectors.rs` walks Rust-generated fixtures to verify signing, verification, and evolution across all tracked periods. |
| **CompactSum{1-7}Kes** | ✅ Vector parity | Serde-gated fixtures in `tests/compact_sum_kes_vectors.rs` assert byte-for-byte signatures for levels 1–7, including evolution and tamper checks. |
| **Forward security** | ✅ Regression in place | `tests/kes_forward_security.rs` now walks every period for `Sum4Kes` and `CompactSum4Kes`, re-verifies historic signatures, rejects stale-period signing after each evolution, and asserts that rewind attempts fail with the expected errors. |
| **Cross-language parity harness** | ✅ Unified | `tests/kes_haskell_parity.rs` loads the hierarchical JSON fixtures (Single / CompactSingle vectors and Sum / CompactSum levels 1–7) and asserts byte-for-byte verification key & signature parity while evolving keys across periods, mirroring the Haskell generator structure. |

The harness now also cross-checks the stored raw signature envelopes and ensures every fixture ships with a description, keeping the JSON metadata exercised instead of silently drifting.

### Forward security & period evolution

Forward security means that once a signing key advances to a later period it is
cryptographically infeasible to recover the ability to sign for any previous
period. This crate enforces the property by:

1. Deriving fresh descendant keys (or leaf DSIGN keys) from limited-use secret
  material.
2. Zeroizing intermediate secrets immediately after they become unnecessary for
  future signatures.
3. Routing verification strictly by period and reconstructing internal node
  hashes / verification keys rather than storing all of them persistently.

Evolution behaviour per family:

| Family | Periods | Evolution behaviour | Notes |
|--------|---------|---------------------|-------|
| Single / CompactSingle | 1 | Fixed at period 0 | Any other period rejected. |
| Sum n | 2^n | Binary tree: first half uses left child; after midpoint switch to right; discard internal secrets as soon as children derived | Mirrors Haskell `Cardano.Crypto.KES.Sum` |
| CompactSum n | 2^n | Same schedule; signatures carry off-path VK enabling reconstruction | Saves space vs Sum by embedding fewer VKs |

Historic signatures remain valid after each `update_kes` step; attempts to sign
for an earlier period or beyond `total_periods()` return `KesError::PeriodOutOfRange`
or `KesError::KeyExpired` respectively.

### Haskell → Rust mapping

| Haskell Module / Symbol | Rust Equivalent |
|-------------------------|-----------------|
| `Cardano.Crypto.KES.Class` | `kes::KesAlgorithm`, helpers in `kes::mod` |
| `hashVerKeyKES` | `KesAlgorithm::hash_verification_key_kes` |
| `Cardano.Crypto.KES.Single` | `kes::single::SingleKes` |
| `Cardano.Crypto.KES.CompactSingle` | `kes::compact_single::CompactSingleKes` |
| `Cardano.Crypto.KES.Sum` | `kes::sum::{Sum0Kes..Sum7Kes}` |
| `Cardano.Crypto.KES.CompactSum` | `kes::compact_sum::{CompactSum0Kes..CompactSum7Kes}` |
| `forgetSignKeyKES` | `KesAlgorithm::forget_signing_key_kes` |
| `updateKES` | `KesAlgorithm::update_kes` |
| `signKES` / `verifyKES` | `KesAlgorithm::sign_kes` / `verify_kes` |

> This table is intentionally kept minimal; additions should stay aligned with
> the original Haskell naming to ease audit and parity review.


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
# Unified parity harness (covers Single, CompactSingle, Sum1–7, CompactSum1–7):
cargo test -p cardano-crypto-class --test kes_haskell_parity
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

## Benchmarks

Hash throughput benchmarks exercise every helper (`sha256`, `sha256d`, `sha512`, `sha3_256`,
`sha3_512`, `keccak256`, `ripemd160`, `hash160`, `Blake2b256`, `Blake2b512`) across patterned
payloads of 32 bytes, 1 KiB, 64 KiB, and 1 MiB. Criterion reports MB/s per algorithm and input
size in `target/criterion` (HTML + JSON) so regressions can be tracked over time.

To flag regressions manually, archive the latest run by copying the `target/criterion/hash_bench`
directory (HTML + `benchmark.json`) into release notes or attaching it to the phase tracker. Future
runs can be diffed with `criterion compare baseline current` or simple JSON diffs to detect
throughput drops before formal automation lands.

```bash
cargo bench -p cardano-crypto-class --bench hash_bench
```

Experimental KES performance benchmarks are provided (dev-only) via Criterion:

```bash
cargo bench -p cardano-crypto-class --bench kes_bench
```

The harness measures:

- Key generation (`keygen`)
- Signing (`sign` for sampled periods)
- Verification (`verify` for the same sampled periods)
- Bounded evolution cycles (`evolve+sign`, capped at 16 periods per iteration)

Algorithms included: `SingleKes`, `Sum4Kes`, and `CompactSum4Kes` to give a
representative leaf, mid-level tree, and compact branching comparison. Period
sampling (first/last windows) keeps runtime short while still exercising edge
periods.

Results (HTML + stdout summaries) form the initial baseline; future optimisations
should update both the README and CHANGELOG with notable improvements or changes
in asymptotic behaviour.

The `serialized_sizes` benchmark function prints raw byte lengths for verification keys
and signatures (plus total periods) to establish a reproducible size baseline without
platform-specific RSS sampling. (Signing key raw serialization is intentionally excluded
because exposing it publicly would require an "unsound" test-only trait not implemented
for production benchmarks.)

### Feature-gated metrics

Two lightweight, allocation-free instrumentation features exist to support diagnostics:

| Feature | Module | Counters | Overhead | Use Case |
|---------|--------|----------|----------|----------|
| `kes-metrics` | `kes::metrics` | signing_keys, signing_key_bytes, signatures, signature_bytes, updates | Single relaxed atomics per event | Benchmarking KES workload composition and mix |
| `mlocked-metrics` | `mlocked_metrics` | allocations, allocation_bytes, zeroizations, failed_locks | Single relaxed atomics per event | Observing secure memory allocation patterns |

Fetch a metrics snapshot (returns zeros when feature disabled):

```rust
#[cfg(feature = "mlocked-metrics")]
{
  let snap = cardano_crypto_class::mlocked_metrics::snapshot();
  eprintln!("allocations={} bytes={} zeroizations={}", snap.allocations, snap.allocation_bytes, snap.zeroizations);
}
```

Enable both in a one-off diagnostics run:

```bash
cargo test -p cardano-crypto-class --features kes-metrics,mlocked-metrics,serde -- --ignored
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

> **Security note**: The `mlocked-metrics` feature reports aggregate counters only; it never exposes raw pointers or secret material sizes beyond total bytes allocated. Disable the feature for production builds.
