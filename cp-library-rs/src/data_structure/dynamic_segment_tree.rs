//! 必要なところだけ作る（区間分割型）セグ木（arena 版）
//!
//! - dynamic segment tree（implicit segment tree）
//! - 添字区間は [min, max)

use std::{
    fmt::{self, Debug},
    ops::{Bound::*, Deref, DerefMut, RangeBounds},
};

use num::ToPrimitive;
use num_traits::PrimInt;

use crate::{
    algebraic_structure::actedmonoid_with_size::ActedMonoidWithSize,
    tree::arena::{Arena, ArenaNode, Ptr},
    tree::show_binary_tree::ShowBinaryTree,
};

type A<M> = Arena<NodeInner<M>>;

// ========== node ==========

/// 区間分割型ノード（`I` を持たない）
struct NodeInner<M: ActedMonoidWithSize> {
    /// 区間の集約値
    sum: M::Val,
    /// 遅延作用
    act: M::Act,
    left: Option<Ptr>,
    right: Option<Ptr>,
}

impl<M: ActedMonoidWithSize> ArenaNode for NodeInner<M> {}

impl<M: ActedMonoidWithSize> NodeInner<M> {
    fn with_length(len: usize) -> Self {
        Self {
            sum: M::e_with_size(len),
            act: M::id(),
            left: None,
            right: None,
        }
    }
}

// ========== dynamic segment tree ==========

pub struct DynamicSegmentTree<I: PrimInt, M: ActedMonoidWithSize> {
    min_index: I,
    max_index: I,
    pub n: I,
    arena: A<M>,
    root: Option<Ptr>,
}

impl<I: PrimInt + ToPrimitive, M: ActedMonoidWithSize> DynamicSegmentTree<I, M> {
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
        self.root = self.update_inner(root, self.min_index, self.max_index, index, val);
    }

    /// 点取得（未生成は `M::e_with_size()`）
    /// - 遅延があるので `push` するため `&mut self`
    pub fn get(&mut self, index: I) -> M::Val {
        assert!(self.min_index <= index && index < self.max_index);
        self.get_inner(self.root, self.min_index, self.max_index, index)
    }

    /// 区間の集約値（RangeBounds を受け取る）
    /// - 遅延があるので `push` するため `&mut self`
    pub fn get_range<R: RangeBounds<I> + Debug>(&mut self, range: R) -> M::Val {
        let (l, r) = self
            .parse_range(&range)
            .unwrap_or_else(|| panic!("The given range is wrong: {:?}", range));
        self.get_range_inner(self.root, self.min_index, self.max_index, l, r)
    }

    /// 区間に作用を適用（遅延）
    pub fn apply<R: RangeBounds<I> + Debug>(&mut self, range: R, act: M::Act) {
        let (l, r) = self
            .parse_range(&range)
            .unwrap_or_else(|| panic!("The given range is wrong: {:?}", range));
        let root = self.root.take();
        self.root = self.apply_inner(root, self.min_index, self.max_index, l, r, &act);
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

    // ========== internal helpers ==========

    #[inline]
    fn is_leaf(seg_l: I, seg_r: I) -> bool {
        seg_r - seg_l == I::one()
    }

    #[inline]
    fn two() -> I {
        I::one() + I::one()
    }

    #[inline]
    fn mid(l: I, r: I) -> I {
        l + (r - l) / Self::two()
    }

    #[inline]
    fn len(l: I, r: I) -> usize {
        (r - l).to_usize().unwrap()
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
    fn sum_of(arena: &A<M>, node: Option<Ptr>, len: usize) -> M::Val {
        node.map(|p| arena.get(p).sum.clone())
            .unwrap_or_else(|| M::e_with_size(len))
    }

    #[inline]
    fn apply_node(arena: &mut A<M>, ptr: Ptr, act: &M::Act) {
        let nsum = {
            let v = arena.get(ptr);
            M::mapping(&v.sum, act)
        };
        let nact = {
            let v = arena.get(ptr);
            M::compose(&v.act, act)
        };
        let v = arena.get_mut(ptr);
        v.sum = nsum;
        v.act = nact;
    }

    #[inline]
    fn ensure_left(arena: &mut A<M>, ptr: Ptr, len: usize) -> Ptr {
        if let Some(lp) = arena.get(ptr).left {
            lp
        } else {
            let lp = arena.alloc(NodeInner::with_length(len));
            arena.get_mut(ptr).left = Some(lp);
            lp
        }
    }

    #[inline]
    fn ensure_right(arena: &mut A<M>, ptr: Ptr, len: usize) -> Ptr {
        if let Some(rp) = arena.get(ptr).right {
            rp
        } else {
            let rp = arena.alloc(NodeInner::with_length(len));
            arena.get_mut(ptr).right = Some(rp);
            rp
        }
    }

    /// 子へ遅延伝播
    #[inline]
    fn push(&mut self, ptr: Ptr, seg_l: I, seg_r: I) {
        if Self::is_leaf(seg_l, seg_r) {
            return;
        }
        let act = { self.arena.get(ptr).act.clone() };
        if act == M::id() {
            return;
        }

        let mid = Self::mid(seg_l, seg_r);
        let llen = Self::len(seg_l, mid);
        let rlen = Self::len(mid, seg_r);
        let lp = Self::ensure_left(&mut self.arena, ptr, llen);
        let rp = Self::ensure_right(&mut self.arena, ptr, rlen);

        Self::apply_node(&mut self.arena, lp, &act);
        Self::apply_node(&mut self.arena, rp, &act);

        self.arena.get_mut(ptr).act = M::id();
    }

    /// 子の情報を吸い上げ
    #[inline]
    fn pull(&mut self, ptr: Ptr, l: I, r: I) {
        let lp = self.arena.get(ptr).left;
        let rp = self.arena.get(ptr).right;

        let mid = Self::mid(l, r);
        let llen = Self::len(l, mid);
        let rlen = Self::len(mid, r);

        let lsum = Self::sum_of(&self.arena, lp, llen);
        let rsum = Self::sum_of(&self.arena, rp, rlen);

        self.arena.get_mut(ptr).sum = M::op(&lsum, &rsum);
    }

    // ========== recursions ==========

    fn update_inner(
        &mut self,
        node: Option<Ptr>,
        seg_l: I,
        seg_r: I,
        index: I,
        val: M::Val,
    ) -> Option<Ptr> {
        let len = Self::len(seg_l, seg_r);
        let ptr = node.unwrap_or_else(|| self.arena.alloc(NodeInner::with_length(len)));

        if Self::is_leaf(seg_l, seg_r) {
            let v = self.arena.get_mut(ptr);
            v.sum = val;
            v.act = M::id();
            v.left = None;
            v.right = None;
            return Some(ptr);
        }

        self.push(ptr, seg_l, seg_r);

        let mid = Self::mid(seg_l, seg_r);
        if index < mid {
            let left = self.arena.get(ptr).left;
            let nl = self.update_inner(left, seg_l, mid, index, val);
            self.arena.get_mut(ptr).left = nl;
        } else {
            let right = self.arena.get(ptr).right;
            let nr = self.update_inner(right, mid, seg_r, index, val);
            self.arena.get_mut(ptr).right = nr;
        }

        self.pull(ptr, seg_l, seg_r);
        Some(ptr)
    }

    fn get_inner(&mut self, node: Option<Ptr>, seg_l: I, seg_r: I, index: I) -> M::Val {
        let len = Self::len(seg_l, seg_r);

        let Some(ptr) = node else {
            return M::e_with_size(len);
        };
        if Self::is_leaf(seg_l, seg_r) {
            return self.arena.get(ptr).sum.clone();
        }

        self.push(ptr, seg_l, seg_r);

        let mid = Self::mid(seg_l, seg_r);
        if index < mid {
            self.get_inner(self.arena.get(ptr).left, seg_l, mid, index)
        } else {
            self.get_inner(self.arena.get(ptr).right, mid, seg_r, index)
        }
    }

    fn get_range_inner(&mut self, node: Option<Ptr>, seg_l: I, seg_r: I, ql: I, qr: I) -> M::Val {
        let len = Self::len(seg_l, seg_r);

        if qr <= seg_l || seg_r <= ql {
            return M::e_with_size(len);
        }

        let Some(ptr) = node else {
            return M::e_with_size(len);
        };

        if ql <= seg_l && seg_r <= qr {
            return self.arena.get(ptr).sum.clone();
        }

        if Self::is_leaf(seg_l, seg_r) {
            return self.arena.get(ptr).sum.clone();
        }

        self.push(ptr, seg_l, seg_r);

        let mid = Self::mid(seg_l, seg_r);
        let a = self.get_range_inner(self.arena.get(ptr).left, seg_l, mid, ql, qr);
        let b = self.get_range_inner(self.arena.get(ptr).right, mid, seg_r, ql, qr);
        M::op(&a, &b)
    }

    fn apply_inner(
        &mut self,
        node: Option<Ptr>,
        seg_l: I,
        seg_r: I,
        ql: I,
        qr: I,
        act: &M::Act,
    ) -> Option<Ptr> {
        if qr <= seg_l || seg_r <= ql {
            return node;
        }

        let len = Self::len(seg_l, seg_r);
        let ptr = node.unwrap_or_else(|| self.arena.alloc(NodeInner::with_length(len)));

        if ql <= seg_l && seg_r <= qr {
            Self::apply_node(&mut self.arena, ptr, act);
            return Some(ptr);
        }

        if Self::is_leaf(seg_l, seg_r) {
            Self::apply_node(&mut self.arena, ptr, act);
            return Some(ptr);
        }

        self.push(ptr, seg_l, seg_r);

        let mid = Self::mid(seg_l, seg_r);

        let left = self.arena.get(ptr).left;
        let nl = self.apply_inner(left, seg_l, mid, ql, qr, act);
        self.arena.get_mut(ptr).left = nl;

        let right = self.arena.get(ptr).right;
        let nr = self.apply_inner(right, mid, seg_r, ql, qr, act);
        self.arena.get_mut(ptr).right = nr;

        self.pull(ptr, seg_l, seg_r);
        Some(ptr)
    }
}

// ========== ValMut ==========

pub struct ValMut<'a, I, M>
where
    I: PrimInt,
    M: ActedMonoidWithSize,
{
    segself: &'a mut DynamicSegmentTree<I, M>,
    idx: I,
    new_val: M::Val,
}

impl<I, M> Debug for ValMut<'_, I, M>
where
    I: PrimInt,
    M: ActedMonoidWithSize,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ValMut").field(&self.new_val).finish()
    }
}

impl<I, M> Drop for ValMut<'_, I, M>
where
    I: PrimInt,
    M: ActedMonoidWithSize,
{
    fn drop(&mut self) {
        self.segself.update(self.idx, self.new_val.clone());
    }
}

impl<I, M> Deref for ValMut<'_, I, M>
where
    I: PrimInt,
    M: ActedMonoidWithSize,
{
    type Target = M::Val;
    fn deref(&self) -> &Self::Target {
        &self.new_val
    }
}

impl<I, M> DerefMut for ValMut<'_, I, M>
where
    I: PrimInt,
    M: ActedMonoidWithSize,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

// ========== max_right / min_left ==========
//
// 遅延があるので `push` が必要．そのため `&mut self` にする．

impl<I, M> DynamicSegmentTree<I, M>
where
    I: PrimInt,
    M: ActedMonoidWithSize,
    M::Val: Debug,
{
    /// 左端固定二分探索
    /// 返り値：(`get_range(l..x)`, `x`)
    pub fn max_right<F>(&mut self, l: I, f: F) -> (M::Val, I)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(f(M::e_with_size(0)));
        assert!(self.min_index <= l && l <= self.max_index);

        let mut acc = M::e_with_size(0);
        let x = self.max_right_inner(self.root, self.min_index, self.max_index, l, &f, &mut acc);
        (acc, x)
    }

    fn max_right_inner<F>(
        &mut self,
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

        let Some(ptr) = node else {
            // 未生成区間は全て `M::e_with_size()` なので，どこまで進んでも `acc` は変わらない
            return seg_r;
        };

        if ql <= seg_l {
            let tmp = M::op(acc, &self.arena.get(ptr).sum);
            if f(tmp.clone()) {
                *acc = tmp;
                return seg_r;
            }
        }

        if Self::is_leaf(seg_l, seg_r) {
            return seg_l;
        }

        self.push(ptr, seg_l, seg_r);

        let mid = Self::mid(seg_l, seg_r);
        let left_res = self.max_right_inner(self.arena.get(ptr).left, seg_l, mid, ql, f, acc);
        if left_res != mid {
            return left_res;
        }
        self.max_right_inner(self.arena.get(ptr).right, mid, seg_r, ql, f, acc)
    }

    /// 右端固定二分探索
    /// 返り値：(`get_range(x..r)`, `x`)
    pub fn min_left<F>(&mut self, r: I, f: F) -> (M::Val, I)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(f(M::e_with_size(0)));
        assert!(self.min_index <= r && r <= self.max_index);

        let mut acc = M::e_with_size(0);
        let x = self.min_left_inner(self.root, self.min_index, self.max_index, r, &f, &mut acc);
        (acc, x)
    }

    fn min_left_inner<F>(
        &mut self,
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

        let Some(ptr) = node else {
            return seg_l;
        };

        if seg_r <= qr {
            let tmp = M::op(&self.arena.get(ptr).sum, acc);
            if f(tmp.clone()) {
                *acc = tmp;
                return seg_l;
            }
        }

        if Self::is_leaf(seg_l, seg_r) {
            return seg_r;
        }

        self.push(ptr, seg_l, seg_r);

        let mid = Self::mid(seg_l, seg_r);
        let right_res = self.min_left_inner(self.arena.get(ptr).right, mid, seg_r, qr, f, acc);
        if right_res != mid {
            return right_res;
        }
        self.min_left_inner(self.arena.get(ptr).left, seg_l, mid, qr, f, acc)
    }
}

// ========== ShowBinaryTree ==========

impl<I, M> ShowBinaryTree<Ptr> for DynamicSegmentTree<I, M>
where
    I: PrimInt,
    M: ActedMonoidWithSize,
    M::Val: Debug,
    M::Act: Debug,
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
        let node = self.arena.get(*ptr);
        format!("[val:{:?}, act:{:?}]", node.sum, node.act)
    }
}
