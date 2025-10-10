# cardano-slotting (Rust)

Pure Rust port of the Haskell `Cardano.Slotting` package. It supplies the
strongly typed slot, epoch, and time primitives that underpin Ouroboros
consensus and Shelley-era ledger logic.

## Highlights

- **Newtype wrappers** — `BlockNo`, `SlotNo`, `EpochNo`, `EpochSize`, and
  `EpochInterval` enforce type safety while remaining `serde` friendly.
- **`WithOrigin<T>`** — faithfully mirrors the Haskell encoding (`"origin"`
  sentinel plus wrapped value) for genesis-aware APIs.
- **Time utilities** — `SystemStart`, `RelativeTime`, `SlotLength`, and
  helpers such as `slot_length_from_sec` provide nanosecond precision using the
  `time` crate.
- **Epoch information** — `EpochInfo` and helpers (`fixed_epoch_info`,
  `unsafe_linear_extend_epoch_info`, `epoch_info_slot_to_relative_time`, …)
  reproduce the variable-epoch calculations required by Ouroboros.
- **Error mapping** — `EpochInfo::map_error`, `hoist_epoch_info`, and
  `generalize_epoch_info` keep error types flexible without sacrificing safety.
- **serde/JSON parity** — round-trippable JSON representation for all wrappers
  matches the canonical Haskell instances (e.g. `SystemStart` as RFC3339).

## Quick start

```rust
use cardano_slotting::epoch_info::{
    epoch_info_epoch, epoch_info_slot_to_utc_time, fixed::fixed_epoch_info,
};
use cardano_slotting::{EpochNo, EpochSize, SlotNo, SystemStart, WithOrigin, at};
use cardano_slotting::time::slot_length_from_sec;
use serde_json::json;
use time::macros::datetime;

let system_start = SystemStart(datetime!(2023-01-01 00:00:00 UTC));
let epochs = fixed_epoch_info(EpochSize(100), slot_length_from_sec(2));

let slot = SlotNo(150);
let utc = epoch_info_slot_to_utc_time(&epochs, system_start, slot).unwrap();
assert_eq!(utc, datetime!(2023-01-01 00:05:00 UTC));
assert_eq!(epoch_info_epoch(&epochs, slot).unwrap(), EpochNo(1));

let with_origin = WithOrigin::from(slot);
let encoded = serde_json::to_value(with_origin).unwrap();
assert_eq!(encoded, json!(150));
let genesis = at::<SlotNo>(slot).map(|s| s + 1);
assert_eq!(genesis.into_option(), Some(SlotNo(151)));
```

## Haskell ↔ Rust mapping

| Haskell module/symbol | Rust counterpart |
|-----------------------|------------------|
| `Cardano.Slotting.Block` | `cardano_slotting::block` (`BlockNo`) |
| `Cardano.Slotting.Slot` | `cardano_slotting::slot` (`SlotNo`, `EpochNo`, `WithOrigin`, …) |
| `Cardano.Slotting.Time` | `cardano_slotting::time` (`SystemStart`, `RelativeTime`, …) |
| `Cardano.Slotting.EpochInfo` | `cardano_slotting::epoch_info` module |
| `fixedEpochInfo` | `cardano_slotting::epoch_info::fixed::fixed_epoch_info` |
| `epochInfoEpoch` | `cardano_slotting::epoch_info::epoch_info_epoch` |
| `slotToUTCTime` | `cardano_slotting::epoch_info::epoch_info_slot_to_utc_time` |
| `WithOrigin` JSON instances | `cardano_slotting::slot::WithOrigin` serde impl |

## Integration notes

- Designed to pair with `cardano-binary` for CBOR encodings of slotting types.
- Time calculations rely on the `time` crate; ensure the `macros` feature is
  enabled (already on by default in this workspace) when using `time::macros`.
- `WithOrigin` and the newtype wrappers implement `serde::Serialize`/
  `Deserialize`, so REST/gRPC front-ends can share the same wire format as the
  Haskell nodes.
- `EpochInfo` helpers are thread-safe and clone inexpensively thanks to
  internal `Arc` usage—cache them in services that perform repeated conversions.

## Testing

```bash
cargo test -p cardano-slotting
```

The suite covers slot/epoch arithmetic, JSON round-trips, and both fixed and
extended epoch information flows.

## License

Dual-licensed under Apache-2.0 OR MIT. See [`LICENSE`](../LICENSE) and
[`NOTICE`](../NOTICE) for details.
