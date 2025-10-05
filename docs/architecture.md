# Workspace Architecture

This repository contains thirteen tightly scoped crates. The sections below describe each
crate, the public APIs it exports today, and where to look in the source tree.

## Cryptography Core

### `cardano-crypto-class`

Location: [`cardano-crypto-class/src`](../cardano-crypto-class/src)

This is the largest crate in the workspace and provides the primitives used by Cardano
consensus and cross-chain tooling.

- **DSIGN algorithms** (`dsign/`): implements Ed25519 (including mlocked variants for
  secure memory), ECDSA over secp256k1, and Schnorr secp256k1 bindings. Traits are defined
  in [`dsign/mod.rs`](../cardano-crypto-class/src/dsign/mod.rs) with algorithm-specific
  implementations under the same directory.
- **KES (Key Evolving Signatures)** (`kes/`): exposes `SingleKes`, `Sum{0-7}Kes`, and
  compact variants alongside Blake2b hash helpers. All behaviour hangs off the
  [`KesAlgorithm`](../cardano-crypto-class/src/kes/mod.rs) trait.
- **VRF wrappers** (`vrf/`): provides Praos VRF types plus helper wrappers for mock
  implementations. The high-level API lives in
  [`vrf/mod.rs`](../cardano-crypto-class/src/vrf/mod.rs) with Praos-specific code under
  `praos.rs` and `praos_batch.rs`.
- **Seed and secure memory utilities** (`seed.rs`, `mlocked_bytes.rs`,
  `mlocked_seed.rs`, `pinned_sized_bytes.rs`): zero-overhead wrappers around byte buffers
  that erase sensitive material on drop.
- **Direct serialisation helpers** (`direct_serialise.rs`): zero-copy readers/writers used
  across cryptographic keys.
- **Hashing utilities** (`hash.rs`): Blake2b plus SHA-2, SHA-3, Keccak, and RIPEMD-160
  helpers for cross-chain workflows.
- **Support code** (`util.rs`, `packed_bytes.rs`): big integer helpers and packed byte
  buffers mirroring the Haskell behaviour.

Tests: `cargo test -p cardano-crypto-class` currently executes 83 unit tests and 39
integration tests across the cryptographic surface (see the crate-level `tests/`
directory).

### `cardano-vrf-pure`

Location: [`cardano-vrf-pure/src`](../cardano-vrf-pure/src)

Implements Draft-03 and Draft-13 VRF suites directly on top of `curve25519-dalek`:

- [`draft03.rs`](../cardano-vrf-pure/src/draft03.rs) and
  [`draft13.rs`](../cardano-vrf-pure/src/draft13.rs) provide deterministic key generation,
  proof creation, and verification.
- [`common.rs`](../cardano-vrf-pure/src/common.rs) contains hash-to-curve helpers and
  shared math utilities.

The crate exposes `VrfDraft03` / `VrfDraft13` along with the `VrfError` enum.

## Data, Serialisation, and Time Primitives

### `cardano-binary`

Location: [`cardano-binary/src`](../cardano-binary/src)

Offers CBOR serialisation helpers with APIs mirroring the Haskell `cardano-binary`
package. See [`serialize.rs`](../cardano-binary/src/serialize.rs) for encoding helpers and
[`deserialize.rs`](../cardano-binary/src/deserialize.rs) for strict decoding and nested
CBOR support. Errors are reported through
[`BinaryError`](../cardano-binary/src/error.rs).

### `cardano-slotting`

Location: [`cardano-slotting/src`](../cardano-slotting/src)

Provides strongly typed wrappers around slots, epochs, relative time, and schedule
information. Key modules include:

- [`slot.rs`](../cardano-slotting/src/slot.rs): `SlotNo`, `EpochNo`, and `WithOrigin`
  helpers.
- [`time.rs`](../cardano-slotting/src/time.rs): conversions between wall-clock and Cardano
  slot timing.
- [`epoch_info`](../cardano-slotting/src/epoch_info): utilities for fixed and derived
  epoch schedules.
- [`block.rs`](../cardano-slotting/src/block.rs): simple block number wrapper.

### `cardano-base`

Location: [`cardano-base/src`](../cardano-base/src)

Currently supplies the `CardanoFeatureFlag` enum and parsing helpers used by downstream
components that need to toggle experimental protocol features.

## Supporting Libraries

### Strictness and Evaluation Utilities

- **`cardano-strict-containers`** — strict versions of `Maybe`, `Seq`, and finger trees.
  Modules live under [`cardano-strict-containers/src`](../cardano-strict-containers/src).
- **`deepseq`** — ports the Haskell `NFData` hierarchy. See
  [`deepseq/src/lib.rs`](../deepseq/src/lib.rs).
- **`nothunks`** — structural thunk detection wrappers (mirroring the Haskell
  `nothunks` package). Source at [`nothunks/src/lib.rs`](../nothunks/src/lib.rs).
- **`measures`** — size accounting helpers and the `Measure` trait, implemented in
  [`measures/src/measure.rs`](../measures/src/measure.rs).
- **`heapwords`** — heuristic memory accounting for various data structures, keeping parity
  with the Haskell implementation. Located at
  [`heapwords/src/lib.rs`](../heapwords/src/lib.rs).

### Generic Deriving Utilities

- **`base-deriving-via`** — provides `Generic`, `Semigroup`, and `Monoid` derivation
  helpers plus the `InstantiatedAt` wrapper (see
  [`base-deriving-via/src/lib.rs`](../base-deriving-via/src/lib.rs)).
- **`orphans-deriving-via`** — re-exports the `deepseq` / `nothunks` integration helpers so
  downstream code can depend on a single crate.

### Build & Metadata Helpers

- **`cardano-git-rev`** — exposes [`git_rev()`](../cardano-git-rev/src/lib.rs) to bundle the
  repo revision into binaries without shelling out at runtime.

## Tests at a Glance

Running `cargo test --workspace` (October 2025) executes roughly 190 unit and integration
checks. Notable suites:

- `cardano-crypto-class/tests/cross_algorithm_verification.rs` covers cross-chain signing
  workflows and hash checks.
- `cardano-crypto-class/tests/vrf_praos_vectors.rs` validates outputs against known test
  vectors under `test_vectors/`.
- `cardano-slotting/tests/slotting.rs` enforces arithmetic over slot numbers and epoch
  schedules.
- Strict container, `deepseq`, and `nothunks` crates contain comprehensive unit tests to
  guarantee compatibility with the original Haskell behaviour.

Refer back to this document whenever you add a new crate or expand responsibilities so the
map stays aligned with the code.
