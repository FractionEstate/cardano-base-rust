use core::convert::TryFrom;
use core::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::{Duration, OffsetDateTime};

/// System start timestamp (slots are counted from this instant).
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SystemStart(pub OffsetDateTime);

impl fmt::Debug for SystemStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SystemStart({})", self.0)
    }
}

/// Time relative to the system start.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct RelativeTime(Duration);

impl RelativeTime {
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }

    #[must_use]
    pub fn duration(self) -> Duration {
        self.0
    }
}

impl fmt::Debug for RelativeTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RelativeTime({:?})", self.0)
    }
}

/// Slot length represented as a nominal difference in time.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SlotLength(Duration);

impl fmt::Debug for SlotLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SlotLength({:?})", self.0)
    }
}

impl SlotLength {
    #[must_use]
    pub fn new(duration: Duration) -> Self {
        SlotLength(duration)
    }

    #[must_use]
    pub fn duration(self) -> Duration {
        self.0
    }
}

/// Error returned when attempting to evaluate `to_relative_time` with an input
/// earlier than the system start.
#[derive(Debug, Error)]
#[error("time {provided} is earlier than system start {system_start}")]
pub struct TimeOrderingError {
    pub provided: OffsetDateTime,
    pub system_start: OffsetDateTime,
}

#[must_use]
pub fn add_relative_time(delta: Duration, relative: RelativeTime) -> RelativeTime {
    RelativeTime(relative.0 + delta)
}

#[must_use]
pub fn diff_relative_time(lhs: RelativeTime, rhs: RelativeTime) -> Duration {
    lhs.0 - rhs.0
}

#[must_use]
pub fn mult_relative_time(relative: RelativeTime, factor: i128) -> RelativeTime {
    let nanos = relative
        .0
        .whole_nanoseconds()
        .checked_mul(factor)
        .expect("relative time multiplication overflow");
    let nanos = i64::try_from(nanos).expect("relative time nanoseconds exceed i64 range");
    RelativeTime(Duration::nanoseconds(nanos))
}

pub fn to_relative_time(
    system_start: SystemStart,
    instant: OffsetDateTime,
) -> Result<RelativeTime, TimeOrderingError> {
    if instant < system_start.0 {
        return Err(TimeOrderingError {
            provided: instant,
            system_start: system_start.0,
        });
    }

    Ok(RelativeTime(instant - system_start.0))
}

#[must_use]
pub fn from_relative_time(system_start: SystemStart, relative: RelativeTime) -> OffsetDateTime {
    system_start.0 + relative.0
}

#[must_use]
pub fn mult_nominal_diff_time(duration: Duration, factor: u64) -> Duration {
    let nanos = duration
        .whole_nanoseconds()
        .checked_mul(i128::from(factor))
        .expect("duration multiplication overflow");
    let nanos = i64::try_from(nanos).expect("duration nanoseconds exceed i64 range");
    Duration::nanoseconds(nanos)
}

#[must_use]
pub fn get_slot_length(slot_length: SlotLength) -> Duration {
    slot_length.0
}

#[must_use]
pub fn mk_slot_length(duration: Duration) -> SlotLength {
    SlotLength::new(duration)
}

#[must_use]
pub fn slot_length_from_millisec(milliseconds: i128) -> SlotLength {
    let nanos = milliseconds
        .checked_mul(1_000_000)
        .expect("slot length millisecond conversion overflow");
    let nanos = i64::try_from(nanos).expect("slot length nanoseconds exceed i64 range");
    SlotLength(Duration::nanoseconds(nanos))
}

#[must_use]
pub fn slot_length_from_sec(seconds: i128) -> SlotLength {
    slot_length_from_millisec(seconds * 1_000)
}

#[must_use]
pub fn slot_length_to_millisec(slot_length: SlotLength) -> i128 {
    slot_length.0.whole_nanoseconds() / 1_000_000
}

#[must_use]
pub fn slot_length_to_sec(slot_length: SlotLength) -> i128 {
    slot_length_to_millisec(slot_length) / 1_000
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn slot_length_roundtrip_millis() {
        let sl = slot_length_from_millisec(1000);
        assert_eq!(slot_length_to_millisec(sl), 1000);
    }

    #[test]
    fn system_start_conversion() {
        let start = SystemStart(datetime!(2020-01-01 00:00:00 UTC));
        let later = datetime!(2020-01-01 00:00:05 UTC);
        let rel = to_relative_time(start, later).unwrap();
        assert_eq!(rel.duration().whole_seconds(), 5);
        let back = from_relative_time(start, rel);
        assert_eq!(back, later);
    }

    #[test]
    fn time_ordering_error() {
        let start = SystemStart(datetime!(2020-01-01 00:00:00 UTC));
        let earlier = datetime!(2019-12-31 23:59:59 UTC);
        let err = to_relative_time(start, earlier).unwrap_err();
        assert_eq!(err.system_start, start.0);
    }

    #[test]
    fn multiply_duration() {
        let d = Duration::seconds(2);
        assert_eq!(mult_nominal_diff_time(d, 3), Duration::seconds(6));
    }

    #[test]
    fn multiply_relative_time() {
        let rel = RelativeTime(Duration::seconds(2));
        assert_eq!(mult_relative_time(rel, 4).duration().whole_seconds(), 8);
    }

    #[test]
    fn add_relative() {
        let rel = RelativeTime(Duration::seconds(5));
        let res = add_relative_time(Duration::seconds(3), rel);
        assert_eq!(res.duration().whole_seconds(), 8);
    }
}
