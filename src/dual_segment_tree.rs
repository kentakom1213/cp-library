//! 双対セグメント木：区間加算・一点取得

use std::fmt::{self, Debug};
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
};

/// 可換モノイド
pub trait CommutativeMonoid {
    /// 元の型
    type Val: fmt::Debug + Clone + PartialEq;
    /// 単位元
    const E: Self::Val;
    /// 演算
    fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val;
}

/// 双対セグ木
/// - 区間への作用
/// - 一点の取得
/// を行うセグメント木
pub struct DualSegmentTree<M: CommutativeMonoid> {
    pub size: usize,
    offset: usize,
    data: Vec<M::Val>,
}

impl<M: CommutativeMonoid> DualSegmentTree<M> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: &R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        };
        if start <= end && end <= self.size {
            Some((start, end))
        } else {
            None
        }
    }

    /// 双対セグメント木を初期化する
    pub fn new(n: usize) -> Self {
        let offset = n;

        Self {
            size: n,
            offset,
            data: vec![M::E; offset << 1],
        }
    }

    /// 配列から双対セグメント木を構築する
    pub fn build(arr: &Vec<M::Val>) -> Self {
        let offset = arr.len();
        let mut seg = Self::new(offset);
        seg.data[offset..].clone_from_slice(arr);
        seg
    }

    /// 区間更新:
    /// - 区間`range`を`x`で更新する
    pub fn apply_range<R: RangeBounds<usize> + Debug>(&mut self, range: R, x: M::Val) {
        let Some((start, end)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
        // 値の更新
        let mut l = self.offset + start;
        let mut r = self.offset + end;

        while l < r {
            if l & 1 == 1 {
                let tmp = M::op(&self.data[l], &x);
                self.data[l] = tmp;
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                let tmp = M::op(&self.data[r], &x);
                self.data[r] = tmp;
            }
            l >>= 1;
            r >>= 1;
        }
    }

    /// 一点取得
    pub fn get_point(&self, index: usize) -> M::Val {
        let mut i = index + self.offset;
        let mut res = self.data[i].clone();
        while i > 1 {
            i >>= 1;
            let tmp = M::op(&self.data[i], &res);
            res = tmp;
        }
        res
    }
}

impl<M: CommutativeMonoid + Debug> Debug for DualSegmentTree<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DualSegmentTree {{ [").ok();
        for i in 0..self.size {
            if i + 1 < self.size {
                write!(f, "{:?}, ", self.get_point(i)).ok();
            } else {
                write!(f, "{:?}", self.get_point(i)).ok();
            }
        }
        write!(f, "] }}")
    }
}

pub mod Alg {
    use std::fmt::Debug;

    use super::CommutativeMonoid;

    /// 整数の和
    #[derive(Debug)]
    pub struct Add;
    impl CommutativeMonoid for Add {
        type Val = isize;
        const E: Self::Val = 0;
        fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
            arg1 + arg2
        }
    }

    /// bit単位の排他的論理和
    pub struct Xor;
    impl CommutativeMonoid for Xor {
        type Val = usize;
        const E: Self::Val = 0;
        fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
            arg1 ^ arg2
        }
    }

    /// chmin操作
    #[derive(Debug)]
    pub struct Min;
    impl CommutativeMonoid for Min {
        type Val = isize;
        const E: Self::Val = isize::MAX;
        fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
            *arg1.min(arg2)
        }
    }
}
