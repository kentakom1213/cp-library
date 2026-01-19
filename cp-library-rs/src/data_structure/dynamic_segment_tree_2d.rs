//! 2D dynamic segment tree（implicit，交互 2 分木）
//!
//! 方針：
//! - 本体 API は `apply(rx, ry, act)` と `get_range(rx, ry)`
//! - `get(x, y)` は `get_range(x..x+1, y..y+1)` のラッパー
//! - split 不能（軸長 1）なら，同深さのまま軸反転して split を試みる
//! - `ActedMonoidWithSize::e_len(len)` の `len` は「面積 area」とみなす

use std::fmt::Debug;
use std::ops::{Bound::*, RangeBounds};

use num::ToPrimitive;
use num_traits::PrimInt;

use crate::{
    algebraic_structure::actedmonoid_with_size::ActedMonoidWithSize,
    tree::arena::{Arena, ArenaNode, Ptr},
};

type A<M> = Arena<NodeInner<M>>;

type Range<I> = (I, I);
type Rect<I> = (Range<I>, Range<I>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    X,
    Y,
}

impl Axis {
    #[inline]
    fn flip(self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }
}

// ========== node ==========

struct NodeInner<M: ActedMonoidWithSize> {
    sum: M::Val,
    act: M::Act,
    left: Option<Ptr>,
    right: Option<Ptr>,
}

impl<M: ActedMonoidWithSize> ArenaNode for NodeInner<M> {}

impl<M: ActedMonoidWithSize> NodeInner<M> {
    fn with_area(area: usize) -> Self {
        Self {
            sum: M::e_len(area),
            act: M::id(),
            left: None,
            right: None,
        }
    }
}

// ========== main structure ==========

pub struct DynamicSegmentTree2D<I: PrimInt, M: ActedMonoidWithSize> {
    xmin: I,
    xmax: I,
    ymin: I,
    ymax: I,

    arena: A<M>,
    root: Option<Ptr>,
}

impl<I, M> DynamicSegmentTree2D<I, M>
where
    I: PrimInt + ToPrimitive + Debug,
    M: ActedMonoidWithSize,
{
    pub fn new((xmin, xmax): Range<I>, (ymin, ymax): Range<I>) -> Self {
        assert!(xmin < xmax);
        assert!(ymin < ymax);
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
            arena: A::new(),
            root: None,
        }
    }

    // ---------- public API ----------

    /// 矩形 apply（遅延）
    pub fn apply<RX, RY>(&mut self, rx: RX, ry: RY, act: M::Act)
    where
        RX: RangeBounds<I> + Debug,
        RY: RangeBounds<I> + Debug,
    {
        let (qxl, qxr) = self
            .parse_range_x(&rx)
            .unwrap_or_else(|| panic!("bad x-range: {:?}", rx));
        let (qyl, qyr) = self
            .parse_range_y(&ry)
            .unwrap_or_else(|| panic!("bad y-range: {:?}", ry));

        let root = self.root.take();
        self.root = self.apply_inner(
            root,
            ((self.xmin, self.xmax), (self.ymin, self.ymax)),
            ((qxl, qxr), (qyl, qyr)),
            &act,
            Axis::X,
        );
    }

    /// 矩形 query（遅延があるので `&mut self`）
    pub fn get_range<RX, RY>(&mut self, rx: RX, ry: RY) -> M::Val
    where
        RX: RangeBounds<I> + Debug,
        RY: RangeBounds<I> + Debug,
    {
        let (qxl, qxr) = self
            .parse_range_x(&rx)
            .unwrap_or_else(|| panic!("bad x-range: {:?}", rx));
        let (qyl, qyr) = self
            .parse_range_y(&ry)
            .unwrap_or_else(|| panic!("bad y-range: {:?}", ry));

        self.get_range_inner(
            self.root,
            (self.xmin, self.xmax),
            (self.ymin, self.ymax),
            (qxl, qxr),
            (qyl, qyr),
            Axis::X,
        )
    }

    /// 点取得：`get_range(x..x+1, y..y+1)` のラッパー
    pub fn get(&mut self, x: I, y: I) -> M::Val {
        assert!(self.xmin <= x && x < self.xmax);
        assert!(self.ymin <= y && y < self.ymax);
        let one = I::one();
        // 範囲外に出ないことを保証（x < xmax なので x+1 <= xmax）
        self.get_range(x..(x + one), y..(y + one))
    }

    // ---------- range parsing ----------

    #[inline]
    fn parse_range<R: RangeBounds<I>>(range: &R, min: I, max: I) -> Option<Range<I>> {
        let start = match range.start_bound() {
            Unbounded => min,
            Excluded(&v) => v + I::one(),
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => max,
            Excluded(&v) => v,
            Included(&v) => v + I::one(),
        };
        if min <= start && start <= end && end <= max {
            Some((start, end))
        } else {
            None
        }
    }

    #[inline]
    fn parse_range_x<R: RangeBounds<I>>(&self, range: &R) -> Option<Range<I>> {
        Self::parse_range(range, self.xmin, self.xmax)
    }

    #[inline]
    fn parse_range_y<R: RangeBounds<I>>(&self, range: &R) -> Option<Range<I>> {
        Self::parse_range(range, self.ymin, self.ymax)
    }

    // ---------- math helpers ----------

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
    fn area((xl, xr): Range<I>, (yl, yr): Range<I>) -> usize {
        let xlen = Self::len(xl, xr);
        let ylen = Self::len(yl, yr);
        xlen * ylen
    }

    #[inline]
    fn intersects(
        (xl, xr): Range<I>,
        (yl, yr): Range<I>,
        (qxl, qxr): Range<I>,
        (qyl, qyr): Range<I>,
    ) -> bool {
        !(qxr <= xl || xr <= qxl || qyr <= yl || yr <= qyl)
    }

    #[inline]
    fn covered_by(
        (xl, xr): Range<I>,
        (yl, yr): Range<I>,
        (qxl, qxr): Range<I>,
        (qyl, qyr): Range<I>,
    ) -> bool {
        qxl <= xl && xr <= qxr && qyl <= yl && yr <= qyr
    }

    // ---------- node helpers ----------

    #[inline]
    fn sum_of(&self, node: Option<Ptr>, area: usize) -> M::Val {
        node.map(|p| self.arena.get(p).sum.clone())
            .unwrap_or_else(|| M::e_len(area))
    }

    #[inline]
    fn apply_node(&mut self, ptr: Ptr, act: &M::Act) {
        let nsum = {
            let v = self.arena.get(ptr);
            M::mapping(&v.sum, act)
        };
        let nact = {
            let v = self.arena.get(ptr);
            M::compose(&v.act, act)
        };
        let v = self.arena.get_mut(ptr);
        v.sum = nsum;
        v.act = nact;
    }

    #[inline]
    fn ensure_left(&mut self, ptr: Ptr, area: usize) -> Ptr {
        if let Some(lp) = self.arena.get(ptr).left {
            lp
        } else {
            let lp = self.arena.alloc(NodeInner::<M>::with_area(area));
            self.arena.get_mut(ptr).left = Some(lp);
            lp
        }
    }

    #[inline]
    fn ensure_right(&mut self, ptr: Ptr, area: usize) -> Ptr {
        if let Some(rp) = self.arena.get(ptr).right {
            rp
        } else {
            let rp = self.arena.alloc(NodeInner::<M>::with_area(area));
            self.arena.get_mut(ptr).right = Some(rp);
            rp
        }
    }

    // ---------- axis / split rule ----------

    #[inline]
    fn can_split(axis: Axis, (xl, xr): Range<I>, (yl, yr): Range<I>) -> bool {
        match axis {
            Axis::X => xr - xl > I::one(),
            Axis::Y => yr - yl > I::one(),
        }
    }

    /// 代替案ルール：split 不能なら「同深さで」軸反転して split
    ///
    /// 戻り値：
    /// - `axis_eff`：実際に split に用いる軸
    /// - `mid`：その軸での中点（split 不能な場合でも返すが，呼び出し側で葉判定すること）
    fn choose_axis_and_mid(&self, axis: Axis, (xl, xr): Range<I>, (yl, yr): Range<I>) -> (Axis, I) {
        if Self::can_split(axis, (xl, xr), (yl, yr)) {
            let mid = match axis {
                Axis::X => Self::mid(xl, xr),
                Axis::Y => Self::mid(yl, yr),
            };
            return (axis, mid);
        }
        let axis2 = axis.flip();
        if Self::can_split(axis2, (xl, xr), (yl, yr)) {
            let mid = match axis2 {
                Axis::X => Self::mid(xl, xr),
                Axis::Y => Self::mid(yl, yr),
            };
            return (axis2, mid);
        }
        // 両方 split 不能（= 真の葉）．mid はダミー
        (axis, xl)
    }

    fn child_regions(
        &self,
        axis_eff: Axis,
        (xl, xr): Range<I>,
        (yl, yr): Range<I>,
        mid: I,
    ) -> ((Rect<I>, usize), (Rect<I>, usize)) {
        match axis_eff {
            Axis::X => {
                let xm = mid;
                let a1 = Self::area((xl, xm), (yl, yr));
                let a2 = Self::area((xm, xr), (yl, yr));
                ((((xl, xm), (yl, yr)), a1), (((xm, xr), (yl, yr)), a2))
            }
            Axis::Y => {
                let ym = mid;
                let a1 = Self::area((xl, xr), (yl, ym));
                let a2 = Self::area((xl, xr), (ym, yr));
                ((((xl, xr), (yl, ym)), a1), (((xl, xr), (ym, yr)), a2))
            }
        }
    }

    // ---------- lazy propagation ----------

    fn push(&mut self, ptr: Ptr, (xl, xr): Range<I>, (yl, yr): Range<I>, axis: Axis) {
        // 真の葉
        if xr - xl == I::one() && yr - yl == I::one() {
            return;
        }

        let act = { self.arena.get(ptr).act.clone() };
        if act == M::id() {
            return;
        }

        let (axis_eff, mid) = self.choose_axis_and_mid(axis, (xl, xr), (yl, yr));
        if !Self::can_split(axis_eff, (xl, xr), (yl, yr)) {
            // 真の葉（念のため）
            return;
        }

        let (c1, c2) = self.child_regions(axis_eff, (xl, xr), (yl, yr), mid);
        let (((_xl1, _xr1), (_yl1, _yr1)), a1) = c1;
        let (((_xl2, _xr2), (_yl2, _yr2)), a2) = c2;

        let lp = Self::ensure_left(self, ptr, a1);
        let rp = Self::ensure_right(self, ptr, a2);

        self.apply_node(lp, &act);
        self.apply_node(rp, &act);

        self.arena.get_mut(ptr).act = M::id();
    }

    fn pull(&mut self, ptr: Ptr, (xl, xr): Range<I>, (yl, yr): Range<I>, axis: Axis) {
        // 真の葉なら sum は既に正しい前提（apply_node が更新する）
        if xr - xl == I::one() && yr - yl == I::one() {
            return;
        }

        let (axis_eff, mid) = self.choose_axis_and_mid(axis, (xl, xr), (yl, yr));
        if !Self::can_split(axis_eff, (xl, xr), (yl, yr)) {
            return;
        }

        let (c1, c2) = self.child_regions(axis_eff, (xl, xr), (yl, yr), mid);
        let (((_xl1, _xr1), (_yl1, _yr1)), a1) = c1;
        let (((_xl2, _xr2), (_yl2, _yr2)), a2) = c2;

        let lp = self.arena.get(ptr).left;
        let rp = self.arena.get(ptr).right;

        let lsum = self.sum_of(lp, a1);
        let rsum = self.sum_of(rp, a2);

        self.arena.get_mut(ptr).sum = M::op(&lsum, &rsum);
    }

    // ---------- recursions ----------

    fn apply_inner(
        &mut self,
        node: Option<Ptr>,
        ((xl, xr), (yl, yr)): Rect<I>,
        ((qxl, qxr), (qyl, qyr)): Rect<I>,
        act: &M::Act,
        axis: Axis,
    ) -> Option<Ptr> {
        if !Self::intersects((xl, xr), (yl, yr), (qxl, qxr), (qyl, qyr)) {
            return node;
        }

        let area = Self::area((xl, xr), (yl, yr));
        let ptr = node.unwrap_or_else(|| self.arena.alloc(NodeInner::<M>::with_area(area)));

        if Self::covered_by((xl, xr), (yl, yr), (qxl, qxr), (qyl, qyr)) {
            self.apply_node(ptr, act);
            return Some(ptr);
        }

        // 真の葉
        if xr - xl == I::one() && yr - yl == I::one() {
            self.apply_node(ptr, act);
            return Some(ptr);
        }

        self.push(ptr, (xl, xr), (yl, yr), axis);

        let (axis_eff, mid) = self.choose_axis_and_mid(axis, (xl, xr), (yl, yr));
        if !Self::can_split(axis_eff, (xl, xr), (yl, yr)) {
            // 念のため（真の葉）
            self.apply_node(ptr, act);
            return Some(ptr);
        }

        let next_axis = axis_eff.flip();
        let (c1, c2) = self.child_regions(axis_eff, (xl, xr), (yl, yr), mid);
        let (((xl1, xr1), (yl1, yr1)), _a1) = c1;
        let (((xl2, xr2), (yl2, yr2)), _a2) = c2;

        let left = self.arena.get(ptr).left;
        let nl = self.apply_inner(
            left,
            ((xl1, xr1), (yl1, yr1)),
            ((qxl, qxr), (qyl, qyr)),
            act,
            next_axis,
        );
        self.arena.get_mut(ptr).left = nl;

        let right = self.arena.get(ptr).right;
        let nr = self.apply_inner(
            right,
            ((xl2, xr2), (yl2, yr2)),
            ((qxl, qxr), (qyl, qyr)),
            act,
            next_axis,
        );
        self.arena.get_mut(ptr).right = nr;

        self.pull(ptr, (xl, xr), (yl, yr), axis);
        Some(ptr)
    }

    fn get_range_inner(
        &mut self,
        node: Option<Ptr>,
        (xl, xr): Range<I>,
        (yl, yr): Range<I>,
        (qxl, qxr): Range<I>,
        (qyl, qyr): Range<I>,
        axis: Axis,
    ) -> M::Val {
        if !Self::intersects((xl, xr), (yl, yr), (qxl, qxr), (qyl, qyr)) {
            return M::e_len(0);
        }

        let area = Self::area((xl, xr), (yl, yr));

        if Self::covered_by((xl, xr), (yl, yr), (qxl, qxr), (qyl, qyr)) {
            return node
                .map(|p| self.arena.get(p).sum.clone())
                .unwrap_or_else(|| M::e_len(area));
        }

        // 真の葉（交差していて完全被覆でない，は半開区間では起きにくいが安全側に）
        if xr - xl == I::one() && yr - yl == I::one() {
            return node
                .map(|p| self.arena.get(p).sum.clone())
                .unwrap_or_else(|| M::e_len(area));
        }

        let Some(ptr) = node else {
            // 未生成かつ部分被覆：分割して足し合わせる
            let (axis_eff, mid) = self.choose_axis_and_mid(axis, (xl, xr), (yl, yr));
            if !Self::can_split(axis_eff, (xl, xr), (yl, yr)) {
                return M::e_len(area);
            }
            let next_axis = axis_eff.flip();
            let (c1, c2) = self.child_regions(axis_eff, (xl, xr), (yl, yr), mid);
            let (((xl1, xr1), (yl1, yr1)), _a1) = c1;
            let (((xl2, xr2), (yl2, yr2)), _a2) = c2;

            let a = self.get_range_inner(
                None,
                (xl1, xr1),
                (yl1, yr1),
                (qxl, qxr),
                (qyl, qyr),
                next_axis,
            );
            let b = self.get_range_inner(
                None,
                (xl2, xr2),
                (yl2, yr2),
                (qxl, qxr),
                (qyl, qyr),
                next_axis,
            );
            return M::op(&a, &b);
        };

        self.push(ptr, (xl, xr), (yl, yr), axis);

        let (axis_eff, mid) = self.choose_axis_and_mid(axis, (xl, xr), (yl, yr));
        if !Self::can_split(axis_eff, (xl, xr), (yl, yr)) {
            return self.arena.get(ptr).sum.clone();
        }

        let next_axis = axis_eff.flip();
        let (c1, c2) = self.child_regions(axis_eff, (xl, xr), (yl, yr), mid);
        let (((xl1, xr1), (yl1, yr1)), _a1) = c1;
        let (((xl2, xr2), (yl2, yr2)), _a2) = c2;

        let a = self.get_range_inner(
            self.arena.get(ptr).left,
            (xl1, xr1),
            (yl1, yr1),
            (qxl, qxr),
            (qyl, qyr),
            next_axis,
        );
        let b = self.get_range_inner(
            self.arena.get(ptr).right,
            (xl2, xr2),
            (yl2, yr2),
            (qxl, qxr),
            (qyl, qyr),
            next_axis,
        );
        M::op(&a, &b)
    }
}
