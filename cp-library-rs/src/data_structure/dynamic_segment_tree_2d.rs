//! 2D dynamic segment tree（implicit，交互 2 分木 + split 軸キャッシュ）
//!
//! 方針：
//! - 本体 API は `apply(rx, ry, act)` と `get_range(rx, ry)`
//! - `get(x, y)` は `get_range(x..x+1, y..y+1)` のラッパー
//! - 重要：ノードごとに「このノードはどちらの軸で split するか」を 1 度だけ決めて保持する
//!   - クエリ依存の軸選択は「未決定ノードの初回 split 決定」にだけ使う
//!   - これにより `left/right` が表す領域の意味が常に一意になり，正しさが保たれる
//! - split 不能（軸長 1）なら反転して試す，両方不能なら真の葉
//! - `ActedMonoidWithSize::e_len(len)` の `len` は「面積 area」とみなす

use std::fmt::Debug;
use std::ops::{Bound::*, RangeBounds};

use num::ToPrimitive;
use num_traits::PrimInt;

use crate::{
    algebraic_structure::actedmonoid_with_size::ActedMonoidWithSize,
    tree::arena::{Arena, ArenaNode, Ptr},
};

type Range<I> = (I, I);
type Rect<I> = (Range<I>, Range<I>);

type A<M> = Arena<NodeInner<M>>;

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
    /// このノードの split 軸（未決定なら `None`）
    split_axis: Option<Axis>,
}

impl<M: ActedMonoidWithSize> ArenaNode for NodeInner<M> {}

impl<M: ActedMonoidWithSize> NodeInner<M> {
    fn with_area(area: usize) -> Self {
        Self {
            sum: M::e_len(area),
            act: M::id(),
            left: None,
            right: None,
            split_axis: None,
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
            ((self.xmin, self.xmax), (self.ymin, self.ymax)),
            ((qxl, qxr), (qyl, qyr)),
            Axis::X,
        )
    }

    /// 点取得：`get_range(x..x+1, y..y+1)` のラッパー
    pub fn get(&mut self, x: I, y: I) -> M::Val {
        assert!(self.xmin <= x && x < self.xmax);
        assert!(self.ymin <= y && y < self.ymax);
        let one = I::one();
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
    fn area(((xl, xr), (yl, yr)): Rect<I>) -> usize {
        Self::len(xl, xr) * Self::len(yl, yr)
    }

    #[inline]
    fn is_leaf(((xl, xr), (yl, yr)): Rect<I>) -> bool {
        xr - xl == I::one() && yr - yl == I::one()
    }

    #[inline]
    fn intersects(((xl, xr), (yl, yr)): Rect<I>, ((qxl, qxr), (qyl, qyr)): Rect<I>) -> bool {
        !(qxr <= xl || xr <= qxl || qyr <= yl || yr <= qyl)
    }

    #[inline]
    fn covered_by(((xl, xr), (yl, yr)): Rect<I>, ((qxl, qxr), (qyl, qyr)): Rect<I>) -> bool {
        qxl <= xl && xr <= qxr && qyl <= yl && yr <= qyr
    }

    #[inline]
    fn can_split(axis: Axis, ((xl, xr), (yl, yr)): Rect<I>) -> bool {
        match axis {
            Axis::X => xr - xl > I::one(),
            Axis::Y => yr - yl > I::one(),
        }
    }

    // ---------- node helpers ----------

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

    // ---------- split decision（固定） ----------

    /// 未決定ノードに対してのみ呼ぶこと．決めたら `split_axis` に保存する．
    ///
    /// 優先順位：
    /// 1) クエリがその軸で部分被覆ならその軸（境界を跨ぐ軸を先に潰す）
    /// 2) それ以外は長い方
    /// 3) tie は `axis_hint`
    /// 4) split 不能なら反転して試す
    fn decide_split_axis(
        &self,
        axis_hint: Axis,
        node_rect: Rect<I>,
        query: Rect<I>,
    ) -> Axis {
        let ((xl, xr), (yl, yr)) = node_rect;
        let ((qxl, qxr), (qyl, qyr)) = query;

        let x_can = Self::can_split(Axis::X, node_rect);
        let y_can = Self::can_split(Axis::Y, node_rect);

        if !x_can && !y_can {
            return axis_hint;
        }
        if x_can && !y_can {
            return Axis::X;
        }
        if y_can && !x_can {
            return Axis::Y;
        }

        // 交差している前提だが，軸方向の交差も念のため見る
        let x_intersects = !(qxr <= xl || xr <= qxl);
        let y_intersects = !(qyr <= yl || yr <= qyl);

        let x_full = qxl <= xl && xr <= qxr;
        let y_full = qyl <= yl && yr <= qyr;

        let x_partial = x_intersects && !x_full;
        let y_partial = y_intersects && !y_full;

        if x_partial {
            return Axis::X;
        }
        if y_partial {
            return Axis::Y;
        }

        // 長い方
        let x_len = xr - xl;
        let y_len = yr - yl;
        if x_len > y_len {
            return Axis::X;
        }
        if y_len > x_len {
            return Axis::Y;
        }

        // tie
        axis_hint
    }

    /// ノード `ptr` の split 軸を確定して返す（既にあればそれを返す）．
    fn ensure_split_axis(&mut self, ptr: Ptr, node_rect: Rect<I>, query: Rect<I>, hint: Axis) -> Axis {
        if let Some(ax) = self.arena.get(ptr).split_axis {
            return ax;
        }
        let ax = self.decide_split_axis(hint, node_rect, query);
        self.arena.get_mut(ptr).split_axis = Some(ax);
        ax
    }

    /// split 軸が与えられたときの子領域を返す．
    fn child_regions(&self, axis: Axis, ((xl, xr), (yl, yr)): Rect<I>) -> (Rect<I>, usize, Rect<I>, usize) {
        match axis {
            Axis::X => {
                let xm = Self::mid(xl, xr);
                let r1 = ((xl, xm), (yl, yr));
                let r2 = ((xm, xr), (yl, yr));
                (r1, Self::area(r1), r2, Self::area(r2))
            }
            Axis::Y => {
                let ym = Self::mid(yl, yr);
                let r1 = ((xl, xr), (yl, ym));
                let r2 = ((xl, xr), (ym, yr));
                (r1, Self::area(r1), r2, Self::area(r2))
            }
        }
    }

    // ---------- lazy propagation ----------

    /// 遅延を子へ伝播（split 軸はノードに保存済みのものを使用）
    fn push(&mut self, ptr: Ptr, node_rect: Rect<I>) {
        if Self::is_leaf(node_rect) {
            return;
        }
        let act = { self.arena.get(ptr).act.clone() };
        if act == M::id() {
            return;
        }

        // クエリ非依存に push したいので，split 軸は「既に決まっている」前提にする
        // 未決定の可能性があるなら，ここで「領域だけ」で決める fallback を入れる
        let axis = self.arena.get(ptr).split_axis.unwrap_or_else(|| {
            // 領域だけで決める（tie は X）
            let ((xl, xr), (yl, yr)) = node_rect;
            let x_can = xr - xl > I::one();
            let y_can = yr - yl > I::one();
            if x_can && !y_can {
                Axis::X
            } else if y_can && !x_can {
                Axis::Y
            } else {
                let x_len = xr - xl;
                let y_len = yr - yl;
                if x_len >= y_len { Axis::X } else { Axis::Y }
            }
        });

        let (r1, a1, r2, a2) = self.child_regions(axis, node_rect);

        let lp = self.ensure_left(ptr, a1);
        let rp = self.ensure_right(ptr, a2);

        self.apply_node(lp, &act);
        self.apply_node(rp, &act);

        self.arena.get_mut(ptr).act = M::id();

        // push の時点で split 軸が未決定だったなら，ここで固定しておく（以後の一貫性のため）
        if self.arena.get(ptr).split_axis.is_none() {
            self.arena.get_mut(ptr).split_axis = Some(axis);
        }

        let _ = (r1, r2); // 参照用
    }

    fn pull(&mut self, ptr: Ptr, node_rect: Rect<I>) {
        if Self::is_leaf(node_rect) {
            return;
        }
        let axis = self.arena.get(ptr).split_axis.unwrap_or_else(|| {
            // `push` と同じ fallback（理想は必ず Some になっていること）
            let ((xl, xr), (yl, yr)) = node_rect;
            let x_can = xr - xl > I::one();
            let y_can = yr - yl > I::one();
            if x_can && !y_can {
                Axis::X
            } else if y_can && !x_can {
                Axis::Y
            } else {
                let x_len = xr - xl;
                let y_len = yr - yl;
                if x_len >= y_len { Axis::X } else { Axis::Y }
            }
        });

        let (r1, a1, r2, a2) = self.child_regions(axis, node_rect);

        let lp = self.arena.get(ptr).left;
        let rp = self.arena.get(ptr).right;

        let lsum = lp.map(|p| self.arena.get(p).sum.clone()).unwrap_or_else(|| M::e_len(a1));
        let rsum = rp.map(|p| self.arena.get(p).sum.clone()).unwrap_or_else(|| M::e_len(a2));

        self.arena.get_mut(ptr).sum = M::op(&lsum, &rsum);

        // 念のため固定
        if self.arena.get(ptr).split_axis.is_none() {
            self.arena.get_mut(ptr).split_axis = Some(axis);
        }

        let _ = (r1, r2);
    }

    // ---------- recursions ----------

    fn apply_inner(
        &mut self,
        node: Option<Ptr>,
        node_rect: Rect<I>,
        query: Rect<I>,
        act: &M::Act,
        axis_hint: Axis,
    ) -> Option<Ptr> {
        if !Self::intersects(node_rect, query) {
            return node;
        }

        let area = Self::area(node_rect);
        let ptr = node.unwrap_or_else(|| self.arena.alloc(NodeInner::<M>::with_area(area)));

        if Self::covered_by(node_rect, query) {
            self.apply_node(ptr, act);
            return Some(ptr);
        }

        if Self::is_leaf(node_rect) {
            self.apply_node(ptr, act);
            return Some(ptr);
        }

        // 子へ遅延伝播（split 軸は固定されたものを使う）
        self.push(ptr, node_rect);

        // split 軸の確定（未決定なら，このクエリと hint を使って決めて固定）
        let axis = self.ensure_split_axis(ptr, node_rect, query, axis_hint);

        let (r1, a1, r2, a2) = self.child_regions(axis, node_rect);

        let left = self.arena.get(ptr).left;
        let nl = self.apply_inner(left, r1, query, act, axis.flip());
        self.arena.get_mut(ptr).left = nl;

        let right = self.arena.get(ptr).right;
        let nr = self.apply_inner(right, r2, query, act, axis.flip());
        self.arena.get_mut(ptr).right = nr;

        // 吸い上げ
        self.pull(ptr, node_rect);

        let _ = (a1, a2);
        Some(ptr)
    }

    fn get_range_inner(
        &mut self,
        node: Option<Ptr>,
        node_rect: Rect<I>,
        query: Rect<I>,
        axis_hint: Axis,
    ) -> M::Val {
        if !Self::intersects(node_rect, query) {
            return M::e_len(0);
        }

        let area = Self::area(node_rect);
        if Self::covered_by(node_rect, query) {
            return node
                .map(|p| self.arena.get(p).sum.clone())
                .unwrap_or_else(|| M::e_len(area));
        }

        if Self::is_leaf(node_rect) {
            return node
                .map(|p| self.arena.get(p).sum.clone())
                .unwrap_or_else(|| M::e_len(area));
        }

        let Some(ptr) = node else {
            // 一般の `ActedMonoidWithSize` に対して安全にするため，
            // `None` でも分割して `op` で合成する（面積だけで即返ししない）
            let axis = self.decide_split_axis(axis_hint, node_rect, query);
            let (r1, _a1, r2, _a2) = self.child_regions(axis, node_rect);

            let a = self.get_range_inner(None, r1, query, axis.flip());
            let b = self.get_range_inner(None, r2, query, axis.flip());
            return M::op(&a, &b);
        };

        self.push(ptr, node_rect);

        let axis = self.ensure_split_axis(ptr, node_rect, query, axis_hint);
        let (r1, _a1, r2, _a2) = self.child_regions(axis, node_rect);

        let a = self.get_range_inner(self.arena.get(ptr).left, r1, query, axis.flip());
        let b = self.get_range_inner(self.arena.get(ptr).right, r2, query, axis.flip());
        M::op(&a, &b)
    }
}
