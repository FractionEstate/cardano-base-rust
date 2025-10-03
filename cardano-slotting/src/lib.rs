//! Slotting primitives for Cardano consensus components.
//!
//! The crate mirrors the original Haskell modules under
//! `Cardano.Slotting.*`, providing strongly typed wrappers for blocks,
//! slots, epochs, relative time conversions, and helper utilities to
//! extend epoch information.

pub mod block;
pub mod epoch_info;
pub mod slot;
pub mod time;

pub use block::BlockNo;
pub use epoch_info::{
    fixed::fixed_epoch_info, fixed::fixed_epoch_info_epoch, fixed::fixed_epoch_info_first,
    generalize_epoch_info, hoist_epoch_info, unsafe_linear_extend_epoch_info, EpochInfo,
};
pub use slot::{
    add_epoch_interval, at, bin_op_epoch_no, from_with_origin, origin, with_origin,
    with_origin_from_maybe, with_origin_to_maybe, EpochInterval, EpochNo, EpochSize, SlotNo,
    WithOrigin,
};
pub use time::{
    add_relative_time, diff_relative_time, from_relative_time, get_slot_length, mk_slot_length,
    mult_nominal_diff_time, mult_relative_time, slot_length_from_millisec, slot_length_from_sec,
    slot_length_to_millisec, slot_length_to_sec, to_relative_time, RelativeTime, SlotLength,
    SystemStart, TimeOrderingError,
};
