//! Estimating heap usage for Cardano data structures.
//!
//! This crate mirrors the behaviour of the original Haskell
//! `Cardano.HeapWords` helpers. The heuristics assume a 64-bit runtime and
//! report sizes in machine *words* (8 bytes on the supported target).

#![cfg_attr(not(target_pointer_width = "64"), allow(dead_code))]
#![allow(clippy::too_many_arguments)]

#[cfg(not(target_pointer_width = "64"))]
compile_error!("heapwords assumes a 64-bit target platform");

use num_bigint::{BigInt, BigUint, Sign};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;
use time::{Date, OffsetDateTime};

/// Size of a machine word in bytes (assuming a 64-bit architecture).
pub const WORD_SIZE: usize = 8;

/// Convert a number of heap words into megabytes.
#[must_use]
pub fn heap_size_mb(words: usize) -> usize {
    words.saturating_mul(WORD_SIZE) / (1024 * 1024)
}

/// Convert a number of heap words into kilobytes.
#[must_use]
pub fn heap_size_kb(words: usize) -> usize {
    words.saturating_mul(WORD_SIZE) / 1024
}

/// A trait for estimating heap usage (measured in machine words).
pub trait HeapWords {
    /// Return the number of heap words required to store the value.
    fn heap_words(&self) -> usize;
}

#[inline]
#[must_use]
pub fn heap_words0() -> usize {
    0
}

#[inline]
pub fn heap_words1<A>(a: &A) -> usize
where
    A: HeapWords + ?Sized,
{
    2 + a.heap_words()
}

#[inline]
pub fn heap_words2<A, B>(a: &A, b: &B) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
{
    3 + a.heap_words() + b.heap_words()
}

#[inline]
pub fn heap_words3<A, B, C>(a: &A, b: &B, c: &C) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
{
    4 + a.heap_words() + b.heap_words() + c.heap_words()
}

#[inline]
pub fn heap_words4<A, B, C, D>(a: &A, b: &B, c: &C, d: &D) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
{
    5 + a.heap_words() + b.heap_words() + c.heap_words() + d.heap_words()
}

#[inline]
pub fn heap_words5<A, B, C, D, E>(a: &A, b: &B, c: &C, d: &D, e: &E) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
{
    6 + a.heap_words() + b.heap_words() + c.heap_words() + d.heap_words() + e.heap_words()
}

#[inline]
pub fn heap_words6<A, B, C, D, E, F>(a: &A, b: &B, c: &C, d: &D, e: &E, f: &F) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
{
    7 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
}

#[inline]
pub fn heap_words7<A, B, C, D, E, F, G>(a: &A, b: &B, c: &C, d: &D, e: &E, f: &F, g: &G) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
    G: HeapWords + ?Sized,
{
    8 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
        + g.heap_words()
}

#[inline]
pub fn heap_words8<A, B, C, D, E, F, G, H>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    g: &G,
    h: &H,
) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
    G: HeapWords + ?Sized,
    H: HeapWords + ?Sized,
{
    9 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
        + g.heap_words()
        + h.heap_words()
}

#[inline]
pub fn heap_words9<A, B, C, D, E, F, G, H, I>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    g: &G,
    h: &H,
    i: &I,
) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
    G: HeapWords + ?Sized,
    H: HeapWords + ?Sized,
    I: HeapWords + ?Sized,
{
    10 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
        + g.heap_words()
        + h.heap_words()
        + i.heap_words()
}

#[inline]
pub fn heap_words10<A, B, C, D, E, F, G, H, I, J>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    g: &G,
    h: &H,
    i: &I,
    j: &J,
) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
    G: HeapWords + ?Sized,
    H: HeapWords + ?Sized,
    I: HeapWords + ?Sized,
    J: HeapWords + ?Sized,
{
    11 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
        + g.heap_words()
        + h.heap_words()
        + i.heap_words()
        + j.heap_words()
}

#[inline]
pub fn heap_words11<A, B, C, D, E, F, G, H, I, J, K>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    g: &G,
    h: &H,
    i: &I,
    j: &J,
    k: &K,
) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
    G: HeapWords + ?Sized,
    H: HeapWords + ?Sized,
    I: HeapWords + ?Sized,
    J: HeapWords + ?Sized,
    K: HeapWords + ?Sized,
{
    12 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
        + g.heap_words()
        + h.heap_words()
        + i.heap_words()
        + j.heap_words()
        + k.heap_words()
}

#[inline]
pub fn heap_words12<A, B, C, D, E, F, G, H, I, J, K, L>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    g: &G,
    h: &H,
    i: &I,
    j: &J,
    k: &K,
    l: &L,
) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
    G: HeapWords + ?Sized,
    H: HeapWords + ?Sized,
    I: HeapWords + ?Sized,
    J: HeapWords + ?Sized,
    K: HeapWords + ?Sized,
    L: HeapWords + ?Sized,
{
    13 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
        + g.heap_words()
        + h.heap_words()
        + i.heap_words()
        + j.heap_words()
        + k.heap_words()
        + l.heap_words()
}

#[inline]
pub fn heap_words13<A, B, C, D, E, F, G, H, I, J, K, L, M>(
    a: &A,
    b: &B,
    c: &C,
    d: &D,
    e: &E,
    f: &F,
    g: &G,
    h: &H,
    i: &I,
    j: &J,
    k: &K,
    l: &L,
    m: &M,
) -> usize
where
    A: HeapWords + ?Sized,
    B: HeapWords + ?Sized,
    C: HeapWords + ?Sized,
    D: HeapWords + ?Sized,
    E: HeapWords + ?Sized,
    F: HeapWords + ?Sized,
    G: HeapWords + ?Sized,
    H: HeapWords + ?Sized,
    I: HeapWords + ?Sized,
    J: HeapWords + ?Sized,
    K: HeapWords + ?Sized,
    L: HeapWords + ?Sized,
    M: HeapWords + ?Sized,
{
    14 + a.heap_words()
        + b.heap_words()
        + c.heap_words()
        + d.heap_words()
        + e.heap_words()
        + f.heap_words()
        + g.heap_words()
        + h.heap_words()
        + i.heap_words()
        + j.heap_words()
        + k.heap_words()
        + l.heap_words()
        + m.heap_words()
}

/// Estimate the heap words used by an unpacked field.
#[inline]
pub fn heap_words_unpacked<T>(value: &T) -> usize
where
    T: HeapWords + ?Sized,
{
    value.heap_words().saturating_sub(2)
}

/// Estimate the heap words of an array of primitive values.
#[inline]
#[must_use]
pub fn heap_words_uarray(element_size_bytes: usize, len: usize) -> usize {
    13 + element_size_bytes.saturating_mul(len) / WORD_SIZE
}

/// Estimate the heap words of an unboxed vector.
#[inline]
#[must_use]
pub fn heap_words_uvector(element_size_bytes: usize, len: usize) -> usize {
    5 + element_size_bytes.saturating_mul(len) / WORD_SIZE
}

#[inline]
fn ceil_words(bytes: usize) -> usize {
    if bytes == 0 {
        0
    } else {
        1 + (bytes - 1) / WORD_SIZE
    }
}

/// Strict ByteString compatibility wrapper.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteString(pub Vec<u8>);

impl From<Vec<u8>> for ByteString {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl HeapWords for ByteString {
    fn heap_words(&self) -> usize {
        5 + ceil_words(self.0.len())
    }
}

/// ShortByteString compatibility wrapper.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShortByteString(pub Vec<u8>);

impl From<Vec<u8>> for ShortByteString {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl HeapWords for ShortByteString {
    fn heap_words(&self) -> usize {
        4 + ceil_words(self.0.len())
    }
}

/// Lazy ByteString compatibility wrapper.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LazyByteString(pub Vec<ByteString>);

impl From<Vec<ByteString>> for LazyByteString {
    fn from(chunks: Vec<ByteString>) -> Self {
        Self(chunks)
    }
}

impl HeapWords for LazyByteString {
    fn heap_words(&self) -> usize {
        self.0.iter().map(|chunk| 1 + chunk.heap_words()).sum()
    }
}

/// Text compatibility wrapper (assuming UTF-16 backing like `Data.Text`).
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Text(pub String);

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl HeapWords for Text {
    fn heap_words(&self) -> usize {
        let code_units = self.0.chars().count();
        let units_per_word = WORD_SIZE / 2;
        let words = code_units / units_per_word;
        let remainder = code_units % units_per_word;
        5 + words + usize::from(remainder > 0)
    }
}

impl HeapWords for String {
    fn heap_words(&self) -> usize {
        Text(self.clone()).heap_words()
    }
}

impl HeapWords for &str {
    fn heap_words(&self) -> usize {
        Text((*self).to_owned()).heap_words()
    }
}

/// `IntMap` compatibility wrapper using a `BTreeMap` under the hood.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IntMap<V>(pub BTreeMap<i64, V>);

impl<V> From<BTreeMap<i64, V>> for IntMap<V> {
    fn from(map: BTreeMap<i64, V>) -> Self {
        Self(map)
    }
}

impl<V> HeapWords for IntMap<V>
where
    V: HeapWords,
{
    fn heap_words(&self) -> usize {
        self.0.values().map(|v| 8 + v.heap_words()).sum()
    }
}

/// `IntSet` compatibility wrapper using a `BTreeSet` under the hood.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IntSet(pub BTreeSet<i64>);

impl From<BTreeSet<i64>> for IntSet {
    fn from(set: BTreeSet<i64>) -> Self {
        Self(set)
    }
}

impl HeapWords for IntSet {
    fn heap_words(&self) -> usize {
        4 * self.0.len()
    }
}

/// `Seq` compatibility wrapper modelled with `VecDeque`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Seq<T>(pub VecDeque<T>);

impl<T> From<VecDeque<T>> for Seq<T> {
    fn from(seq: VecDeque<T>) -> Self {
        Self(seq)
    }
}

impl<T> HeapWords for Seq<T>
where
    T: HeapWords,
{
    fn heap_words(&self) -> usize {
        self.0.iter().map(|v| 5 + v.heap_words()).sum()
    }
}

macro_rules! impl_heap_words_for_int {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl HeapWords for $ty {
                fn heap_words(&self) -> usize {
                    2
                }
            }
        )+
    };
}

impl_heap_words_for_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

impl HeapWords for bool {
    fn heap_words(&self) -> usize {
        0
    }
}

impl HeapWords for char {
    fn heap_words(&self) -> usize {
        0
    }
}

impl HeapWords for () {
    fn heap_words(&self) -> usize {
        0
    }
}

impl HeapWords for OffsetDateTime {
    fn heap_words(&self) -> usize {
        7
    }
}

impl HeapWords for Date {
    fn heap_words(&self) -> usize {
        2
    }
}

impl HeapWords for BigInt {
    fn heap_words(&self) -> usize {
        let (sign, bytes) = self.to_bytes_be();
        match sign {
            Sign::NoSign => 2,
            _ => 4 + ceil_words(bytes.len()),
        }
    }
}

impl HeapWords for BigUint {
    fn heap_words(&self) -> usize {
        if self.bits() == 0 {
            2
        } else {
            let bytes = self.to_bytes_be();
            4 + ceil_words(bytes.len())
        }
    }
}

impl<T> HeapWords for Vec<T>
where
    T: HeapWords,
{
    fn heap_words(&self) -> usize {
        5 + self.len() + self.iter().map(HeapWords::heap_words).sum::<usize>()
    }
}

impl<T> HeapWords for VecDeque<T>
where
    T: HeapWords,
{
    fn heap_words(&self) -> usize {
        5 + self.len() + self.iter().map(HeapWords::heap_words).sum::<usize>()
    }
}

impl<T> HeapWords for [T]
where
    T: HeapWords,
{
    fn heap_words(&self) -> usize {
        self.iter().map(|item| 3 + item.heap_words()).sum()
    }
}

impl<T, const N: usize> HeapWords for [T; N]
where
    T: HeapWords,
{
    fn heap_words(&self) -> usize {
        self.as_slice().heap_words()
    }
}

impl<T> HeapWords for Option<T>
where
    T: HeapWords,
{
    fn heap_words(&self) -> usize {
        match self {
            None => heap_words0(),
            Some(value) => heap_words1(value),
        }
    }
}

impl<T, E> HeapWords for Result<T, E>
where
    T: HeapWords,
    E: HeapWords,
{
    fn heap_words(&self) -> usize {
        match self {
            Ok(value) => heap_words1(value),
            Err(err) => heap_words1(err),
        }
    }
}

impl<T> HeapWords for Box<T>
where
    T: HeapWords + ?Sized,
{
    fn heap_words(&self) -> usize {
        heap_words1(self.as_ref())
    }
}

impl<T> HeapWords for Rc<T>
where
    T: HeapWords + ?Sized,
{
    fn heap_words(&self) -> usize {
        heap_words1(self.as_ref())
    }
}

impl<T> HeapWords for Arc<T>
where
    T: HeapWords + ?Sized,
{
    fn heap_words(&self) -> usize {
        heap_words1(self.as_ref())
    }
}

impl<T> HeapWords for &T
where
    T: HeapWords + ?Sized,
{
    fn heap_words(&self) -> usize {
        (*self).heap_words()
    }
}

impl<T> HeapWords for &mut T
where
    T: HeapWords + ?Sized,
{
    fn heap_words(&self) -> usize {
        (**self).heap_words()
    }
}

impl<T> HeapWords for BTreeSet<T>
where
    T: HeapWords + Ord,
{
    fn heap_words(&self) -> usize {
        self.iter().map(|value| 5 + value.heap_words()).sum()
    }
}

impl<T> HeapWords for HashSet<T>
where
    T: HeapWords + Eq + Hash,
{
    fn heap_words(&self) -> usize {
        self.iter().map(|value| 5 + value.heap_words()).sum()
    }
}

impl<K, V> HeapWords for BTreeMap<K, V>
where
    K: HeapWords + Ord,
    V: HeapWords,
{
    fn heap_words(&self) -> usize {
        self.iter()
            .map(|(k, v)| 6 + k.heap_words() + v.heap_words())
            .sum()
    }
}

impl<K, V> HeapWords for HashMap<K, V>
where
    K: HeapWords + Eq + Hash,
    V: HeapWords,
{
    fn heap_words(&self) -> usize {
        self.iter()
            .map(|(k, v)| 6 + k.heap_words() + v.heap_words())
            .sum()
    }
}

impl<A, B> HeapWords for (A, B)
where
    A: HeapWords,
    B: HeapWords,
{
    fn heap_words(&self) -> usize {
        heap_words2(&self.0, &self.1)
    }
}

impl<A, B, C> HeapWords for (A, B, C)
where
    A: HeapWords,
    B: HeapWords,
    C: HeapWords,
{
    fn heap_words(&self) -> usize {
        heap_words3(&self.0, &self.1, &self.2)
    }
}

impl<A, B, C, D> HeapWords for (A, B, C, D)
where
    A: HeapWords,
    B: HeapWords,
    C: HeapWords,
    D: HeapWords,
{
    fn heap_words(&self) -> usize {
        heap_words4(&self.0, &self.1, &self.2, &self.3)
    }
}

macro_rules! impl_heap_words_for_fn {
    ($( $args:ident ),* ) => {
        impl<R, $( $args ),*> HeapWords for fn($( $args ),*) -> R {
            fn heap_words(&self) -> usize {
                0
            }
        }
    };
}

impl_heap_words_for_fn!();
impl_heap_words_for_fn!(A);
impl_heap_words_for_fn!(A, B);
impl_heap_words_for_fn!(A, B, C);
impl_heap_words_for_fn!(A, B, C, D);
impl_heap_words_for_fn!(A, B, C, D, E);
impl_heap_words_for_fn!(A, B, C, D, E, F);
impl_heap_words_for_fn!(A, B, C, D, E, F, G);
impl_heap_words_for_fn!(A, B, C, D, E, F, G, H);
impl_heap_words_for_fn!(A, B, C, D, E, F, G, H, I);
impl_heap_words_for_fn!(A, B, C, D, E, F, G, H, I, J);
impl_heap_words_for_fn!(A, B, C, D, E, F, G, H, I, J, K);
impl_heap_words_for_fn!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_heap_words_for_fn!(A, B, C, D, E, F, G, H, I, J, K, L, M);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_sizes_match_constants() {
        assert_eq!(2, 42_i64.heap_words());
        assert_eq!(0, true.heap_words());
        assert_eq!(0, 'a'.heap_words());
    }

    #[test]
    fn vector_accounts_for_elements() {
        let values = vec![1_u32, 2, 3];
        let expected = 5 + values.len() + values.iter().map(|v| v.heap_words()).sum::<usize>();
        assert_eq!(expected, values.heap_words());
    }

    #[test]
    fn byte_string_matches_haskell_layout() {
        let bytes = ByteString(vec![0u8; 17]);
        assert_eq!(5 + ceil_words(17), bytes.heap_words());
    }

    #[test]
    fn lazy_byte_string_adds_chunk_headers() {
        let chunk = ByteString(vec![0u8; 8]);
        let lbs = LazyByteString(vec![chunk.clone(), chunk]);
        assert_eq!(
            2 * (1 + ByteString(vec![0u8; 8]).heap_words()),
            lbs.heap_words()
        );
    }

    #[test]
    fn text_counts_code_units() {
        let text = Text::from("abcd");
        assert_eq!(5 + 1, text.heap_words());
    }

    #[test]
    fn int_map_uses_per_entry_constant() {
        let mut inner = BTreeMap::new();
        inner.insert(0_i64, 10_u32);
        inner.insert(1_i64, 20_u32);
        let map = IntMap::from(inner);
        let expected = 2 * (8 + 10_u32.heap_words());
        assert_eq!(expected, map.heap_words());
    }

    #[test]
    fn seq_estimate_matches_sum() {
        let mut deque = VecDeque::new();
        deque.push_back(1_u8);
        deque.push_back(2_u8);
        let seq = Seq::from(deque);
        let expected = (5 + 1_u8.heap_words()) * 2;
        assert_eq!(expected, seq.heap_words());
    }
}
