//! SparseTable

use std::fmt;
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
};

use crate::algebraic_structure::semilattice::Semilattice;

#[derive(Debug)]
pub struct SparseTable<S: Semilattice> {
    pub size: usize,
    table: Vec<Vec<S::Val>>,
    logs: Vec<usize>,
}

impl<S: Semilattice> SparseTable<S> {
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

    /// SparseTableを構築する
    pub fn build(arr: &[S::Val]) -> Self {
        let size = arr.len();
        // 区間取得用の配列
        let mut logs = vec![0; size + 1];
        for i in 2..=size {
            logs[i] = logs[i >> 1] + 1;
        }
        // テーブルの高さ
        let lg = logs[size] + 1;
        // 繰り返し適用した結果
        let mut table = vec![vec![]; lg];
        for a in arr {
            table[0].push(a.clone());
        }
        for i in 1..lg {
            let mut j = 0;
            while j + (1 << i) <= size {
                let a = &table[i - 1][j];
                let b = &table[i - 1][j + (1 << (i - 1))];
                let res = S::op(a, b);
                table[i].push(res);
                j += 1;
            }
        }
        Self { size, table, logs }
    }

    /// 区間取得
    pub fn get_range<R: RangeBounds<usize> + fmt::Debug>(&self, range: R) -> S::Val {
        let Some((start, end)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };

        if start >= end {
            return S::id();
        }

        let lg = self.logs[end - start];
        let left = &self.table[lg][start];
        let right = &self.table[lg][end - (1 << lg)];

        S::op(left, right)
    }
}
