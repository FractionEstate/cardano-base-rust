use core::array::{IntoIter, from_fn};
use core::time::Duration;

/// The algebraic structure with an associative binary operation.
pub trait Semigroup {
    /// Combine two values using the associative operation.
    fn combine(self, other: Self) -> Self;

    /// Combine `n` copies of the value. Panics when `n == 0`.
    fn stimes(mut self, n: usize) -> Self
    where
        Self: Clone,
    {
        assert!(n > 0, "stimes requires n > 0");
        for _ in 1..n {
            let other = self.clone();
            self = self.combine(other);
        }
        self
    }
}

/// A monoid is a semigroup that additionally has an identity element.
pub trait Monoid: Semigroup {
    /// The identity element.
    fn empty() -> Self;

    /// Fold an iterator of values by repeatedly combining them. The empty
    /// iterator produces the identity element.
    fn concat<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        iter.into_iter()
            .fold(Self::empty(), |acc, value| acc.combine(value))
    }
}

impl Semigroup for () {
    fn combine(self, _other: Self) -> Self {}
}

impl Monoid for () {
    fn empty() -> Self {}
}

macro_rules! impl_numeric_semigroup {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Semigroup for $ty {
                fn combine(self, other: Self) -> Self {
                    self + other
                }
            }

            impl Monoid for $ty {
                fn empty() -> Self {
                    Self::default()
                }
            }
        )+
    };
}

impl_numeric_semigroup!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
);

impl Semigroup for Duration {
    fn combine(self, other: Self) -> Self {
        self + other
    }
}

impl Monoid for Duration {
    fn empty() -> Self {
        Duration::default()
    }
}

impl<T> Semigroup for Option<T>
where
    T: Semigroup,
{
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Some(left), Some(right)) => Some(left.combine(right)),
            (Some(value), None) | (None, Some(value)) => Some(value),
            (None, None) => None,
        }
    }
}

impl<T> Monoid for Option<T>
where
    T: Semigroup,
{
    fn empty() -> Self {
        None
    }
}

impl<T> Semigroup for Vec<T> {
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

impl<T> Monoid for Vec<T> {
    fn empty() -> Self {
        Vec::new()
    }
}

impl Semigroup for String {
    fn combine(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}

impl Monoid for String {
    fn empty() -> Self {
        String::new()
    }
}

impl<T, const N: usize> Semigroup for [T; N]
where
    T: Semigroup,
{
    fn combine(self, other: Self) -> Self {
        let mut left: IntoIter<T, N> = IntoIterator::into_iter(self);
        let mut right: IntoIter<T, N> = IntoIterator::into_iter(other);
        from_fn(|_| {
            let lhs = left.next().expect("length mismatch");
            let rhs = right.next().expect("length mismatch");
            lhs.combine(rhs)
        })
    }
}

impl<T, const N: usize> Monoid for [T; N]
where
    T: Monoid,
{
    fn empty() -> Self {
        from_fn(|_| T::empty())
    }
}

macro_rules! impl_tuple_semigroup {
    ($(($idx:tt, $name:ident)),+ $(,)?) => {
        impl<$($name),+> Semigroup for ($($name,)+)
        where
            $($name: Semigroup),+
        {
            fn combine(self, other: Self) -> Self {
                (
                    $(<$name as Semigroup>::combine(self.$idx, other.$idx),)+
                )
            }
        }

        impl<$($name),+> Monoid for ($($name,)+)
        where
            $($name: Monoid),+
        {
            fn empty() -> Self {
                (
                    $(<$name as Monoid>::empty(),)+
                )
            }
        }
    };
}

impl_tuple_semigroup!((0, A));
impl_tuple_semigroup!((0, A), (1, B));
impl_tuple_semigroup!((0, A), (1, B), (2, C));
impl_tuple_semigroup!((0, A), (1, B), (2, C), (3, D));
impl_tuple_semigroup!((0, A), (1, B), (2, C), (3, D), (4, E));
impl_tuple_semigroup!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
impl_tuple_semigroup!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
impl_tuple_semigroup!(
    (0, A),
    (1, B),
    (2, C),
    (3, D),
    (4, E),
    (5, F),
    (6, G),
    (7, H)
);
