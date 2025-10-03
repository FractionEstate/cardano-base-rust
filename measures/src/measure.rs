use std::cmp::Ordering;

use thiserror::Error;

/// Error raised when a measurement addition would overflow the underlying type.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("measure addition overflowed")]
pub struct MeasureOverflowError;

/// Core trait describing a possibly multidimensional measurement.
///
/// Implementations must satisfy the same algebraic laws as the original
/// Haskell version: `(zero, plus)` forms a commutative monoid, `min_measure`
/// and `max_measure` define a lattice, and `plus` distributes over `min_measure`.
pub trait Measure: PartialEq + Clone {
    /// Zero measurement representing the absence of usage.
    fn zero() -> Self;

    /// Combine two measurements component-wise.
    fn plus(&self, other: &Self) -> Self;

    /// Component-wise minimum.
    fn min_measure(&self, other: &Self) -> Self;

    /// Component-wise maximum.
    fn max_measure(&self, other: &Self) -> Self;

    /// Convenience comparison mirroring the Haskell `(<=)` helper.
    fn less_equal(&self, other: &Self) -> bool {
        self == &self.min_measure(other)
    }

    /// Convenience comparison mirroring the Haskell `(>=)` helper.
    fn greater_equal(&self, other: &Self) -> bool {
        self == &self.max_measure(other)
    }

    /// Total ordering derived from the lattice structure.
    fn partial_cmp_measure(&self, other: &Self) -> Option<Ordering> {
        match (self.less_equal(other), self.greater_equal(other)) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Less),
            (false, true) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

/// Extension trait exposing a maximal measurement value.
pub trait BoundedMeasure: Measure {
    fn max_bound() -> Self;
}

macro_rules! impl_numeric_measure {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Measure for $ty {
                fn zero() -> Self { 0 }

                fn plus(&self, other: &Self) -> Self {
                    self.checked_add(*other)
                        .unwrap_or_else(|| panic!("{}", MeasureOverflowError))
                }

                fn min_measure(&self, other: &Self) -> Self {
                    (*self).min(*other)
                }

                fn max_measure(&self, other: &Self) -> Self {
                    (*self).max(*other)
                }
            }

            impl BoundedMeasure for $ty {
                fn max_bound() -> Self { <$ty>::MAX }
            }
        )+
    };
}

impl_numeric_measure!(u8, u16, u32, u64, u128, usize);

/// Natural numbers correspond to `u128` in the Rust port.
pub type Natural = u128;

macro_rules! tuple_measure_impl {
    ($(( $( $name:ident : $index:tt ),+ )),+ $(,)?) => {
        $(
            impl<$( $name ),+> Measure for ( $( $name, )+ )
            where
                $( $name: Measure ),+
            {
                fn zero() -> Self {
                    ( $( $name::zero(), )+ )
                }

                fn plus(&self, other: &Self) -> Self {
                    ( $( Measure::plus(&self.$index, &other.$index), )+ )
                }

                fn min_measure(&self, other: &Self) -> Self {
                    ( $( Measure::min_measure(&self.$index, &other.$index), )+ )
                }

                fn max_measure(&self, other: &Self) -> Self {
                    ( $( Measure::max_measure(&self.$index, &other.$index), )+ )
                }
            }

            impl<$( $name ),+> BoundedMeasure for ( $( $name, )+ )
            where
                $( $name: BoundedMeasure ),+
            {
                fn max_bound() -> Self {
                    ( $( $name::max_bound(), )+ )
                }
            }
        )+
    };
}

tuple_measure_impl! {
    (A0:0),
    (A0:0, A1:1),
    (A0:0, A1:1, A2:2),
    (A0:0, A1:1, A2:2, A3:3),
    (A0:0, A1:1, A2:2, A3:3, A4:4),
    (A0:0, A1:1, A2:2, A3:3, A4:4, A5:5),
    (A0:0, A1:1, A2:2, A3:3, A4:4, A5:5, A6:6),
}

/// Split an iterator of items once the accumulated measurement would exceed the limit.
pub fn measure_split_at<T, M, F, I>(measure: F, limit: M, iter: I) -> (Vec<T>, Vec<T>)
where
    M: Measure,
    F: Fn(&T) -> M,
    I: IntoIterator<Item = T>,
{
    let mut total = M::zero();
    let mut prefix = Vec::new();
    let mut remainder = Vec::new();
    let mut iter = iter.into_iter();

    while let Some(item) = iter.next() {
        let candidate_total = total.plus(&measure(&item));
        if candidate_total.less_equal(&limit) {
            prefix.push(item);
            total = candidate_total;
        } else {
            remainder.push(item);
            remainder.extend(iter);
            return (prefix, remainder);
        }
    }

    (prefix, remainder)
}

/// Return the longest prefix whose accumulated measurement does not exceed the limit.
pub fn measure_take<T, M, F, I>(measure: F, limit: M, iter: I) -> Vec<T>
where
    M: Measure,
    F: Fn(&T) -> M,
    I: IntoIterator<Item = T>,
{
    measure_split_at(measure, limit, iter).0
}

/// Drop the longest prefix whose accumulated measurement stays under the limit.
pub fn measure_drop<T, M, F, I>(measure: F, limit: M, iter: I) -> Vec<T>
where
    M: Measure,
    F: Fn(&T) -> M,
    I: IntoIterator<Item = T>,
{
    measure_split_at(measure, limit, iter).1
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn less_equal_matches_min() {
        let a = (1u32, 2u32, 3u32);
        let b = (1u32, 3u32, 4u32);
        assert!(a.less_equal(&b));
        assert!(!b.less_equal(&a));
    }

    #[test]
    fn tuple_plus_componentwise() {
        let a = (1u16, 2u16);
        let b = (3u16, 4u16);
        assert_eq!(a.plus(&b), (4, 6));
    }

    #[test]
    #[should_panic(expected = "measure addition overflowed")]
    fn plus_panics_on_overflow() {
        let a: u8 = 250;
        let b: u8 = 10;
        let _ = a.plus(&b);
    }

    #[test]
    fn split_at_respects_limit() {
        let items = vec![1u32, 2, 3, 4];
        let (prefix, rest) = measure_split_at(|x| *x, 3u32, items.clone());
        assert_eq!(prefix, vec![1, 2]);
        assert_eq!(rest, vec![3, 4]);
    }

    #[test]
    fn take_stops_before_exceeding_limit() {
        let items = vec![3u16, 1, 1];
        let taken = measure_take(|x| *x, 2u16, items.clone());
        assert!(taken.is_empty());

        let taken2 = measure_take(|x| *x, 4u16, items);
        assert_eq!(taken2, vec![3, 1]);
    }

    #[test]
    fn drop_returns_remaining_suffix() {
        let items = vec![1u8, 1, 2, 1];
        let dropped = measure_drop(|x| *x, 2u8, items);
        assert_eq!(dropped, vec![2, 1]);
    }

    proptest! {
        #[test]
        fn proptest_split_at_roundtrip(limit in 0u32..10_000, values in proptest::collection::vec(0u32..1_000, 0..16)) {
            let (prefix, rest) = measure_split_at(|x| *x, limit, values.clone());
            let mut recombined = prefix;
            recombined.extend(rest);
            prop_assert_eq!(recombined, values);
        }

        #[test]
        fn proptest_take_drop_agree(limit in 0u32..10_000, values in proptest::collection::vec(0u32..1_000, 0..16)) {
            let taken = measure_take(|x| *x, limit, values.clone());
            let dropped = measure_drop(|x| *x, limit, values.clone());
            let split = measure_split_at(|x| *x, limit, values);
            prop_assert_eq!((taken, dropped), split);
        }
    }
}
