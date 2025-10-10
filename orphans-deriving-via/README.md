# orphans-deriving-via (Rust)

Rust shim that preserves the Haskell `orphans-deriving-via` package boundary. In
Haskell the package exposed orphan instances connecting `Cardano.Base.DerivingVia`
with `deepseq` and `nothunks`. Rust’s coherence rules prevent those instances
from living in a separate crate, so this crate re-exports the traits and helper
macros to keep the module layout familiar for downstream code.

## What’s inside

- Re-exports from `deepseq` (`NFData`, `deepseq`, `force`) and `nothunks`
  (`NoThunks`, `no_thunks`, `unsafe_no_thunks`) under a single crate.
- Prelude module that mirrors the Haskell module hierarchy and pulls in the
  deriving helpers from `base-deriving-via` (`InstantiatedAt`,
  `impl_generic_for_struct`).
- Unit tests demonstrating that the re-exported deriving helpers compose with
  both `NFData` and `NoThunks` via generic representations.

## Usage

Add the crate to your dependency graph alongside the other strictness utilities
and import the prelude to access the entire surface area:

```rust
use orphans_deriving_via::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Metrics {
    count: u64,
    name: String,
}

impl_generic_for_struct!(
    struct Metrics {
        count: u64,
        name: String,
    }
);

let metrics = InstantiatedAt::new(Metrics {
    count: 42,
    name: "ledger".to_owned(),
});

// `force` comes from `deepseq` and ensures the data hits normal form.
let forced = force(metrics.clone());
assert_eq!(forced.as_ref().count, 42);

// `unsafe_no_thunks` comes from `nothunks` and checks for latent laziness.
assert!(unsafe_no_thunks(&metrics).is_none());
```

## Haskell ↔ Rust mapping

| Haskell module/symbol | Rust counterpart |
|-----------------------|------------------|
| `Cardano.Orphans.DerivingVia` | `orphans_deriving_via` crate root |
| `Cardano.Orphans.DerivingVia.Prelude` | `orphans_deriving_via::prelude` |
| `Cardano.Base.DerivingVia.InstantiatedAt` | `base_deriving_via::InstantiatedAt` (re-exported) |
| `Cardano.Base.DerivingVia.implGenericForStruct` | `base_deriving_via::impl_generic_for_struct` |
| `Control.DeepSeq.NFData` | `deepseq::NFData` (re-exported) |
| `NoThunks.Class.NoThunks` | `nothunks::NoThunks` (re-exported) |

## Integration notes

- Keep `orphans-deriving-via` in sync with the versions of `deepseq`,
  `nothunks`, and `base-deriving-via` shipped in the workspace so the re-exports
  match exactly.
- The crate contains no code beyond re-exports and tests, so updates generally
  amount to dependency bumps or documentation refreshes.
- Rust coherence rules mean new implementations must land in the owning crate;
  update this shim to surface them through the prelude for consumers expecting
  the Haskell layout.

## Testing

Run the crate tests to verify the re-export wiring remains intact:

```bash
cargo test -p orphans-deriving-via
```

The suite validates that the generic deriving helpers integrate with
`NFData`/`NoThunks` and that the prelude exposes the expected API.

## License

Dual-licensed under Apache-2.0 or MIT. See [`LICENSE`](../LICENSE) and
[`NOTICE`](../NOTICE) for details.
