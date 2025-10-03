# base-deriving-via (Rust)

This crate is a Rust translation of the original Haskell-only
`base-deriving-via` package from the Cardano code base. It provides a tiny
layer for expressing `Semigroup` and `Monoid` instances in terms of a
“generic” representation—much like `GHC.Generics`—and a convenience wrapper
[`InstantiatedAt`] that mirrors the `deriving via` pattern familiar to Haskell
users.

The implementation is intentionally conservative so it can serve as a starting
point for a wider migration of the repository toward Rust. Future work can
expand coverage (e.g. tuple structs, enums, or automatically generated
representations) as additional modules are ported.

Key features already available:

- Blanket [`Semigroup`](src/semigroup.rs) and [`Monoid`](src/semigroup.rs)
    implementations for tuples up to eight elements, matching the behaviour of
    the original generics-based derivations.
- Ready-made helpers to treat `core::time::Duration`, primitive numbers,
    strings, options, vectors, and fixed-size arrays as algebraic structures.

## Getting started

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

let total = InstantiatedAt::new(Counter {
    label: "alpha".into(),
    ticks: 2,
})
.combine(InstantiatedAt::new(Counter {
    label: "beta".into(),
    ticks: 5,
}))
.into_inner();

assert_eq!(total.label, "alphabeta");
assert_eq!(total.ticks, 7);
```

Run the test suite with:

```bash
cargo test
```

## Module mapping

The original Haskell modules translate to the following Rust modules:

| Haskell module | Rust equivalent |
| --- | --- |
| `Data.DerivingVia` | `src/instantiated_at.rs` (re-exported via `lib.rs`) |
| `Data.DerivingVia.GHC.Generics.Semigroup` | `src/semigroup.rs` together with `src/generic.rs` |
| `Data.DerivingVia.GHC.Generics.Monoid` | `src/semigroup.rs` and `src/generic.rs` |
| `Data/DerivingVia/GHC/Generics/Semigroup.hs` macro usage | `src/macros.rs` (`impl_generic_for_struct!`) |

## Migration checklist

1. **Integrate with the wider workspace:** add the crate to a top-level
    `Cargo.toml` workspace (or create one) so that other packages can depend on
    it.
2. **Wire up CI:** ensure your continuous-integration pipeline executes the
    Rust tests (e.g. by invoking `cargo test -p base-deriving-via`).
3. **Port consumers incrementally:** replace Haskell call sites that used the
    original package with Rust equivalents. Start with simple record types to
    validate the API.
4. **Extend macro coverage:** the current macro supports record structs. Add
    patterns for tuple structs or enums as your migration requires them.
5. **Audit trait coverage:** implement additional `Semigroup`/`Monoid`
    instances for domain-specific types once they are ported.
