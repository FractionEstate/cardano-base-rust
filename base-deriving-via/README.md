# base-deriving-via (Rust)

Rust port of the Haskell `base-deriving-via` package. It offers a lightweight
way to derive `Semigroup`/`Monoid` implementations from a “generic” tuple
representation, mirroring the `deriving via` pattern used across the Cardano
code base.

## Highlights

- **`InstantiatedAt<T>` wrapper** — grants `Semigroup`/`Monoid` instances to
  your type once a compatible generic representation exists.
- **`Generic`, `GenericSemigroup`, `GenericMonoid` traits** — encode the
  mapping between the concrete type and its tuple-based representation.
- **`impl_generic_for_struct!` macro** — derives `Generic` automatically for
  common record and tuple structs without relying on `unsafe`.
- **Blanket tuple implementations** — tuples up to arity eight already provide
  `Semigroup`/`Monoid`, covering most ledger records out of the box.
- **Standard library support** — integers, durations, strings, vectors,
  arrays, and options ship with ready-made instances.
- **Pure Rust** — no macros beyond declarative `macro_rules!`, no runtime
  reflection, and no external dependencies.

## Quick start

```rust
use base_deriving_via::{
    impl_generic_for_struct, InstantiatedAt, Monoid, Semigroup,
};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Counter {
    label: String,
    ticks: i64,
}

impl_generic_for_struct!(struct Counter {
    label: String,
    ticks: i64,
});

let combined = InstantiatedAt::new(Counter {
    label: "alpha".into(),
    ticks: 2,
})
.combine(InstantiatedAt::new(Counter {
    label: "beta".into(),
    ticks: 5,
}))
.into_inner();

assert_eq!(combined.label, "alphabeta");
assert_eq!(combined.ticks, 7);
```

### Customising the representation

Generic derivation works for tuple structs as well. You can also hand-roll a
representation when the default tuple layout is insufficient:

```rust
use base_deriving_via::{Generic, GenericMonoid, InstantiatedAt, Monoid};

#[derive(Debug, Clone, PartialEq, Eq)]
struct RollingAverage {
    sum: i64,
    count: u32,
}

// Implement Generic manually to encode additional invariants.
impl Generic for RollingAverage {
    type Repr = (i64, u32);
  type ReprRef<'a> = (&'a i64, &'a u32) where Self: 'a;

    fn into_repr(self) -> Self::Repr {
        (self.sum, self.count)
    }

    fn from_repr(repr: Self::Repr) -> Self {
        RollingAverage {
            sum: repr.0,
            count: repr.1,
        }
    }

  fn as_repr(&self) -> Self::ReprRef<'_> {
    (&self.sum, &self.count)
  }
}

impl GenericMonoid for RollingAverage {}

let average = InstantiatedAt::new(RollingAverage { sum: 6, count: 2 })
    .combine(InstantiatedAt::new(RollingAverage { sum: 9, count: 3 }))
    .into_inner();

assert_eq!(average.sum, 15);
assert_eq!(average.count, 5);
```

## Haskell ↔ Rust mapping

| Haskell module/symbol | Rust counterpart |
|-----------------------|------------------|
| `Data.DerivingVia.InstantiatedAt` | `base_deriving_via::InstantiatedAt` |
| `Data.DerivingVia.Generic` | `base_deriving_via::Generic` |
| `Data.DerivingVia.GenericSemigroup` | `base_deriving_via::GenericSemigroup` |
| `Data.DerivingVia.GenericMonoid` | `base_deriving_via::GenericMonoid` |
| `implGenericForStruct` TH helper | `base_deriving_via::impl_generic_for_struct!` |
| Tuple `Semigroup`/`Monoid` instances | `base_deriving_via::semigroup` module |

## Integration notes

- Downstream crates (`deepseq`, `nothunks`, `orphans-deriving-via`) rely on the
  `InstantiatedAt` wrapper to bridge evaluation traits. Keep version alignment
  across the workspace to avoid API drift.
- The current macro supports record structs and tuple structs. If you need enum
  coverage, add a new macro pattern or implement `Generic` manually.
- The crate has no optional features; it is safe to depend on in `no_std`
  contexts that support heap allocations for `String`/`Vec`.

## Testing

Run the crate tests to validate the derivations:

```bash
cargo test -p base-deriving-via
```

The suite mirrors the Haskell QuickCheck coverage for `InstantiatedAt`, tuple
instances, and generic macro behaviour.

## License

Dual-licensed under Apache-2.0 or MIT. See [`LICENSE`](../LICENSE) and
[`NOTICE`](../NOTICE) in the workspace root.
