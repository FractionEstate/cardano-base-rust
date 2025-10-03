use std::collections::VecDeque;
use std::fmt;
use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

/// Strict counterpart of `Data.Sequence.Seq` backed by a `VecDeque`.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct StrictSeq<T> {
    data: VecDeque<T>,
}

impl<T> StrictSeq<T> {
    pub fn empty() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    pub fn singleton(item: T) -> Self {
        let mut data = VecDeque::new();
        data.push_back(item);
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn length(&self) -> usize {
        self.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn null(&self) -> bool {
        self.is_empty()
    }

    pub fn push_front(&mut self, item: T) {
        self.data.push_front(item);
    }

    pub fn push_back(&mut self, item: T) {
        self.data.push_back(item);
    }

    pub fn cons(mut self, item: T) -> Self {
        self.push_front(item);
        self
    }

    pub fn snoc(mut self, item: T) -> Self {
        self.push_back(item);
        self
    }

    pub fn concat(mut self, mut other: Self) -> Self {
        self.data.append(&mut other.data);
        self
    }

    pub fn from_list<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: iter.into_iter().collect(),
        }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        Self {
            data: VecDeque::from(vec),
        }
    }

    pub fn from_vec_deque(deque: VecDeque<T>) -> Self {
        Self { data: deque }
    }

    pub fn into_vec_deque(self) -> VecDeque<T> {
        self.data
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data.into_iter().collect()
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.data.iter().cloned().collect()
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.data.back()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn take(&self, n: usize) -> Self
    where
        T: Clone,
    {
        Self::from_list(self.data.iter().take(n).cloned())
    }

    pub fn drop(&self, n: usize) -> Self
    where
        T: Clone,
    {
        Self::from_list(self.data.iter().skip(n).cloned())
    }

    pub fn take_last(&self, n: usize) -> Self
    where
        T: Clone,
    {
        let len = self.len();
        if n >= len {
            return self.clone();
        }
        self.drop(len - n)
    }

    pub fn drop_last(&self, n: usize) -> Self
    where
        T: Clone,
    {
        let len = self.len();
        if n >= len {
            return Self::empty();
        }
        self.take(len - n)
    }

    pub fn split_at(&self, index: usize) -> (Self, Self)
    where
        T: Clone,
    {
        (self.take(index), self.drop(index))
    }

    pub fn split_at_end(&self, n: usize) -> (Self, Self)
    where
        T: Clone,
    {
        let len = self.len();
        if n >= len {
            (Self::empty(), self.clone())
        } else {
            self.split_at(len - n)
        }
    }

    pub fn scanl<B, F>(&self, init: B, mut f: F) -> StrictSeq<B>
    where
        B: Clone,
        F: FnMut(B, &T) -> B,
    {
        let mut acc = init.clone();
        let mut out = VecDeque::with_capacity(self.len() + 1);
        out.push_back(init);
        for item in &self.data {
            acc = f(acc.clone(), item);
            out.push_back(acc.clone());
        }
        StrictSeq { data: out }
    }

    pub fn take_while_l<F>(&self, mut predicate: F) -> Self
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        let mut collected = VecDeque::new();
        for item in &self.data {
            if predicate(item) {
                collected.push_back(item.clone());
            } else {
                break;
            }
        }
        StrictSeq { data: collected }
    }

    pub fn take_while_r<F>(&self, mut predicate: F) -> Self
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        let mut collected = VecDeque::new();
        for item in self.data.iter().rev() {
            if predicate(item) {
                collected.push_front(item.clone());
            } else {
                break;
            }
        }
        StrictSeq { data: collected }
    }

    pub fn drop_while_l<F>(&self, mut predicate: F) -> Self
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        let mut iter = self.data.iter();
        while let Some(item) = iter.next() {
            if !predicate(item) {
                let rest: VecDeque<T> =
                    std::iter::once(item.clone()).chain(iter.cloned()).collect();
                return StrictSeq { data: rest };
            }
        }
        StrictSeq::empty()
    }

    pub fn drop_while_r<F>(&self, mut predicate: F) -> Self
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        let mut items: Vec<T> = self.data.iter().cloned().collect();
        while let Some(last) = items.last() {
            if predicate(last) {
                items.pop();
            } else {
                break;
            }
        }
        StrictSeq::from_list(items)
    }

    pub fn spanl<F>(&self, mut predicate: F) -> (Self, Self)
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        let mut count = 0;
        for item in &self.data {
            if predicate(item) {
                count += 1;
            } else {
                break;
            }
        }
        (self.take(count), self.drop(count))
    }

    pub fn spanr<F>(&self, mut predicate: F) -> (Self, Self)
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        let mut count = 0;
        for item in self.data.iter().rev() {
            if predicate(item) {
                count += 1;
            } else {
                break;
            }
        }
        let suffix = self.take_last(count);
        let prefix = self.drop_last(count);
        (suffix, prefix)
    }

    pub fn find_index_l<F>(&self, mut predicate: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool,
    {
        self.data.iter().position(|item| predicate(item))
    }

    pub fn find_index_r<F>(&self, mut predicate: F) -> Option<usize>
    where
        F: FnMut(&T) -> bool,
    {
        self.data.iter().rposition(|item| predicate(item))
    }

    pub fn find_indices_l<F>(&self, mut predicate: F) -> Vec<usize>
    where
        F: FnMut(&T) -> bool,
    {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| predicate(item).then_some(idx))
            .collect()
    }

    pub fn find_indices_r<F>(&self, mut predicate: F) -> Vec<usize>
    where
        F: FnMut(&T) -> bool,
    {
        self.data
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(idx, item)| predicate(item).then_some(idx))
            .collect()
    }

    pub fn lookup(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.lookup(index)
    }

    pub fn zip<U>(self, other: StrictSeq<U>) -> StrictSeq<(T, U)> {
        self.zip_with(other, |a, b| (a, b))
    }

    pub fn zip_with<U, V, F>(self, other: StrictSeq<U>, mut f: F) -> StrictSeq<V>
    where
        F: FnMut(T, U) -> V,
    {
        let len = self.len().min(other.len());
        let mut data = VecDeque::with_capacity(len);
        let mut left = self.data.into_iter();
        let mut right = other.data.into_iter();
        for _ in 0..len {
            if let (Some(a), Some(b)) = (left.next(), right.next()) {
                data.push_back(f(a, b));
            }
        }
        StrictSeq { data }
    }

    pub fn unzip<A, B>(self) -> (StrictSeq<A>, StrictSeq<B>)
    where
        T: Into<(A, B)>,
    {
        self.unzip_with(|item| item.into())
    }

    pub fn unzip_with<A, B, F>(self, mut f: F) -> (StrictSeq<A>, StrictSeq<B>)
    where
        F: FnMut(T) -> (A, B),
    {
        let mut left = VecDeque::with_capacity(self.len());
        let mut right = VecDeque::with_capacity(self.len());
        for item in self.data {
            let (a, b) = f(item);
            left.push_back(a);
            right.push_back(b);
        }
        (StrictSeq { data: left }, StrictSeq { data: right })
    }

    pub fn filter<F>(&self, mut predicate: F) -> Self
    where
        T: Clone,
        F: FnMut(&T) -> bool,
    {
        StrictSeq::from_list(self.data.iter().filter(|item| predicate(item)).cloned())
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::collections::vec_deque::IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T> FromIterator<T> for StrictSeq<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            data: VecDeque::from_iter(iter),
        }
    }
}

impl<T> IntoIterator for StrictSeq<T> {
    type Item = T;
    type IntoIter = std::collections::vec_deque::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Extend<T> for StrictSeq<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

impl<'a, T: 'a> Extend<&'a T> for StrictSeq<T>
where
    T: Clone,
{
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.data.extend(iter.into_iter().cloned());
    }
}

impl<T> Index<usize> for StrictSeq<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for StrictSeq<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<'a, T> IntoIterator for &'a StrictSeq<T> {
    type Item = &'a T;
    type IntoIter = std::collections::vec_deque::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T: fmt::Debug> fmt::Debug for StrictSeq<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.data).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction_and_basic_ops() {
        let seq = StrictSeq::from_list([1, 2, 3]);
        assert_eq!(seq.len(), 3);
        assert_eq!(seq.front(), Some(&1));
        assert_eq!(seq.back(), Some(&3));
    }

    #[test]
    fn take_drop_variants() {
        let seq = StrictSeq::from_list([1, 2, 3, 4, 5]);
        assert_eq!(seq.take(2), StrictSeq::from_list([1, 2]));
        assert_eq!(seq.drop(3), StrictSeq::from_list([4, 5]));
        assert_eq!(seq.take_last(2), StrictSeq::from_list([4, 5]));
        assert_eq!(seq.drop_last(2), StrictSeq::from_list([1, 2, 3]));
        assert_eq!(
            seq.split_at(3),
            (
                StrictSeq::from_list([1, 2, 3]),
                StrictSeq::from_list([4, 5])
            )
        );
        assert_eq!(
            seq.split_at_end(2),
            (
                StrictSeq::from_list([1, 2, 3]),
                StrictSeq::from_list([4, 5])
            )
        );
    }

    #[test]
    fn conversions_and_iterators() {
        let mut seq = StrictSeq::from_vec(vec![1, 2, 3]);
        assert_eq!(seq.length(), 3);
        assert!(!seq.null());

        seq.extend([4, 5]);
        assert_eq!(seq[0], 1);
        seq[1] = 99;
        assert_eq!(seq.get(1), Some(&99));

        let collected: Vec<_> = seq.iter().copied().collect();
        assert_eq!(collected, vec![1, 99, 3, 4, 5]);

        for value in seq.iter_mut() {
            *value += 1;
        }

        let vec = seq.clone().into_vec();
        assert_eq!(vec, vec![2, 100, 4, 5, 6]);

        let deque = seq.into_vec_deque();
        assert_eq!(deque.len(), 5);
    }

    #[test]
    fn take_drop_while_variants() {
        let seq = StrictSeq::from_list([1, 2, 3, 2]);
        assert_eq!(seq.take_while_l(|&x| x < 3), StrictSeq::from_list([1, 2]));
        assert_eq!(seq.drop_while_l(|&x| x < 3), StrictSeq::from_list([3, 2]));
        assert_eq!(seq.take_while_r(|&x| x < 3), StrictSeq::from_list([2]));
        assert_eq!(
            seq.drop_while_r(|&x| x < 3),
            StrictSeq::from_list([1, 2, 3])
        );
    }

    #[test]
    fn span_variants() {
        let seq = StrictSeq::from_list([1, 2, 3, 2, 1]);
        let (prefix, suffix) = seq.spanl(|&x| x < 3);
        assert_eq!(prefix, StrictSeq::from_list([1, 2]));
        assert_eq!(suffix, StrictSeq::from_list([3, 2, 1]));

        let (suffix_r, prefix_r) = seq.spanr(|&x| x < 3);
        assert_eq!(suffix_r, StrictSeq::from_list([2, 1]));
        assert_eq!(prefix_r, StrictSeq::from_list([1, 2, 3]));
    }

    #[test]
    fn zipping_and_unzipping() {
        let a = StrictSeq::from_list([1, 2, 3]);
        let b = StrictSeq::from_list([4, 5, 6]);
        let zipped = a.clone().zip(b.clone());
        assert_eq!(zipped, StrictSeq::from_list([(1, 4), (2, 5), (3, 6)]));
        let zipped_sum = a.clone().zip_with(b.clone(), |x, y| x + y);
        assert_eq!(zipped_sum, StrictSeq::from_list([5, 7, 9]));
        let (left, right) = zipped.unzip();
        assert_eq!(left, a);
        assert_eq!(right, b);
    }

    #[test]
    fn filter_and_find_helpers() {
        let seq = StrictSeq::from_list([1, 2, 3, 2]);
        assert_eq!(seq.filter(|&x| x % 2 == 0), StrictSeq::from_list([2, 2]));
        assert_eq!(seq.find_index_l(|&x| x == 2), Some(1));
        assert_eq!(seq.find_index_r(|&x| x == 2), Some(3));
        assert_eq!(seq.find_indices_l(|&x| x == 2), vec![1, 3]);
        assert_eq!(seq.find_indices_r(|&x| x == 2), vec![3, 1]);
    }
}
