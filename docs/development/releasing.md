# Development Guide: Releasing

The workspace does not yet publish crates on crates.io, but the following checklist keeps
releases predictable when we tag versions for downstream consumers.

## 1. Decide What Changes Ship

- Identify the crates touched since the last tag by inspecting `git log` or
  `git diff <last-tag>..HEAD`.
- For each crate, decide whether the changes warrant a patch, minor, or major bump.

## 2. Update Versions and Changelogs

- Edit the crate-specific `Cargo.toml` and bump the `version` field.
- Update the matching `CHANGELOG.md` entry. Each crate owns its changelog
  (see the root [`CHANGELOG.md`](../CHANGELOG.md) for pointers).
- Ensure dependent crates update their dependency requirement if they rely on the new API.

## 3. Run the Quality Gates

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Optional but recommended:

```bash
cargo doc --workspace --no-deps
cargo deny check
```

## 4. Tag the Release

- Commit the version bumps and changelog updates.
- Create an annotated tag, for example:

  ```bash
  git tag -a cardano-crypto-class-v0.2.0 -m "cardano-crypto-class v0.2.0"
  git push origin cardano-crypto-class-v0.2.0
  ```

- If you publish to crates.io, run `cargo publish` from the crate directory after the tag
  is pushed.

## 5. Document the Release

- Open a pull request summarising the changes and link to any new documentation pages.
- Update `docs/architecture.md` or `docs/cryptography.md` if the release introduced new
  components or behaviour.

Keeping this checklist in sync with actual releases makes it easier to onboard new
contributors and audit previous versions.
