use crate::epoch_info::api::{
    epoch_info_epoch, epoch_info_first, epoch_info_size, epoch_info_slot_length,
    epoch_info_slot_to_relative_time, EpochInfo,
};
use crate::slot::{EpochNo, EpochSize, SlotNo};
use crate::time::{add_relative_time, get_slot_length, mult_nominal_diff_time};

pub fn unsafe_linear_extend_epoch_info<E>(
    basis_slot: SlotNo,
    underlying: EpochInfo<E>,
) -> EpochInfo<E>
where
    E: Send + Sync + 'static,
{
    let size_info = underlying.clone();
    let first_info = underlying.clone();
    let epoch_info_closure = underlying.clone();
    let time_info = underlying.clone();
    let length_info = underlying.clone();

    EpochInfo::new(
        move |epoch| {
            let last_known = epoch_info_epoch(&size_info, basis_slot)?;
            if epoch <= last_known {
                epoch_info_size(&size_info, epoch)
            } else {
                epoch_info_size(&size_info, last_known)
            }
        },
        move |epoch| {
            let last_known = epoch_info_epoch(&first_info, basis_slot)?;
            if epoch <= last_known {
                epoch_info_first(&first_info, epoch)
            } else {
                let lke_start = epoch_info_first(&first_info, last_known)?;
                let EpochSize(size) = epoch_info_size(&first_info, epoch)?;
                let diff_epochs = epoch.0.saturating_sub(last_known.0);
                let offset = diff_epochs.saturating_mul(size);
                Ok(SlotNo(lke_start.0.saturating_add(offset)))
            }
        },
        move |slot| {
            if slot <= basis_slot {
                epoch_info_epoch(&epoch_info_closure, slot)
            } else {
                let last_known = epoch_info_epoch(&epoch_info_closure, basis_slot)?;
                let last_start = epoch_info_first(&epoch_info_closure, last_known)?;
                let EpochSize(size) = epoch_info_size(&epoch_info_closure, last_known)?;
                let slots_forward = slot.0.saturating_sub(last_start.0);
                Ok(EpochNo(last_known.0 + slots_forward / size))
            }
        },
        move |slot| {
            if slot <= basis_slot {
                epoch_info_slot_to_relative_time(&time_info, slot)
            } else {
                let slot_diff = slot.0.saturating_sub(basis_slot.0);
                let anchor = epoch_info_slot_to_relative_time(&time_info, basis_slot)?;
                let length = epoch_info_slot_length(&time_info, basis_slot)?;
                let delta = mult_nominal_diff_time(get_slot_length(length), slot_diff);
                Ok(add_relative_time(delta, anchor))
            }
        },
        move |slot| {
            if slot <= basis_slot {
                epoch_info_slot_length(&length_info, slot)
            } else {
                epoch_info_slot_length(&length_info, basis_slot)
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::epoch_info::api::{epoch_info_slot_length, epoch_info_slot_to_relative_time};
    use crate::epoch_info::fixed::fixed_epoch_info;
    use crate::slot::{EpochNo, EpochSize, SlotNo};
    use crate::time::slot_length_from_sec;
    use time::Duration;

    #[test]
    fn extends_past_basis_slot() {
        let base = fixed_epoch_info(EpochSize(10), slot_length_from_sec(1));
        let basis_slot = SlotNo(20);
        let extended = unsafe_linear_extend_epoch_info(basis_slot, base);

        // Slots before the basis still use the underlying info.
        assert_eq!(epoch_info_epoch(&extended, SlotNo(10)).unwrap(), EpochNo(1));

        // Slots after the basis continue the linear projection.
        let far_slot = SlotNo(45);
        assert_eq!(epoch_info_epoch(&extended, far_slot).unwrap(), EpochNo(4));
        assert_eq!(
            epoch_info_slot_length(&extended, far_slot).unwrap(),
            slot_length_from_sec(1)
        );

        let relative = epoch_info_slot_to_relative_time(&extended, far_slot).unwrap();
        assert_eq!(relative.duration(), Duration::seconds(45));
    }
}
