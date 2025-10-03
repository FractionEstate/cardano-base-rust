use std::convert::Infallible;

use crate::epoch_info::api::EpochInfo;
use crate::slot::{EpochNo, EpochSize, SlotNo};
use crate::time::{get_slot_length, mult_nominal_diff_time, RelativeTime, SlotLength};

#[must_use] 
pub fn fixed_epoch_info(epoch_size: EpochSize, slot_length: SlotLength) -> EpochInfo<Infallible> {
    EpochInfo::from_pure(
        move |_| epoch_size,
        move |epoch| fixed_epoch_info_first(epoch_size, epoch),
        move |slot| fixed_epoch_info_epoch(epoch_size, slot),
        move |slot| {
            let slot_index = slot.0;
            let len = get_slot_length(slot_length);
            RelativeTime::new(mult_nominal_diff_time(len, slot_index))
        },
        move |_| slot_length,
    )
}

#[must_use] 
pub fn fixed_epoch_info_first(epoch_size: EpochSize, epoch: EpochNo) -> SlotNo {
    SlotNo(epoch.0 * epoch_size.0)
}

#[must_use] 
pub fn fixed_epoch_info_epoch(epoch_size: EpochSize, slot: SlotNo) -> EpochNo {
    EpochNo(slot.0 / epoch_size.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epoch_info::api::{
        epoch_info_epoch, epoch_info_first, epoch_info_slot_length,
        epoch_info_slot_to_relative_time,
    };
    use crate::time::slot_length_from_sec;
    use time::Duration;

    #[test]
    fn fixed_epoch_info_consistency() {
        let size = EpochSize(10);
        let slot_length = slot_length_from_sec(2);
        let info = fixed_epoch_info(size, slot_length);

        assert_eq!(epoch_info_first(&info, EpochNo(3)).unwrap(), SlotNo(30));
        assert_eq!(epoch_info_epoch(&info, SlotNo(25)).unwrap(), EpochNo(2));
        assert_eq!(
            epoch_info_slot_length(&info, SlotNo(42)).unwrap(),
            slot_length
        );

        let relative = epoch_info_slot_to_relative_time(&info, SlotNo(5)).unwrap();
        assert_eq!(relative.duration(), Duration::seconds(10));
    }
}
