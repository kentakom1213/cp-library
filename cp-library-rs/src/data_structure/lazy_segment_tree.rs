//! 遅延評価セグメント木
//! - 参考: <https://drken1215.hatenablog.com/entry/2024/11/17/035045>

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
    log: usize,
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
        let mut log = 0usize;
        let mut offset = 1usize;
        while offset < n {
            log += 1;
            offset <<= 1;
        }
        Self {
            size: n,
            log,
            offset,
            data: vec![M::id_x(); offset << 1],
            lazy: vec![M::id_f(); offset << 1],
        }
    }

    /// 配列から構築する
    pub fn from_slice(src: &[M::X]) -> Self {
        let mut seg = Self::new(src.len());
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for k in (1..seg.offset).rev() {
            seg.pull_dat(k);
        }
        seg
    }

    /// ノードkが表す区間長
    fn node_len(&self, k: usize) -> usize {
        let depth = (usize::BITS - 1 - k.leading_zeros()) as usize;
        self.offset >> depth
    }

    /// 1つ上の情報を更新
    fn pull_dat(&mut self, k: usize) {
        self.data[k] = M::op(&self.data[k << 1], &self.data[k << 1 | 1]);
    }

    /// 作用をノードに反映
    fn apply_lazy(&mut self, k: usize, f: &M::F) {
        let agg = M::aggregate(f, self.node_len(k));
        self.data[k] = M::mapping(&self.data[k], &agg);
        if k < self.offset {
            self.lazy[k] = M::composition(&self.lazy[k], f);
        }
    }

    /// 遅延値を子へ伝播
    fn push_lazy(&mut self, k: usize) {
        if self.lazy[k] == M::id_f() {
            return;
        }
        let f = self.lazy[k].clone();
        self.apply_lazy(k << 1, &f);
        self.apply_lazy(k << 1 | 1, &f);
        self.lazy[k] = M::id_f();
    }

    /// 葉から親方向へ更新
    fn pull_dat_deep(&mut self, k: usize) {
        for h in 1..=self.log {
            self.pull_dat(k >> h);
        }
    }

    /// 根から葉方向へ遅延評価
    fn push_lazy_deep(&mut self, k: usize) {
        for h in (1..=self.log).rev() {
            self.push_lazy(k >> h);
        }
    }

    /// 要素を更新する
    pub fn set(&mut self, i: usize, v: M::X) {
        assert!(i < self.size);
        let k = i + self.offset;
        self.push_lazy_deep(k);
        self.data[k] = v;
        self.pull_dat_deep(k);
    }

    /// 要素を取得する
    pub fn get_at(&mut self, i: usize) -> M::X {
        assert!(i < self.size);
        let k = i + self.offset;
        self.push_lazy_deep(k);
        self.data[k].clone()
    }

    /// 要素へ作用させる
    pub fn apply_at(&mut self, i: usize, f: M::F) {
        assert!(i < self.size);
        let k = i + self.offset;
        self.push_lazy_deep(k);
        let agg = M::aggregate(&f, 1);
        self.data[k] = M::mapping(&self.data[k], &agg);
        self.pull_dat_deep(k);
    }

    /// 区間に`val`を作用させる
    /// - `range`: `[left, right)`
    pub fn apply<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R, val: M::F) {
        let Some((left, right)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
        self.apply_range(left, right, &val);
    }

    fn apply_range(&mut self, mut l: usize, mut r: usize, f: &M::F) {
        if l == r {
            return;
        }
        l += self.offset;
        r += self.offset;
        for h in (1..=self.log).rev() {
            if ((l >> h) << h) != l {
                self.push_lazy(l >> h);
            }
            if ((r >> h) << h) != r {
                self.push_lazy((r - 1) >> h);
            }
        }
        let (l0, r0) = (l, r);
        while l < r {
            if (l & 1) == 1 {
                self.apply_lazy(l, f);
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                self.apply_lazy(r, f);
            }
            l >>= 1;
            r >>= 1;
        }
        l = l0;
        r = r0;
        for h in 1..=self.log {
            if ((l >> h) << h) != l {
                self.pull_dat(l >> h);
            }
            if ((r >> h) << h) != r {
                self.pull_dat((r - 1) >> h);
            }
        }
    }

    /// 区間を取得する
    /// - `range`: `[left, right)`
    pub fn get<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R) -> M::X {
        let Some((left, right)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
        self.prod_range(left, right)
    }

    fn prod_range(&mut self, mut l: usize, mut r: usize) -> M::X {
        if l == r {
            return M::id_x();
        }
        l += self.offset;
        r += self.offset;
        for h in (1..=self.log).rev() {
            if ((l >> h) << h) != l {
                self.push_lazy(l >> h);
            }
            if ((r >> h) << h) != r {
                self.push_lazy((r - 1) >> h);
            }
        }
        let (mut val_left, mut val_right) = (M::id_x(), M::id_x());
        while l < r {
            if (l & 1) == 1 {
                val_left = M::op(&val_left, &self.data[l]);
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                val_right = M::op(&self.data[r], &val_right);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&val_left, &val_right)
    }

    /// 全区間の集約
    pub fn all_prod(&self) -> M::X {
        self.data[1].clone()
    }

    /// 左端を固定した2分探索
    /// - 返り値: (prod([l, x)), x)
    pub fn max_right<F>(&mut self, l: usize, f: F) -> (M::X, usize)
    where
        F: Fn(M::X) -> bool,
    {
        assert!(f(M::id_x()));
        if l == self.size {
            return (M::id_x(), self.size);
        }
        let mut l = l + self.offset;
        self.push_lazy_deep(l);
        let mut sum = M::id_x();
        loop {
            while l % 2 == 0 {
                l >>= 1;
            }
            let tmp = M::op(&sum, &self.data[l]);
            if !f(tmp.clone()) {
                while l < self.offset {
                    self.push_lazy(l);
                    l <<= 1;
                    let tmp = M::op(&sum, &self.data[l]);
                    if f(tmp.clone()) {
                        sum = tmp;
                        l += 1;
                    }
                }
                return (sum, l - self.offset);
            }
            sum = tmp;
            l += 1;
            if (l & l.wrapping_neg()) == l {
                break;
            }
        }
        (sum, self.size)
    }

    /// 右端を固定した2分探索
    /// - 返り値: (prod([x, r)), x)
    pub fn min_left<F>(&mut self, r: usize, f: F) -> (M::X, usize)
    where
        F: Fn(M::X) -> bool,
    {
        assert!(f(M::id_x()));
        if r == 0 {
            return (M::id_x(), 0);
        }
        let mut r = r + self.offset;
        self.push_lazy_deep(r - 1);
        let mut sum = M::id_x();
        loop {
            r -= 1;
            while r > 1 && (r % 2) == 1 {
                r >>= 1;
            }
            let tmp = M::op(&self.data[r], &sum);
            if !f(tmp.clone()) {
                while r < self.offset {
                    self.push_lazy(r);
                    r = r * 2 + 1;
                    let tmp = M::op(&self.data[r], &sum);
                    if f(tmp.clone()) {
                        sum = tmp;
                        r -= 1;
                    }
                }
                return (sum, r + 1 - self.offset);
            }
            sum = tmp;
            if (r & r.wrapping_neg()) == r {
                break;
            }
        }
        (sum, 0)
    }
}

impl<M> ShowBinaryTree<usize> for LazySegmentTree<M>
where
    M: ExtMonoid,
    M::F: Debug,
    M::X: Debug,
{
    fn get_root(&self) -> Option<usize> {
        Some(1)
    }
    fn get_left(&self, &i: &usize) -> Option<usize> {
        (i * 2 < self.offset * 2).then_some(i * 2)
    }
    fn get_right(&self, &i: &usize) -> Option<usize> {
        (i * 2 + 1 < self.offset * 2).then_some(i * 2 + 1)
    }
    fn print_node(&self, &i: &usize) -> String {
        format!("[data:{:?}, lazy:{:?}]", self.data[i], self.lazy[i])
    }
}
