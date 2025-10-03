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

The function prefers the build-time value embedded by the build script. If that value
is still the all-zero placeholder, it attempts to call `git rev-parse --verify HEAD`
at runtime before finally falling back to the placeholder. Any trailing whitespace is
stripped and the result is always lower-case hexadecimal.

See [set-git-rev.hs][set-git-rev.hs] for the patching step performed by nix builds.

[set-git-rev.hs]: https://github.com/input-output-hk/iohk-nix/blob/master/overlays/haskell-nix-extra/utils/set-git-rev.hs
