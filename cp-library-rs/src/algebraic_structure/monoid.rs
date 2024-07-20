//! # モノイド
//!
//! - [`Monoid::Val`] ： データの型 $`S`$
//! - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
//! - [`Monoid::op`] ： 演算 $`S\times S \to S`$

use std::fmt::Debug;

/// モノイド
///
/// - [`Monoid::Val`] ： データの型 $`S`$
/// - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
/// - [`Monoid::op`] ： 演算 $`S\times S \to S`$
pub trait Monoid {
    /// データの型 （$`S`$）
    type Val: Debug + Clone;
    /// 単位元 （$`\varnothing \to S`$）
    fn id() -> Self::Val;
    /// 演算 （$`S \times S \to S`$）
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

/// モノイドの例
pub mod examples {

    use std::{fmt::Debug, marker::PhantomData};

    use crate::{
        algebraic_structure::monoid::Monoid,
        utils::num_traits::{One, Zero},
    };

    /// 和
    #[derive(Debug, Clone)]
    pub struct Add<T>(PhantomData<T>);
    impl<T: Zero + Clone + Debug> Monoid for Add<T> {
        type Val = T;
        fn id() -> Self::Val {
            T::zero()
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left.clone() + right.clone()
        }
    }

    /// 積
    #[derive(Debug, Clone)]
    pub struct Mul<T>(PhantomData<T>);
    impl<T: One + Clone + Debug> Monoid for Mul<T> {
        type Val = T;
        fn id() -> Self::Val {
            T::one()
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left.clone() * right.clone()
        }
    }

    /// bit単位の排他的論理和
    #[derive(Debug, Clone)]
    pub struct Xor;
    impl Monoid for Xor {
        type Val = usize;
        fn id() -> Self::Val {
            0
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left ^ right
        }
    }

    /// 最小値
    #[derive(Debug, Clone)]
    pub struct Min;
    impl Monoid for Min {
        type Val = isize;
        fn id() -> Self::Val {
            Self::Val::MAX
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.min(right)
        }
    }

    /// 最大値
    #[derive(Debug, Clone)]
    pub struct Max;
    impl Monoid for Max {
        type Val = isize;
        fn id() -> Self::Val {
            Self::Val::MIN
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.max(right)
        }
    }

    /// 最小公倍数
    #[derive(Debug, Clone)]
    pub struct GCD;
    impl Monoid for GCD {
        type Val = usize;
        fn id() -> Self::Val {
            Self::Val::MAX
        }
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
}
