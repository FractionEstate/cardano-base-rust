pub mod api;
pub mod extend;
pub mod fixed;

pub use api::EpochInfo;
pub use api::generalize_epoch_info;
pub use api::hoist_epoch_info;
pub use api::{
    epoch_info_epoch, epoch_info_first, epoch_info_range, epoch_info_size, epoch_info_slot_length,
    epoch_info_slot_to_relative_time, epoch_info_slot_to_utc_time,
};
pub use extend::unsafe_linear_extend_epoch_info;
