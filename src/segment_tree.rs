//! セグメント木

// ===== Segment Tree =====
use std::fmt;
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, Index, RangeBounds,
};

/// モノイド
pub trait Monoid {
    /// 元の型
    type Val: fmt::Debug + Clone + PartialEq;
    /// 単位元
    const E: Self::Val;
    /// 演算
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

/// # SegmentTree (Monoid)
/// - 抽象化セグメント木
pub struct SegmentTree<M: Monoid> {
    pub size: usize,
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

    /// セグメント木を初期化する
    pub fn new(n: usize) -> Self {
        let offset = n;

        Self {
            size: n,
            offset,
            data: vec![M::E; offset << 1],
        }
    }

    fn update(&mut self, index: usize, value: M::Val) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = M::op(&self.data[lch], &self.data[lch + 1]);
        }
    }

    /// 可変な参照を返す
    pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, M>> {
        if i < self.offset {
            let default = self.index(i).clone();
            Some(ValMut {
                segtree: self,
                idx: i,
                new_val: default,
            })
        } else {
            None
        }
    }

    /// 区間`range`の集約を行う
    pub fn get_range<R: RangeBounds<usize> + fmt::Debug>(&self, range: R) -> M::Val {
        let Some((start, end)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
        // 値の取得
        let mut l = self.offset + start;
        let mut r = self.offset + end;
        let (mut res_l, mut res_r) = (M::E, M::E);

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

impl<M: Monoid> From<&Vec<M::Val>> for SegmentTree<M> {
    fn from(src: &Vec<M::Val>) -> Self {
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

impl<M: Monoid> std::fmt::Debug for SegmentTree<M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentTree {{ [").ok();
        for i in 0..self.size {
            if i + 1 < self.size {
                write!(f, "{:?}, ", self.data[self.offset + i]).ok();
            } else {
                write!(f, "{:?}", self.data[self.offset + i]).ok();
            }
        }
        write!(f, "] }}")
    }
}

pub struct ValMut<'a, M: 'a + Monoid> {
    segtree: &'a mut SegmentTree<M>,
    idx: usize,
    new_val: M::Val,
}

impl<M: Monoid> fmt::Debug for ValMut<'_, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ValMut")
            .field(&self.segtree.index(self.idx))
            .finish()
    }
}

impl<M: Monoid> Drop for ValMut<'_, M> {
    fn drop(&mut self) {
        self.segtree.update(self.idx, self.new_val.clone());
    }
}

impl<M: Monoid> Deref for ValMut<'_, M> {
    type Target = M::Val;
    fn deref(&self) -> &Self::Target {
        &self.segtree[self.idx]
    }
}

impl<M: Monoid> DerefMut for ValMut<'_, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

/// さまざまな代数的構造
pub mod Alg {
    use super::Monoid;

    /// 和
    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }

    /// 積
    pub struct Mul;
    impl Monoid for Mul {
        type Val = isize;
        const E: Self::Val = 1;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left * right
        }
    }

    /// bit単位の排他的論理和
    pub struct Xor;
    impl Monoid for Xor {
        type Val = usize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left ^ right
        }
    }

    /// 最小値
    pub struct Min;
    impl Monoid for Min {
        type Val = isize;
        const E: Self::Val = (1 << 31) - 1;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.min(right)
        }
    }

    /// 最大値
    pub struct Max;
    impl Monoid for Max {
        type Val = isize;
        const E: Self::Val = -((1 << 31) - 1);
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.max(right)
        }
    }

    /// 最小公倍数
    pub struct GCD;
    impl Monoid for GCD {
        type Val = usize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            gcd(*left, *right)
        }
    }

    pub fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    // use super::Modint;
    // /// あまりをとる和
    // pub struct ModAdd;
    // impl Monoid for ModAdd {
    //     type Val = Modint<998244353>;
    //     const E: Self::Val = Modint::<998244353>(0);
    //     fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
    //         *left + *right
    //     }
    // }

    // /// あまりをとる積
    // pub struct ModMul;
    // impl Monoid for ModMul {
    //     type Val = Modint<998244353>;
    //     const E: Self::Val = Modint::<998244353>(1);
    //     fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
    //         *left * *right
    //     }
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_point() {
        let segtree = SegmentTree::<Alg::Mul>::from(&vec![1, 2, 3, 4, 5]);

        assert_eq!(segtree[0], 1);
        assert_eq!(segtree[3], 4);
    }

    #[test]
    fn test_RSQ() {
        let mut segtree = SegmentTree::<Alg::Add>::new(3);

        // segtree.update(0, 1);
        *segtree.get_mut(0).unwrap() += 1;
        *segtree.get_mut(1).unwrap() += 2;
        *segtree.get_mut(2).unwrap() += 3;
        // [1, 2, 3]

        assert_eq!(segtree.get_range(0..2), 3);
        assert_eq!(segtree.get_range(1..2), 2);
        assert_eq!(segtree.get_range(1..=2), 5);
        assert_eq!(segtree.get_range(..), 6);
    }

    #[test]
    fn test_RMQ() {
        const INF: usize = (1 << 31) - 1;
        let mut segtree = SegmentTree::<Alg::Min>::new(3);

        assert_eq!(segtree.get_range(..1), (1 << 31) - 1);
        *segtree.get_mut(0).unwrap() = 5;
        assert_eq!(segtree.get_range(..1), 5);
    }

    #[test]
    fn test_from_slice() {
        const INF: isize = -(1 << 31) + 1;
        let arr = vec![20, 4, 5, 6, 8, 9, 100];
        let mut segtree = SegmentTree::<Alg::Max>::from(&arr);

        assert_eq!(segtree.get_range(0..), 100);
        assert_eq!(segtree.get_range(2..5), 8);

        segtree.update(0, 200);
        eprintln!("{:?}", segtree);

        assert_eq!(segtree.get_range(..), 200);
        assert_eq!(segtree.get_range(2..5), 8);
    }

    #[test]
    #[should_panic]
    fn test_wrong_range() {
        let segtree = SegmentTree::<Alg::Add>::from(&vec![0, 1, 2, 3, 4, 5]);

        assert_eq!(segtree.get_range(..), 15);
        assert_eq!(segtree.get_range(..2), 1);
        assert_eq!(segtree.get_range(..6), 15);
        assert_eq!(segtree.get_range(0..), 15);
        assert_eq!(segtree.get_range(..7), 15);
    }

    #[test]
    fn test_debug_print() {
        const INF: isize = -(1 << 31) + 1;
        let arr = vec![20, 4, 5, 6, 8, 9, 100];
        let segtree = SegmentTree::<Alg::Max>::from(&arr);

        let dbg = format!("{:?}", &segtree);
        assert_eq!(&dbg, "SegmentTree { [20, 4, 5, 6, 8, 9, 100] }");
    }
}
