# cardano-slotting (Rust)

A pure-Rust port of the original `Cardano.Slotting` Haskell package. It
provides strongly typed wrappers around block numbers, slot indices, epochs,
and the time conversions required to interpret Ouroboros slotting.

## Highlights

- `BlockNo`, `SlotNo`, `EpochNo`, and friends implemented as newtype-style
  wrappers with serde support.
- `WithOrigin<T>` mirrors the Haskell encoding, serialising as either the JSON
  string `"origin"` or the wrapped value.
- `SystemStart`, `RelativeTime`, and `SlotLength` are based on the `time` crate,
  offering nanosecond precision relative-time arithmetic.
- `EpochInfo` captures varying epoch sizes, slot lengths, and conversions to
  relative time. Helpers mirror the original API, including
  `fixed_epoch_info` and the `unsafe_linear_extend_epoch_info` extension logic.

## Example

```rust
use cardano_slotting::epoch_info::{fixed::fixed_epoch_info, epoch_info_first};
use cardano_slotting::slot::{EpochNo, EpochSize, SlotNo};
use cardano_slotting::time::slot_length_from_sec;

let info = fixed_epoch_info(EpochSize(432_000), slot_length_from_sec(1));
assert_eq!(epoch_info_first(&info, EpochNo(3)).unwrap(), SlotNo(1_296_000));
```

## Testing

```bash
cargo test -p cardano-slotting
```

> **Note:** The shared development environment currently lacks a Rust
> toolchain. Install Rust (for example via `rustup`) before executing the
> command locally.
