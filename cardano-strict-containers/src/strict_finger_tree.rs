use std::collections::VecDeque;
use std::fmt;
use std::iter::FromIterator;
use std::marker::PhantomData;

/// Minimal semigroup abstraction mirroring the Haskell API.
pub trait Semigroup: Sized {
    fn combine(&self, other: &Self) -> Self;
}

/// Minimal monoid abstraction mirroring the Haskell API.
pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

macro_rules! impl_numeric_monoid {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Semigroup for $ty {
                fn combine(&self, other: &Self) -> Self {
                    *self + *other
                }
            }

            impl Monoid for $ty {
                fn empty() -> Self {
                    0
                }
            }
        )*
    };
}

impl_numeric_monoid!(u8, u16, u32, u64, usize, i32, i64, isize);

impl Semigroup for () {
    fn combine(&self, _other: &Self) -> Self {
        ()
    }
}

impl Monoid for () {
    fn empty() -> Self {
        ()
    }
}

/// Trait equivalent to `Data.FingerTree.Measured`.
pub trait Measured<V>: Clone
where
    V: Monoid + Clone,
{
    fn measure(&self) -> V;
}

/// View from the left of a strict finger tree.
pub enum ViewL<V, A> {
    EmptyL,
    Cons(A, StrictFingerTree<V, A>),
}

/// View from the right of a strict finger tree.
pub enum ViewR<V, A> {
    EmptyR,
    Cons(StrictFingerTree<V, A>, A),
}

/// Result of searching a strict finger tree.
#[derive(Clone, PartialEq, Eq)]
pub enum SearchResult<V, A>
where
    V: Monoid + Clone,
    A: Measured<V> + Clone,
{
    Position(StrictFingerTree<V, A>, A, StrictFingerTree<V, A>),
    OnLeft,
    OnRight,
    Nowhere,
}

impl<V, A> fmt::Debug for SearchResult<V, A>
where
    V: Monoid + Clone,
    A: Measured<V> + Clone + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchResult::Position(left, focus, right) => f
                .debug_struct("Position")
                .field("left", left)
                .field("focus", focus)
                .field("right", right)
                .finish(),
            SearchResult::OnLeft => f.write_str("OnLeft"),
            SearchResult::OnRight => f.write_str("OnRight"),
            SearchResult::Nowhere => f.write_str("Nowhere"),
        }
    }
}

/// Strict finger tree implemented on top of `VecDeque`.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct StrictFingerTree<V, A> {
    data: VecDeque<A>,
    _marker: PhantomData<V>,
}

/// Convenience helper mirroring the original `addMeasure` utility.
pub fn add_measure<V, A>(item: &A, acc: &V) -> V
where
    V: Monoid + Clone,
    A: Measured<V>,
{
    item.measure().combine(acc)
}

/// Convenience helper mirroring the original `binMeasure` utility.
pub fn bin_measure<V, A, B>(left: &A, right: &B) -> V
where
    V: Monoid + Clone,
    A: Measured<V>,
    B: Measured<V>,
{
    left.measure().combine(&right.measure())
}

impl<V, A> StrictFingerTree<V, A>
where
    V: Monoid + Clone,
    A: Measured<V>,
{
    pub fn empty() -> Self {
        Self {
            data: VecDeque::new(),
            _marker: PhantomData,
        }
    }

    pub fn singleton(item: A) -> Self {
        let mut data = VecDeque::new();
        data.push_back(item);
        Self {
            data,
            _marker: PhantomData,
        }
    }

    pub fn from_list<I: IntoIterator<Item = A>>(iter: I) -> Self {
        Self {
            data: iter.into_iter().collect(),
            _marker: PhantomData,
        }
    }

    pub fn from_strict(tree: VecDeque<A>) -> Self {
        Self {
            data: tree,
            _marker: PhantomData,
        }
    }

    pub fn force_to_strict(tree: VecDeque<A>) -> Self {
        Self {
            data: tree,
            _marker: PhantomData,
        }
    }

    pub fn into_inner(self) -> VecDeque<A> {
        self.data
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn null(&self) -> bool {
        self.data.is_empty()
    }

    pub fn viewl(&self) -> ViewL<V, A> {
        match self.data.front() {
            None => ViewL::EmptyL,
            Some(_) => {
                let mut rest = self.data.clone();
                let first = rest.pop_front().expect("front element exists");
                ViewL::Cons(
                    first,
                    Self {
                        data: rest,
                        _marker: PhantomData,
                    },
                )
            }
        }
    }

    pub fn viewr(&self) -> ViewR<V, A> {
        match self.data.back() {
            None => ViewR::EmptyR,
            Some(_) => {
                let mut rest = self.data.clone();
                let last = rest.pop_back().expect("back element exists");
                ViewR::Cons(
                    Self {
                        data: rest,
                        _marker: PhantomData,
                    },
                    last,
                )
            }
        }
    }

    pub fn prepend(mut self, item: A) -> Self {
        self.data.push_front(item);
        self
    }

    pub fn append(mut self, item: A) -> Self {
        self.data.push_back(item);
        self
    }

    pub fn concat(mut self, mut other: Self) -> Self {
        self.data.append(&mut other.data);
        self
    }

    pub fn reverse(&self) -> Self {
        Self {
            data: self.data.iter().cloned().rev().collect(),
            _marker: PhantomData,
        }
    }

    pub fn measure(&self) -> V {
        self.data
            .iter()
            .fold(V::empty(), |acc, item| acc.combine(&item.measure()))
    }

    pub fn search<P>(&self, mut predicate: P) -> SearchResult<V, A>
    where
        P: FnMut(&V, &V) -> bool,
    {
        let total = self.measure();
        let zero = V::empty();
        if predicate(&zero, &total) {
            return SearchResult::OnLeft;
        }
        if !predicate(&total, &zero) {
            return SearchResult::OnRight;
        }

        let mut suffix_measures = Vec::with_capacity(self.data.len() + 1);
        let mut acc_suffix = V::empty();
        suffix_measures.push(acc_suffix.clone());
        for item in self.data.iter().rev() {
            acc_suffix = item.measure().combine(&acc_suffix);
            suffix_measures.push(acc_suffix.clone());
        }
        suffix_measures.reverse();

        let mut prefix_measure = V::empty();
        let mut prefix_items: VecDeque<A> = VecDeque::new();

        for (idx, item) in self.data.iter().cloned().enumerate() {
            let item_measure = item.measure();
            let new_prefix = prefix_measure.combine(&item_measure);
            let suffix_measure = suffix_measures[idx + 1].clone();
            if predicate(&new_prefix, &suffix_measure) {
                let suffix_items: VecDeque<A> = self.data.iter().skip(idx + 1).cloned().collect();
                return SearchResult::Position(
                    Self {
                        data: prefix_items.clone(),
                        _marker: PhantomData,
                    },
                    item,
                    Self {
                        data: suffix_items,
                        _marker: PhantomData,
                    },
                );
            }
            prefix_items.push_back(item);
            prefix_measure = new_prefix;
        }

        SearchResult::Nowhere
    }

    pub fn split<P>(&self, mut predicate: P) -> (Self, Self)
    where
        P: FnMut(&V) -> bool,
        A: Clone,
    {
        let mut acc = V::empty();
        let mut left: VecDeque<A> = VecDeque::new();

        for (idx, item) in self.data.iter().cloned().enumerate() {
            let new_acc = acc.combine(&item.measure());
            left.push_back(item);
            if predicate(&new_acc) {
                let right: VecDeque<A> = self.data.iter().skip(idx + 1).cloned().collect();
                return (
                    Self {
                        data: left,
                        _marker: PhantomData,
                    },
                    Self {
                        data: right,
                        _marker: PhantomData,
                    },
                );
            }
            acc = new_acc;
        }

        (
            Self {
                data: left,
                _marker: PhantomData,
            },
            Self {
                data: VecDeque::new(),
                _marker: PhantomData,
            },
        )
    }

    pub fn take_until<P>(&self, predicate: P) -> Self
    where
        P: FnMut(&V) -> bool,
        A: Clone,
    {
        self.split(predicate).0
    }

    pub fn drop_until<P>(&self, predicate: P) -> Self
    where
        P: FnMut(&V) -> bool,
        A: Clone,
    {
        self.split(predicate).1
    }

    pub fn fmap<B, V2, F>(&self, mut f: F) -> StrictFingerTree<V2, B>
    where
        V2: Monoid + Clone,
        B: Measured<V2>,
        F: FnMut(&A) -> B,
    {
        StrictFingerTree {
            data: self.data.iter().map(|item| f(item)).collect(),
            _marker: PhantomData,
        }
    }

    pub fn unsafe_fmap<B, V2, F>(&self, mut f: F) -> StrictFingerTree<V2, B>
    where
        V2: Monoid + Clone,
        B: Clone,
        F: FnMut(&A) -> B,
    {
        StrictFingerTree {
            data: self.data.iter().map(|item| f(item)).collect(),
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, A> {
        self.data.iter()
    }
}

impl<V, A> FromIterator<A> for StrictFingerTree<V, A>
where
    V: Monoid + Clone,
    A: Measured<V>,
{
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        Self::from_list(iter)
    }
}

impl<V, A> IntoIterator for StrictFingerTree<V, A>
where
    V: Monoid + Clone,
    A: Measured<V>,
{
    type Item = A;
    type IntoIter = std::collections::vec_deque::IntoIter<A>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<V, A> fmt::Debug for StrictFingerTree<V, A>
where
    V: Monoid + Clone,
    A: Measured<V> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.data).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Counted(u64);

    impl Measured<u64> for Counted {
        fn measure(&self) -> u64 {
            self.0
        }
    }

    #[test]
    fn construction_and_views() {
        let tree = StrictFingerTree::<u64, _>::from_list([Counted(1), Counted(2), Counted(3)]);
        assert_eq!(tree.len(), 3);
        assert!(!tree.is_empty());

        match tree.viewl() {
            ViewL::Cons(head, rest) => {
                assert_eq!(head, Counted(1));
                assert_eq!(rest.len(), 2);
            }
            _ => panic!("unexpected left view"),
        }

        match tree.viewr() {
            ViewR::Cons(rest, tail) => {
                assert_eq!(tail, Counted(3));
                assert_eq!(rest.len(), 2);
            }
            _ => panic!("unexpected right view"),
        }

        let reversed: Vec<_> = tree.reverse().into_iter().collect();
        assert_eq!(reversed, vec![Counted(3), Counted(2), Counted(1)]);
    }

    #[test]
    fn append_and_concat() {
        let left = StrictFingerTree::<u64, _>::singleton(Counted(1)).append(Counted(2));
        let right = StrictFingerTree::<u64, _>::from_list([Counted(3)]);
        let combined = left.concat(right);
        assert_eq!(
            combined.into_iter().collect::<Vec<_>>(),
            vec![Counted(1), Counted(2), Counted(3)]
        );
    }

    #[test]
    fn split_and_search_behaviour() {
        let tree = StrictFingerTree::<u64, _>::from_list([Counted(1), Counted(2), Counted(3)]);

        let (left, right) = tree.split(|m| *m >= 3);
        assert_eq!(
            left.into_iter().collect::<Vec<_>>(),
            vec![Counted(1), Counted(2)]
        );
        assert_eq!(right.into_iter().collect::<Vec<_>>(), vec![Counted(3)]);

        let search = tree.search(|l, r| *l >= 3 || *r <= 3);
        match search {
            SearchResult::Position(prefix, pivot, suffix) => {
                assert_eq!(prefix.into_iter().collect::<Vec<_>>(), vec![Counted(1)]);
                assert_eq!(pivot, Counted(2));
                assert_eq!(suffix.into_iter().collect::<Vec<_>>(), vec![Counted(3)]);
            }
            _ => panic!("unexpected search result"),
        }
    }

    #[test]
    fn search_extremes() {
        let tree = StrictFingerTree::<u64, _>::from_list([Counted(2), Counted(2)]);

        let on_left = tree.search(|_, _| true);
        assert!(matches!(on_left, SearchResult::OnLeft));

        let on_right = tree.search(|left, _right| *left > 100);
        assert!(matches!(on_right, SearchResult::OnRight));
    }

    #[test]
    fn take_drop_until() {
        let tree = StrictFingerTree::<u64, _>::from_list([Counted(1), Counted(2), Counted(3)]);

        let taken = tree.take_until(|m| *m >= 3);
        assert_eq!(
            taken.into_iter().collect::<Vec<_>>(),
            vec![Counted(1), Counted(2)]
        );

        let dropped = tree.drop_until(|m| *m >= 3);
        assert_eq!(dropped.into_iter().collect::<Vec<_>>(), vec![Counted(3)]);
    }

    #[test]
    fn fmap_preserves_measurement_logic() {
        let tree = StrictFingerTree::<u64, _>::from_list([Counted(1), Counted(2), Counted(3)]);

        let doubled = tree.fmap(|item| Counted(item.0 * 2));
        assert_eq!(doubled.measure(), 12);
    }

    #[test]
    fn measure_helpers_work() {
        let left = Counted(2);
        let right = Counted(3);
        let acc = add_measure(&left, &0u64);
        assert_eq!(acc, 2);
        let combined = bin_measure::<u64, _, _>(&left, &right);
        assert_eq!(combined, 5);
    }
}
