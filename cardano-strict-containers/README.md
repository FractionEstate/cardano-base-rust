# cardano-strict-containers (Rust)

Rust translation of the Haskell
[`cardano-strict-containers`](https://github.com/IntersectMBO/cardano-base/tree/master/cardano-strict-containers)
package. The crate supplies strict versions of the container abstractions that
Cardano ledger and networking components rely on.

## Highlights

- **`StrictMaybe<T>`** — strict optional values with familiar helpers
  (`strict_maybe`, `from_s_maybe`, conversions to/from `Option`).
- **`StrictSeq<T>`** — strict sequence backed by `VecDeque` with zipping,
  splitting, filtering, and `serde` support.
- **`StrictFingerTree<V, A>`** — ledger-oriented finger tree with measurement
  helpers (`add_measure`, `bin_measure`), structural views (`ViewL`, `ViewR`),
  and search/split utilities.
- **`force_elems_to_whnf`** — preserves the upstream API where laziness once
  mattered; it returns its input unchanged but documents evaluation intent.
- **Typeclass shims** — `Measured`, `Semigroup`, `Monoid`, `SearchResult`
  mirror the Haskell class hierarchy so ported code stays idiomatic.
- **Inter-crate integration** — blanket `NoThunks` implementations and
  heap-size helpers live in sibling crates (`nothunks`, `heapwords`), keeping
  the strict containers drop-in ready for auditing pipelines.

## Quick start

```rust
use cardano_strict_containers::{
  Measured, StrictFingerTree, StrictMaybe, StrictSeq, ViewL, force_elems_to_whnf,
};

#[derive(Clone, Debug, PartialEq)]
struct SizedBlock {
  bytes: usize,
}

impl Measured<usize> for SizedBlock {
  fn measure(&self) -> usize {
    self.bytes
  }
}

let backlog = StrictSeq::from_list([
  SizedBlock { bytes: 16_000 },
  SizedBlock { bytes: 8_000 },
  SizedBlock { bytes: 12_000 },
]);

// Finger tree search & split mirror the Haskell combinators.
let tree = StrictFingerTree::<usize, SizedBlock>::from_list(backlog.iter().cloned());
let (ready, queued) = tree.split(|total| *total >= 24_000);

assert_eq!(ready.len(), 2);
assert_eq!(queued.len(), 1);

// Views provide O(1) access without sacrificing strictness.
match ready.viewl() {
  ViewL::Cons(block, _) => assert_eq!(block.bytes, 16_000),
  ViewL::EmptyL => unreachable!(),
}

// StrictMaybe matches the upstream semantics 1:1.
let witnesses = StrictMaybe::s_just("KES key");
assert_eq!(witnesses.unwrap_or(""), "KES key");

// force_elems_to_whnf retains the legacy API where laziness once mattered.
let eager_backlog = force_elems_to_whnf(backlog.clone());
assert_eq!(eager_backlog.len(), backlog.len());
```

### Working with `StrictSeq`

`StrictSeq` implements `IntoIterator`, `FromIterator`, `serde::Serialize`, and
`serde::Deserialize`, making it a drop-in replacement for ledger data. A quick
example using JSON:

```rust
use cardano_strict_containers::StrictSeq;
use serde_json::{json, Value};

fn demo() -> serde_json::Result<()> {
  let seq = StrictSeq::from_list([1u32, 2, 3]);
  let encoded = serde_json::to_value(&seq)?;
  assert_eq!(encoded, json!([1, 2, 3]));

  let decoded: StrictSeq<u32> = serde_json::from_value(Value::from(vec![4, 5]))?;
  assert_eq!(decoded.len(), 2);

  Ok(())
}
```

## Haskell ↔ Rust mapping

| Haskell module/symbol | Rust equivalent |
|-----------------------|-----------------|
| `Cardano.Strict.Maybe.StrictMaybe` | `cardano_strict_containers::StrictMaybe` |
| `Cardano.Strict.Maybe.strictMaybe` | `cardano_strict_containers::strict_maybe` / `StrictMaybe::s_just` |
| `Cardano.Strict.Sequence.StrictSeq` | `cardano_strict_containers::StrictSeq` |
| `Cardano.Strict.Sequence.forceElemsToWHNF` | `cardano_strict_containers::force_elems_to_whnf` |
| `Data.FingerTree.StrictFingerTree` | `cardano_strict_containers::StrictFingerTree` |
| `Data.FingerTree.Measured` | `cardano_strict_containers::Measured` |
| `addMeasure` / `binMeasure` | `cardano_strict_containers::{add_measure, bin_measure}` |
| `Node`, `ViewL`, `ViewR`, `SearchResult` utilities | Same names under `cardano_strict_containers` |

## Integration notes

- All containers derive `serde::Serialize`/`Deserialize` and can be used with
  `cardano-binary` codecs out of the box.
- `StrictSeq` pairs cleanly with `nothunks` to assert thunk-free invariants via
  the blanket implementations provided there.
- The crate deliberately keeps trait bounds minimal and avoids `unsafe`, making
  it straightforward to audit.
- Feature flags: enable `serde` (default) to derive serialization. Disabling it
  mirrors the upstream “no serialization” build for constrained environments.

## Testing

Run the local test suite to exercise the strict semantics and parity helpers:

```bash
cargo test -p cardano-strict-containers
```

Unit tests cover the combinator surface, search/split routines, and conversions
backed by the Haskell reference behaviour.

## License

Dual-licensed under Apache-2.0 or MIT. See [`LICENSE`](../LICENSE) and
[`NOTICE`](../NOTICE) in the workspace root.
