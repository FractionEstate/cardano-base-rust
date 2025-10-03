/// Implement [`crate::generic::Generic`] for a record-style struct using a
/// tuple representation of its fields.
///
/// The macro mirrors the ergonomics of writing `deriving via` instances in the
/// original Haskell code base.  It currently supports structs with named
/// fields (optionally generic) and can be extended to cover more shapes as the
/// port matures.
///
/// ```rust
/// use base_deriving_via::{impl_generic_for_struct, Generic};
///
/// #[derive(Debug, Clone, PartialEq, Eq)]
/// struct Example<T> {
///     left: T,
///     right: T,
/// }
///
/// impl_generic_for_struct!(struct Example<T> {
///     left: T,
///     right: T,
/// });
///
/// fn roundtrip<T: Clone>(value: Example<T>) -> Example<T> {
///     let repr = Example::<T>::into_repr(value.clone());
///     Example::<T>::from_repr(repr)
/// }
///
/// let input = Example { left: 1, right: 2 };
/// assert_eq!(input.clone(), roundtrip(input));
/// ```
#[macro_export]
macro_rules! impl_generic_for_struct {
    (
        struct $name:ident $(<$($generics:tt),*>)? {
            $($field:ident : $ty:ty),* $(,)?
        }
    ) => {
        impl $(<$($generics),*>)? $crate::Generic for $name $(<$($generics),*>)? {
            type Repr = ($($ty,)*);
            type ReprRef<'a> = ($(&'a $ty,)* ) where Self: 'a;

            fn into_repr(self) -> Self::Repr {
                let $name { $($field),* } = self;
                ($($field,)* )
            }

            fn from_repr(repr: Self::Repr) -> Self {
                let ($($field,)* ) = repr;
                Self { $($field),* }
            }

            fn as_repr(&self) -> Self::ReprRef<'_> {
                ($(&self.$field,)* )
            }
        }
    };
}
