//! 半束

use std::ops::Rem;

use crate::utils::num_traits::{Bounded, Zero};

use super::operation::{Max, Min, GCD};

/// 半束
///
/// - 単位元をもち，可環かつ冪等な2項演算
pub trait Semilattice {
    /// 元の型
    type Val: Clone;
    /// 単位元
    fn id() -> Self::Val;
    /// 可換かつ冪等な二項演算
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val;
}

// ========== 実装 ==========
impl<T: Ord + Bounded + Clone> Semilattice for Min<T> {
    type Val = T;
    fn id() -> Self::Val {
        T::max_value()
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        x.min(y).clone()
    }
}

impl<T: Ord + Bounded + Clone> Semilattice for Max<T> {
    type Val = T;
    fn id() -> Self::Val {
        T::max_value()
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        x.max(y).clone()
    }
}

impl<T: Zero + Rem<T>> Semilattice for GCD<T> {
    type Val = usize;
    fn id() -> Self::Val {
        0
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        GCD::gcd(x, y)
    }
}
