//! 作用付きモノイド

/// 作用付きモノイド
pub trait ExtMonoid {
    /// 要素のデータ型
    type X: Clone + PartialEq;
    /// 作用素のデータ型
    type F: Clone + PartialEq;
    /// 要素Xの単位元を返す
    fn id_x() -> Self::X;
    /// 作用素Fの単位元を返す
    fn id_f() -> Self::F;
    /// 要素同士の演算
    fn op(x: &Self::X, y: &Self::X) -> Self::X;
    /// 要素に対する作用
    fn mapping(x: &Self::X, y: &Self::F) -> Self::X;
    /// 作用素同士の演算
    fn composition(x: &Self::F, y: &Self::F) -> Self::F;
    /// 作用素の集約
    fn aggregate(x: &Self::F, p: usize) -> Self::F;
}

/// （遅延セグ木）作用付きモノイド
pub mod examples {

    use super::ExtMonoid;

    /// ## AddSum
    /// - 区間加算
    /// - 区間和
    #[derive(Debug)]
    pub struct AddSum;

    impl ExtMonoid for AddSum {
        type X = isize;
        type F = isize;
        fn id_x() -> Self::X {
            0
        }
        fn id_f() -> Self::F {
            0
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            x + y
        }
        fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
            x + y
        }
        fn composition(x: &Self::F, y: &Self::F) -> Self::F {
            x + y
        }
        fn aggregate(x: &Self::F, p: usize) -> Self::F {
            x * p as isize
        }
    }

    /// ## UpdateMin
    /// - 区間更新
    /// - 区間最小値
    #[derive(Debug)]
    pub struct UpdateMin;

    impl ExtMonoid for UpdateMin {
        type X = isize;
        type F = isize;
        fn id_x() -> Self::X {
            isize::MAX
        }
        fn id_f() -> Self::F {
            isize::MAX
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            *x.min(y)
        }
        fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
            *y
        }
        fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
            *y
        }
        fn aggregate(x: &Self::F, _p: usize) -> Self::F {
            *x
        }
    }

    /// ## UpdateMax
    /// - 区間更新
    /// - 区間最大値
    #[derive(Debug)]
    pub struct UpdateMax;

    impl ExtMonoid for UpdateMax {
        type X = isize;
        type F = isize;
        fn id_x() -> Self::X {
            isize::MIN
        }
        fn id_f() -> Self::F {
            isize::MIN
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            *x.max(y)
        }
        fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
            *y
        }
        fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
            *y
        }
        fn aggregate(x: &Self::F, _p: usize) -> Self::F {
            *x
        }
    }

    /// ## AddMin
    /// - 区間加算
    /// - 区間最小値
    #[derive(Debug)]
    pub struct AddMin;
    impl ExtMonoid for AddMin {
        type X = isize;
        type F = isize;
        fn id_x() -> Self::X {
            isize::MAX
        }
        fn id_f() -> Self::F {
            0
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            *x.min(y)
        }
        fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
            x + y
        }
        fn composition(x: &Self::F, y: &Self::F) -> Self::F {
            x + y
        }
        fn aggregate(x: &Self::F, _p: usize) -> Self::F {
            *x
        }
    }

    /// ## UpdateSum
    /// - 区間更新
    /// - 区間和取得
    #[derive(Debug)]
    pub struct UpdateSum;
    impl ExtMonoid for UpdateSum {
        type X = isize;
        type F = Option<isize>;
        fn id_x() -> Self::X {
            0
        }
        fn id_f() -> Self::F {
            None
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            x + y
        }
        fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
            y.unwrap()
        }
        fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
            *y
        }
        fn aggregate(x: &Self::F, p: usize) -> Self::F {
            x.map(|x| x * p as isize)
        }
    }
}
