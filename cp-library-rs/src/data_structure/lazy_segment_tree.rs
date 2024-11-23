//! 遅延評価セグメント木

use crate::{algebraic_structure::extmonoid::ExtMonoid, utils::show_binary_tree::ShowBinaryTree};
use core::fmt;
use std::{
    fmt::Debug,
    ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    },
};

/// 遅延評価セグメント木
#[derive(Debug)]
pub struct LazySegmentTree<M: ExtMonoid> {
    pub size: usize,
    offset: usize,
    data: Vec<M::X>,
    lazy: Vec<M::F>,
}

impl<M: ExtMonoid> LazySegmentTree<M> {
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

    /// 遅延評価セグメント木を初期化する
    /// - `n`: 配列サイズ
    pub fn new(n: usize) -> Self {
        let offset = n.next_power_of_two();
        Self {
            size: n,
            offset,
            data: vec![M::id_x(); offset << 1],
            lazy: vec![M::id_f(); offset << 1],
        }
    }

    /// 遅延値を評価
    fn eval(&mut self, idx: usize, len: usize) {
        if self.lazy[idx] == M::id_f() {
            return;
        }
        // 葉でなければ子に伝搬
        if idx < self.offset {
            self.lazy[idx * 2] = M::composition(&self.lazy[idx * 2], &self.lazy[idx]);
            self.lazy[idx * 2 + 1] = M::composition(&self.lazy[idx * 2 + 1], &self.lazy[idx]);
        }
        // 自身を更新
        self.data[idx] = M::mapping(&self.data[idx], &M::aggregate(&self.lazy[idx], len));
        self.lazy[idx] = M::id_f();
    }

    /// 区間に`val`を作用させる
    /// - `range`: `[left, right)`
    pub fn apply<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R, val: M::F) {
        let Some((left, right)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
        self.apply_inner(left, right, val, 0, self.offset, 1);
    }

    fn apply_inner(
        &mut self,
        left: usize,
        right: usize,
        val: M::F,
        begin: usize,
        end: usize,
        idx: usize,
    ) {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を内包するとき
        if left <= begin && end <= right {
            self.lazy[idx] = M::composition(&self.lazy[idx], &val);
            self.eval(idx, end - begin);
        }
        // 区間が重なるとき
        else if left < end && begin < right {
            let mid = (begin + end) / 2;
            // 左の子を更新
            self.apply_inner(left, right, val.clone(), begin, mid, idx * 2);
            // 右の子を更新
            self.apply_inner(left, right, val, mid, end, idx * 2 + 1);
            // 値を更新
            self.data[idx] = M::op(&self.data[idx * 2], &self.data[idx * 2 + 1]);
        }
    }

    /// 区間を取得する
    /// - `range`: `[left, right)`
    pub fn get<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R) -> M::X {
        let Some((left, right)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
        self.get_inner(left, right, 0, self.offset, 1)
    }

    fn get_inner(
        &mut self,
        left: usize,
        right: usize,
        begin: usize,
        end: usize,
        idx: usize,
    ) -> M::X {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を含まない
        if end <= left || right <= begin {
            M::id_x()
        }
        // 区間を包含する
        else if left <= begin && end <= right {
            self.data[idx].clone()
        }
        // 区間が重なる
        else {
            let mid = (begin + end) / 2;
            let l_val = self.get_inner(left, right, begin, mid, idx * 2);
            let r_val = self.get_inner(left, right, mid, end, idx * 2 + 1);
            M::op(&l_val, &r_val)
        }
    }
}

impl<M: ExtMonoid> From<&Vec<M::X>> for LazySegmentTree<M> {
    fn from(src: &Vec<M::X>) -> Self {
        let mut seg = Self::new(src.len());
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = M::op(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }
}

impl<M> ShowBinaryTree<(usize, usize, usize)> for LazySegmentTree<M>
where
    M: ExtMonoid,
    M::F: Debug,
    M::X: Debug,
{
    fn get_root(&mut self) -> (usize, usize, usize) {
        (0, self.size, self.offset / 2)
    }
    fn get_left(&mut self, &(l, r, w): &(usize, usize, usize)) -> Option<(usize, usize, usize)> {
        (w > 0).then_some((l, r.min(l + w), w / 2))
    }
    fn get_right(&mut self, &(l, r, w): &(usize, usize, usize)) -> Option<(usize, usize, usize)> {
        (w > 0 && l + w < r).then_some((l + w, r, w / 2))
    }
    fn print_node(&mut self, &(l, r, _): &(usize, usize, usize)) -> String {
        format!("[{:?}]", self.get(l..r))
    }
}
