//! モノイド

use std::fmt::Debug;

/// モノイド
pub trait Monoid {
    /// 元の型
    type Val: Debug + Clone + PartialEq;
    /// 単位元
    const E: Self::Val;
    /// 演算
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}
