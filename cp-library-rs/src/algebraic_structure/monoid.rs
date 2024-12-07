//! ## モノイド
//!
//! - [`Monoid::Val`] ： データの型 $`S`$
//! - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
//! - [`Monoid::op`] ： 演算 $`S\times S \to S`$

macro_rules! impl_monoid_add {
    ($ty:ty, $id:expr) => {
        impl Monoid for Add<$ty> {
            type Val = $ty;
            fn id() -> Self::Val {
                $id
            }
            fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
                *left + *right
            }
        }
    };
}

use crate::{
    algebraic_structure::operation::{Add, Max, Min, Mul, Xor, GCD},
    number_theory::modint::{M107, M998},
    utils::num_traits::{Bounded, One},
};

use super::operation::MinMax;

/// モノイド
///
/// - [`Monoid::Val`] ： データの型 $`S`$
/// - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
/// - [`Monoid::op`] ： 演算 $`S\times S \to S`$
pub trait Monoid {
    /// データの型 （$`S`$）
    type Val: Clone;
    /// 単位元 （$`\varnothing \to S`$）
    fn id() -> Self::Val;
    /// 演算 （$`S \times S \to S`$）
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

// ========== 実装 ==========
impl<T: Ord + Bounded + Clone> Monoid for Min<T> {
    type Val = T;
    fn id() -> Self::Val {
        Self::Val::max_value()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.min(right).clone()
    }
}

impl<T: Ord + Bounded + Clone> Monoid for Max<T> {
    type Val = T;
    fn id() -> Self::Val {
        Self::Val::min_value()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.max(right).clone()
    }
}

impl<T: Ord + Bounded + Clone> Monoid for MinMax<T> {
    type Val = (T, T);
    fn id() -> Self::Val {
        (Min::id(), Max::id())
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        (Min::op(&left.0, &right.0), Max::op(&left.1, &right.1))
    }
}

impl Monoid for Add<usize> {
    type Val = usize;
    fn id() -> Self::Val {
        0
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.wrapping_add(*right)
    }
}
impl_monoid_add!(isize, 0);
impl_monoid_add!(f64, 0.0);
impl_monoid_add!(M107, M107::new(0));
impl_monoid_add!(M998, M998::new(0));

impl<T: One + Clone> Monoid for Mul<T> {
    type Val = T;
    fn id() -> Self::Val {
        T::one()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.clone() * right.clone()
    }
}

impl Monoid for Xor {
    type Val = usize;
    fn id() -> Self::Val {
        0
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left ^ right
    }
}

impl<T> Monoid for GCD<T> {
    type Val = usize;
    fn id() -> Self::Val {
        0
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        GCD::gcd(left, right)
    }
}
