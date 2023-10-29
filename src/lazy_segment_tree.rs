//! 遅延評価セグメント木

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
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        }
        .min(self.size);
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        }
        .min(self.size);
        if start <= end {
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
    pub fn set_range<R: RangeBounds<usize>>(&mut self, range: R, val: T::M) {
        if let Some((left, right)) = self.parse_range(range) {
            self.set_range_inner(left, right, val, 0, self.offset, 1);
        }
    }

    fn set_range_inner(
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
            self.set_range_inner(left, right, val.clone(), begin, mid, idx * 2);
            // 右の子を更新
            self.set_range_inner(left, right, val, mid, end, idx * 2 + 1);
            // 値を更新
            self.data[idx] = T::operate_x(&self.data[idx * 2], &self.data[idx * 2 + 1]);
        }
    }

    /// 区間を取得する
    /// - `range`: `[left, right)`
    pub fn get_range<R: RangeBounds<usize>>(&mut self, range: R) -> T::X {
        if let Some((left, right)) = self.parse_range(range) {
            self.get_range_inner(left, right, 0, self.offset, 1)
        } else {
            T::IX
        }
    }

    fn get_range_inner(
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
            let l_val = self.get_range_inner(left, right, begin, mid, idx * 2);
            let r_val = self.get_range_inner(left, right, mid, end, idx * 2 + 1);
            T::operate_x(&l_val, &r_val)
        }
    }
}

pub mod ExtAlg {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_RSQ_and_RAQ_hand() {
        let mut seg = LazySegmentTree::<ExtAlg::RSQandRAQ>::new(10);
        // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

        assert_eq!(seg.get_range(..), 0);
        assert_eq!(seg.get_range(..5), 0);
        assert_eq!(seg.get_range(5..), 0);
        assert_eq!(seg.get_range(3..8), 0);

        seg.set_range(0..4, 2);
        // [2, 2, 2, 2, 0, 0, 0, 0, 0, 0]

        assert_eq!(seg.get_range(..), 8);
        assert_eq!(seg.get_range(..5), 8);
        assert_eq!(seg.get_range(5..), 0);
        assert_eq!(seg.get_range(3..8), 2);

        seg.set_range(4.., 5);
        // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

        assert_eq!(seg.get_range(..), 38);
        assert_eq!(seg.get_range(..5), 13);
        assert_eq!(seg.get_range(5..), 25);
        assert_eq!(seg.get_range(3..8), 22);

        seg.set_range(2..=5, -3);
        // [2, 2, -1, -1, 2, 2, 5, 5, 5, 5]

        assert_eq!(seg.get_range(..), 26);
        assert_eq!(seg.get_range(..5), 4);
        assert_eq!(seg.get_range(5..), 22);
        assert_eq!(seg.get_range(3..8), 13);

        seg.set_range(8..=10, -10);
        // [2, 2, -1, -1, 2, 2, 5, 5, -5, -5]

        assert_eq!(seg.get_range(..), 6);
        assert_eq!(seg.get_range(..5), 4);
        assert_eq!(seg.get_range(5..), 2);
        assert_eq!(seg.get_range(3..8), 13);
    }

    #[test]
    fn test_RMQ_and_RUQ_hand() {
        const INF: isize = (1 << 31) - 1;
        let mut seg = LazySegmentTree::<ExtAlg::RMQandRUQ>::new(10);
        // [INF, INF, INF, INF, INF, INF, INF, INF, INF, INF]

        assert_eq!(seg.get_range(..), INF);
        assert_eq!(seg.get_range(..5), INF);
        assert_eq!(seg.get_range(5..), INF);
        assert_eq!(seg.get_range(3..8), INF);

        seg.set_range(0..4, 2);
        // [2, 2, 2, 2, INF, INF, INF, INF, INF, INF]

        assert_eq!(seg.get_range(..), 2);
        assert_eq!(seg.get_range(..5), 2);
        assert_eq!(seg.get_range(5..), INF);
        assert_eq!(seg.get_range(3..8), 2);

        seg.set_range(4.., 5);
        // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

        assert_eq!(seg.get_range(..), 2);
        assert_eq!(seg.get_range(..5), 2);
        assert_eq!(seg.get_range(5..), 5);
        assert_eq!(seg.get_range(3..8), 2);

        seg.set_range(2..=5, -3);
        // [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

        assert_eq!(seg.get_range(..), -3);
        assert_eq!(seg.get_range(..5), -3);
        assert_eq!(seg.get_range(5..), -3);
        assert_eq!(seg.get_range(3..8), -3);

        seg.set_range(8..=10, -10);
        // [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

        assert_eq!(seg.get_range(..), -10);
        assert_eq!(seg.get_range(..5), -3);
        assert_eq!(seg.get_range(5..), -10);
        assert_eq!(seg.get_range(3..8), -3);
    }
}
