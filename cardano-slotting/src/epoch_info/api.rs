use std::convert::Infallible;
use std::fmt;
use std::sync::Arc;

use crate::slot::{EpochNo, EpochSize, SlotNo};
use crate::time::{from_relative_time, RelativeTime, SlotLength, SystemStart};
use time::OffsetDateTime;

pub type EpochResult<T, E> = Result<T, E>;

pub struct EpochInfo<E> {
    pub(crate) size: Arc<dyn Fn(EpochNo) -> EpochResult<EpochSize, E> + Send + Sync>,
    pub(crate) first: Arc<dyn Fn(EpochNo) -> EpochResult<SlotNo, E> + Send + Sync>,
    pub(crate) epoch: Arc<dyn Fn(SlotNo) -> EpochResult<EpochNo, E> + Send + Sync>,
    pub(crate) slot_to_relative: Arc<dyn Fn(SlotNo) -> EpochResult<RelativeTime, E> + Send + Sync>,
    pub(crate) slot_length: Arc<dyn Fn(SlotNo) -> EpochResult<SlotLength, E> + Send + Sync>,
}

impl<E> Clone for EpochInfo<E> {
    fn clone(&self) -> Self {
        Self {
            size: Arc::clone(&self.size),
            first: Arc::clone(&self.first),
            epoch: Arc::clone(&self.epoch),
            slot_to_relative: Arc::clone(&self.slot_to_relative),
            slot_length: Arc::clone(&self.slot_length),
        }
    }
}

impl<E> fmt::Debug for EpochInfo<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EpochInfoHasNoUsefulDebugInstance")
    }
}

impl<E> EpochInfo<E>
where
    E: Send + Sync + 'static,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        size: impl Fn(EpochNo) -> EpochResult<EpochSize, E> + Send + Sync + 'static,
        first: impl Fn(EpochNo) -> EpochResult<SlotNo, E> + Send + Sync + 'static,
        epoch: impl Fn(SlotNo) -> EpochResult<EpochNo, E> + Send + Sync + 'static,
        slot_to_relative: impl Fn(SlotNo) -> EpochResult<RelativeTime, E> + Send + Sync + 'static,
        slot_length: impl Fn(SlotNo) -> EpochResult<SlotLength, E> + Send + Sync + 'static,
    ) -> Self {
        Self {
            size: Arc::new(size),
            first: Arc::new(first),
            epoch: Arc::new(epoch),
            slot_to_relative: Arc::new(slot_to_relative),
            slot_length: Arc::new(slot_length),
        }
    }

    pub fn map_error<F>(self, f: impl Fn(E) -> F + Send + Sync + 'static) -> EpochInfo<F>
    where
        F: Send + Sync + 'static,
    {
        let EpochInfo {
            size,
            first,
            epoch,
            slot_to_relative,
            slot_length,
        } = self;
        let f = Arc::new(f);
        EpochInfo {
            size: Arc::new({
                let f = Arc::clone(&f);
                let size = Arc::clone(&size);
                move |epoch_no| (*size)(epoch_no).map_err(|err| f(err))
            }),
            first: Arc::new({
                let f = Arc::clone(&f);
                let first = Arc::clone(&first);
                move |epoch_no| (*first)(epoch_no).map_err(|err| f(err))
            }),
            epoch: Arc::new({
                let f = Arc::clone(&f);
                let epoch = Arc::clone(&epoch);
                move |slot| (*epoch)(slot).map_err(|err| f(err))
            }),
            slot_to_relative: Arc::new({
                let f = Arc::clone(&f);
                let slot_to_relative = Arc::clone(&slot_to_relative);
                move |slot| (*slot_to_relative)(slot).map_err(|err| f(err))
            }),
            slot_length: Arc::new({
                let f = Arc::clone(&f);
                let slot_length = Arc::clone(&slot_length);
                move |slot| (*slot_length)(slot).map_err(|err| f(err))
            }),
        }
    }
}

impl EpochInfo<Infallible> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_pure(
        size: impl Fn(EpochNo) -> EpochSize + Send + Sync + 'static,
        first: impl Fn(EpochNo) -> SlotNo + Send + Sync + 'static,
        epoch: impl Fn(SlotNo) -> EpochNo + Send + Sync + 'static,
        slot_to_relative: impl Fn(SlotNo) -> RelativeTime + Send + Sync + 'static,
        slot_length: impl Fn(SlotNo) -> SlotLength + Send + Sync + 'static,
    ) -> Self {
        Self {
            size: Arc::new(move |epoch| Ok(size(epoch))),
            first: Arc::new(move |epoch| Ok(first(epoch))),
            epoch: Arc::new(move |slot| Ok(epoch(slot))),
            slot_to_relative: Arc::new(move |slot| Ok(slot_to_relative(slot))),
            slot_length: Arc::new(move |slot| Ok(slot_length(slot))),
        }
    }
}

pub fn hoist_epoch_info<E, F>(
    info: EpochInfo<E>,
    f: impl Fn(E) -> F + Send + Sync + 'static,
) -> EpochInfo<F>
where
    E: Send + Sync + 'static,
    F: Send + Sync + 'static,
{
    info.map_error(f)
}

#[must_use] 
pub fn generalize_epoch_info<E>(info: EpochInfo<Infallible>) -> EpochInfo<E>
where
    E: Send + Sync + 'static,
{
    info.map_error(|never| match never {})
}

pub fn epoch_info_size<E>(info: &EpochInfo<E>, epoch: EpochNo) -> EpochResult<EpochSize, E> {
    (info.size)(epoch)
}

pub fn epoch_info_first<E>(info: &EpochInfo<E>, epoch: EpochNo) -> EpochResult<SlotNo, E> {
    (info.first)(epoch)
}

pub fn epoch_info_epoch<E>(info: &EpochInfo<E>, slot: SlotNo) -> EpochResult<EpochNo, E> {
    (info.epoch)(slot)
}

pub fn epoch_info_slot_to_relative_time<E>(
    info: &EpochInfo<E>,
    slot: SlotNo,
) -> EpochResult<RelativeTime, E> {
    (info.slot_to_relative)(slot)
}

pub fn epoch_info_slot_to_utc_time<E>(
    info: &EpochInfo<E>,
    system_start: SystemStart,
    slot: SlotNo,
) -> EpochResult<OffsetDateTime, E> {
    epoch_info_slot_to_relative_time(info, slot)
        .map(|relative| from_relative_time(system_start, relative))
}

pub fn epoch_info_slot_length<E>(info: &EpochInfo<E>, slot: SlotNo) -> EpochResult<SlotLength, E> {
    (info.slot_length)(slot)
}

pub fn epoch_info_range<E>(
    info: &EpochInfo<E>,
    epoch: EpochNo,
) -> EpochResult<(SlotNo, SlotNo), E> {
    let first = epoch_info_first(info, epoch)?;
    let size = epoch_info_size(info, epoch)?;
    let start = first;
    let end = SlotNo(first.0 + size.0.saturating_sub(1));
    Ok((start, end))
}
