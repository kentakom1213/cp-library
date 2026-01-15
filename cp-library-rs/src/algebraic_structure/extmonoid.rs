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

    use std::{
        marker::PhantomData,
        ops::{Add, Mul},
    };

    use num::{Bounded, FromPrimitive, Zero};

    use super::ExtMonoid;

    /// 区間加算 + 区間和
    #[derive(Debug)]
    pub struct AddSum<T>(PhantomData<T>);
    impl<T> ExtMonoid for AddSum<T>
    where
        T: Zero + Clone + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialEq,
    {
        type X = T;
        type F = T;
        fn id_x() -> Self::X {
            T::zero()
        }
        fn id_f() -> Self::F {
            T::zero()
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            x.clone() + y.clone()
        }
        fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
            x.clone() + y.clone()
        }
        fn composition(x: &Self::F, y: &Self::F) -> Self::F {
            x.clone() + y.clone()
        }
        fn aggregate(x: &Self::F, p: usize) -> Self::F {
            x.clone() * T::from_usize(p).unwrap()
        }
    }

    /// 区間更新 + 区間最小値
    #[derive(Debug)]
    pub struct UpdateMin<T>(PhantomData<T>);
    impl<T: Bounded + Ord + Clone> ExtMonoid for UpdateMin<T> {
        type X = T;
        type F = T;
        fn id_x() -> Self::X {
            T::max_value()
        }
        fn id_f() -> Self::F {
            T::max_value()
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            x.clone().min(y.clone())
        }
        fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
            y.clone()
        }
        fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
            y.clone()
        }
        fn aggregate(x: &Self::F, _p: usize) -> Self::F {
            x.clone()
        }
    }

    /// 区間更新 + 区間最大値
    #[derive(Debug)]
    pub struct UpdateMax<T>(PhantomData<T>);
    impl<T: Bounded + Ord + Clone> ExtMonoid for UpdateMax<T> {
        type X = T;
        type F = T;
        fn id_x() -> Self::X {
            T::min_value()
        }
        fn id_f() -> Self::F {
            T::min_value()
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            x.clone().max(y.clone())
        }
        fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
            y.clone()
        }
        fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
            y.clone()
        }
        fn aggregate(x: &Self::F, _p: usize) -> Self::F {
            x.clone()
        }
    }

    /// 最大値と最小値 + 区間更新
    pub struct UpdateMinMax<T>(PhantomData<T>);
    impl<T: Ord + Bounded + Clone> ExtMonoid for UpdateMinMax<T> {
        type X = (T, T);
        type F = (T, T);
        fn id_x() -> Self::X {
            (T::max_value(), T::min_value())
        }
        fn id_f() -> Self::F {
            Self::id_x()
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            let (xmin, xmax) = x.clone();
            let (ymin, ymax) = y.clone();
            (xmin.min(ymin), xmax.max(ymax))
        }
        fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
            y.clone()
        }
        fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
            y.clone()
        }
        fn aggregate(x: &Self::F, _p: usize) -> Self::F {
            x.clone()
        }
    }

    /// 区間加算 + 区間最小値
    #[derive(Debug)]
    pub struct AddMin<T>(PhantomData<T>);
    impl<T> ExtMonoid for AddMin<T>
    where
        T: Zero + Clone + Add<Output = T> + Ord + Bounded,
    {
        type X = T;
        type F = T;
        fn id_x() -> Self::X {
            T::max_value()
        }
        fn id_f() -> Self::F {
            T::zero()
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            x.clone().min(y.clone())
        }
        fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
            x.clone() + y.clone()
        }
        fn composition(x: &Self::F, y: &Self::F) -> Self::F {
            x.clone() + y.clone()
        }
        fn aggregate(x: &Self::F, _p: usize) -> Self::F {
            x.clone()
        }
    }

    /// 区間更新 + 区間和取得
    #[derive(Debug)]
    pub struct UpdateSum<T>(PhantomData<T>);
    impl<T> ExtMonoid for UpdateSum<T>
    where
        T: Zero + Clone + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialEq,
    {
        type X = T;
        type F = Option<T>;
        fn id_x() -> Self::X {
            T::zero()
        }
        fn id_f() -> Self::F {
            None
        }
        fn op(x: &Self::X, y: &Self::X) -> Self::X {
            x.clone() + y.clone()
        }
        fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
            y.clone().unwrap()
        }
        fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
            y.clone().clone()
        }
        fn aggregate(x: &Self::F, p: usize) -> Self::F {
            x.clone().map(|x| x * T::from_usize(p).unwrap())
        }
    }
}
