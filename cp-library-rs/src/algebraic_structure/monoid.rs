//! ## モノイド
//!
//! - [`Monoid::Val`] ： データの型 $`S`$
//! - [`Monoid::id`] ： 単位元を返す関数 $`\varnothing \to S`$
//! - [`Monoid::op`] ： 演算 $`S\times S \to S`$

use std::fmt::Debug;

use crate::utils::num_traits::{Bounded, One};

use super::operation::{Add, Max, Min, Mul, Xor, GCD};

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

// ========== 実装 ==========
impl<T: Ord + Bounded + Clone + Debug> Monoid for Min<T> {
    type Val = T;
    fn id() -> Self::Val {
        Self::Val::max_value()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.min(right).clone()
    }
}

impl Monoid for Add<isize> {
    type Val = isize;
    fn id() -> Self::Val {
        0
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left + right
    }
}
impl Monoid for Add<f64> {
    type Val = f64;
    fn id() -> Self::Val {
        0.0
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left + right
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

impl<T: One + Clone + Debug> Monoid for Mul<T> {
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

impl<T: Ord + Bounded + Clone + Debug> Monoid for Max<T> {
    type Val = T;
    fn id() -> Self::Val {
        Self::Val::min_value()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left.max(right).clone()
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
