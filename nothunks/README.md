# nothunks (Rust)

Pure Rust port of the Haskell `nothunks` package used across the Cardano stack
to assert that suspended computations (thunks) never leak into strict code
paths. Rust evaluates eagerly, yet keeping the `NoThunks` surface intact lets
the rewritten crates preserve the same invariants and diagnostic tooling as the
original ecosystem.

## Highlights

- **`NoThunks` trait** – mirrors the Haskell contract and ships blanket
  implementations for primitives, standard library collections, `Rc`/`Arc`, and
  Cardano-specific wrappers.
- **Diagnostic helpers** – `no_thunks`, `unsafe_no_thunks`, and
  `no_thunks_via_generic` propagate human-readable paths so callers know exactly
  which field still contains latent laziness.
- **Thunk metadata** – `ThunkInfo` carries the offending path, optional message,
  and source hints; `NoThunksResult` keeps the API ergonomic for downstream
  error conversions.
- **Generic deriving** – integrates with
  [`base-deriving-via`](../base-deriving-via/README.md) so Cardano structs gain
  `NoThunks` with zero boilerplate.
- **Weak-head wrappers** – `OnlyCheckWhnf` and `OnlyCheckWhnfNamed` preserve the
  upstream escape hatches where only weak-head checks are expected.

## Usage example

```rust
use base_deriving_via::{InstantiatedAt, impl_generic_for_struct};
use nothunks::{NoThunks, no_thunks, no_thunks_via_generic, ThunkInfo};

#[derive(Clone)]
struct LedgerState {
    tip_slot: u64,
    pending: Vec<String>,
}

impl_generic_for_struct!(
    struct LedgerState {
        tip_slot: u64,
        pending: Vec<String>,
    }
);

impl NoThunks for LedgerState {
    fn no_thunks(&self, context: &[&str]) -> Result<(), ThunkInfo> {
        no_thunks_via_generic(self, context)
    }
}

let state = InstantiatedAt::new(LedgerState { tip_slot: 42, pending: vec!["tx1".into()] });
no_thunks(&["ledger"], &state)?; // Ok: nothing lazily suspended
# Ok::<_, ThunkInfo>(())
```

Diagnostic contexts bubble up automatically when a thunk-like wrapper is
encountered, helping developers pinpoint the location where an unevaluated value
slipped through.

## Generic deriving patterns

- Use `impl_generic_for_struct!` (from `base-deriving-via`) to derive
  `Generic`, then delegate `NoThunks::no_thunks` to `no_thunks_via_generic` so
  future field additions stay covered automatically.
- Pair `OnlyCheckWhnf`/`OnlyCheckWhnfNamed` with values that were intentionally
  WHNF-only in the Haskell codebase to document parity decisions.
- Combine `deepseq::deepseq` with `nothunks::no_thunks` when porting audit
  pipelines: force evaluation first, then confirm the structure contains no
  lingering thunks.

## Integration notes

- `nothunks` underpins strictness assertions inside `cardano-strict-containers`
  and other crates. Most workspace types already implement `NoThunks`, so
  downstream code rarely needs bespoke impls.
- Diagnostics are string-based and cheap to clone, making the helpers safe to
  use in hot code paths during tests.
- Custom `NoThunks` impls should prepend field names to the provided `context`
  when recursing so reported paths line up with Haskell error messages (e.g.
  call `field.no_thunks(&["slot"])` inside the impl).

## Haskell → Rust mapping

| Haskell symbol | Rust counterpart |
|----------------|------------------|
| `NoThunks` | `nothunks::NoThunks` |
| `noThunks` | `nothunks::no_thunks` |
| `unsafeNoThunks` | `nothunks::unsafe_no_thunks` |
| `ThunkInfo` | `nothunks::ThunkInfo` |
| `OnlyCheckWhnf`, `OnlyCheckWhnfNamed` | `nothunks::OnlyCheckWhnf`, `nothunks::OnlyCheckWhnfNamed` |
| `noThunksInValues` via generics | `nothunks::no_thunks_via_generic` + `base-deriving-via` |

## Crate layout

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Trait definitions, diagnostics, blanket impls, and WHNF wrappers. |
| `src/generic.rs` | Helpers backing `no_thunks_via_generic`. |
| `tests/` | Regression coverage for diagnostics, WHNF wrappers, and collection impls. |

## Testing

Execute the unit tests once a Rust toolchain is present:

```bash
cargo test -p nothunks
```

The suite exercises diagnostic path propagation, WHNF helpers, blanket
implementations for collections, and integration with `base-deriving-via`.

## License

Dual-licensed under Apache-2.0 OR MIT, matching the rest of the workspace. See
[`LICENSE`](../LICENSE) and [`NOTICE`](../NOTICE) at the repository root.
