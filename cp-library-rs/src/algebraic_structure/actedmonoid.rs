//! 作用付きモノイド

/// 作用付きモノイド
pub trait ActedMonoid {
    /// 要素のデータ型
    type Val: Clone + PartialEq;
    /// 作用素のデータ型
    type Act: Clone + PartialEq;
    /// 要素Xの単位元を返す
    fn e() -> Self::Val;
    /// 作用素Fの単位元を返す
    fn id() -> Self::Act;
    /// 要素同士の演算
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val;
    /// 要素に対する作用
    fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val;
    /// 作用素同士の演算
    fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act;
}

/// （遅延セグ木）作用付きモノイド
pub mod examples {

    use std::{
        marker::PhantomData,
        ops::{Add, Mul},
    };

    use num::{Bounded, FromPrimitive, Zero};

    use super::ActedMonoid;

    /// 区間加算 + 区間和
    #[derive(Debug)]
    pub struct AddSum<T>(PhantomData<T>);
    impl<T> ActedMonoid for AddSum<T>
    where
        T: Zero + Clone + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialEq,
    {
        type Val = (T, usize);
        type Act = T;
        fn e() -> Self::Val {
            (T::zero(), 0)
        }
        fn id() -> Self::Act {
            T::zero()
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            let (xv, xs) = x.clone();
            let (yv, ys) = y.clone();
            (xv + yv, xs + ys)
        }
        fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val {
            let (val, size) = x.clone();
            (val + y.clone() * T::from_usize(size).unwrap(), size)
        }
        fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act {
            x.clone() + y.clone()
        }
    }

    /// 区間更新 + 区間最小値
    #[derive(Debug)]
    pub struct UpdateMin<T>(PhantomData<T>);
    impl<T: Bounded + Ord + Clone> ActedMonoid for UpdateMin<T> {
        type Val = T;
        type Act = T;
        fn e() -> Self::Val {
            T::max_value()
        }
        fn id() -> Self::Act {
            T::max_value()
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            x.clone().min(y.clone())
        }
        fn mapping(_x: &Self::Val, y: &Self::Act) -> Self::Val {
            if *y == Self::id() {
                _x.clone()
            } else {
                y.clone()
            }
        }
        fn compose(_x: &Self::Act, y: &Self::Act) -> Self::Act {
            if *y == Self::id() {
                _x.clone()
            } else {
                y.clone()
            }
        }
    }

    /// 区間更新 + 区間最大値
    #[derive(Debug)]
    pub struct UpdateMax<T>(PhantomData<T>);
    impl<T: Bounded + Ord + Clone> ActedMonoid for UpdateMax<T> {
        type Val = T;
        type Act = T;
        fn e() -> Self::Val {
            T::min_value()
        }
        fn id() -> Self::Act {
            T::min_value()
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            x.clone().max(y.clone())
        }
        fn mapping(_x: &Self::Val, y: &Self::Act) -> Self::Val {
            if *y == Self::id() {
                _x.clone()
            } else {
                y.clone()
            }
        }
        fn compose(_x: &Self::Act, y: &Self::Act) -> Self::Act {
            if *y == Self::id() {
                _x.clone()
            } else {
                y.clone()
            }
        }
    }

    /// 最大値と最小値 + 区間更新
    pub struct UpdateMinMax<T>(PhantomData<T>);
    impl<T: Ord + Bounded + Clone> ActedMonoid for UpdateMinMax<T> {
        type Val = (T, T);
        type Act = Option<(T, T)>;
        fn e() -> Self::Val {
            (T::max_value(), T::min_value())
        }
        fn id() -> Self::Act {
            None
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            let (xmin, xmax) = x.clone();
            let (ymin, ymax) = y.clone();
            (xmin.min(ymin), xmax.max(ymax))
        }
        fn mapping(_x: &Self::Val, y: &Self::Act) -> Self::Val {
            match y.clone() {
                Some(v) => v,
                None => _x.clone(),
            }
        }
        fn compose(_x: &Self::Act, y: &Self::Act) -> Self::Act {
            match y.clone() {
                Some(v) => Some(v),
                None => _x.clone(),
            }
        }
    }

    /// 区間加算 + 区間最小値
    #[derive(Debug)]
    pub struct AddMin<T>(PhantomData<T>);
    impl<T> ActedMonoid for AddMin<T>
    where
        T: Zero + Clone + Add<Output = T> + Ord + Bounded,
    {
        type Val = T;
        type Act = T;
        fn e() -> Self::Val {
            T::max_value()
        }
        fn id() -> Self::Act {
            T::zero()
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            x.clone().min(y.clone())
        }
        fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val {
            x.clone() + y.clone()
        }
        fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act {
            x.clone() + y.clone()
        }
    }

    /// 区間加算 + 区間最大値
    #[derive(Debug)]
    pub struct AddMax<T>(PhantomData<T>);
    impl<T> ActedMonoid for AddMax<T>
    where
        T: Zero + Clone + Add<Output = T> + Ord + Bounded,
    {
        type Val = T;
        type Act = T;
        fn e() -> Self::Val {
            T::min_value()
        }
        fn id() -> Self::Act {
            T::zero()
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            x.clone().max(y.clone())
        }
        fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val {
            x.clone() + y.clone()
        }
        fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act {
            x.clone() + y.clone()
        }
    }

    /// 区間更新 + 区間和取得
    #[derive(Debug)]
    pub struct UpdateSum<T>(PhantomData<T>);
    impl<T> ActedMonoid for UpdateSum<T>
    where
        T: Zero + Clone + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialEq,
    {
        type Val = (T, usize);
        type Act = Option<T>;
        fn e() -> Self::Val {
            (T::zero(), 0)
        }
        fn id() -> Self::Act {
            None
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            let (xv, xs) = x.clone();
            let (yv, ys) = y.clone();
            (xv + yv, xs + ys)
        }
        fn mapping(_x: &Self::Val, y: &Self::Act) -> Self::Val {
            let (val, size) = _x.clone();
            match y.clone() {
                Some(v) => (v * T::from_usize(size).unwrap(), size),
                None => (val, size),
            }
        }
        fn compose(_x: &Self::Act, y: &Self::Act) -> Self::Act {
            match y.clone() {
                Some(v) => Some(v),
                None => _x.clone(),
            }
        }
    }

    /// 等差数列加算 + 区間和
    pub struct ArithSum<T>(PhantomData<T>);

    #[derive(Debug, Clone, PartialEq)]
    pub struct ArithVal<T> {
        pub val_sum: T,
        pub index_sum: usize,
        pub len: usize,
    }

    impl<T: Zero> ArithVal<T> {
        /// インデックス i の値を初期化する
        pub fn new(i: usize) -> Self {
            Self {
                val_sum: T::zero(),
                index_sum: i,
                len: 1,
            }
        }
    }

    /// 等差数列
    #[derive(Debug, Clone, PartialEq)]
    pub struct ArithAct<T> {
        /// 公差
        pub a: T,
        /// 切片
        pub b: T,
    }

    impl<T> ArithAct<T> {
        /// 等差数列 `a*i + b` を初期化する
        pub fn new(a: T, b: T) -> Self {
            Self { a, b }
        }
    }

    impl<T> ActedMonoid for ArithSum<T>
    where
        T: Zero + Clone + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialEq,
    {
        type Val = ArithVal<T>;
        type Act = ArithAct<T>;
        fn e() -> Self::Val {
            ArithVal {
                val_sum: T::zero(),
                index_sum: 0,
                len: 0,
            }
        }
        fn id() -> Self::Act {
            ArithAct {
                a: T::zero(),
                b: T::zero(),
            }
        }
        fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
            let x = x.clone();
            let y = y.clone();
            ArithVal {
                val_sum: x.val_sum + y.val_sum,
                index_sum: x.index_sum + y.index_sum,
                len: x.len + y.len,
            }
        }
        fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val {
            let ArithVal {
                val_sum,
                index_sum,
                len,
            } = x.clone();
            let ArithAct { a, b } = y.clone();
            ArithVal {
                val_sum: val_sum
                    + a * T::from_usize(index_sum).unwrap()
                    + b * T::from_usize(len).unwrap(),
                index_sum,
                len,
            }
        }
        fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act {
            let x = x.clone();
            let y = y.clone();
            ArithAct {
                a: x.a + y.a,
                b: x.b + y.b,
            }
        }
    }
}
