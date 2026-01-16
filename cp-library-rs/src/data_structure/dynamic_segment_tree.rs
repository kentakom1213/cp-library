//! 必要なところだけ作る（区間分割型）セグ木（arena 版）

use crate::algebraic_structure::monoid::Monoid;
use crate::tree::arena::{Arena, ArenaNode, Ptr};
use crate::utils::show_binary_tree::ShowBinaryTree;

use std::fmt::{self, Debug};
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, RangeBounds,
};

use num_traits::PrimInt;

/// 区間分割型 dynamic segment tree（implicit segment tree）
/// - 取り扱う添字区間は [min, max)
pub struct DynamicSegmentTree<I: PrimInt, M: Monoid> {
    min_index: I,
    max_index: I,
    pub n: I,
    root: Option<Ptr>,
    arena: Arena<NodeInner<M>>,
}

/// 区間分割型ノード（I を持たない）
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

impl<I: PrimInt, M: Monoid> DynamicSegmentTree<I, M> {
    /// インデックスの最小値，最大値（半開区間 [min, max)）から生成する
    pub fn new(min: I, max: I) -> Self {
        assert!(min < max);
        Self {
            min_index: min,
            max_index: max,
            n: max - min,
            root: None,
            arena: Arena::new(),
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

    /// 点取得（未生成は id）
    /// - 計算量：\(O(\log (max-min))\)
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

    /// 区間積（半開区間 [l, r)）
    /// - 計算量：\(O(\log (max-min))\) 程度
    pub fn prod(&self, l: I, r: I) -> M::Val {
        assert!(self.min_index <= l && l <= r && r <= self.max_index);
        Self::prod_inner(&self.arena, self.root, self.min_index, self.max_index, l, r)
    }

    /// 全体の積
    pub fn all_prod(&self) -> M::Val {
        self.root
            .map(|i| self.arena.get(i).sum.clone())
            .unwrap_or_else(M::e)
    }

    /// get_mut（Drop で update）
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

    /// RangeBounds を受け取る区間積
    pub fn get_range<R>(&self, range: R) -> M::Val
    where
        R: RangeBounds<I> + Debug,
    {
        let (l, r) = match self.parse_range(&range) {
            Some(x) => x,
            None => panic!("The given range is wrong: {:?}", range),
        };
        self.prod(l, r)
    }

    // ----------------
    // 内部ユーティリティ
    // ----------------

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

    fn update_inner(
        arena: &mut Arena<NodeInner<M>>,
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
            let new_left = Self::update_inner(arena, left, l, mid, index, val);
            arena.get_mut(idx).left = new_left;
        } else {
            let right = arena.get(idx).right;
            let new_right = Self::update_inner(arena, right, mid, r, index, val);
            arena.get_mut(idx).right = new_right;
        }

        // eval（借用を切ってから集約）
        let left = arena.get(idx).left;
        let right = arena.get(idx).right;
        let lsum = left.map(|p| arena.get(p).sum.clone()).unwrap_or(M::e());
        let rsum = right.map(|p| arena.get(p).sum.clone()).unwrap_or(M::e());
        arena.get_mut(idx).sum = M::op(&lsum, &rsum);

        Some(idx)
    }

    fn get_inner(arena: &Arena<NodeInner<M>>, node: Option<Ptr>, l: I, r: I, index: I) -> M::Val {
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

    fn prod_inner(
        arena: &Arena<NodeInner<M>>,
        node: Option<Ptr>,
        l: I,
        r: I,
        ql: I,
        qr: I,
    ) -> M::Val {
        if qr <= l || r <= ql {
            return M::e();
        }
        let Some(idx) = node else {
            return M::e();
        };
        if ql <= l && r <= qr {
            return arena.get(idx).sum.clone();
        }
        if r - l == I::one() {
            return arena.get(idx).sum.clone();
        }
        let mid = Self::mid(l, r);
        let a = Self::prod_inner(arena, arena.get(idx).left, l, mid, ql, qr);
        let b = Self::prod_inner(arena, arena.get(idx).right, mid, r, ql, qr);
        M::op(&a, &b)
    }
}

/// SegmentTree の ValMut と同じ設計（idx は I）
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

impl<I, M> Debug for DynamicSegmentTree<I, M>
where
    I: PrimInt,
    M: Monoid,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DynamicSegmentTree {{ [").ok();
        let mut i = self.min_index;
        while i < self.max_index {
            let v = self.get(i);
            let ni = i + I::one();
            if ni < self.max_index {
                write!(f, "{:?}, ", v).ok();
            } else {
                write!(f, "{:?}", v).ok();
            }
            i = ni;
        }
        write!(f, "] }}")
    }
}

// セグ木上の 2 分探索（OrderedMonoid）
impl<I, M> DynamicSegmentTree<I, M>
where
    I: PrimInt + Debug,
    M: Monoid,
    M::Val: Debug,
{
    /// 左端固定二分探索
    /// 返り値： (prod([l, x)), x)
    pub fn max_right<F>(&self, l: I, f: F) -> (M::Val, I)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(f(M::e()));
        assert!(self.min_index <= l && l <= self.max_index);

        if l == self.max_index {
            return (M::e(), self.max_index);
        }

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
        arena: &Arena<NodeInner<M>>,
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

        let left_res = Self::max_right_inner(arena, arena.get(idx).left, seg_l, mid, ql, f, acc);
        if left_res != mid {
            return left_res;
        }
        Self::max_right_inner(arena, arena.get(idx).right, mid, seg_r, ql, f, acc)
    }

    /// 右端固定二分探索
    /// 返り値： (prod([x, r)), x)
    pub fn min_left<F>(&self, r: I, f: F) -> (M::Val, I)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(f(M::e()));
        assert!(self.min_index <= r && r <= self.max_index);

        if r == self.min_index {
            return (M::e(), self.min_index);
        }

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
        arena: &Arena<NodeInner<M>>,
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

        let right_res = Self::min_left_inner(arena, arena.get(idx).right, mid, seg_r, qr, f, acc);
        if right_res != mid {
            return right_res;
        }
        Self::min_left_inner(arena, arena.get(idx).left, seg_l, mid, qr, f, acc)
    }
}

/// ShowBinaryTree 用の「ポインタ」
/// - idx: arena 上のノード番号
/// - l,r: このノードが担当する区間 [l,r)
#[derive(Clone, Copy)]
pub struct DynSegPtr<I>
where
    I: PrimInt,
{
    ptr: Ptr,
    l: I,
    r: I,
}

impl<I, M> DynamicSegmentTree<I, M>
where
    I: PrimInt,
    M: Monoid,
{
    #[inline]
    fn mk_ptr(&self, ptr: Ptr, l: I, r: I) -> DynSegPtr<I> {
        DynSegPtr { ptr, l, r }
    }
}

impl<I, M> ShowBinaryTree<DynSegPtr<I>> for DynamicSegmentTree<I, M>
where
    I: PrimInt + Debug,
    M: Monoid,
    M::Val: Debug,
{
    fn get_left(&self, ptr: &DynSegPtr<I>) -> Option<DynSegPtr<I>> {
        let t = &self.arena.get(ptr.ptr);
        let left = t.left?;
        let mid = Self::mid(ptr.l, ptr.r);
        Some(self.mk_ptr(left, ptr.l, mid))
    }

    fn get_right(&self, ptr: &DynSegPtr<I>) -> Option<DynSegPtr<I>> {
        let t = &self.arena.get(ptr.ptr);
        let right = t.right?;
        let mid = Self::mid(ptr.l, ptr.r);
        Some(self.mk_ptr(right, mid, ptr.r))
    }

    fn get_root(&self) -> Option<DynSegPtr<I>> {
        let root = self.root?;
        Some(self.mk_ptr(root, self.min_index, self.max_index))
    }

    fn print_node(&self, ptr: &DynSegPtr<I>) -> String {
        let t = &self.arena.get(ptr.ptr);
        format!("[{:?}, {:?}) sum={:?}", ptr.l, ptr.r, t.sum)
    }
}
