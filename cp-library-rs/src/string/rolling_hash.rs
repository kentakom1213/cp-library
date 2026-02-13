//! ローリングハッシュ

use std::{
    fmt::Debug,
    ops::{Bound, RangeBounds},
};

use crate::number_theory::modint_for_rollinghash::modint::Modint;

use num_traits::{One, Zero};

/// ローリングハッシュ
///
/// 文字列をハッシュし，連続部分列の一致判定を $`O(1)`$ で行う．
#[derive(Debug)]
pub struct RollingHash {
    pub size: usize,
    power: Vec<Modint>,
    hash: Vec<Modint>,
    base: Modint,
}

impl RollingHash {
    /// 初期化
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

    /// 文字列から生成
    pub fn from_str(s: &str, base: Modint) -> Self {
        let arr: Vec<Modint> = s.chars().map(Self::ord).map(Modint).collect();
        Self::build(&arr, base)
    }

    /// `l..r`のハッシュを取得
    /// - 時間計算量: $`O(1)`$
    pub fn hash<'a, R: RangeBounds<usize> + Debug>(&'a self, range: R) -> HashVal<'a> {
        let (l, r) = self.parse_range(&range);
        HashVal {
            rolling_hash: self,
            length: r - l,
            hash: self.hash[r] - self.hash[l] * self.power[r - l],
        }
    }

    /// `S[a..]`, `S[b..]` の最長共通接頭辞の長さを調べる
    /// - 時間計算量: $`O(\log N)`$
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

    /// 文字 c の ASCII コードを返す
    #[inline]
    fn ord(c: char) -> usize {
        c as usize
    }
}

#[derive(Clone)]
pub struct HashVal<'a> {
    rolling_hash: &'a RollingHash,
    length: usize,
    hash: Modint,
}

impl<'a> HashVal<'a> {
    /// ハッシュの値を返す
    /// - 時間計算量: $`O(1)`$
    pub fn val(&self) -> Modint {
        self.hash
    }

    /// ハッシュ同士を連結
    /// - 時間計算量: $`O(1)`$
    pub fn chain(&self, other: &HashVal<'a>) -> HashVal<'a> {
        Self {
            rolling_hash: self.rolling_hash,
            length: self.len() + other.len(),
            hash: self.val() * self.rolling_hash.power[other.len()] + other.val(),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl PartialEq for HashVal<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.val() == other.val()
    }
}

impl<'a> Eq for HashVal<'a> {}

impl Debug for HashVal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entry(&"length", &self.length)
            .entry(&"hash", &self.hash)
            .finish()
    }
}
