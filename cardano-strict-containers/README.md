# cardano-strict-containers

Rust port of the original Haskell `cardano-strict-containers` package. It
provides strict variants of common container abstractions that downstream
Cardano components expect, mirroring the original API surface as closely as
possible while embracing idiomatic Rust.

## Crate highlights

- `StrictMaybe<T>` – strict analogue of `Option<T>` with helper routines that
  match the Haskell API (`strict_maybe`, `from_s_maybe`, etc.).
- `StrictSeq<T>` – strict sequence backed by `VecDeque`, offering familiar
  combinators (`take_while`, `spanl`, `zip_with`, and more), seamless
  conversions from/to `Vec`, `VecDeque`, iterators, and `serde` support.
- `StrictFingerTree<V, A>` – minimal strict finger tree used for indexed
  structures. Includes measurement helpers (`add_measure`, `bin_measure`) and
  search/split operations tailored for Cardano’s ledger code.
- `force_elems_to_whnf` – identity helper that preserves the original
  evaluation semantics API even though Rust is strict by default.

## Usage

Add the crate to your workspace and import the modules you need:

```rust
use cardano_strict_containers::{StrictMaybe, StrictSeq, StrictFingerTree, add_measure};

let maybe = StrictMaybe::SJust(42);
let seq = StrictSeq::from_list([1, 2, 3]);
let tree = StrictFingerTree::<u64, _>::from_list(seq.iter().map(|x| Counted(*x)));
let measure = add_measure(&Counted(5), &0u64);
```

The exhaustive unit test suite documents the intended behaviour of each
container and can be used as executable examples.

## Development

Run the crate’s tests once a Rust toolchain is available:

```bash
cargo test -p cardano-strict-containers
```

At the time of writing the shared development container does not ship with a
Rust toolchain; install one (for example via `rustup`) before running the
command above.
