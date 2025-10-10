# cardano-git-rev (Rust)

Rust port of the Haskell `Cardano.Git.Rev` helpers. The crate embeds the Git
revision associated with a build and exposes a single ergonomic API for
retrieving it at runtime, mirroring the behaviour relied upon by
`cardano-node`, `cardano-wallet`, and other downstream components.

## Why this crate exists

- **Stable symbol for Nix builds** – exports `_cardano_git_rev`, a 40-byte
	static buffer that Nix tooling patches post-build (identical layout to the
	Haskell library).
- **Uniform lookup order** – prefers the embedded value, then an override from
	the build script, and finally queries `git` at runtime. Every fallback emits
	the same warnings as the Haskell implementation.
- **Pure Rust implementation** – no `unsafe` or FFI usage; behaviour is
	covered by unit and integration tests.
- **Test hooks** – utilities for overriding the embedded revision or the
	runtime `git` command make failure scenarios easy to exercise.

## How revision discovery works

1. **Build script (`build.rs`)** – captures `CARDANO_GIT_REV` from the
	 environment or runs `git rev-parse --verify HEAD`. The result is sanitised
	 and written to `OUT_DIR/git-rev.txt`, then exported via the
	 `CARDANO_GIT_REV` compile-time environment variable.
2. **Embedded payload** – the compiled library/binary keeps the 40-byte static
	 symbol `_cardano_git_rev`. Nix builds patch this slot after the fact using
	 [`set-git-rev.hs`][set-git-rev.hs].
3. **Runtime API** – `cardano_git_rev::git_rev()` first checks the embedded
	 payload, then the build-script value, and finally attempts to invoke `git`.
	 All errors produce a single warning and fall back to the all-zero
	 placeholder (`0000…000`).

The ordering matches the original `cardano-git-rev` Haskell module so existing
deployment scripts remain untouched.

## Quick start

```rust
use cardano_git_rev::git_rev;

fn main() {
		let rev = git_rev();
		println!("running commit {rev}");
}
```

### Forcing a specific revision during CI

Set `CARDANO_GIT_REV` before invoking the build and the build script will use
it verbatim (provided it is a valid 40-character hex string):

```bash
export CARDANO_GIT_REV=$(git rev-parse --verify HEAD)
cargo build -p cardano-git-rev
```

### Verifying the patched symbol

After a Nix build completes, confirm the `_cardano_git_rev` payload matches the
expected hash:

```bash
nm -gU target/release/cardano-node | grep _cardano_git_rev
```

The emitted bytes are ASCII and can be compared directly against the Git
revision your CI stamped into the binary.

## Testing and diagnostics

```bash
cargo test -p cardano-git-rev
```

The suite covers the runtime fallback chain and the guard helpers that let
tests override the embedded revision or the `git` command. For targeted
scenarios, use:

- `set_embedded_revision_for_testing("<sha>")` – temporarily updates the
	embedded slot.
- `override_git_command_for_testing(|args| { … })` – replaces the runtime `git`
	invocation with a custom closure.

Warnings are rate-limited via `OnceLock` so users see the same single warning
as with the upstream Haskell library.

## Haskell ↔ Rust mapping

| Haskell symbol/module | Rust counterpart |
|-----------------------|------------------|
| `Cardano.Git.Rev.gitRev` | `cardano_git_rev::git_rev` |
| `_cardanoGitRev` symbol  | `_cardano_git_rev` static exported by this crate |
| `CARDANO_GIT_REV` Cabal flag | `CARDANO_GIT_REV` env read by `build.rs` |
| Runtime fallback via `git rev-parse` | `git_rev_runtime()` helper |
| `set-git-rev.hs` (Nix post-build) | unchanged – patches `_cardano_git_rev` |

## Troubleshooting

- **All-zero revision** – the binary was not patched. Ensure your pipeline ran
	[`set-git-rev.hs`][set-git-rev.hs] or exported a real `CARDANO_GIT_REV`
	during compilation.
- **Warning about invalid revision** – the supplied hash was not a 40-character
	hexadecimal string. Sanitisation forces it back to the zero placeholder to
	avoid leaking malformed values.
- **Runtime `git` invocation fails** – the crate emits a single warning and
	returns the placeholder. On systems without Git (e.g. stripped containers)
	set `CARDANO_GIT_REV` explicitly or rely on the patched symbol.

## License

Dual-licensed under Apache-2.0 OR MIT. See [`LICENSE`](../LICENSE) and
[`NOTICE`](../NOTICE) for details.

[set-git-rev.hs]: https://github.com/input-output-hk/iohk-nix/blob/master/overlays/haskell-nix-extra/utils/set-git-rev.hs
