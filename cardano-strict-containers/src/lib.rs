//! Strict container counterparts used throughout Cardano Rust components.
//!
//! The crate mirrors the original Haskell `cardano-strict-containers`
//! package by providing strict variants of `Maybe`, `Seq`, and `FingerTree`
//! plus helper utilities.

#![allow(clippy::missing_panics_doc)]
#![cfg_attr(test, allow(clippy::panic))]

pub mod strict_finger_tree;
pub mod strict_maybe;
pub mod strict_seq;
pub mod unit;

pub use strict_finger_tree::{
    Measured, Monoid, SearchResult, Semigroup, StrictFingerTree, ViewL, ViewR, add_measure,
    bin_measure,
};
pub use strict_maybe::{
    StrictMaybe, from_s_maybe, is_s_just, is_s_nothing, maybe_to_strict_maybe, strict_maybe,
    strict_maybe_to_maybe,
};
pub use strict_seq::StrictSeq;
pub use unit::force_elems_to_whnf;
