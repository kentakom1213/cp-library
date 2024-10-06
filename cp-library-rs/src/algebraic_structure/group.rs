//! 群の実装

use crate::algebraic_structure::{monoid::Monoid, operation::*};

/// 群
pub trait Group: Monoid {
    fn inv(val: &Self::Val) -> Self::Val;
}

// 実装
impl Group for Add<isize> {
    fn inv(val: &Self::Val) -> Self::Val {
        -val
    }
}

impl Group for Add<usize> {
    fn inv(val: &Self::Val) -> Self::Val {
        val.wrapping_neg()
    }
}

impl Group for Xor {
    fn inv(val: &Self::Val) -> Self::Val {
        *val
    }
}
