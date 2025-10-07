# cardano-base-rust

A pure-Rust port of selected components from the Haskell
[`cardano-base`](https://github.com/IntersectMBO/cardano-base) project. The workspace
provides cryptographic primitives (VRF, KES, DSIGN), Cardano-specific slotting utilities,
CBOR serialisation helpers, and strictness libraries that underpin Rust services in the
Cardano ecosystem.

## Highlights

- **Cryptography**: Ed25519 (standard and mlocked), ECDSA secp256k1, Schnorr secp256k1,
  and Praos VRF implementations in [`cardano-crypto-class`](cardano-crypto-class/README.md).
- **Key Evolving Signatures**: `SingleKes`, `Sum{0-7}Kes`, and compact variants with
  Blake2b hashing, shared structural inspectors, serde-gated regression vectors for
  CompactSum levels 1–7, and boundary tests that enforce expiry and tamper resistance in
  [`cardano-crypto-class`](cardano-crypto-class/tests).
- **VRF reference implementation** ✅: Draft-03 and Draft-13 VRFs over Curve25519 in
  [`cardano-vrf-pure`](cardano-vrf-pure/README.md) with **byte-for-byte parity** to Cardano
  libsodium. See [`VRF_PARITY_COMPLETE.md`](cardano-vrf-pure/VRF_PARITY_COMPLETE.md).
- **CBOR tooling**: Strict CBOR serialisation with tag-24 helpers in
  [`cardano-binary`](cardano-binary/README.md).
- **Slotting primitives**: Epoch and time arithmetic in
  [`cardano-slotting`](cardano-slotting/README.md).
- **Strictness utilities**: Strict containers, `NoThunks`, `NFData`, and deriving helpers
  for predictable evaluation across the [`cardano-strict-containers`](cardano-strict-containers/README.md),
  [`base-deriving-via`](base-deriving-via), and [`orphans-deriving-via`](orphans-deriving-via)
  crates.

## Build and test

Prerequisites: Rust 1.85 or newer with `cargo`, installed via
[rustup](https://rustup.rs/).

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

The full test suite includes VRF and KES golden tests that depend on vectors in
[`cardano-test-vectors/test_vectors`](cardano-test-vectors/test_vectors).

### Tooling

Refer to [docs/development/testing.md](docs/development/testing.md) for a curated list of
recommended commands, feature flags, and editor integrations. The repository’s dev
container configuration lives in the workspace-level `.devcontainer/` folder when required
for CI parity; if the folder is absent, follow the documented local setup instead.

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

Security reports should follow the process described in the Cardano engineering handbook
linked from [`CONTRIBUTING.md`](CONTRIBUTING.md); conduct expectations live in
[`CODE-OF-CONDUCT.md`](CODE-OF-CONDUCT.md).

## Workspace crates

| Crate | Description |
|-------|-------------|
| [`cardano-crypto-class`](cardano-crypto-class) | Cryptographic primitives, secure memory, hashing |
| [`cardano-vrf-pure`](cardano-vrf-pure) | Curve25519 VRF implementations |
| [`cardano-binary`](cardano-binary) | CBOR serialisation helpers |
| [`cardano-slotting`](cardano-slotting) | Epoch and slot arithmetic |
| [`cardano-base`](cardano-base) | Feature-flag wiring |
| [`cardano-test-vectors`](cardano-test-vectors) | Golden data consumed by crypto tests |
| [`cardano-strict-containers`](cardano-strict-containers) | Strict container types |
| [`deepseq`](deepseq/src/lib.rs), [`nothunks`](nothunks/src/lib.rs) | Evaluation traits |
| [`measures`](measures/src), [`heapwords`](heapwords/src) | Measurement helpers |
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

Security-sensitive reports should follow the Cardano engineering handbook guidance linked
from [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

Dual-licensed under the Apache License, Version 2.0 and the MIT license. See
[`LICENSE`](LICENSE) and [`NOTICE`](NOTICE) for details.
