//! 二次元セグメント木
//! - 参考：[二次元セグメント木 - Nyaan's Library](https://nyaannyaan.github.io/library/data-structure-2d/2d-segment-tree.hpp.html)

use crate::monoid::Monoid;
use std::fmt::{self, Debug};
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, RangeBounds,
};

macro_rules! cfor {
    ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
        $def
        while $fin {
            $bl
            $incr
        }
    }}
}

/// # SegmentTree2D (Monoid)
/// - 2次元セグメント木
pub struct SegmentTree2D<M: Monoid> {
    pub oh: usize,
    pub ow: usize,
    pub data: Vec<M::Val>,
}

impl<M: Monoid> SegmentTree2D<M> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: &R, max: usize) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => max,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        };
        if start <= end && end <= max {
            Some((start, end))
        } else {
            None
        }
    }

    #[inline]
    fn idx(&self, i: usize, j: usize) -> usize {
        2 * self.ow * i + j
    }

    /// セグメント木を初期化する
    pub fn new(H: usize, W: usize) -> Self {
        let oh = H.next_power_of_two();
        let ow = W.next_power_of_two();

        Self {
            oh,
            ow,
            data: vec![M::id(); 4 * oh * ow],
        }
    }

    /// 座標 `(r,c)` の値を `x` に更新する
    pub fn update(&mut self, mut r: usize, mut c: usize, x: M::Val) {
        r += self.oh;
        c += self.ow;
        let idx = self.idx(r, c);
        self.data[idx] = x;
        // col方向の更新
        cfor! {let mut i = r >> 1; i > 0; i >>= 1;; {
            let idx = self.idx(i, c);
            self.data[idx] = M::op(
                &self.data[self.idx(2 * i, c)],
                &self.data[self.idx(2 * i + 1, c)],
            );
        }}
        // row方向の更新
        cfor! {let mut i = r; i > 0; i >>= 1;; {
            cfor! {let mut j = c >> 1; j > 0; j >>= 1;; {
                let idx = self.idx(i, j);
                self.data[idx] = M::op(
                    &self.data[self.idx(i, 2 * j)],
                    &self.data[self.idx(i, 2 * j + 1)],
                );
            }}
        }}
    }

    /// 可変な参照を返す
    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<ValMut<'_, M>> {
        if r < self.oh && c < self.ow {
            let old_val = self.data[self.idx(r + self.oh, c + self.ow)].clone();
            Some(ValMut {
                segtree: self,
                r,
                c,
                new_val: old_val,
            })
        } else {
            None
        }
    }

    /// row方向での集約を行う
    fn aggregate_row(&self, r: usize, mut cs: usize, mut ce: usize) -> M::Val {
        // 集約
        let mut res = M::id();
        while cs < ce {
            if cs & 1 == 1 {
                res = M::op(&res, &self.data[self.idx(r, cs)]);
                cs += 1;
            }
            if ce & 1 == 1 {
                ce -= 1;
                res = M::op(&res, &self.data[self.idx(r, ce)]);
            }
            cs >>= 1;
            ce >>= 1;
        }
        res
    }

    /// 区間の集約を行う
    pub fn get_range<R, C>(&self, row: R, col: C) -> M::Val
    where
        R: RangeBounds<usize> + fmt::Debug,
        C: RangeBounds<usize> + fmt::Debug,
    {
        let Some((mut rs, mut re)) = self.parse_range(&row, self.oh) else {
            panic!("The given range is wrong (row): {:?}", row);
        };
        let Some((mut cs, mut ce)) = self.parse_range(&col, self.ow) else {
            panic!("The given range is wrong (col): {:?}", col);
        };
        rs += self.oh;
        re += self.oh;
        cs += self.ow;
        ce += self.ow;
        // 値の取得
        let mut res = M::id();
        while rs < re {
            if rs & 1 == 1 {
                res = M::op(&res, &self.aggregate_row(rs, cs, ce));
                rs += 1;
            }
            if re & 1 == 1 {
                re -= 1;
                res = M::op(&res, &self.aggregate_row(re, cs, ce));
            }
            rs >>= 1;
            re >>= 1;
        }
        res
    }
}

impl<M: Monoid> From<&Vec<Vec<M::Val>>> for SegmentTree2D<M> {
    fn from(src: &Vec<Vec<M::Val>>) -> Self {
        let (H, W) = (src.len(), src[0].len());
        let mut seg = SegmentTree2D::new(H, W);
        let (oh, ow) = (seg.oh, seg.ow);
        // セグ木の値を埋める
        for i in 0..H {
            for j in 0..W {
                let idx = seg.idx(oh + i, ow + j);
                seg.data[idx] = src[i][j].clone();
            }
        }
        // col方向の集約
        for j in ow..2 * ow {
            for i in (1..oh).rev() {
                let idx = seg.idx(i, j);
                seg.data[idx] = M::op(
                    &seg.data[seg.idx(2 * i, j)],
                    &seg.data[seg.idx(2 * i + 1, j)],
                );
            }
        }
        // row方向の集約
        for i in 0..2 * oh {
            for j in (1..ow).rev() {
                let idx = seg.idx(i, j);
                seg.data[idx] = M::op(
                    &seg.data[seg.idx(i, 2 * j)],
                    &seg.data[seg.idx(i, 2 * j + 1)],
                );
            }
        }
        seg
    }
}

pub struct ValMut<'a, M: 'a + Monoid> {
    segtree: &'a mut SegmentTree2D<M>,
    r: usize,
    c: usize,
    new_val: M::Val,
}

impl<M: Monoid> fmt::Debug for ValMut<'_, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValMut")
            .field("r", &self.r)
            .field("c", &self.c)
            .field("new_val", &self.new_val)
            .finish()
    }
}

impl<M: Monoid> Drop for ValMut<'_, M> {
    fn drop(&mut self) {
        self.segtree.update(self.r, self.c, self.new_val.clone());
    }
}

impl<M: Monoid> Deref for ValMut<'_, M> {
    type Target = M::Val;
    fn deref(&self) -> &Self::Target {
        &self.new_val
    }
}

impl<M: Monoid> DerefMut for ValMut<'_, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

impl<M> SegmentTree2D<M>
where
    M: Monoid,
    M::Val: Debug,
{
    /// セグ木を簡易的に表示する
    pub fn show(&self) -> String {
        let mut res = String::new();

        let H = self.oh;
        let W = self.ow;
        let idx = |r: usize, c: usize| -> usize { 2 * r * W + c };

        let mut r = 1;
        let mut h = 1;
        let mut logh = 0;
        while r + h <= 2 * H {
            for i in 1..=h {
                let mut c = 1;
                let mut w = 1;
                while c + w <= 2 * W {
                    res += &format!(
                        "{}{:?}\n",
                        "  ".repeat(logh),
                        &self.data[idx(r + i - 1, c)..idx(r + i - 1, c + w)]
                    );
                    c += w;
                    w <<= 1;
                }
            }
            r += h;
            h <<= 1;
            logh += 1;
        }
        res
    }
}
