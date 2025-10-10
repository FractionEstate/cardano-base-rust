# deepseq (Rust)

Pure Rust port of the Haskell `Control.DeepSeq` package. Rust evaluates eagerly,
but keeping the `NFData` typeclass surface lets Cardano crates preserve the same
“fully evaluated” guarantees relied upon throughout the Haskell ecosystem.

## Highlights

- **`NFData`, `NFData1`, `NFData2` traits** – drop-in replacements covering the
  standard library collections (`Vec`, `VecDeque`, `HashMap`, `BTreeMap`, …),
  smart pointers (`Box`, `Rc`, `Arc`), owned strings/paths, and `Cow`.
- **Normal-form helpers** – `deepseq` and `force` evaluate a value before
  returning, matching the semantics of the Haskell combinators.
- **Generic deriving** – `rnf_via_generic` integrates with
  [`base-deriving-via`](../base-deriving-via/README.md) so Cardano types gain
  `NFData` without boilerplate.
- **Weak-head utilities** – `OnlyCheckWhnf` and `OnlyCheckWhnfNamed` preserve
  the upstream toggles for callers that only need weak-head normal form checks.
- **Zero-cost implementations** – compiled code inlines away after
  monomorphisation; there is no `unsafe` and no dynamic dispatch.

## Quick start

Add the crate as a dependency and derive `Generic` via `base-deriving-via` to pull in
`NFData` automatically:

```rust
use base_deriving_via::{impl_generic_for_struct, InstantiatedAt};
use deepseq::{force, rnf_via_generic, NFData};

#[derive(Clone)]
struct Config {
    network_magic: u32,
    label: String,
}

impl_generic_for_struct!(
    struct Config {
        network_magic: u32,
        label: String,
    }
);

// Provide NFData by delegating to the generic representation.
impl NFData for Config {
    fn rnf(&self) {
        rnf_via_generic(self);
    }
}

let cfg = InstantiatedAt::new(Config {
    network_magic: 764_824_073,
    label: "mainnet".into(),
});
let forced = force(cfg);
assert_eq!(forced.as_ref().label, "mainnet");
```

Any Cardano struct whose fields implement `NFData` inherits the trait automatically, so
ledger states, protocol configurations, and network payloads can be forced without custom
boilerplate.

## Generic deriving patterns

- Derive `Generic` via `impl_generic_for_struct!` and delegate to
  `rnf_via_generic` so future field additions remain covered.
- Combine `deepseq` with `nothunks::no_thunks` to emulate the Haskell audit
  pipeline: force evaluation first, then validate the absence of thunks.
- Wrap `OnlyCheckWhnf`/`OnlyCheckWhnfNamed` around values when porting code
  that intentionally avoids deep evaluation, documenting the parity choice even
  though Rust evaluates eagerly.

## Crate layout

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Trait definitions, blanket impls, and the `force`/`deepseq` helpers. |
| `src/generic.rs` | `rnf_via_generic` helper used by generic deriving macros. |
| `src/whnf.rs` | Weak-head wrappers mirroring the upstream constructors. |
| `tests/` | Regression tests covering blanket impls, WHNF helpers, and deriving integration. |

## Haskell → Rust mapping

| Haskell symbol | Rust counterpart |
|----------------|------------------|
| `Control.DeepSeq.NFData` | `deepseq::NFData` |
| `Control.DeepSeq.NFData1` | `deepseq::NFData1` |
| `Control.DeepSeq.NFData2` | `deepseq::NFData2` |
| `Control.DeepSeq.deepseq` | `deepseq::deepseq` |
| `Control.DeepSeq.force` | `deepseq::force` |
| `Control.DeepSeq.rnf` (generic deriving) | `deepseq::rnf_via_generic` + `base_deriving_via` |
| `OnlyCheckWhnf`, `OnlyCheckWhnfNamed` | `deepseq::OnlyCheckWhnf`, `deepseq::OnlyCheckWhnfNamed` |

## Testing

Run the unit test suite once a toolchain is installed:

```bash
cargo test -p deepseq
```

The tests cover the WHNF wrappers, generic deriving integration, and the blanket
implementations for collections and smart pointers.

## License

Dual-licensed under Apache-2.0 OR MIT, identical to the rest of the workspace. See
[`LICENSE`](../LICENSE) and [`NOTICE`](../NOTICE) in the repository root for details.
