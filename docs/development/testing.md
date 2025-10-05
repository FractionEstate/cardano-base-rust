# Development Guide: Testing & Tooling

These instructions cover the commands we run locally to validate the workspace. All
commands assume you are at the repository root.

## Prerequisites

- Rust toolchain 1.70 or newer (managed with [`rustup`](https://rustup.rs)).
- The `wasm32` target is **not** required; all crates compile natively.

## Test Suite

Run every unit and integration test across all crates:

```bash
cargo test --workspace
```

As of October 2025 this executes roughly 190 checks, including:

- 83 unit tests and 39 integration tests inside `cardano-crypto-class`
- VRF golden tests fed from `test_vectors/`
- Slotting arithmetic tests in `cardano-slotting`
- Strictness helpers in `cardano-strict-containers`, `deepseq`, and `nothunks`

To focus on a single crate:

```bash
cargo test -p cardano-crypto-class
cargo test -p cardano-slotting
```

Many integration tests live under `cardano-crypto-class/tests/`. Pass `-- --nocapture` if
you need to see diagnostic output while exploring failures.

## Formatting & Lints

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
```

Formatting uses the repository `rustfmt.toml`. Clippy warnings are elevated to errors so
new code lands clean.

## Documentation Builds

Generate Rustdoc for the entire workspace:

```bash
cargo doc --workspace --no-deps
```

Refer to the rendered docs at `target/doc/index.html`. Building documentation is
especially useful after modifying public APIs in `cardano-crypto-class` or
`cardano-vrf-pure`.

## Optional Quality Gates

- Dependency audit: `cargo deny check` (configuration lives in `deny.toml`).
- Security advisories: `cargo audit` (install via `cargo install cargo-audit`).
- Binary size comparison: use `cargo bloat` when optimising cryptographic code paths.

## Test Hygiene Checklist

1. Keep deterministic seeds and test vectors under `test_vectors/` to avoid drift.
2. Prefer integration tests for cross-crate behaviour (e.g. DSIGN + KES interactions).
3. When updating algorithms or byte formats, update the documentation in
   `docs/cryptography.md` alongside the tests.
