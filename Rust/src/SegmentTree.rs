//! # SegmentTree
//! セグメント木の実装

#![allow(dead_code)]

use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Index, RangeBounds,
};

/// # SegmentTree (Monoid)
pub struct SegmentTree<F, T> {
    pub len: usize,
    offset: usize,
    data: Vec<T>,
    op: F,
    e: T,
}

impl<F, T> Index<usize> for SegmentTree<F, T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[self.offset + idx]
    }
}

impl<F, T> SegmentTree<F, T>
where
    F: Fn(T, T) -> T,
    T: Copy + Eq + std::fmt::Debug,
{
    /// ## new
    /// セグメント木を初期化する
    pub fn new(n: usize, e: T, op: F) -> Self {
        let len = n.next_power_of_two();

        Self {
            len: len,
            offset: len,
            data: vec![e; len << 1],
            op: op,
            e: e,
        }
    }

    /// ## from
    /// 配列からセグメント木を生成する
    pub fn from(arr: &[T], e: T, op: F) -> Self {
        let mut seg = Self::new(arr.len(), e, op);
        for (i, &v) in arr.iter().enumerate() {
            seg.data[seg.offset + i] = v;
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = (seg.op)(seg.data[lch], seg.data[lch + 1]);
        }
        seg
    }

    /// ## update
    /// 要素`index`を`value`に上書きする
    /// （`index`：0-indexed）
    pub fn update(&mut self, index: usize, value: T) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = (self.op)(self.data[lch], self.data[lch + 1]);
        }
    }

    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.len,
            Excluded(&v) => v,
            Included(&v) => v - 1,
        };
        assert!(start <= end);
        // 全体の値を取得
        if (start, end) == (0, self.len) {
            return self.data[1];
        }

        // 値の取得
        let mut l = self.offset + start;
        let mut r = self.offset + end;
        let (mut res_l, mut res_r) = (self.e, self.e);

        while l < r {
            if l & 1 == 1 {
                res_l = (self.op)(res_l, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = (self.op)(self.data[r], res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(res_l, res_r)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_point() {
        let mut segtree = SegmentTree::from(&vec![1, 2, 3, 4, 5], 1, |a, b| a * b);

        assert_eq!(segtree[0], 1);
        assert_eq!(segtree[3], 4);
    }

    #[test]
    fn test_RSQ() {
        let mut segtree = SegmentTree::new(3, 0, |a, b| a ^ b);

        segtree.update(0, 1);
        segtree.update(1, 2);
        segtree.update(2, 3);

        assert_eq!(segtree.get_range(0..2), 3);
        assert_eq!(segtree.get_range(1..2), 2);
    }

    #[test]
    fn test_RMQ() {
        const INF: usize = (1 << 31) - 1;
        let mut segtree = SegmentTree::new(3, INF, |a, b| a.min(b));

        assert_eq!(segtree.get_range(..1), (1 << 31) - 1);
        segtree.update(0, 5);
        assert_eq!(segtree.get_range(..1), 5);
    }

    #[test]
    fn test_from_slice() {
        const INF: isize = -(1 << 31) + 1;
        let arr = vec![20, 4, 5, 6, 8, 9, 100];
        let mut segtree = SegmentTree::from(&arr, INF, |a, b| a.max(b));

        assert_eq!(segtree.get_range(0..), 100);
        assert_eq!(segtree.get_range(2..5), 8);

        segtree.update(0, 200);

        assert_eq!(segtree.get_range(..), 200);
        assert_eq!(segtree.get_range(2..5), 8);
    }
}
