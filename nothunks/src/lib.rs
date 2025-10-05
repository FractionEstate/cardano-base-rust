//! A lightweight translation of the Haskell `nothunks` package.
//!
//! Since Rust evaluates eagerly there are no lazy thunks to detect at runtime.
//! Nevertheless, we retain the structure so that the rewritten code can keep the
//! same invariants and diagnostics.  The implementation therefore focuses on
//! propagating contextual information while ensuring that all contained values
//! participate in the `NoThunks` trait.

#![allow(clippy::missing_errors_doc)]

use base_deriving_via::{Generic, InstantiatedAt};

use std::borrow::{Cow, ToOwned};
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

/// Information about a thunk that was encountered while traversing a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThunkInfo {
    /// Path to the offending value (outermost context first).
    pub path: Vec<String>,
    /// Optional diagnostic message.
    pub message: Option<String>,
}

impl ThunkInfo {
    fn with_context(mut self, context: &[&str]) -> Self {
        for segment in context.iter().rev() {
            self.path.insert(0, segment.to_string());
        }
        self
    }

    fn prepend(mut self, segment: impl Into<String>) -> Self {
        self.path.insert(0, segment.into());
        self
    }
}

impl fmt::Display for ThunkInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.path.is_empty() {
            write!(f, "thunk detected")?
        } else {
            write!(f, "thunk detected at {}", self.path.join("."))?;
        }
        if let Some(message) = &self.message {
            write!(f, ": {message}")?;
        }
        Ok(())
    }
}

/// The result type returned by `NoThunks` checks.
pub type NoThunksResult = Result<(), ThunkInfo>;

fn apply_context(result: NoThunksResult, context: &[&str]) -> NoThunksResult {
    result.map_err(|info| info.with_context(context))
}

/// Trait ensuring that a value contains no unexpected laziness.
pub trait NoThunks {
    /// Check for thunks, adding `context` to any reported paths.
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult;

    /// Check for thunks without a context, mirroring the `unsafeNoThunks`
    /// function from the Haskell library.
    fn unsafe_no_thunks(&self) -> Option<ThunkInfo> {
        self.no_thunks(&[]).err()
    }
}

/// Check a value for thunks while providing an explicit context.
pub fn no_thunks<T: NoThunks>(context: &[&str], value: &T) -> NoThunksResult {
    value.no_thunks(context)
}

/// Check a value without a context, returning information about the first thunk
/// encountered.
pub fn unsafe_no_thunks<T: NoThunks>(value: &T) -> Option<ThunkInfo> {
    value.unsafe_no_thunks()
}

/// Helper for implementing [`NoThunks`] via a [`Generic`] representation.
pub fn no_thunks_via_generic<T>(value: &T, context: &[&str]) -> NoThunksResult
where
    T: Generic,
    for<'a> T::ReprRef<'a>: NoThunks,
{
    let repr = <T as Generic>::as_repr(value);
    repr.no_thunks(context)
}

/// Wrapper that limits thunk checking to the value's top-level (WHNF).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OnlyCheckWhnf<T>(pub T);

impl<T> OnlyCheckWhnf<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> NoThunks for OnlyCheckWhnf<T> {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        // Rust is eager; reaching this point means the wrapper itself is in WHNF.
        Ok(())
    }
}

/// Wrapper that limits checking to WHNF but records a diagnostic label.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OnlyCheckWhnfNamed<T> {
    pub label: String,
    pub value: T,
}

impl<T> OnlyCheckWhnfNamed<T> {
    pub fn new(label: impl Into<String>, value: T) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> NoThunks for OnlyCheckWhnfNamed<T> {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        Ok(())
    }
}

macro_rules! impl_nothunks_for_copy {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl NoThunks for $ty {
                fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
                    Ok(())
                }
            }
        )+
    };
}

impl_nothunks_for_copy!(
    (),
    bool,
    char,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

impl NoThunks for String {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        Ok(())
    }
}

impl NoThunks for OsString {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        Ok(())
    }
}

impl NoThunks for OsStr {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        Ok(())
    }
}

impl NoThunks for PathBuf {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        Ok(())
    }
}

impl NoThunks for Path {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        Ok(())
    }
}

impl<'a, B> NoThunks for Cow<'a, B>
where
    B: ?Sized + ToOwned,
    B: NoThunks,
    B::Owned: NoThunks,
{
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        match self {
            Cow::Borrowed(v) => v.no_thunks(context),
            Cow::Owned(v) => v.no_thunks(context),
        }
    }
}

impl NoThunks for str {
    fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
        Ok(())
    }
}

impl<T: NoThunks + ?Sized> NoThunks for &T {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        T::no_thunks(self, context)
    }
}

impl<T: NoThunks + ?Sized> NoThunks for &mut T {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        T::no_thunks(self, context)
    }
}

impl<T: NoThunks + ?Sized> NoThunks for Box<T> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        apply_context(self.as_ref().no_thunks(context), context)
    }
}

impl<T: NoThunks + ?Sized> NoThunks for Rc<T> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        apply_context(self.as_ref().no_thunks(context), context)
    }
}

impl<T: NoThunks + ?Sized> NoThunks for Arc<T> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        apply_context(self.as_ref().no_thunks(context), context)
    }
}

impl<T: NoThunks> NoThunks for Vec<T> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        for (idx, item) in self.iter().enumerate() {
            if let Err(info) = apply_context(item.no_thunks(context), context) {
                return Err(info.prepend(idx.to_string()));
            }
        }
        Ok(())
    }
}

impl<T: NoThunks> NoThunks for VecDeque<T> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        for (idx, item) in self.iter().enumerate() {
            if let Err(info) = apply_context(item.no_thunks(context), context) {
                return Err(info.prepend(idx.to_string()));
            }
        }
        Ok(())
    }
}

impl<T: NoThunks> NoThunks for [T] {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        for (idx, item) in self.iter().enumerate() {
            if let Err(info) = apply_context(item.no_thunks(context), context) {
                return Err(info.prepend(idx.to_string()));
            }
        }
        Ok(())
    }
}

impl<T: NoThunks, const N: usize> NoThunks for [T; N] {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        self.as_slice().no_thunks(context)
    }
}

impl<T: NoThunks> NoThunks for Option<T> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        if let Some(value) = self.as_ref() {
            apply_context(value.no_thunks(context), context)
        } else {
            Ok(())
        }
    }
}

impl<T: NoThunks, E: NoThunks> NoThunks for Result<T, E> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        match self {
            Ok(value) => apply_context(value.no_thunks(context), context),
            Err(err) => apply_context(err.no_thunks(context), context),
        }
    }
}

impl<T: NoThunks> NoThunks for BTreeSet<T> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        for (idx, item) in self.iter().enumerate() {
            if let Err(info) = apply_context(item.no_thunks(context), context) {
                return Err(info.prepend(idx.to_string()));
            }
        }
        Ok(())
    }
}

impl<T: NoThunks> NoThunks for HashSet<T>
where
    T: Eq + Hash,
{
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        for (idx, item) in self.iter().enumerate() {
            if let Err(info) = apply_context(item.no_thunks(context), context) {
                return Err(info.prepend(idx.to_string()));
            }
        }
        Ok(())
    }
}

impl<K: NoThunks + Ord, V: NoThunks> NoThunks for BTreeMap<K, V> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        for (key, value) in self.iter() {
            if let Err(info) = apply_context(key.no_thunks(context), context) {
                return Err(info.prepend("key"));
            }
            if let Err(info) = apply_context(value.no_thunks(context), context) {
                return Err(info.prepend("value"));
            }
        }
        Ok(())
    }
}

impl<K: NoThunks + Eq + Hash, V: NoThunks> NoThunks for HashMap<K, V> {
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        for (key, value) in self.iter() {
            if let Err(info) = apply_context(key.no_thunks(context), context) {
                return Err(info.prepend("key"));
            }
            if let Err(info) = apply_context(value.no_thunks(context), context) {
                return Err(info.prepend("value"));
            }
        }
        Ok(())
    }
}

macro_rules! impl_nothunks_for_tuple {
    ($($name:ident),+ $(,)?) => {
        impl<$($name: NoThunks),+> NoThunks for ($($name,)+) {
            #[allow(non_snake_case)]
            #[allow(clippy::question_mark)]
            fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
                let ($($name,)+) = self;
                $( if let Err(info) = apply_context($name.no_thunks(context), context) {
                    return Err(info);
                } )+
                Ok(())
            }
        }
    };
}

impl_nothunks_for_tuple!(A);
impl_nothunks_for_tuple!(A, B);
impl_nothunks_for_tuple!(A, B, C);
impl_nothunks_for_tuple!(A, B, C, D);
impl_nothunks_for_tuple!(A, B, C, D, E);
impl_nothunks_for_tuple!(A, B, C, D, E, F);
impl_nothunks_for_tuple!(A, B, C, D, E, F, G);
impl_nothunks_for_tuple!(A, B, C, D, E, F, G, H);
impl_nothunks_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_nothunks_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_nothunks_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_nothunks_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

impl<T> NoThunks for InstantiatedAt<T>
where
    T: Generic,
    for<'a> T::ReprRef<'a>: NoThunks,
{
    fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
        let repr = <T as Generic>::as_repr(self.as_ref());
        repr.no_thunks(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use base_deriving_via::impl_generic_for_struct;

    #[derive(Debug, Clone)]
    struct Example {
        label: String,
        values: Vec<u64>,
    }

    impl NoThunks for Example {
        fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
            apply_context(self.label.no_thunks(context), context)?;
            apply_context(self.values.no_thunks(context), context)
        }
    }

    #[test]
    fn unsafe_no_thunks_reports_none_for_plain_values() {
        let value = Example {
            label: "example".to_owned(),
            values: vec![1, 2, 3],
        };
        assert!(unsafe_no_thunks(&value).is_none());
    }

    #[derive(Debug, Clone)]
    struct GenericExample {
        left: Option<String>,
        right: Vec<u8>,
    }

    impl_generic_for_struct!(
        struct GenericExample {
            left: Option<String>,
            right: Vec<u8>,
        }
    );

    impl NoThunks for GenericExample {
        fn no_thunks(&self, context: &[&str]) -> NoThunksResult {
            no_thunks_via_generic(self, context)
        }
    }

    #[test]
    fn generic_helper_delegates_to_repr() {
        let value = GenericExample {
            left: Some("left".to_owned()),
            right: vec![1, 2, 3],
        };

        assert!(value.no_thunks(&[]).is_ok());
    }

    #[derive(Debug, Clone)]
    struct AlwaysThunk;

    impl NoThunks for AlwaysThunk {
        fn no_thunks(&self, _context: &[&str]) -> NoThunksResult {
            Err(ThunkInfo {
                path: vec!["AlwaysThunk".to_string()],
                message: Some("simulated thunk".to_string()),
            })
        }
    }

    #[test]
    fn only_check_whnf_skips_nested_checks() {
        let wrapped = OnlyCheckWhnf(AlwaysThunk);
        assert!(wrapped.no_thunks(&[]).is_ok());
    }
}
