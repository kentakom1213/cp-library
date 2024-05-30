//! 作用付きモノイド

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

/// （遅延セグ木）作用付きモノイド
pub mod examples {

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
