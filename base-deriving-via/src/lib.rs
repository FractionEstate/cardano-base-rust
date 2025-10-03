//! A Rust port of Cardano's `base-deriving-via` helpers.
//!
//! This crate provides a lightweight abstraction that mirrors the original
//! Haskell package: a newtype wrapper `InstantiatedAt` together with a handful
//! of traits that let you derive [`Semigroup`] and [`Monoid`] implementations
//! from a “generic” representation of your data type.  The approach is heavily
//! inspired by `GHC.Generics`, but relies purely on safe Rust.
//!
//! # Overview
//!
//! 1. Implement [`Generic`] for your type.  The provided macro
//!    [`impl_generic_for_struct!`] covers common structs and tuple structs by
//!    using tuples as the representation type.
//! 2. Ensure the representation implements [`Semigroup`] (and optionally
//!    [`Monoid`]) by relying on the blanket implementations for tuples or by
//!    implementing the traits manually for custom components.
//! 3. Wrap your value in [`InstantiatedAt`] to obtain [`Semigroup`] and
//!    [`Monoid`] instances automatically.
//!
//! ```rust
//! use base_deriving_via::{
//!     impl_generic_for_struct, InstantiatedAt, Monoid, Semigroup,
//! };
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct Accumulator {
//!     total: i64,
//!     description: String,
//! }
//!
//! impl_generic_for_struct!(struct Accumulator {
//!     total: i64,
//!     description: String,
//! });
//!
//! // The built-in instances cover integers (via addition) and strings.
//!
//! fn combine(left: Accumulator, right: Accumulator) -> Accumulator {
//!     let wrapped_left = InstantiatedAt::new(left);
//!     let wrapped_right = InstantiatedAt::new(right);
//!     wrapped_left.combine(wrapped_right).into_inner()
//! }
//!
//! assert_eq!(
//!     combine(
//!         Accumulator { total: 7, description: "foo".into() },
//!         Accumulator { total: 8, description: "bar".into() }
//!     ),
//!     Accumulator { total: 15, description: "foobar".into() }
//! );
//! ```
//!
//! The crate purposefully keeps the API small so it can be expanded gradually
//! as more of the original functionality is ported.

pub mod generic;
pub mod instantiated_at;
pub mod macros;
pub mod semigroup;

pub use generic::{Generic, GenericMonoid, GenericSemigroup};
pub use instantiated_at::InstantiatedAt;
pub use semigroup::{Monoid, Semigroup};
