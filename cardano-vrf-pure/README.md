# cardano-vrf-pure

A pure-Rust implementation of Cardano’s Curve25519 VRF (Verifiable Random
Function) that achieves byte-for-byte parity with the production libsodium
reference. The crate mirrors the structure of
[`cardano-base`](https://github.com/IntersectMBO/cardano-base)/`cardano-vrf`
and provides the proving and verification primitives needed by the Cardano
consensus layer.

## Status

- ✅ **Phase 03 complete** – proofs and betas match the Cardano libsodium
  implementation exactly for both draft-03 and draft-13 vector suites.
- ✅ **Sign bit fix** – matches the reference behaviour by clearing the sign bit
  before Elligator2, eliminating discrepancies in gamma serialization.
- ✅ **Full vector coverage** – all 14 official vectors (`vrf_ver03_*` and
  `vrf_ver13_*`) pass with exact byte matches (see `tests/cardano_compat`).
- ✅ **Documentation** – refer to [`VRF_PARITY_COMPLETE.md`](VRF_PARITY_COMPLETE.md)
  for the detailed post-mortem, compatibility matrix, and lessons learned.

## Layout

```
cardano-vrf-pure/
├── src/cardano_compat/
│   ├── prove.rs    # VRF proof generation
│   ├── verify.rs   # VRF verification
│   ├── point.rs    # Hash-to-curve helpers and Edwards arithmetic
│   └── tests.rs    # Integration tests backed by official vectors
├── tests/
│   ├── haskell_vrf_cross_validation.rs  # Cross-checks with Haskell outputs
│   ├── debug_vrf_trace.rs               # Feature-gated diagnostics
│   └── performance.rs                   # Throughput smoke tests
├── benches/vrf_benchmark.rs             # Criterion benchmark harness
└── CHANGELOG.md
```

## Using the crate

```rust
use cardano_vrf_pure::cardano_compat::{prove, verify, Keypair};

let keypair = Keypair::generate_from_seed([0u8; 32]);
let input = b"slot leader schedule";

let proof = prove(&keypair, input).expect("VRF proof");
let verified = verify(&keypair.public, input, &proof).expect("beta");

println!("beta = {}", hex::encode(verified.beta()));
```

The `cardano_compat` module mirrors the Haskell API shape, including the suite
identifier (0x04) and proof layout expected by downstream Cardano services.

## Test suite

```bash
cargo test -p cardano-vrf-pure
```

The test suite exercises:

- The full set of draft-03 and draft-13 official vectors
- Hash-to-curve factorisation checks to ensure internal invariants hold
- Cross-validation against the Haskell test harness (requires the reference
  project; see `tests/haskell_vrf_cross_validation.rs`)
- Optional tracing when the `vrf-debug` feature flag and
  `CARDANO_VRF_DEBUG=1` are enabled

Performance smoke tests live in `tests/performance.rs` and run in about
4.5 seconds (1000 iterations per operation).

### Coverage snapshot

| Suite | Tests passing |
|-------|---------------|
| Unit (cardano_compat) | 40 |
| Integration | 3 |
| Performance | 2 |
| Official vectors | 14 |

The counts mirror the `Quick Reference` report that accompanied the Phase 03
completion and act as a quick regression checklist.

## Benchmarks

A Criterion benchmark harness is available:

```bash
cargo bench -p cardano-vrf-pure
```

Representative throughput (release mode, 1000 iterations):

| Operation | Avg time (µs) | Throughput |
|-----------|---------------|------------|
| Prove     | ~293          | ~3,400 ops/s |
| Verify    | ~365          | ~2,700 ops/s |
| Roundtrip | ~656          | ~1,500 ops/s |

These numbers are recorded in the VRF parity documentation and serve as a
regression baseline for future optimisations.

## Related documentation

- [`VRF_PARITY_COMPLETE.md`](VRF_PARITY_COMPLETE.md) – in-depth report covering
  the sign-bit bug, vector verification output, and compatibility matrix.
- [`CHANGELOG.md`](CHANGELOG.md) – milestone history and release notes.
- `.github/tasks/phase-03-vrf-parity.md` – phase tracker with acceptance
  criteria and verification checklist.

## Next steps

With parity achieved, subsequent milestones focus on:

1. Maintaining the shared debug/testing strategy used across the workspace
   (feature-gated logging, trace helpers, performance smoke tests).
2. Preparing formal releases (see `.github/RELEASE_v0.2.0_PREPARATION.md`).
3. Executing Phase 04 to bring DSIGN implementations to the same level of
detail.
