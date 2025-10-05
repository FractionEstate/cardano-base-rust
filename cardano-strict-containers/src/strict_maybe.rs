use crate::strict_finger_tree::{Monoid as TreeMonoid, Semigroup as TreeSemigroup};
use core::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Strict analogue of `Option` where the inner value is eagerly evaluated.
///
/// In Rust, evaluation is already strict, but the type provides API parity
/// with the original Haskell `StrictMaybe`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum StrictMaybe<T> {
    #[default]
    SNothing,
    SJust(T),
}

impl<T: Copy> Copy for StrictMaybe<T> {}

impl<T> StrictMaybe<T> {
    pub const fn is_s_nothing(&self) -> bool {
        matches!(self, StrictMaybe::SNothing)
    }

    pub const fn is_s_just(&self) -> bool {
        matches!(self, StrictMaybe::SJust(_))
    }

    pub fn s_just(value: T) -> Self {
        StrictMaybe::SJust(value)
    }

    pub fn map<U, F>(self, f: F) -> StrictMaybe<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            StrictMaybe::SNothing => StrictMaybe::SNothing,
            StrictMaybe::SJust(value) => StrictMaybe::SJust(f(value)),
        }
    }

    pub fn and_then<U, F>(self, f: F) -> StrictMaybe<U>
    where
        F: FnOnce(T) -> StrictMaybe<U>,
    {
        match self {
            StrictMaybe::SNothing => StrictMaybe::SNothing,
            StrictMaybe::SJust(value) => f(value),
        }
    }

    pub fn apply<U, R>(self, value: StrictMaybe<U>) -> StrictMaybe<R>
    where
        T: FnOnce(U) -> R,
    {
        match self {
            StrictMaybe::SNothing => StrictMaybe::SNothing,
            StrictMaybe::SJust(func) => value.map(func),
        }
    }

    pub fn or(self, other: StrictMaybe<T>) -> StrictMaybe<T> {
        match self {
            StrictMaybe::SNothing => other,
            StrictMaybe::SJust(value) => StrictMaybe::SJust(value),
        }
    }

    pub fn or_else<F>(self, f: F) -> StrictMaybe<T>
    where
        F: FnOnce() -> StrictMaybe<T>,
    {
        match self {
            StrictMaybe::SNothing => f(),
            StrictMaybe::SJust(value) => StrictMaybe::SJust(value),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            StrictMaybe::SNothing => default,
            StrictMaybe::SJust(value) => value,
        }
    }

    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            StrictMaybe::SNothing => f(),
            StrictMaybe::SJust(value) => value,
        }
    }

    pub fn as_ref(&self) -> StrictMaybe<&T> {
        match self {
            StrictMaybe::SNothing => StrictMaybe::SNothing,
            StrictMaybe::SJust(value) => StrictMaybe::SJust(value),
        }
    }
}

impl<T> From<Option<T>> for StrictMaybe<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(inner) => StrictMaybe::SJust(inner),
            None => StrictMaybe::SNothing,
        }
    }
}

impl<T> From<StrictMaybe<T>> for Option<T> {
    fn from(value: StrictMaybe<T>) -> Self {
        strict_maybe_to_maybe(value)
    }
}

/// Same as `std::option::Option::map_or_else` but for `StrictMaybe`.
pub fn strict_maybe<T, U, F>(default: T, f: F, value: StrictMaybe<U>) -> T
where
    F: FnOnce(U) -> T,
{
    match value {
        StrictMaybe::SNothing => default,
        StrictMaybe::SJust(inner) => f(inner),
    }
}

pub fn strict_maybe_to_maybe<T>(value: StrictMaybe<T>) -> Option<T> {
    match value {
        StrictMaybe::SNothing => None,
        StrictMaybe::SJust(inner) => Some(inner),
    }
}

pub fn maybe_to_strict_maybe<T>(value: Option<T>) -> StrictMaybe<T> {
    value.into()
}

pub fn from_s_maybe<T: Clone>(default: T, value: &StrictMaybe<T>) -> T {
    match value {
        StrictMaybe::SNothing => default,
        StrictMaybe::SJust(inner) => inner.clone(),
    }
}

pub fn is_s_nothing<T>(value: &StrictMaybe<T>) -> bool {
    value.is_s_nothing()
}

pub fn is_s_just<T>(value: &StrictMaybe<T>) -> bool {
    value.is_s_just()
}

impl<T: fmt::Debug> fmt::Debug for StrictMaybe<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrictMaybe::SNothing => f.write_str("SNothing"),
            StrictMaybe::SJust(value) => f.debug_tuple("SJust").field(value).finish(),
        }
    }
}

impl<T: Serialize> Serialize for StrictMaybe<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            StrictMaybe::SNothing => serializer.serialize_none(),
            StrictMaybe::SJust(value) => serializer.serialize_some(value),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for StrictMaybe<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(maybe_to_strict_maybe)
    }
}

impl<T> TreeSemigroup for StrictMaybe<T>
where
    T: TreeSemigroup + Clone,
{
    fn combine(&self, other: &Self) -> Self {
        match (self, other) {
            (StrictMaybe::SNothing, value) => value.clone(),
            (value, StrictMaybe::SNothing) => value.clone(),
            (StrictMaybe::SJust(left), StrictMaybe::SJust(right)) => {
                StrictMaybe::SJust(left.combine(right))
            },
        }
    }
}

impl<T> TreeMonoid for StrictMaybe<T>
where
    T: TreeSemigroup + Clone,
{
    fn empty() -> Self {
        StrictMaybe::SNothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strict_finger_tree::{Monoid as TreeMonoid, Semigroup as TreeSemigroup};

    #[test]
    fn conversions_work() {
        let some: StrictMaybe<u32> = StrictMaybe::SJust(5);
        assert!(some.is_s_just());
        assert_eq!(strict_maybe_to_maybe(some), Some(5));
        assert_eq!(strict_maybe_to_maybe(StrictMaybe::SNothing::<u32>), None);

        let from_opt: StrictMaybe<u32> = maybe_to_strict_maybe(Some(7));
        assert_eq!(from_opt, StrictMaybe::SJust(7));
        assert!(is_s_nothing(&StrictMaybe::<u32>::SNothing));
        assert!(is_s_just(&StrictMaybe::SJust(1)));
    }

    #[test]
    fn strict_maybe_function() {
        let value = strict_maybe(0, |x| x + 1, StrictMaybe::SJust(10));
        assert_eq!(value, 11);
        let default = strict_maybe(42, |x| x, StrictMaybe::<u32>::SNothing);
        assert_eq!(default, 42);

        let fallback = from_s_maybe(99, &StrictMaybe::SNothing);
        assert_eq!(fallback, 99);
    }

    #[test]
    fn or_prefers_left_value() {
        let left = StrictMaybe::SJust(10);
        let right = StrictMaybe::SJust(20);
        assert_eq!(left.or(right), left);
        assert_eq!(StrictMaybe::SNothing.or(right), right);

        let called = StrictMaybe::SNothing.or_else(|| StrictMaybe::SJust(5));
        assert_eq!(called, StrictMaybe::SJust(5));
    }

    #[test]
    fn apply_behaves_like_applicative() {
        let func = StrictMaybe::SJust(|value: u32| value + 1);
        let applied = func.apply(StrictMaybe::SJust(4));
        assert_eq!(applied, StrictMaybe::SJust(5));

        let missing: StrictMaybe<u32> =
            StrictMaybe::<fn(u32) -> u32>::SNothing.apply(StrictMaybe::SJust(1));
        assert_eq!(missing, StrictMaybe::SNothing);
        let missing_value =
            StrictMaybe::SJust(|value: u32| value).apply(StrictMaybe::<u32>::SNothing);
        assert_eq!(missing_value, StrictMaybe::SNothing);
    }

    #[test]
    fn semigroup_and_monoid_instances_match_haskell() {
        let left = StrictMaybe::SJust(3u32);
        let right = StrictMaybe::SJust(4u32);

        let combined = TreeSemigroup::combine(&left, &right);
        assert_eq!(combined, StrictMaybe::SJust(7));

        let left_identity = TreeSemigroup::combine(&TreeMonoid::empty(), &right);
        assert_eq!(left_identity, right);

        let right_identity = TreeSemigroup::combine(&left, &TreeMonoid::empty());
        assert_eq!(right_identity, left);
    }
}
