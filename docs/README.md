# Documentation Overview

This workspace implements a pure-Rust subset of the Cardano `cardano-base` libraries. The
Markdown guides in this folder focus on the code that exists today and link directly to
relevant modules so the documentation can be kept honest.

## Available Guides

- [Architecture](./architecture.md) &mdash; high-level tour of every crate in the
  workspace and how they fit together.
- [Cryptography](./cryptography.md) &mdash; details for VRF, KES, DSIGN, secure memory,
  and hash utilities implemented in `cardano-crypto-class` and `cardano-vrf-pure`.
- [Development / Testing](./development/testing.md) &mdash; how to run checks, test
  suites, and style tooling locally.
- [Development / Releasing](./development/releasing.md) &mdash; expectations for version
  bumps, changelog discipline, and publishing crates.

## Quick Links

- Workspace source tree: [`Cargo.toml`](../Cargo.toml)
- Test vectors used by the VRF suites: [`test_vectors/`](../test_vectors)
- Security and conduct policies live at repository root: [`SECURITY.md`](../SECURITY.md),
  [`CODE-OF-CONDUCT.md`](../CODE-OF-CONDUCT.md)

## Keeping Docs Accurate

When adding new functionality:

1. Update or extend the relevant guide.
2. Cross-link new modules so future readers can trace documentation to code.
3. Re-run `cargo test --workspace` to confirm examples and feature descriptions still
   match reality.
4. Reference the documentation update in your pull request description.

If a document becomes stale or redundant, remove it in the same change that updates the
code. Historic planning notes from the Haskell migration now live in git history.
