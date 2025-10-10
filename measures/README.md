# measures

Rust port of the Haskell [`Cardano.Measure`](https://github.com/IntersectMBO/cardano-base/tree/master/measures)
package. It provides lattice-based measurement combinators that underpin
chunking, budgeting, and resource accounting across the Cardano stack.

## Highlights

- **Commutative lattice traits** – `Measure` encodes the additive and
	lattice structure required by Haskell’s budgeting code, with blanket impls
	for unsigned integers and custom structs.
- **Bounded arithmetic** – `BoundedMeasure` adds a maximum sentinel so callers
	can short-circuit when a budget threshold is exceeded.
- **Iterator helpers** – `measure_split_at`, `measure_take`, and
	`measure_drop` carve iterators precisely where cumulative metrics would
	overflow, mirroring the Haskell API.
- **Multi-dimensional metrics** – tuple implementations cover arities 2–7 so
	callers can track bytes, witnesses, and other resources in lockstep.
- **Explicit overflow signalling** – panics with `MeasureOverflowError` when
	arithmetic can’t be represented, preserving the upstream semantics.

## Crate layout

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Public re-exports and helper constructors (e.g. `Natural`). |
| `src/measure.rs` | Trait definitions, tuple impls, iterator helpers, and tests. |

## Haskell ↔ Rust mapping

| Haskell artefact | Rust counterpart | Notes |
|------------------|-----------------|-------|
| `Cardano.Measure.Measure` | `measures::Measure` | Trait with `zero`, `plus`, `min_measure`, `max_measure`. |
| `Cardano.Measure.BoundedMeasure` | `measures::BoundedMeasure` | Adds `max_bound` and `is_within_bound`. |
| `MeasureOverflow` | `measures::MeasureOverflowError` | Raised via panic in overflow situations. |
| `splitAtMeasure` | `measures::measure_split_at` | Splits iterators at the first overflowing element. |
| `takeUntilMeasure` | `measures::measure_take` | Lazily yields a prefix that fits within the budget. |
| `dropUntilMeasure` | `measures::measure_drop` | Skips elements that would overflow the budget. |
| Tuple instances `(m1, …, m7)` | Blanket tuple implementations | Use Rust tuple arithmetic to mirror Haskell instances. |
| `Natural` newtype | `measures::Natural` | Wraps `u128` while keeping trait bounds explicit. |

## Getting started

```rust
use measures::{measure_split_at, Measure};

#[derive(Clone, Debug, PartialEq)]
struct BlockMeasure {
		bytes: usize,
		witnesses: usize,
}

impl Measure for BlockMeasure {
		fn zero() -> Self {
				BlockMeasure { bytes: 0, witnesses: 0 }
		}

		fn plus(&self, other: &Self) -> Self {
				BlockMeasure {
						bytes: self
								.bytes
								.checked_add(other.bytes)
								.expect("measure addition overflowed"),
						witnesses: self
								.witnesses
								.checked_add(other.witnesses)
								.expect("measure addition overflowed"),
				}
		}

		fn min_measure(&self, other: &Self) -> Self {
				BlockMeasure {
						bytes: self.bytes.min(other.bytes),
						witnesses: self.witnesses.min(other.witnesses),
				}
		}

		fn max_measure(&self, other: &Self) -> Self {
				BlockMeasure {
						bytes: self.bytes.max(other.bytes),
						witnesses: self.witnesses.max(other.witnesses),
				}
		}
}

let blocks = vec![
		BlockMeasure { bytes: 16_000, witnesses: 20 },
		BlockMeasure { bytes: 18_000, witnesses: 12 },
		BlockMeasure { bytes: 22_000, witnesses: 25 },
];

let limit = BlockMeasure { bytes: 36_000, witnesses: 40 };
let (fits, rest) = measure_split_at(|block| block.clone(), limit, blocks);
assert_eq!(fits.len(), 2);
assert_eq!(rest.len(), 1);
```

> ℹ️ Numeric measures are even simpler: the blanket implementations cover all
> standard unsigned integers and panic on overflow with `MeasureOverflowError`,
> matching Haskell’s strict budgeting behaviour.

## Integration notes

- Pair with `cardano-strict-containers` when building strict finger tree-based
	schedulers or chunkers—`Measure` bounds control tree rebalancing.
- Ledger-style consumers can wrap `Natural` to model `Coin`/`Lovelace`
	accounting without pulling in the heavier ledger crates.
- Iterator helpers return owned collections; convert them into streaming
	iterators by chaining `.into_iter()` if you need lazy evaluation in callers.

## Validation & diagnostics

Run the crate’s unit and property tests with:

```bash
cargo test -p measures
```

The suite covers overflow signalling, tuple arithmetic, iterator combinators,
and proptest-based checks that mirror the Haskell reference implementation. No
extra feature flags are required.

## License

Dual-licensed under Apache-2.0 or MIT. See the workspace [`LICENSE`](../LICENSE)
and [`NOTICE`](../NOTICE) files for details.
