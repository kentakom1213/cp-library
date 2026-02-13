//! ## セグメント木
//!
//! 集合 $`S`$ と演算 $`\circ`$ の組 $`(S,\circ)`$ がモノイド（[`Monoid`]）であるとき，
//! $`S`$ の要素の列 $`A`$ に対し，
//!
//! - 区間積の取得 ： $`A[l] \circ A[l+1] \circ \cdots \circ A[r]`$
//! - 要素の更新 ： $`A[i] \leftarrow x`$
//!
//! をそれぞれ $`O(\log N)`$ で行う．（$`N = |A|`$）

use crate::{algebraic_structure::monoid::Monoid, tree::show_binary_tree::ShowBinaryTree};
use std::{
    fmt::{self, Debug},
    ops::{
        Bound::{Excluded, Included, Unbounded},
        Deref, DerefMut, Index, RangeBounds,
    },
};

/// セグメント木
pub struct SegmentTree<M: Monoid> {
    /// 要素数
    pub N: usize,
    offset: usize,
    data: Vec<M::Val>,
}

impl<M: Monoid> Index<usize> for SegmentTree<M> {
    type Output = M::Val;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[self.offset + idx]
    }
}

impl<M: Monoid> SegmentTree<M> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: &R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.N,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        };
        if start <= end && end <= self.N {
            Some((start, end))
        } else {
            None
        }
    }

    /// セグメント木を初期化する
    /// - 時間計算量: $`O(N)`$
    pub fn new(N: usize) -> Self {
        let offset = N.next_power_of_two();

        Self {
            N,
            offset,
            data: vec![M::e(); offset << 1],
        }
    }

    /// 配列から初期化する
    /// - 時間計算量: $`O(N)`$
    pub fn from_vec(src: Vec<M::Val>) -> Self {
        let mut seg = Self::new(src.len());
        for (i, v) in src.into_iter().enumerate() {
            seg.data[seg.offset + i] = v;
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = M::op(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }

    /// `index`番目の要素を`value`に更新する
    /// - 時間計算量: $`O(\log N)`$
    pub fn update(&mut self, index: usize, value: M::Val) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = M::op(&self.data[lch], &self.data[lch + 1]);
        }
    }

    /// `i`番目の要素の可変な参照を返す
    /// - 時間計算量: $`O(\log N)`$
    pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, M>> {
        if i < self.offset {
            let default = self.index(i).clone();
            Some(ValMut {
                segself: self,
                idx: i,
                new_val: default,
            })
        } else {
            None
        }
    }

    /// 区間`range`の集約を行う
    /// - 時間計算量: $`O(\log N)`$
    pub fn get_range<R: RangeBounds<usize> + Debug>(&self, range: R) -> M::Val {
        let (start, end) = match self.parse_range(&range) {
            Some(r) => r,
            None => panic!("The given range is wrong: {:?}", range),
        };
        // 値の取得
        let mut l = self.offset + start;
        let mut r = self.offset + end;
        let (mut res_l, mut res_r) = (M::e(), M::e());

        while l < r {
            if l & 1 == 1 {
                res_l = M::op(&res_l, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = M::op(&self.data[r], &res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        M::op(&res_l, &res_r)
    }
}

impl<M: Monoid> FromIterator<M::Val> for SegmentTree<M> {
    fn from_iter<T: IntoIterator<Item = M::Val>>(iter: T) -> Self {
        // 配列にする
        let arr: Vec<M::Val> = iter.into_iter().collect();
        Self::from_vec(arr)
    }
}

impl<M> Debug for SegmentTree<M>
where
    M: Monoid,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentTree {{ [").ok();
        for i in 0..self.N {
            if i + 1 < self.N {
                write!(f, "{:?}, ", self.data[self.offset + i]).ok();
            } else {
                write!(f, "{:?}", self.data[self.offset + i]).ok();
            }
        }
        write!(f, "] }}")
    }
}

/// セグメント木の要素の可変参照
pub struct ValMut<'a, M: 'a + Monoid> {
    segself: &'a mut SegmentTree<M>,
    idx: usize,
    new_val: M::Val,
}

impl<M> Debug for ValMut<'_, M>
where
    M: Monoid,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ValMut").field(&self.new_val).finish()
    }
}

impl<M: Monoid> Drop for ValMut<'_, M> {
    fn drop(&mut self) {
        self.segself.update(self.idx, self.new_val.clone());
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

// セグ木上の2分探索
impl<M: Monoid> SegmentTree<M> {
    /// 左端を固定した2分探索
    /// - 引数`l`と関数`f`に対して，
    ///     - `f( seg.get(l..x) ) = true`
    ///     - `f( seg.get(l..x+1) ) = false`
    ///
    ///   \
    ///   を満たす`x`を返す
    ///
    /// **引数**
    /// - `f` :
    ///   - `f(e) = true`
    ///   - 任意の`i`に対して，`f( seg.get(l..i) ) = false`ならば，`f( seg.get(l..i+1) ) = false`
    pub fn max_right<F>(&self, mut l: usize, f: F) -> (M::Val, usize)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(f(M::e()));

        if l >= self.N {
            return (M::e(), self.N);
        }

        l += self.offset;
        let mut sum = M::e();

        // 第1段階: 条件を満たさない区間を見つける
        'fst: loop {
            while l & 1 == 0 {
                l >>= 1;
            }

            let tmp = M::op(&sum, &self.data[l]);

            // 満たさない区間を発見した場合
            if !f(tmp.clone()) {
                break 'fst;
            }

            sum = tmp;
            l += 1;

            // すべての領域を見終わったら終了
            if (l & l.wrapping_neg()) == l {
                return (sum, self.N);
            }
        }

        // 第2段階: 子方向に移動しながら2分探索
        while l < self.offset {
            // 左に潜る
            l <<= 1;

            let tmp = M::op(&sum, &self.data[l]);

            // 左に潜っても大丈夫な場合
            if f(tmp.clone()) {
                sum = tmp;
                // 右に潜る
                l += 1;
            }
        }

        (sum, l - self.offset)
    }

    /// 右端を固定した2分探索
    /// - 引数`r`と関数`f`に対して，
    ///    - `f( seg.get(x..r) ) = true`
    ///    - `f( seg.get(x-1..r) ) = false`
    ///
    ///   \
    ///   となるような`x`を返す
    ///
    /// **引数**
    /// - `f` :
    ///   - `f(e) = true`
    ///   - 任意の`i`に対して，`f( seg.get(i..r) ) = false`ならば，`f( seg.get(i-1..r) ) = false`
    pub fn min_left<F>(&self, mut r: usize, f: F) -> (M::Val, usize)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(f(M::e()));

        if r == 0 {
            return (M::e(), 0);
        }

        r += self.offset;
        let mut sum = M::e();

        // 第1段階: 条件を満たさない区間を見つける
        'fst: loop {
            r -= 1;
            while r > 1 && r & 1 == 1 {
                r >>= 1;
            }

            let tmp = M::op(&self.data[r], &sum);

            // 満たさない区間を発見した場合
            if !f(tmp.clone()) {
                break 'fst;
            }

            sum = tmp;

            // すべての領域を見終わったら終了
            if (r & r.wrapping_neg()) == r {
                return (sum, 0);
            }
        }

        // 第2段階: 子方向に移動しながら2分探索
        while r < self.offset {
            // 右に潜る
            r = (r << 1) + 1;

            let tmp = M::op(&self.data[r], &sum);

            // 右に潜っても大丈夫な場合
            if f(tmp.clone()) {
                sum = tmp;
                // 左に潜る
                r -= 1;
            }
        }

        (sum, r + 1 - self.offset)
    }
}

impl<M> ShowBinaryTree<usize> for SegmentTree<M>
where
    M: Monoid,
    M::Val: Debug,
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
        format!("[{:?}]", self.data[i])
    }
}
