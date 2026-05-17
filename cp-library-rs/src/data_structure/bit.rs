//! Fenwick tree (`BinaryIndexedTree`) implementation.

use std::{
    fmt::Debug,
    ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    },
};

use crate::algebraic_structure::{group::Group, monoid::Monoid, ordered_monoid::OrderedMonoid};

/// A Fenwick tree with a zero-based interface.
///
/// This structure supports prefix queries over a [`Monoid`], and range queries
/// when the operation additionally forms a [`Group`].
pub struct BIT<T: Monoid> {
    pub size: usize,
    arr: Vec<T::Val>,
}

impl<T: Monoid> BIT<T> {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    /// Creates a tree of length `n`.
    ///
    /// All entries are initialized to [`Monoid::e`].
    ///
    /// # Complexity
    ///
    /// - $`O(n)`$
    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::e(); n + 1],
        }
    }

    /// Adds `x` to index `i`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
    pub fn add(&mut self, mut i: usize, x: T::Val) {
        i += 1;
        while i <= self.size {
            self.arr[i] = T::op(&self.arr[i], &x);
            i += Self::lsb(i);
        }
    }

    /// Returns the aggregate on the prefix `[0, i)`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
    pub fn prefix_sum(&self, mut i: usize) -> T::Val {
        let mut res = T::e();
        while i != 0 {
            res = T::op(&res, &self.arr[i]);
            i -= Self::lsb(i);
        }
        res
    }
}

impl<T: Group> BIT<T> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        }
        .min(self.size);
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        }
        .min(self.size);
        if start <= end {
            Some((start, end))
        } else {
            None
        }
    }

    /// Returns the aggregate on `range`.
    ///
    /// Out-of-bounds endpoints are clamped to `[0, self.size]`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
    pub fn sum<R: RangeBounds<usize>>(&self, range: R) -> T::Val {
        if let Some((i, j)) = self.parse_range(range) {
            T::op(&self.prefix_sum(j), &T::inv(&self.prefix_sum(i)))
        } else {
            T::e()
        }
    }
}

impl<T: Monoid> From<&Vec<T::Val>> for BIT<T> {
    /// Builds a Fenwick tree from a vector reference.
    ///
    /// # Complexity
    ///
    /// - $`O(n)`$
    fn from(src: &Vec<T::Val>) -> Self {
        let size = src.len();
        let mut arr = vec![T::e(); size + 1];
        for i in 1..=size {
            let x = src[i - 1].clone();
            arr[i] = T::op(&arr[i], &x);
            let j = i + Self::lsb(i);
            if j < size + 1 {
                arr[j] = T::op(&arr[j], &arr[i].clone());
            }
        }
        Self { size, arr }
    }
}

impl<T: OrderedMonoid> BIT<T> {
    /// Shared implementation of [`BIT::lower_bound`] and [`BIT::upper_bound`].
    fn binary_search<F>(&self, w: T::Val, compare: F) -> usize
    where
        F: Fn(&T::Val, &T::Val) -> bool,
    {
        let mut sum = T::e();
        let mut idx = 0;
        let mut d = self.size.next_power_of_two() / 2;
        while d != 0 {
            if idx + d <= self.size {
                let nxt = T::op(&sum, &self.arr[idx + d]);
                if compare(&nxt, &w) {
                    sum = nxt;
                    idx += d;
                }
            }
            d >>= 1;
        }
        idx
    }
    /// Returns the smallest index `i` such that `a_0 + ... + a_i >= w`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
    pub fn lower_bound(&self, w: T::Val) -> usize {
        self.binary_search(w, T::lt)
    }

    /// Returns the smallest index `i` such that `a_0 + ... + a_i > w`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
    pub fn upper_bound(&self, w: T::Val) -> usize {
        self.binary_search(w, T::le)
    }
}

impl<T> Debug for BIT<T>
where
    T: Group,
    T::Val: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BIT {{ [")?;
        for i in 0..self.size - 1 {
            write!(f, "{:?}, ", self.sum(i..i + 1))?;
        }
        write!(f, "{:?}] }}", self.sum(self.size - 1..self.size))
    }
}
