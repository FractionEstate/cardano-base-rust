use crate::generic::Generic;
use crate::semigroup::{Monoid, Semigroup};

/// Wrap a type and reuse its generic representation to provide algebraic
/// instances.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstantiatedAt<T>(T);

impl<T> InstantiatedAt<T> {
    /// Construct a new wrapper.
    pub fn new(inner: T) -> Self {
        Self(inner)
    }

    /// Unwrap the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Borrow the inner value.
    pub fn as_ref(&self) -> &T {
        &self.0
    }

    /// Apply a transformation to the inner value and re-wrap the result.
    pub fn map<U, F>(self, f: F) -> InstantiatedAt<U>
    where
        F: FnOnce(T) -> U,
    {
        InstantiatedAt::new(f(self.0))
    }
}

impl<T> From<T> for InstantiatedAt<T> {
    fn from(value: T) -> Self {
        InstantiatedAt::new(value)
    }
}

impl<T> Generic for InstantiatedAt<T>
where
    T: Generic,
{
    type Repr = T::Repr;
    type ReprRef<'a>
        = T::ReprRef<'a>
    where
        Self: 'a;

    fn into_repr(self) -> Self::Repr {
        T::into_repr(self.0)
    }

    fn from_repr(repr: Self::Repr) -> Self {
        InstantiatedAt::new(T::from_repr(repr))
    }

    fn as_repr(&self) -> Self::ReprRef<'_> {
        T::as_repr(&self.0)
    }
}

impl<T> Semigroup for InstantiatedAt<T>
where
    T: Generic,
    T::Repr: Semigroup,
{
    fn combine(self, other: Self) -> Self {
        let lhs = T::into_repr(self.0);
        let rhs = T::into_repr(other.0);
        InstantiatedAt::new(T::from_repr(<T::Repr as Semigroup>::combine(lhs, rhs)))
    }
}

impl<T> Monoid for InstantiatedAt<T>
where
    T: Generic,
    T::Repr: Monoid,
{
    fn empty() -> Self {
        InstantiatedAt::new(T::from_repr(<T::Repr as Monoid>::empty()))
    }
}
