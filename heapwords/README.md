# heapwords (Rust)

Pure Rust port of the Haskell `Cardano.HeapWords` helpers. The crate estimates
heap usage for Cardano data structures using the same heuristics as the
upstream library: values are measured in **machine words** (8 bytes on 64-bit
targets) and composite structures add the cost of their constituents.

## Highlights

- **`HeapWords` trait** – blanket implementations cover primitives, slices,
  vectors, maps, sets, smart pointers, `BigInt`/`BigUint`, time wrappers, and
  Cardano-specific types.
- **Constructor helpers** – `heap_words0` … `heap_words9` mirror the Haskell
  combinators so algebraic data types can be measured without bespoke code.
- **Reporting utilities** – `heap_size_kb` and `heap_size_mb` convert word
  counts into human-readable units for logs and dashboards.
- **Compile-time guard** – enforces 64-bit targets, matching the assumptions in
  the original package.
- **Integration ready** – designed to plug into profiling and budgeting
  pipelines alongside the other strictness crates.

## Example

```rust
use heapwords::{HeapWords, heap_words3, heap_size_kb};

#[derive(Default)]
struct LedgerCache {
    by_slot: Vec<u8>,
    by_epoch: Vec<u8>,
    recent_hashes: Vec<[u8; 32]>,
}

impl HeapWords for LedgerCache {
    fn heap_words(&self) -> usize {
        heap_words3(&self.by_slot, &self.by_epoch, &self.recent_hashes)
    }
}

let cache = LedgerCache::default();
let words = cache.heap_words();
println!("cache uses ~{} KiB", heap_size_kb(words));
```

The helpers are additive by design and intentionally conservative so they can be
used inside budgeting and profiling code without risking underestimation.

## Integration notes

- Combine `HeapWords` with `heap_size_kb`/`heap_size_mb` when exporting metrics
  to observability stacks; the conversion helpers keep reporting aligned with
  Haskell services.
- Many workspace crates already expose `HeapWords` implementations. When adding
  new structs, accumulate child costs through the `heap_wordsN` helpers to stay
  in sync with the Haskell heuristics.
- The 64-bit compile guard mirrors the upstream assumption that Cardano runs on
  64-bit systems. Document any 32-bit requirements before relaxing the guard.

## Haskell → Rust mapping

| Haskell symbol | Rust counterpart |
|----------------|------------------|
| `Cardano.HeapWords.HeapWords` | `heapwords::HeapWords` |
| `heapWords` helpers (`heapWords0`..`heapWords9`) | `heapwords::heap_words0` .. `heap_words9` |
| `heapWords0` for constructors | `heapwords::heap_words0` |
| `heapWords1`.. etc. | `heapwords::{heap_words1,..}` |
| `mkHeapWords` style reporting | `heapwords::heap_size_kb`, `heapwords::heap_size_mb` |

## Crate layout

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Trait definitions, blanket impls, combinator helpers, and conversions. |
| `tests/` | Regression coverage for helper functions, blanket impls, and the compile-time guard. |

## Testing

Run the suite once a Rust toolchain is installed:

```bash
cargo test -p heapwords
```

Coverage includes the blanket implementations, arithmetic helpers, and
regression checks for the 64-bit compile-time guard.

## License

Dual-licensed under Apache-2.0 OR MIT. Refer to [`LICENSE`](../LICENSE) and
[`NOTICE`](../NOTICE) in the repository root.
