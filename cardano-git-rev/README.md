# Cardano Git Rev

This crate provides a small Rust API for retrieving the git revision that was used to
compile `cardano-node` and other Cardano executables.

`cardano-node` supports building via both `nix` and `cabal`.

When building with `nix` the git executable and git metadata are not available so the
git revision is embedded as a series of 40 zeros during the build. After the nix build
is finished the executable is patched with the correct git SHA. The crate keeps this
behaviour intact by exporting a patch-friendly symbol named `_cardano_git_rev` whose
layout exactly matches the previous implementation.

At runtime you can obtain the revision through

```rust
let current_rev = cardano_git_rev::git_rev();
```

The lookup order mirrors the Haskell implementation:

1. Read the `_cardano_git_rev` symbol. Downstream tooling (for example
	[`set-git-rev.hs`][set-git-rev.hs]) patches this payload after the build completes.
2. Fall back to the build-script supplied `CARDANO_GIT_REV` environment variable
	when available (for local builds or when `git` metadata is present).
3. As a last resort, execute `git rev-parse --verify HEAD` at runtime and use the
	trimmed output when it looks like a valid 40-character SHA1.

If every step fails, the crate returns the all-zero placeholder and emits a single
warning to `stderr`, matching the behaviour of the upstream Haskell library. The
returned value preserves the letter casing reported by `git`.

See [set-git-rev.hs][set-git-rev.hs] for the patching step performed by nix builds.

[set-git-rev.hs]: https://github.com/input-output-hk/iohk-nix/blob/master/overlays/haskell-nix-extra/utils/set-git-rev.hs
