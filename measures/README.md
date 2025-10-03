# measures

Rust port of the Cardano `measures` package providing multidimensional measurement combinators.

## Overview

The crate exposes two core traits:

- `Measure`: describes how to combine and compare measurements in a lattice-ordered monoid.
- `BoundedMeasure`: extends `Measure` with a unique maximal bound.

Common numeric types implement both traits (for checked, saturating arithmetic), along with tuples up to size seven. Helper functions `measure_split_at`, `measure_take`, and `measure_drop` operate on iterators while respecting a measurement limit.

## Usage

Add the crate to your workspace and import the traits:

```rust
use measures::{Measure, measure_split_at};

let items = vec![2u32, 3, 5];
let (fits, rest) = measure_split_at(|x| *x, 5, items);
assert_eq!(fits, vec![2, 3]);
assert_eq!(rest, vec![5]);
```

Overflow during `Measure::plus` triggers a `MeasureOverflowError` panic.

## License

Licensed under the Apache License, Version 2.0.
