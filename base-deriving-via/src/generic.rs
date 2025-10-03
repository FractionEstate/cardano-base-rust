use crate::semigroup::{Monoid, Semigroup};

/// A lightweight alternative to `GHC.Generics.Generic`.
///
/// Implementors describe how a type can be converted to and from a simpler
/// “representation” (`Repr`).  The representation is typically a tuple of the
/// original fields, but any shape is valid as long as you can convert both
/// ways.
///
/// This trait is intentionally tiny so that migration from the Haskell code is
/// straightforward.
pub trait Generic: Sized {
    /// The type-level representation.
    type Repr;

    /// A borrowed view of the representation.
    type ReprRef<'a>
    where
        Self: 'a;

    /// Consume the value and produce its representation.
    fn into_repr(self) -> Self::Repr;

    /// Reconstruct a value from its representation.
    fn from_repr(repr: Self::Repr) -> Self;

    /// Borrow the value as its underlying representation.
    fn as_repr(&self) -> Self::ReprRef<'_>;
}

/// Marker trait signifying that the representation of `Self` forms a
/// [`Semigroup`].
pub trait GenericSemigroup: Generic
where
    Self::Repr: Semigroup,
{
    /// Combine two representations. Usually the default implementation is
    /// sufficient.
    fn gsappend(lhs: Self::Repr, rhs: Self::Repr) -> Self::Repr {
        <Self::Repr as Semigroup>::combine(lhs, rhs)
    }
}

impl<T> GenericSemigroup for T
where
    T: Generic,
    T::Repr: Semigroup,
{
}

/// Marker trait signifying that the representation of `Self` forms a
/// [`Monoid`].
pub trait GenericMonoid: GenericSemigroup
where
    Self::Repr: Monoid,
{
    /// Produce the identity representation. The default delegates to
    /// [`Monoid::empty`].
    fn gmempty() -> Self::Repr {
        <Self::Repr as Monoid>::empty()
    }
}

impl<T> GenericMonoid for T
where
    T: Generic,
    T::Repr: Monoid,
{
}

/// The unit type is its own representation.
impl Generic for () {
    type Repr = ();
    type ReprRef<'a>
        = ()
    where
        Self: 'a;

    fn into_repr(self) -> Self::Repr {
        ()
    }

    fn from_repr(_repr: Self::Repr) -> Self {
        ()
    }

    fn as_repr(&self) -> Self::ReprRef<'_> {
        ()
    }
}
