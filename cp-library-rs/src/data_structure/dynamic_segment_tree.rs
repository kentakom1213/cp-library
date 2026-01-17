//! 必要なところだけ作る（区間分割型）セグ木（arena 版）
//!
//! - dynamic segment tree（implicit segment tree）
//! - 添字区間は [min, max)

use std::{
    fmt::{self, Debug},
    ops::{Bound::*, Deref, DerefMut, RangeBounds},
};

use num_traits::PrimInt;

use crate::{
    algebraic_structure::monoid::Monoid,
    tree::arena::{Arena, ArenaNode, Ptr},
    utils::show_binary_tree::ShowBinaryTree,
};

type A<M> = Arena<NodeInner<M>>;

// ========== node ==========

/// 区間分割型ノード（`I` を持たない）
struct NodeInner<M: Monoid> {
    /// 区間の集約値
    sum: M::Val,
    left: Option<Ptr>,
    right: Option<Ptr>,
}

impl<M: Monoid> ArenaNode for NodeInner<M> {}

impl<M: Monoid> Default for NodeInner<M> {
    fn default() -> Self {
        Self {
            sum: M::e(),
            left: None,
            right: None,
        }
    }
}

// ========== dynamic segment tree ==========

/// Dynamic Segment Tree
///
/// - 必要になったところだけノードを生成するセグ木
pub struct DynamicSegmentTree<I: PrimInt, M: Monoid> {
    min_index: I,
    max_index: I,
    pub n: I,
    arena: A<M>,
    root: Option<Ptr>,
}

impl<I: PrimInt, M: Monoid> DynamicSegmentTree<I, M> {
    /// 添字区間 [min, max) から生成する
    pub fn new(min: I, max: I) -> Self {
        assert!(min < max);
        Self {
            min_index: min,
            max_index: max,
            n: max - min,
            arena: A::new(),
            root: None,
        }
    }

    /// 点更新
    /// - 計算量：\(O(\log (max-min))\)
    pub fn update(&mut self, index: I, val: M::Val) {
        assert!(self.min_index <= index && index < self.max_index);
        let root = self.root.take();
        self.root = Self::update_inner(
            &mut self.arena,
            root,
            self.min_index,
            self.max_index,
            index,
            val,
        );
    }

    /// 点取得（未生成は `M::e()`）
    pub fn get(&self, index: I) -> M::Val {
        assert!(self.min_index <= index && index < self.max_index);
        Self::get_inner(
            &self.arena,
            self.root,
            self.min_index,
            self.max_index,
            index,
        )
    }

    /// 区間の集約値（RangeBounds を受け取る）
    pub fn get_range<R: RangeBounds<I> + Debug>(&self, range: R) -> M::Val {
        let (l, r) = self.parse_range(&range).unwrap_or_else(|| {
            panic!("The given range is wrong: {:?}", range);
        });
        self.get_range_lr(l, r)
    }

    /// `get_mut`（Drop で `update`）
    pub fn get_mut(&mut self, i: I) -> Option<ValMut<'_, I, M>> {
        if self.min_index <= i && i < self.max_index {
            let default = self.get(i);
            Some(ValMut {
                segself: self,
                idx: i,
                new_val: default,
            })
        } else {
            None
        }
    }

    // ========== internal ==========

    #[inline]
    fn two() -> I {
        I::one() + I::one()
    }

    #[inline]
    fn mid(l: I, r: I) -> I {
        l + (r - l) / Self::two()
    }

    #[inline]
    fn parse_range<R: RangeBounds<I>>(&self, range: &R) -> Option<(I, I)> {
        let start = match range.start_bound() {
            Unbounded => self.min_index,
            Excluded(&v) => v + I::one(),
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.max_index,
            Excluded(&v) => v,
            Included(&v) => v + I::one(),
        };
        if self.min_index <= start && start <= end && end <= self.max_index {
            Some((start, end))
        } else {
            None
        }
    }

    #[inline]
    fn sum_of(arena: &A<M>, node: Option<Ptr>) -> M::Val {
        node.map(|p| arena.get(p).sum.clone()).unwrap_or_else(M::e)
    }

    fn get_range_lr(&self, l: I, r: I) -> M::Val {
        Self::get_range_inner(&self.arena, self.root, self.min_index, self.max_index, l, r)
    }

    fn update_inner(
        arena: &mut A<M>,
        node: Option<Ptr>,
        l: I,
        r: I,
        index: I,
        val: M::Val,
    ) -> Option<Ptr> {
        let idx = node.unwrap_or_else(|| arena.alloc_default());

        if r - l == I::one() {
            arena.get_mut(idx).sum = val;
            return Some(idx);
        }

        let mid = Self::mid(l, r);

        if index < mid {
            let left = arena.get(idx).left;
            let nl = Self::update_inner(arena, left, l, mid, index, val);
            arena.get_mut(idx).left = nl;
        } else {
            let right = arena.get(idx).right;
            let nr = Self::update_inner(arena, right, mid, r, index, val);
            arena.get_mut(idx).right = nr;
        }

        let lsum = Self::sum_of(arena, arena.get(idx).left);
        let rsum = Self::sum_of(arena, arena.get(idx).right);
        arena.get_mut(idx).sum = M::op(&lsum, &rsum);

        Some(idx)
    }

    fn get_inner(arena: &A<M>, node: Option<Ptr>, l: I, r: I, index: I) -> M::Val {
        let Some(idx) = node else {
            return M::e();
        };
        if r - l == I::one() {
            return arena.get(idx).sum.clone();
        }
        let mid = Self::mid(l, r);
        if index < mid {
            Self::get_inner(arena, arena.get(idx).left, l, mid, index)
        } else {
            Self::get_inner(arena, arena.get(idx).right, mid, r, index)
        }
    }

    fn get_range_inner(
        arena: &A<M>,
        node: Option<Ptr>,
        seg_l: I,
        seg_r: I,
        ql: I,
        qr: I,
    ) -> M::Val {
        if qr <= seg_l || seg_r <= ql {
            return M::e();
        }
        let Some(idx) = node else {
            return M::e();
        };
        if ql <= seg_l && seg_r <= qr {
            return arena.get(idx).sum.clone();
        }
        if seg_r - seg_l == I::one() {
            return arena.get(idx).sum.clone();
        }

        let mid = Self::mid(seg_l, seg_r);
        let a = Self::get_range_inner(arena, arena.get(idx).left, seg_l, mid, ql, qr);
        let b = Self::get_range_inner(arena, arena.get(idx).right, mid, seg_r, ql, qr);
        M::op(&a, &b)
    }
}

// ========== ValMut ==========

pub struct ValMut<'a, I, M>
where
    I: PrimInt,
    M: Monoid,
{
    segself: &'a mut DynamicSegmentTree<I, M>,
    idx: I,
    new_val: M::Val,
}

impl<I, M> Debug for ValMut<'_, I, M>
where
    I: PrimInt,
    M: Monoid,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ValMut").field(&self.new_val).finish()
    }
}

impl<I, M> Drop for ValMut<'_, I, M>
where
    I: PrimInt,
    M: Monoid,
{
    fn drop(&mut self) {
        self.segself.update(self.idx, self.new_val.clone());
    }
}

impl<I, M> Deref for ValMut<'_, I, M>
where
    I: PrimInt,
    M: Monoid,
{
    type Target = M::Val;
    fn deref(&self) -> &Self::Target {
        &self.new_val
    }
}

impl<I, M> DerefMut for ValMut<'_, I, M>
where
    I: PrimInt,
    M: Monoid,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

// ========== max_right / min_left ==========

impl<I, M> DynamicSegmentTree<I, M>
where
    I: PrimInt + Debug,
    M: Monoid,
    M::Val: Debug,
{
    pub fn max_right<F>(&self, l: I, f: F) -> (M::Val, I)
    where
        F: Fn(M::Val) -> bool,
    {
        let mut acc = M::e();
        let x = Self::max_right_inner(
            &self.arena,
            self.root,
            self.min_index,
            self.max_index,
            l,
            &f,
            &mut acc,
        );
        (acc, x)
    }

    fn max_right_inner<F>(
        arena: &A<M>,
        node: Option<Ptr>,
        seg_l: I,
        seg_r: I,
        ql: I,
        f: &F,
        acc: &mut M::Val,
    ) -> I
    where
        F: Fn(M::Val) -> bool,
    {
        if seg_r <= ql {
            return seg_r;
        }
        let Some(idx) = node else {
            return seg_r;
        };

        if ql <= seg_l {
            let tmp = M::op(acc, &arena.get(idx).sum);
            if f(tmp.clone()) {
                *acc = tmp;
                return seg_r;
            }
        }

        if seg_r - seg_l == I::one() {
            return seg_l;
        }

        let mid = DynamicSegmentTree::<I, M>::mid(seg_l, seg_r);
        let res = Self::max_right_inner(arena, arena.get(idx).left, seg_l, mid, ql, f, acc);
        if res != mid {
            return res;
        }
        Self::max_right_inner(arena, arena.get(idx).right, mid, seg_r, ql, f, acc)
    }

    pub fn min_left<F>(&self, r: I, f: F) -> (M::Val, I)
    where
        F: Fn(M::Val) -> bool,
    {
        let mut acc = M::e();
        let x = Self::min_left_inner(
            &self.arena,
            self.root,
            self.min_index,
            self.max_index,
            r,
            &f,
            &mut acc,
        );
        (acc, x)
    }

    fn min_left_inner<F>(
        arena: &A<M>,
        node: Option<Ptr>,
        seg_l: I,
        seg_r: I,
        qr: I,
        f: &F,
        acc: &mut M::Val,
    ) -> I
    where
        F: Fn(M::Val) -> bool,
    {
        if qr <= seg_l {
            return seg_l;
        }
        let Some(idx) = node else {
            return seg_l;
        };

        if seg_r <= qr {
            let tmp = M::op(&arena.get(idx).sum, acc);
            if f(tmp.clone()) {
                *acc = tmp;
                return seg_l;
            }
        }

        if seg_r - seg_l == I::one() {
            return seg_r;
        }

        let mid = DynamicSegmentTree::<I, M>::mid(seg_l, seg_r);
        let res = Self::min_left_inner(arena, arena.get(idx).right, mid, seg_r, qr, f, acc);
        if res != mid {
            return res;
        }
        Self::min_left_inner(arena, arena.get(idx).left, seg_l, mid, qr, f, acc)
    }
}

// ========== ShowBinaryTree ==========

impl<I, M> ShowBinaryTree<Ptr> for DynamicSegmentTree<I, M>
where
    I: PrimInt + Debug,
    M: Monoid,
    M::Val: Debug,
{
    fn get_root(&self) -> Option<Ptr> {
        self.root
    }

    fn get_left(&self, ptr: &Ptr) -> Option<Ptr> {
        self.arena.get(*ptr).left
    }

    fn get_right(&self, ptr: &Ptr) -> Option<Ptr> {
        self.arena.get(*ptr).right
    }

    fn print_node(&self, ptr: &Ptr) -> String {
        format!("[{:?}]", self.arena.get(*ptr).sum)
    }
}
