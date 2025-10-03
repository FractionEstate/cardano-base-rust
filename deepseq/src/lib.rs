//! A minimal port of Haskell's `Control.DeepSeq` utilities.
//!
//! The original library centres around the `NFData` typeclass which ensures
//! that values can be fully evaluated. Rust evaluates eagerly by default, but
//! we keep the trait to mirror the structure of the Haskell code base and to
//! enable compile-time checks that all fields of a structure support deep
//! evaluation.

use base_deriving_via::{Generic, InstantiatedAt};
use std::borrow::{Cow, ToOwned};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::ffi::{OsStr, OsString};
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

/// Values that can be forced to _normal form_.
///
/// Implementations should recursively force any contained values. Because
/// evaluation in Rust is already strict, most implementations are simple and
/// exist primarily to enforce trait bounds throughout the translated code.
pub trait NFData {
    /// Force the value to normal form.
    fn rnf(&self);
}

/// Type constructors that can be forced when their argument can.
pub trait NFData1<T: ?Sized> {
    /// Force the outer structure, using `f` to force contained values.
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T);

    /// Force the structure assuming the contained values implement `NFData`.
    fn rnf1(&self)
    where
        T: NFData,
    {
        self.lift_rnf(&mut |value| value.rnf());
    }
}

/// Type constructors that take two arguments that can be forced.
pub trait NFData2<A: ?Sized, B: ?Sized> {
    /// Force the outer structure, using `fa`/`fb` to force contained values.
    fn lift_rnf2<FA, FB>(&self, fa: &mut FA, fb: &mut FB)
    where
        FA: FnMut(&A),
        FB: FnMut(&B);

    /// Force the structure assuming both arguments implement `NFData`.
    fn rnf2(&self)
    where
        A: NFData,
        B: NFData,
    {
        self.lift_rnf2(&mut |a| a.rnf(), &mut |b| b.rnf());
    }
}

/// Evaluate `value` to normal form and then return `result`.
pub fn deepseq<T, U>(value: T, result: U) -> U
where
    T: NFData,
{
    value.rnf();
    result
}

/// Evaluate `value` to normal form and return it.
pub fn force<T>(value: T) -> T
where
    T: NFData,
{
    value.rnf();
    value
}

/// Helper for implementing [`NFData`] via the [`Generic`] representation.
pub fn rnf_via_generic<T>(value: &T)
where
    T: Generic,
    for<'a> T::ReprRef<'a>: NFData,
{
    let repr = <T as Generic>::as_repr(value);
    repr.rnf();
}

impl<T> NFData for InstantiatedAt<T>
where
    T: Generic,
    for<'a> T::ReprRef<'a>: NFData,
{
    fn rnf(&self) {
        let repr = <T as Generic>::as_repr(self.as_ref());
        repr.rnf();
    }
}

macro_rules! impl_nfdata_for_copy {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl NFData for $ty {
                fn rnf(&self) {}
            }
        )+
    };
}

impl_nfdata_for_copy!(
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

impl NFData for String {
    fn rnf(&self) {
        self.as_str().rnf();
    }
}

impl NFData for OsString {
    fn rnf(&self) {
        self.as_os_str().rnf();
    }
}

impl NFData for OsStr {
    fn rnf(&self) {}
}

impl NFData for PathBuf {
    fn rnf(&self) {
        self.as_path().rnf();
    }
}

impl NFData for Path {
    fn rnf(&self) {}
}

impl<'a, B> NFData for Cow<'a, B>
where
    B: ToOwned + ?Sized,
    B: NFData,
    B::Owned: NFData,
{
    fn rnf(&self) {
        match self {
            Cow::Borrowed(b) => b.rnf(),
            Cow::Owned(o) => o.rnf(),
        }
    }
}

impl NFData for str {
    fn rnf(&self) {}
}

impl<T: NFData + ?Sized> NFData for &T {
    fn rnf(&self) {
        (*self).rnf();
    }
}

impl<T: NFData + ?Sized> NFData for &mut T {
    fn rnf(&self) {
        (**self).rnf();
    }
}

impl<T: NFData + ?Sized> NFData for Box<T> {
    fn rnf(&self) {
        self.as_ref().rnf();
    }
}

impl<T: ?Sized> NFData1<T> for Box<T> {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        f(self.as_ref());
    }
}

impl<T: NFData + ?Sized> NFData for Rc<T> {
    fn rnf(&self) {
        self.as_ref().rnf();
    }
}

impl<T: ?Sized> NFData1<T> for Rc<T> {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        f(self.as_ref());
    }
}

impl<T: NFData + ?Sized> NFData for Arc<T> {
    fn rnf(&self) {
        self.as_ref().rnf();
    }
}

impl<T: ?Sized> NFData1<T> for Arc<T> {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        f(self.as_ref());
    }
}

impl<T: NFData> NFData for Vec<T> {
    fn rnf(&self) {
        for item in self {
            item.rnf();
        }
    }
}

impl<T> NFData1<T> for Vec<T> {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        for item in self {
            f(item);
        }
    }
}

impl<T: NFData> NFData for VecDeque<T> {
    fn rnf(&self) {
        for item in self {
            item.rnf();
        }
    }
}

impl<T> NFData1<T> for VecDeque<T> {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        for item in self {
            f(item);
        }
    }
}

impl<T: NFData> NFData for [T] {
    fn rnf(&self) {
        for item in self {
            item.rnf();
        }
    }
}

impl<T> NFData1<T> for [T] {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        for item in self {
            f(item);
        }
    }
}

impl<T: NFData, const N: usize> NFData for [T; N] {
    fn rnf(&self) {
        self.as_slice().rnf();
    }
}

impl<T, const N: usize> NFData1<T> for [T; N] {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        self.as_slice().lift_rnf(f);
    }
}

impl<T: NFData> NFData for Option<T> {
    fn rnf(&self) {
        if let Some(value) = self.as_ref() {
            value.rnf();
        }
    }
}

impl<T> NFData1<T> for Option<T> {
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        if let Some(value) = self.as_ref() {
            f(value);
        }
    }
}

impl<T: NFData, E: NFData> NFData for Result<T, E> {
    fn rnf(&self) {
        match self {
            Ok(value) => value.rnf(),
            Err(err) => err.rnf(),
        }
    }
}

impl<T, E> NFData2<T, E> for Result<T, E> {
    fn lift_rnf2<FA, FB>(&self, fa: &mut FA, fb: &mut FB)
    where
        FA: FnMut(&T),
        FB: FnMut(&E),
    {
        match self {
            Ok(value) => fa(value),
            Err(err) => fb(err),
        }
    }
}

impl<T: NFData> NFData for BTreeSet<T> {
    fn rnf(&self) {
        for item in self {
            item.rnf();
        }
    }
}

impl<T> NFData1<T> for BTreeSet<T>
where
    T: Ord,
{
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        for item in self {
            f(item);
        }
    }
}

impl<T: NFData> NFData for HashSet<T>
where
    T: Eq + Hash,
{
    fn rnf(&self) {
        for item in self {
            item.rnf();
        }
    }
}

impl<T> NFData1<T> for HashSet<T>
where
    T: Eq + Hash,
{
    fn lift_rnf<F>(&self, f: &mut F)
    where
        F: FnMut(&T),
    {
        for item in self {
            f(item);
        }
    }
}

impl<K: NFData + Ord, V: NFData> NFData for BTreeMap<K, V> {
    fn rnf(&self) {
        self.iter().for_each(|(k, v)| {
            k.rnf();
            v.rnf();
        });
    }
}

impl<K, V> NFData2<K, V> for BTreeMap<K, V>
where
    K: Ord,
{
    fn lift_rnf2<FA, FB>(&self, fa: &mut FA, fb: &mut FB)
    where
        FA: FnMut(&K),
        FB: FnMut(&V),
    {
        for (k, v) in self.iter() {
            fa(k);
            fb(v);
        }
    }
}

impl<K: NFData + Eq + Hash, V: NFData> NFData for HashMap<K, V> {
    fn rnf(&self) {
        self.iter().for_each(|(k, v)| {
            k.rnf();
            v.rnf();
        });
    }
}

impl<K, V> NFData2<K, V> for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn lift_rnf2<FA, FB>(&self, fa: &mut FA, fb: &mut FB)
    where
        FA: FnMut(&K),
        FB: FnMut(&V),
    {
        for (k, v) in self.iter() {
            fa(k);
            fb(v);
        }
    }
}

macro_rules! impl_nfdata_for_tuple {
    ($($name:ident),+ $(,)?) => {
        impl<$($name: NFData),+> NFData for ($($name,)+) {
            #[allow(non_snake_case)]
            fn rnf(&self) {
                let ($($name,)+) = self;
                $( $name.rnf(); )+
            }
        }
    };
}

impl_nfdata_for_tuple!(A);
impl_nfdata_for_tuple!(A, B);
impl_nfdata_for_tuple!(A, B, C);
impl_nfdata_for_tuple!(A, B, C, D);
impl_nfdata_for_tuple!(A, B, C, D, E);
impl_nfdata_for_tuple!(A, B, C, D, E, F);
impl_nfdata_for_tuple!(A, B, C, D, E, F, G);
impl_nfdata_for_tuple!(A, B, C, D, E, F, G, H);
impl_nfdata_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_nfdata_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_nfdata_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_nfdata_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
    use super::*;
    use base_deriving_via::impl_generic_for_struct;

    #[derive(Debug, Clone)]
    struct Example {
        label: String,
        values: Vec<u64>,
    }

    impl NFData for Example {
        fn rnf(&self) {
            self.label.rnf();
            self.values.rnf();
        }
    }

    #[test]
    fn forcing_returns_the_original_value() {
        let value = Example {
            label: "example".to_owned(),
            values: vec![1, 2, 3],
        };

        let forced = force(value);
        assert_eq!(forced.label, "example");
        assert_eq!(forced.values, vec![1, 2, 3]);
    }

    #[test]
    fn deepseq_evaluates_before_returning_result() {
        let value = vec![1_u8, 2, 3];
        let result = 42;
        assert_eq!(deepseq(value, result), 42);
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

    impl NFData for GenericExample {
        fn rnf(&self) {
            rnf_via_generic(self);
        }
    }

    #[test]
    fn nfdata1_on_vec_uses_lift_rnf() {
        let values = vec!["one".to_string(), "two".to_string()];
        let mut seen = Vec::new();
        values.lift_rnf(&mut |item| seen.push(item.clone()));
        assert_eq!(seen, vec!["one".to_string(), "two".to_string()]);
    }

    #[test]
    fn rnf_via_generic_forces_all_fields() {
        let example = GenericExample {
            left: Some("left".to_owned()),
            right: vec![1, 2, 3],
        };

        // This should not panic and should traverse all fields without issue.
        example.rnf();
    }
}
