# cardano-base-rust

A pure-Rust port of selected components from the Haskell
[`cardano-base`](https://github.com/IntersectMBO/cardano-base) project. The workspace
provides cryptographic primitives (VRF, KES, DSIGN), Cardano-specific slotting utilities,
CBOR serialisation helpers, and strictness libraries that underpin Rust services in the
Cardano ecosystem.

## Highlights

- **Cryptography**: Ed25519, Ed25519 (mlocked), ECDSA secp256k1, Schnorr secp256k1, and
  Praos VRF implementations in [`cardano-crypto-class`](cardano-crypto-class/src).
- **Key Evolving Signatures**: `SingleKes`, `Sum{0-7}Kes`, and compact variants with
  Blake2b hashing, serde-gated regression vectors for CompactSum levels 1–7, and
  boundary tests that enforce expiry and tamper resistance in
  [`cardano-crypto-class`](cardano-crypto-class/tests).
- **VRF reference implementation** ✅: Draft-03 and Draft-13 VRFs over Curve25519 in
  [`cardano-vrf-pure`](cardano-vrf-pure/src) with **byte-for-byte parity** to Cardano
  libsodium. See [`VRF_PARITY_COMPLETE.md`](cardano-vrf-pure/VRF_PARITY_COMPLETE.md).
- **CBOR tooling**: Strict CBOR serialisation with tag-24 helpers in
  [`cardano-binary`](cardano-binary/src).
- **Slotting primitives**: Epoch and time arithmetic in
  [`cardano-slotting`](cardano-slotting/src).
- **Strictness utilities**: Strict containers, `NoThunks`, `NFData`, and deriving helpers
  for predictable evaluation.

## Build and test

Prerequisites: Rust 1.70 or newer with `cargo`, installed via
[rustup](https://rustup.rs/).

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

The full test suite includes VRF and KES golden tests that depend on vectors in
[`test_vectors/`](test_vectors).

### Dev Container (VS Code)

The repository ships with `.devcontainer/devcontainer.json`, preloading the
`mcr.microsoft.com/devcontainers/base:bookworm` image and the official Rust feature.
To (re)build the environment in VS Code:

1. Install the **Dev Containers** extension.
2. Open the command palette and choose `Dev Containers: Rebuild and Reopen in Container`.
3. Wait for the Rust toolchain to install automatically (no manual `rustup` steps required).

The container bundles the same tool versions used in CI, ensuring repeatable
`cargo` builds and tests.

## Documentation

Authoritative documentation lives under [`docs/`](docs):

- [Architecture overview](docs/architecture.md)
- [Cryptography reference](docs/cryptography.md)
- [Development: testing checklist](docs/development/testing.md)
- [Development: release checklist](docs/development/releasing.md)

### Status tracking

Progress across parity phases is captured in the repository root:

- [Phase 04 – DSIGN recap](PHASE_04_DSIGN_QUICK_SUMMARY.md)
- [Phase 04 completion notes](PHASE_04_COMPLETION_REPORT.md)
- [Phase 05 – KES audit](PHASE_05_AUDIT.md)

Security and conduct policies remain at the repository root:
[`SECURITY.md`](SECURITY.md) and [`CODE-OF-CONDUCT.md`](CODE-OF-CONDUCT.md).

## Workspace crates

| Crate | Description |
|-------|-------------|
| [`cardano-crypto-class`](cardano-crypto-class/src) | Cryptographic primitives, secure memory, hashing |
| [`cardano-vrf-pure`](cardano-vrf-pure/src) | Curve25519 VRF implementations |
| [`cardano-binary`](cardano-binary/src) | CBOR serialisation helpers |
| [`cardano-slotting`](cardano-slotting/src) | Epoch and slot arithmetic |
| [`cardano-base`](cardano-base/src) | Feature-flag wiring |
| [`cardano-strict-containers`](cardano-strict-containers/src) | Strict container types |
| [`deepseq`](deepseq/src/lib.rs), [`nothunks`](nothunks/src/lib.rs) | Evaluation traits |
| [`measures`](measures/src/measure.rs), [`heapwords`](heapwords/src/lib.rs) | Measurement helpers |
| [`base-deriving-via`](base-deriving-via/src/lib.rs), [`orphans-deriving-via`](orphans-deriving-via/src/lib.rs) | Deriving utilities |
| [`cardano-git-rev`](cardano-git-rev/src/lib.rs) | Embeds build git revision |

See [docs/architecture.md](docs/architecture.md) for crate relationships and feature
flags.

## Contributing

Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) for workflow guidance. Pull requests
should:

- Update documentation alongside code changes where relevant.
- Pass `cargo fmt`, `cargo clippy`, and the full test suite shown above.
- Include new tests or vectors when introducing cryptographic behaviour changes.

Security-sensitive reports should follow [`SECURITY.md`](SECURITY.md).

## License

Dual-licensed under the Apache License, Version 2.0 and the MIT license. See
[`LICENSE`](LICENSE) and [`NOTICE`](NOTICE) for details.
