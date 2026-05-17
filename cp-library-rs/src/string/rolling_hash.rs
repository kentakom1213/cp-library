//! Rolling hash for strings and arrays.

#![allow(clippy::len_without_is_empty)]

use std::{
    fmt::Debug,
    ops::{Bound, RangeBounds},
};

use crate::number_theory::modint_for_rollinghash::modint::Modint;

use num_traits::{One, Zero};

/// Rolling hash over a sequence of modular values.
///
/// After preprocessing, substring hash queries are answered in $`O(1)`$.
#[derive(Debug)]
pub struct RollingHash {
    pub size: usize,
    power: Vec<Modint>,
    hash: Vec<Modint>,
    base: Modint,
}

impl RollingHash {
    /// Builds a rolling hash from `arr` and `base`.
    ///
    /// # Complexity
    ///
    /// - $`O(n)`$
    pub fn build(arr: &[Modint], base: Modint) -> Self {
        let size = arr.len();
        let mut power = vec![Modint(1); size + 1];
        let mut hash = vec![Modint(0); size + 1];

        // hashを初期化
        let (mut h, mut p) = (Modint::zero(), Modint::one());
        for i in 0..size {
            h = arr[i] + (h * base);
            p *= base;
            hash[i + 1] = h;
            power[i + 1] = p;
        }

        Self {
            size,
            power,
            hash,
            base,
        }
    }

    /// Builds a rolling hash from a string.
    ///
    /// Each character is encoded by its Unicode scalar value.
    ///
    /// # Complexity
    ///
    /// - $`O(n)`$
    pub fn from_str(s: &str, base: Modint) -> Self {
        let arr: Vec<Modint> = s.chars().map(Self::ord).map(Modint).collect();
        Self::build(&arr, base)
    }

    /// Returns the hash value of `range`.
    ///
    /// # Panics
    ///
    /// Panics if `range` is outside `[0, self.len()]`.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn hash<'a, R: RangeBounds<usize> + Debug>(&'a self, range: R) -> HashVal<'a> {
        let (l, r) = self.parse_range(&range);
        HashVal {
            rolling_hash: self,
            length: r - l,
            hash: self.hash[r] - self.hash[l] * self.power[r - l],
        }
    }

    /// Returns the length of the longest common prefix of `S[a..]` and `S[b..]`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
    pub fn get_LCP(&self, a: usize, b: usize) -> usize {
        let len = self.size.saturating_sub(a.max(b));
        let (mut lo, mut hi) = (0, len + 1);
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.hash(a..a + mid) == self.hash(b..b + mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    }

    /// Returns the length of the source sequence.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn len(&self) -> usize {
        self.size
    }

    // ========== internal ==========
    #[inline]
    fn parse_range<R: RangeBounds<usize> + Debug>(&self, range: &R) -> (usize, usize) {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(&v) => v + 1,
            Bound::Included(&v) => v,
        };
        let end = match range.end_bound() {
            Bound::Unbounded => self.size,
            Bound::Excluded(&v) => v,
            Bound::Included(&v) => v + 1,
        };
        if start <= end && end <= self.size {
            (start, end)
        } else {
            panic!(
                "Index out of bounds: the len is {} but the range is {:?}",
                self.size, range
            );
        }
    }

    /// Encodes `c` as an integer.
    #[inline]
    fn ord(c: char) -> usize {
        c as usize
    }
}

/// Hash value extracted from a [`RollingHash`].
#[derive(Clone)]
pub struct HashVal<'a> {
    rolling_hash: &'a RollingHash,
    length: usize,
    hash: Modint,
}

impl<'a> HashVal<'a> {
    /// Returns the raw hash value.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn val(&self) -> Modint {
        self.hash
    }

    /// Concatenates two hash values with the same base.
    ///
    /// # Panics
    ///
    /// Panics if the bases differ.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn chain(&self, other: &HashVal<'a>) -> HashVal<'a> {
        assert_eq!(
            self.rolling_hash.base, other.rolling_hash.base,
            "Cannot chain HashVal with different bases"
        );
        Self {
            rolling_hash: self.rolling_hash,
            length: self.len() + other.len(),
            hash: self.val() * self.rolling_hash.power[other.len()] + other.val(),
        }
    }

    /// Returns the represented substring length.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn len(&self) -> usize {
        self.length
    }
}

impl PartialEq for HashVal<'_> {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(
            self.rolling_hash.base, other.rolling_hash.base,
            "Cannot compare HashVal with different bases"
        );
        self.len() == other.len() && self.val() == other.val()
    }
}

impl<'a> Eq for HashVal<'a> {}

impl Debug for HashVal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashVal")
            .field("length", &self.length)
            .field("hash", &self.hash)
            .finish()
    }
}
