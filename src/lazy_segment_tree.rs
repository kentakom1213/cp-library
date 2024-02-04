//! 遅延評価セグメント木

// ===== Lazy Segment Tree =====
use core::fmt;
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    RangeBounds,
};

/// 作用付きモノイド
pub trait ExtMonoid {
    /// 要素のデータ型
    type X: Clone + PartialEq;
    /// 作用素のデータ型
    type M: Clone + PartialEq;
    /// 要素Xの単位元
    const IX: Self::X;
    /// 作用素Mの単位元
    const IM: Self::M;
    /// 要素同士の演算
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X;
    /// 要素に対する作用
    fn apply(x: &Self::X, y: &Self::M) -> Self::X;
    /// 作用素同士の演算
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M;
    /// 作用素の集約
    fn aggregate(x: &Self::M, p: usize) -> Self::M;
}

/// 遅延評価セグメント木
#[derive(Debug)]
pub struct LazySegmentTree<T: ExtMonoid> {
    pub size: usize,
    offset: usize,
    data: Vec<T::X>,
    lazy: Vec<T::M>,
}

impl<T: ExtMonoid> LazySegmentTree<T> {
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
            data: vec![T::IX; offset << 1],
            lazy: vec![T::IM; offset << 1],
        }
    }

    /// 遅延値を評価
    fn eval(&mut self, idx: usize, len: usize) {
        if self.lazy[idx] == T::IM {
            return;
        }
        // 葉でなければ子に伝搬
        if idx < self.offset {
            self.lazy[idx * 2] = T::operate_m(&self.lazy[idx * 2], &self.lazy[idx]);
            self.lazy[idx * 2 + 1] = T::operate_m(&self.lazy[idx * 2 + 1], &self.lazy[idx]);
        }
        // 自身を更新
        self.data[idx] = T::apply(&self.data[idx], &T::aggregate(&self.lazy[idx], len));
        self.lazy[idx] = T::IM;
    }

    /// 区間に`val`を作用させる
    /// - `range`: `[left, right)`
    pub fn apply<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R, val: T::M) {
        let Some((left, right)) = self.parse_range(&range) else {
            panic!("The given range is wrong: {:?}", range);
        };
        self.apply_inner(left, right, val, 0, self.offset, 1);
    }

    fn apply_inner(
        &mut self,
        left: usize,
        right: usize,
        val: T::M,
        begin: usize,
        end: usize,
        idx: usize,
    ) {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を内包するとき
        if left <= begin && end <= right {
            self.lazy[idx] = T::operate_m(&self.lazy[idx], &val);
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
            self.data[idx] = T::operate_x(&self.data[idx * 2], &self.data[idx * 2 + 1]);
        }
    }

    /// 区間を取得する
    /// - `range`: `[left, right)`
    pub fn get<R: RangeBounds<usize> + fmt::Debug>(&mut self, range: R) -> T::X {
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
    ) -> T::X {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を含まない
        if end <= left || right <= begin {
            T::IX
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
            T::operate_x(&l_val, &r_val)
        }
    }
}

impl<T: ExtMonoid> From<&Vec<T::X>> for LazySegmentTree<T> {
    fn from(src: &Vec<T::X>) -> Self {
        let mut seg = Self::new(src.len());
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = T::operate_x(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }
}

pub mod Alg {
    use super::ExtMonoid;

    /// ## RSQandRAQ
    /// - 区間加算
    /// - 区間和
    #[derive(Debug)]
    pub struct RSQandRAQ;

    impl ExtMonoid for RSQandRAQ {
        type X = isize;
        type M = isize;
        const IX: Self::X = 0;
        const IM: Self::M = 0;
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            x + y
        }
        fn apply(x: &Self::X, y: &Self::M) -> Self::X {
            x + y
        }
        fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
            x + y
        }
        fn aggregate(x: &Self::M, p: usize) -> Self::M {
            x * p as isize
        }
    }

    /// ## RMQandRUQ
    /// - 区間更新
    /// - 区間最小値
    #[derive(Debug)]
    pub struct RMQandRUQ;

    impl ExtMonoid for RMQandRUQ {
        type X = isize;
        type M = isize;
        const IM: Self::M = (1 << 31) - 1;
        const IX: Self::X = (1 << 31) - 1;
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            *x.min(y)
        }
        fn apply(_x: &Self::X, y: &Self::M) -> Self::X {
            *y
        }
        fn operate_m(_x: &Self::M, y: &Self::M) -> Self::M {
            *y
        }
        fn aggregate(x: &Self::M, _p: usize) -> Self::M {
            *x
        }
    }

    /// ## RMQandRAQ
    /// - 区間加算
    /// - 区間最小値
    #[derive(Debug)]
    pub struct RMQandRAQ;
    impl ExtMonoid for RMQandRAQ {
        type X = isize;
        type M = isize;
        const IM: Self::M = 0;
        const IX: Self::X = 1 << 31;
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            *x.min(y)
        }
        fn apply(x: &Self::X, y: &Self::M) -> Self::X {
            x + y
        }
        fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
            x + y
        }
        fn aggregate(x: &Self::M, _p: usize) -> Self::M {
            *x
        }
    }

    /// ## RSQandRUQ
    /// - 区間更新
    /// - 区間和取得
    #[derive(Debug)]
    pub struct RSQandRUQ;
    impl ExtMonoid for RSQandRUQ {
        type X = isize;
        type M = Option<isize>;
        const IX: Self::X = 0;
        const IM: Self::M = None;
        fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
            x + y
        }
        fn apply(_x: &Self::X, y: &Self::M) -> Self::X {
            y.unwrap()
        }
        fn operate_m(_x: &Self::M, y: &Self::M) -> Self::M {
            *y
        }
        fn aggregate(x: &Self::M, p: usize) -> Self::M {
            x.map(|x| x * p as isize)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_RSQ_and_RAQ_hand() {
        let mut seg = LazySegmentTree::<Alg::RSQandRAQ>::new(10);
        // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        assert_eq!(seg.get(..), 0);
        assert_eq!(seg.get(..5), 0);
        assert_eq!(seg.get(5..), 0);
        assert_eq!(seg.get(3..8), 0);

        seg.apply(0..4, 2);
        // [2, 2, 2, 2, 0, 0, 0, 0, 0, 0]

        assert_eq!(seg.get(..), 8);
        assert_eq!(seg.get(..5), 8);
        assert_eq!(seg.get(5..), 0);
        assert_eq!(seg.get(3..8), 2);

        seg.apply(4.., 5);
        // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

        assert_eq!(seg.get(..), 38);
        assert_eq!(seg.get(..5), 13);
        assert_eq!(seg.get(5..), 25);
        assert_eq!(seg.get(3..8), 22);

        seg.apply(2..=5, -3);
        // [2, 2, -1, -1, 2, 2, 5, 5, 5, 5]

        assert_eq!(seg.get(..), 26);
        assert_eq!(seg.get(..5), 4);
        assert_eq!(seg.get(5..), 22);
        assert_eq!(seg.get(3..8), 13);

        seg.apply(8..10, -10);
        // [2, 2, -1, -1, 2, 2, 5, 5, -5, -5]

        assert_eq!(seg.get(..), 6);
        assert_eq!(seg.get(..5), 4);
        assert_eq!(seg.get(5..), 2);
        assert_eq!(seg.get(3..8), 13);
    }

    #[test]
    fn test_RMQ_and_RUQ_hand() {
        const INF: isize = (1 << 31) - 1;
        let mut seg = LazySegmentTree::<Alg::RMQandRUQ>::new(10);
        // [INF, INF, INF, INF, INF, INF, INF, INF, INF, INF]

        assert_eq!(seg.get(..), INF);
        assert_eq!(seg.get(..5), INF);
        assert_eq!(seg.get(5..), INF);
        assert_eq!(seg.get(3..8), INF);

        seg.apply(0..4, 2);
        // [2, 2, 2, 2, INF, INF, INF, INF, INF, INF]

        assert_eq!(seg.get(..), 2);
        assert_eq!(seg.get(..5), 2);
        assert_eq!(seg.get(5..), INF);
        assert_eq!(seg.get(3..8), 2);

        seg.apply(4.., 5);
        // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

        assert_eq!(seg.get(..), 2);
        assert_eq!(seg.get(..5), 2);
        assert_eq!(seg.get(5..), 5);
        assert_eq!(seg.get(3..8), 2);

        seg.apply(2..=5, -3);
        // [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

        assert_eq!(seg.get(..), -3);
        assert_eq!(seg.get(..5), -3);
        assert_eq!(seg.get(5..), -3);
        assert_eq!(seg.get(3..8), -3);

        seg.apply(8..10, -10);
        // [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

        assert_eq!(seg.get(..), -10);
        assert_eq!(seg.get(..5), -3);
        assert_eq!(seg.get(5..), -10);
        assert_eq!(seg.get(3..8), -3);
    }

    /// テストケース: <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I&lang=ja>
    #[test]
    fn test_RSQ_and_RUQ() {
        let mut seg = LazySegmentTree::<Alg::RSQandRUQ>::new(6);

        seg.apply(1..=3, Some(1));
        seg.apply(2..=4, Some(-2));

        assert_eq!(seg.get(..=5), -5);
        assert_eq!(seg.get(..=1), 1);

        seg.apply(3..=5, Some(3));

        assert_eq!(seg.get(3..=4), 6);
        assert_eq!(seg.get(..=5), 8);
    }

    #[test]
    fn test_from() {
        const INF: isize = Alg::RMQandRAQ::IX;

        let arr = vec![5, 2, -3, -1, -9, -2, 5, 0, 0, 5];

        let mut seg = LazySegmentTree::<Alg::RMQandRUQ>::from(&arr);
        // [5, 2, -3, -1, -9, -2, 5, 0, 0, 5]

        assert_eq!(seg.get(..), -9);
        assert_eq!(seg.get(..5), -9);
        assert_eq!(seg.get(5..), -2);
        assert_eq!(seg.get(3..8), -9);

        seg.apply(..4, 2);
        // [2, 2, 2, 2, -9, -2, 5, 0, 0, 5]

        assert_eq!(seg.get(..), -9);
        assert_eq!(seg.get(..5), -9);
        assert_eq!(seg.get(5..), -2);
        assert_eq!(seg.get(3..8), -9);

        seg.apply(4.., 5);
        // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

        assert_eq!(seg.get(..), 2);
        assert_eq!(seg.get(..5), 2);
        assert_eq!(seg.get(5..), 5);
        assert_eq!(seg.get(3..8), 2);

        seg.apply(2..=5, -3);
        // [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

        assert_eq!(seg.get(..), -3);
        assert_eq!(seg.get(..5), -3);
        assert_eq!(seg.get(5..), -3);
        assert_eq!(seg.get(3..8), -3);

        seg.apply(8.., -10);
        // [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

        assert_eq!(seg.get(..), -10);
        assert_eq!(seg.get(..5), -3);
        assert_eq!(seg.get(5..), -10);
        assert_eq!(seg.get(3..8), -3);
    }

    #[test]
    #[should_panic]
    fn get_wrong_range() {
        let mut seg = LazySegmentTree::<Alg::RMQandRAQ>::from(&vec![0, 1, 2, 3, 4, 5]);

        seg.get(..7);
    }

    #[test]
    #[should_panic]
    fn set_wrong_range() {
        let mut seg = LazySegmentTree::<Alg::RMQandRAQ>::from(&vec![0, 1, 2, 3, 4, 5]);

        seg.apply(..7, 0);
    }
}
