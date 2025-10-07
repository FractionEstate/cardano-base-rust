# cardano-test-vectors

A fixture crate that embeds the official Cardano VRF and DSIGN test vectors so
other crates in the workspace can access them without touching the filesystem.
All JSON artifacts are compiled into the binary, giving the tests a stable and
portable source of truth.

## Contents

```
cardano-test-vectors/
├── src/lib.rs                 # `vrf` and `dsign` modules with helper APIs
├── src/debug.rs               # Feature-gated logging helpers (ed25519-debug)
├── test_vectors/              # Embedded JSON fixtures
│   ├── ed25519_test_vectors.json
│   ├── ecdsa_secp256k1_test_vectors.json
│   ├── schnorr_secp256k1_test_vectors.json
│   ├── compact_sum_kes_test_vectors.json (CompactSumKES levels 1–7)
│   ├── sum_kes_period_evolution_vectors.json (SumKES full evolution sequences)
│   ├── compact_sum_kes_period_evolution_vectors.json (CompactSumKES full evolution sequences)
│   ├── vrf_ver03_standard_10 … vrf_ver03_standard_12
│   ├── vrf_ver13_* and generated_* series
│   └── bls12-381/
│       ├── bls_sig_aug_test_vectors
│       ├── ec_operations_test_vectors
│       ├── h2c_large_dst
│       ├── pairing_test_vectors
│       └── serde_test_vectors
└── tests/
    ├── debug_ed25519_trace.rs # Trace helper when ed25519-debug feature is set
    ├── performance.rs         # Signing throughput smoke test
    ├── dsign_ed25519_vectors.rs (crate tests import this)
    └── kes_vectors.rs         # Single/CompactSingle/Sum/CompactSum regression
```

### VRF vectors

The crate bundles the 14 official VRF fixtures published with Cardano’s
libsodium reference implementation:

- Draft-03 standard vectors (`vrf_ver03_standard_{10,11,12}`)
- Draft-03 generated vectors (`vrf_ver03_generated_{1-4}`)
- Draft-13 standard vectors (`vrf_ver13_standard_{10-12}`)
- Draft-13 generated vectors (`vrf_ver13_generated_{1-4}`)

All files are verbatim copies of the upstream JSON manifests and are available
through `cardano_test_vectors::vrf::{get,names,ALL}`.

### DSIGN vectors

Phase 04 DSIGN work migrated the signature fixtures from
`cardano-crypto-class/test_vectors` into this crate. The following files are
kept alongside the VRF set:

| File | Algorithms Covered | Notes |
|------|--------------------|-------|
| `ed25519_test_vectors.json` | Ed25519 deterministic signing | 4 sign/verify cases from Haskell reference, empty/max message coverage |
| `ecdsa_secp256k1_test_vectors.json` | Deterministic ECDSA (RFC 6979) | Sign/verify pairs, verify-only fixtures, negative-s rejection, malformed key lengths |
| `schnorr_secp256k1_test_vectors.json` | Schnorr secp256k1 | Sign/verify pairs plus failure cases for malformed signatures |

The Ed25519 vectors are also used for the RFC 8032 parity checks in
`cardano-crypto-class`.

### KES vectors

`generate_kes_vectors.rs` mirrors the hierarchy from
`Cardano.Crypto.KES.Sum`, emitting deterministic fixtures for:

- `single_kes_test_vectors.json` – SingleKES level 0 cases
- `compact_single_kes_test_vectors.json` – CompactSingleKES (embedded vk parity)
- `sum_kes_test_vectors.json` – SumKES levels 1–7
- `compact_sum_kes_test_vectors.json` – CompactSumKES levels 1–7, using the
    new recursive verification-key reconstruction to keep compact trees in
    lockstep with their SumKES counterparts
- `sum_kes_period_evolution_vectors.json` – SumKES evolution traces with every
    period recorded for deterministic regression checks
- `compact_sum_kes_period_evolution_vectors.json` – CompactSumKES counterpart
    to the evolution traces, guaranteeing compact verification parity

The top-level test `tests/kes_vectors.rs` consumes these files to assert
signature stability and to cross-check the period boundaries for every level.

### BLS12-381 vectors

To support the upcoming BLS12-381 pairing and signature work (see
`Cardano.Crypto.BLS12_381` in the Haskell repository) this crate now embeds the
official fixtures from `cardano-crypto-tests/bls12-381-test-vectors/test_vectors`:

- `bls_sig_aug_test_vectors` – augmented signature suites
- `ec_operations_test_vectors` – group arithmetic checks
- `h2c_large_dst` – hash-to-curve inputs with large domain separation tags
- `pairing_test_vectors` – bilinear pairing validation inputs
- `serde_test_vectors` – serialization/deserialization round-trips

The files are exposed via `cardano_test_vectors::bls12_381::{ALL,get,names}` so
consumers can access them without touching the filesystem.

## Using the crate

```rust
use cardano_test_vectors::{dsign, vrf};

// Iterate over all Ed25519 fixtures
for entry in dsign::ed25519::ALL {
    let raw = entry.contents();
    let vector: serde_json::Value = serde_json::from_str(raw)?;
    println!("{} => {} vectors", entry.name(), vector["vectors"].as_array().unwrap().len());
}

// Fetch a single VRF vector by name
if let Some(raw) = vrf::get("vrf_ver03_standard_10") {
    let parsed: serde_json::Value = serde_json::from_str(raw)?;
    // … run the test harness …
}
```

All helpers return `&'static str` so there is no runtime I/O. Consumers can
bring the module paths they need: `cardano_test_vectors::dsign::ed25519` or
`cardano_test_vectors::vrf`.

## Debugging support

Add the optional feature gate for verbose logging while developing DSIGN test
harnesses:

```bash
cargo test -p cardano-test-vectors --features ed25519-debug -- --nocapture
```

When the feature is enabled and `CARDANO_ED25519_DEBUG=1` is present in the
environment, bodies inside `cardano_test_vectors::debug::log` will emit their
messages. This mirrors the VRF crate’s diagnostics strategy so all cryptographic
modules share the same tooling.

## Generating new vectors

Regenerate the JSON files from the Haskell reference implementation with:

```bash
./.github/scripts/generate_dsign_test_vectors.sh
```

The script documents every step required to pull fixtures from
`cardano-crypto-tests` and writes them directly into
`cardano-test-vectors/test_vectors/`.

## Tests

```bash
cargo test -p cardano-test-vectors
```

The crate ships with lightweight regression tests:

- `debug_ed25519_trace.rs` – dumps trace output for the first Ed25519 vector,
  helpful when chasing parity issues
- `performance.rs` – measures signing/verification throughput (200 iterations)
- `dsign_ed25519_vectors.rs` – referenced by `cardano-crypto-class` for
  RFC 8032 parity checks
- `kes_vectors.rs` – consumes every Sum/CompactSum evolution fixture to ensure
    signature stability across levels and periods

**Latest validation:** `cargo test -p cardano-test-vectors` completed on
2025-10-14 after regenerating the KES evolution fixtures, confirming that the
embedded datasets remain in sync with the Haskell reference generators cited
throughout this README.

All suites pass under the default workspace toolchain.
