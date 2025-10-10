# cardano-vrf-pure

A pure-Rust implementation of Cardano’s Curve25519 VRF (Verifiable Random
Function) that tracks the behaviour of the production libsodium reference bit
for bit. This crate mirrors the structure of
[`cardano-base`](https://github.com/IntersectMBO/cardano-base)/`cardano-vrf`
and exposes Cardano-compatible proving and verification helpers for the Praos
consensus layer.

## Highlights

- **Pure Rust, zero FFI** – implements the Curve25519 VRF directly on top of
  `curve25519-dalek`, eliminating the libsodium dependency while preserving
  byte-for-byte parity.
- **Draft-03 & Draft-13 support** – ships both protocol revisions under one API
  so legacy Byron-era fixtures and modern Praos blocks remain compatible.
- **Cardano compatibility layer** – the `cardano_compat` module reproduces the
  Haskell `Cardano.Crypto.VRF.Class` surface, including suite identifiers,
  proof layout, and beta extraction helpers.
- **Feature-gated diagnostics** – enable the `vrf-debug` Cargo feature (plus
  `CARDANO_VRF_DEBUG=1`) to trace Elligator2, hash-to-curve, and verification
  internals while keeping the default build silent.
- **Benchmarks and fixtures** – Criterion benches and the
  `cardano-test-vectors` crate provide repeatable regression coverage for both
  performance and correctness.

## Crate layout

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Re-exports draft implementations and Cardano compatibility helpers. |
| `src/draft03.rs` / `src/draft13.rs` | Spec-specific primitives that mirror the Haskell legacy and Praos modules. |
| `src/common.rs` | Shared scalar/point helpers, clamping, and cofactor clearing logic. |
| `src/cardano_compat/` | Cardano-facing API surface (`prove`, `verify`, hash-to-curve, debug hooks). |
| `tests/` | Official vector parity checks, debug traces, Haskell cross-validation, and performance smoke tests. |
| `benches/vrf_benchmark.rs` | Criterion harness recording throughput baselines. |

```
cardano-vrf-pure/
├── src/
│   ├── lib.rs
│   ├── common.rs
│   ├── draft03.rs
│   ├── draft13.rs
│   └── cardano_compat/
│       ├── prove.rs
│       ├── verify.rs
│       ├── point.rs
│       ├── montgomery.rs
│       ├── field.rs
│       ├── debug.rs
│       └── tests.rs
├── tests/
│   ├── haskell_vrf_cross_validation.rs
│   ├── debug_vrf_trace.rs
│   └── performance.rs
├── benches/vrf_benchmark.rs
└── CHANGELOG.md
```

## Haskell ↔ Rust mapping

| Haskell module / artefact | Rust counterpart | Notes |
|---------------------------|------------------|-------|
| `Cardano.Crypto.VRF.Class` | `cardano_vrf_pure::cardano_compat::{prove, verify, OutputVRF}` | Public API exposed to downstream crates; preserves suite `0x04` encoding. |
| `Cardano.Crypto.VRF.Simple` (draft-03) | `cardano_vrf_pure::draft03` | Legacy draft used by Byron-era components and historical fixtures. |
| `Cardano.Crypto.VRF.Praos` (draft-13) | `cardano_vrf_pure::draft13` | Praos-era specification that powers current mainnet leadership selection. |
| `cardano-crypto-tests/vrf_ver*` fixtures | `cardano_test_vectors::vrf::{ALL,get}` + `tests/cardano_compat/tests.rs` | The vector corpus is vendored via the `cardano-test-vectors` crate for hermetic regression runs. |
| `.github/tasks/phase-03-vrf-parity.md` | `VRF_PARITY_COMPLETE.md` | Rust parity post-mortem capturing verification evidence and benchmarks. |

## Getting started

```rust
use cardano_vrf_pure::cardano_compat::{prove, verify, Keypair};

let keypair = Keypair::generate_from_seed([0u8; 32]);
let message = b"slot leader schedule";

let proof = prove(&keypair, message).expect("VRF proof generation");
let beta = verify(&keypair.public, message, &proof).expect("VRF output");

println!("beta = {}", hex::encode(beta.beta()));
```

The Cardano compatibility layer follows the Haskell API closely: it accepts the
64-byte secret/public key bundle, emits the 80-byte proof (`pi`), and returns
the 64-byte VRF output (`beta`).

### Feature flags & diagnostics

- Enable `vrf-debug` to pull in the structured trace helpers from
  `cardano_compat::debug`.
- Set `CARDANO_VRF_DEBUG=1` to mirror the Haskell tooling’s conditional logging
  (e.g. `cargo test -p cardano-vrf-pure --features vrf-debug -- --nocapture`).
- The debug harness in `tests/debug_vrf_trace.rs` prints intermediate Elligator2
  state for the `vrf_ver03_generated_1` vector, aiding parity investigations.

## Validation & regeneration

### Test suite

```bash
cargo test -p cardano-vrf-pure
```

The suite covers:

- Draft-03 and draft-13 official vectors distributed with
  `cardano-test-vectors` (14 files, byte-for-byte comparison).
- Hash-to-curve factorisation checks to ensure gamma decomposition matches the
  Haskell reference implementation.
- A small cross-validation harness (`tests/haskell_vrf_cross_validation.rs`)
  that mirrors the historical libsodium fixtures.
- Performance smoke tests (`tests/performance.rs`) that execute a fixed quota of
  proofs and verifications to guard against accidental slowdowns.

### Re-running vector comparisons

1. Regenerate or update the fixtures in `cardano-test-vectors` (see that crate’s
   README for detailed instructions).
2. Run the parity suite above; the `include_str!` calls inside
   `cardano_compat::tests` automatically pull the updated JSON/flat files.
3. (Optional) Capture a debug trace with:

   ```bash
   CARDANO_VRF_DEBUG=1 cargo test -p cardano-vrf-pure --features vrf-debug -- test debug_vrf_trace
   ```

Any byte mismatch will surface in the tests with the offending vector name so
you can cross-reference the Haskell generator output.

## Benchmarks

```bash
cargo bench -p cardano-vrf-pure
```

Typical release-mode throughput on the workspace dev container (1000 iterations):

| Operation | Avg time (µs) | Throughput |
|-----------|---------------|------------|
| Prove     | ~293          | ~3,400 ops/s |
| Verify    | ~365          | ~2,700 ops/s |
| Roundtrip | ~656          | ~1,500 ops/s |

The benchmark harness shares its baselines with
[`VRF_PARITY_COMPLETE.md`](VRF_PARITY_COMPLETE.md); rerun it after touching
scalar arithmetic or curve operations to confirm no regressions.

## Coverage snapshot

| Suite | Tests passing |
|-------|---------------|
| Unit (cardano_compat) | 40 |
| Integration | 3 |
| Performance | 2 |
| Official vectors | 14 |

These counts mirror the quick-reference summary captured during the Phase 03
parity work and provide a ready checklist for regression triage.

## Related documentation

- [`VRF_PARITY_COMPLETE.md`](VRF_PARITY_COMPLETE.md) – parity report, benchmark
  history, and failure analysis for the sign-bit fix.
- [`CHANGELOG.md`](CHANGELOG.md) – release notes and parity milestones using the
  Keep a Changelog format.
- `.github/tasks/phase-03-vrf-parity.md` – project tracker detailing acceptance
  criteria and open follow-up tasks.

## Roadmap hooks

Parity is complete; outstanding roadmap items focus on:

1. Keeping diagnostics aligned with the workspace-wide tracing conventions.
2. Preparing the next release train alongside `cardano-crypto-class`.
3. Extending the performance harness with longer-running Criterion benchmarks
   once upstream profiling requirements are solidified.
