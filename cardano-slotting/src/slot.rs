use core::fmt;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use serde::de::{self, IntoDeserializer, Visitor};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// The zero-based index for the Ouroboros time slot.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct SlotNo(pub u64);

impl SlotNo {
    #[must_use] 
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use] 
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Debug for SlotNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SlotNo({})", self.0)
    }
}

impl fmt::Display for SlotNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for SlotNo {
    fn from(value: u64) -> Self {
        SlotNo(value)
    }
}

impl From<SlotNo> for u64 {
    fn from(value: SlotNo) -> Self {
        value.0
    }
}

impl Add<u64> for SlotNo {
    type Output = SlotNo;

    fn add(self, rhs: u64) -> Self::Output {
        SlotNo(self.0 + rhs)
    }
}

impl Sub<u64> for SlotNo {
    type Output = SlotNo;

    fn sub(self, rhs: u64) -> Self::Output {
        SlotNo(self.0 - rhs)
    }
}

impl AddAssign<u64> for SlotNo {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl SubAssign<u64> for SlotNo {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

/// A value that can be at the origin or at a concrete slot.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(Default)]
pub enum WithOrigin<T> {
    #[default]
    Origin,
    At(T),
}

impl<T> WithOrigin<T> {
    pub const fn is_origin(&self) -> bool {
        matches!(self, WithOrigin::Origin)
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> WithOrigin<U> {
        match self {
            WithOrigin::Origin => WithOrigin::Origin,
            WithOrigin::At(value) => WithOrigin::At(f(value)),
        }
    }

    pub fn map_or<U>(self, default: U, f: impl FnOnce(T) -> U) -> U {
        match self {
            WithOrigin::Origin => default,
            WithOrigin::At(value) => f(value),
        }
    }

    pub fn as_ref(&self) -> WithOrigin<&T> {
        match self {
            WithOrigin::Origin => WithOrigin::Origin,
            WithOrigin::At(value) => WithOrigin::At(value),
        }
    }

    pub fn as_mut(&mut self) -> WithOrigin<&mut T> {
        match self {
            WithOrigin::Origin => WithOrigin::Origin,
            WithOrigin::At(value) => WithOrigin::At(value),
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            WithOrigin::Origin => None,
            WithOrigin::At(value) => Some(value),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.as_ref().into_option().into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> + '_ {
        self.as_mut().into_option().into_iter()
    }
}


impl<T: Serialize> Serialize for WithOrigin<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            WithOrigin::Origin => serializer.serialize_str("origin"),
            WithOrigin::At(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for WithOrigin<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct WithOriginVisitor<T>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>> Visitor<'de> for WithOriginVisitor<T> {
            type Value = WithOrigin<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("the string \"origin\" or a value")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value.eq_ignore_ascii_case("origin") {
                    Ok(WithOrigin::Origin)
                } else {
                    T::deserialize(value.into_deserializer()).map(WithOrigin::At)
                }
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WithOrigin::Origin)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(WithOrigin::Origin)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                T::deserialize(deserializer).map(WithOrigin::At)
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                T::deserialize(deserializer).map(WithOrigin::At)
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                T::deserialize(value.into_deserializer()).map(WithOrigin::At)
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                T::deserialize(value.into_deserializer()).map(WithOrigin::At)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                T::deserialize(value.into_deserializer()).map(WithOrigin::At)
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                T::deserialize(value.into_deserializer()).map(WithOrigin::At)
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                T::deserialize(serde::de::value::SeqAccessDeserializer::new(seq))
                    .map(WithOrigin::At)
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                T::deserialize(serde::de::value::MapAccessDeserializer::new(map))
                    .map(WithOrigin::At)
            }
        }

        deserializer.deserialize_any(WithOriginVisitor(PhantomData))
    }
}

pub fn at<T>(value: T) -> WithOrigin<T> {
    WithOrigin::At(value)
}

#[must_use] 
pub const fn origin<T>() -> WithOrigin<T> {
    WithOrigin::Origin
}

pub fn from_with_origin<T>(default: T, value: WithOrigin<T>) -> T {
    match value {
        WithOrigin::Origin => default,
        WithOrigin::At(t) => t,
    }
}

pub fn with_origin<T, B>(default: B, f: impl FnOnce(T) -> B, value: WithOrigin<T>) -> B {
    match value {
        WithOrigin::Origin => default,
        WithOrigin::At(t) => f(t),
    }
}

pub fn with_origin_to_maybe<T>(value: WithOrigin<T>) -> Option<T> {
    match value {
        WithOrigin::Origin => None,
        WithOrigin::At(t) => Some(t),
    }
}

pub fn with_origin_from_maybe<T>(value: Option<T>) -> WithOrigin<T> {
    match value {
        None => origin(),
        Some(t) => at(t),
    }
}

impl<T> From<Option<T>> for WithOrigin<T> {
    fn from(value: Option<T>) -> Self {
        with_origin_from_maybe(value)
    }
}

impl<T> From<WithOrigin<T>> for Option<T> {
    fn from(value: WithOrigin<T>) -> Self {
        with_origin_to_maybe(value)
    }
}

impl<T> From<T> for WithOrigin<T> {
    fn from(value: T) -> Self {
        at(value)
    }
}

impl<T> IntoIterator for WithOrigin<T> {
    type Item = T;
    type IntoIter = std::option::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_option().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a WithOrigin<T> {
    type Item = &'a T;
    type IntoIter = std::option::IntoIter<&'a T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_option().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut WithOrigin<T> {
    type Item = &'a mut T;
    type IntoIter = std::option::IntoIter<&'a mut T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().into_option().into_iter()
    }
}

/// Epoch number.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct EpochNo(pub u64);

impl fmt::Debug for EpochNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EpochNo({})", self.0)
    }
}

impl fmt::Display for EpochNo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for EpochNo {
    fn from(value: u64) -> Self {
        EpochNo(value)
    }
}

impl From<EpochNo> for u64 {
    fn from(value: EpochNo) -> Self {
        value.0
    }
}

/// Epoch size, expressed in number of slots.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct EpochSize(pub u64);

impl fmt::Debug for EpochSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EpochSize({})", self.0)
    }
}

impl fmt::Display for EpochSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for EpochSize {
    fn from(value: u64) -> Self {
        EpochSize(value)
    }
}

impl From<EpochSize> for u64 {
    fn from(value: EpochSize) -> Self {
        value.0
    }
}

/// Epoch interval represents a positive offset in epochs.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct EpochInterval(pub u32);

impl fmt::Debug for EpochInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EpochInterval({})", self.0)
    }
}

impl fmt::Display for EpochInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u32> for EpochInterval {
    fn from(value: u32) -> Self {
        EpochInterval(value)
    }
}

impl From<EpochInterval> for u32 {
    fn from(value: EpochInterval) -> Self {
        value.0
    }
}

impl From<EpochInterval> for u64 {
    fn from(value: EpochInterval) -> Self {
        value.0 as u64
    }
}

pub fn bin_op_epoch_no(op: impl Fn(u64, u64) -> u64, lhs: EpochNo, rhs: EpochNo) -> EpochNo {
    EpochNo(op(lhs.0, rhs.0))
}

#[must_use] 
pub fn add_epoch_interval(epoch_no: EpochNo, interval: EpochInterval) -> EpochNo {
    EpochNo(epoch_no.0 + u64::from(interval))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_origin_helpers() {
        assert_eq!(with_origin_to_maybe(origin::<u64>()), None);
        assert_eq!(with_origin_to_maybe(at(3u64)), Some(3));
        assert_eq!(with_origin_from_maybe::<u64>(None), origin());
        assert_eq!(with_origin_from_maybe(Some(4u64)), at(4));
    }

    #[test]
    fn with_origin_functor_and_iters() {
        let mapped = at(2u32).map(|v| v + 3);
        assert_eq!(mapped, at(5));
        assert_eq!(mapped.map_or(0, |v| v), 5);
        assert_eq!(origin::<u32>().map_or(7, |v| v * 2), 7);

        let values: Vec<_> = at(9u8).into_iter().collect();
        assert_eq!(values, vec![9]);

        let mut origin_value: WithOrigin<u8> = origin();
        assert!(origin_value.iter().next().is_none());
        assert!(origin_value.iter_mut().next().is_none());

        let mut with_value = at(1u8);
        if let Some(slot) = with_value.iter_mut().next() {
            *slot = 42;
        }
        assert_eq!(with_value, at(42));

        let option: Option<u8> = at(5).into();
        assert_eq!(option, Some(5));
        let from_option = WithOrigin::from(Some(5u8));
        assert_eq!(from_option, at(5));
    }

    #[test]
    fn add_epoch_interval_adds() {
        let epoch = EpochNo(10);
        let interval = EpochInterval(3);
        assert_eq!(add_epoch_interval(epoch, interval), EpochNo(13));
    }
}
