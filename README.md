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
  Blake2b hashing and serialisation helpers.
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

## Documentation

Authoritative documentation lives under [`docs/`](docs):

- [Architecture overview](docs/architecture.md)
- [Cryptography reference](docs/cryptography.md)
- [Development: testing checklist](docs/development/testing.md)
- [Development: release checklist](docs/development/releasing.md)

### Code Audit and Compatibility

Comprehensive audit documentation comparing this Rust implementation with the official
Haskell cardano-base:

- [**Audit Report**](AUDIT_REPORT.md) - Detailed comparison and accuracy assessment
- [**Compatibility Matrix**](COMPATIBILITY_MATRIX.md) - Algorithm-by-algorithm compatibility
- [**Missing Features**](MISSING_FEATURES.md) - Tracking of unimplemented features
- [**Action Plan**](ACTION_PLAN.md) - Prioritized recommendations and timeline

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
