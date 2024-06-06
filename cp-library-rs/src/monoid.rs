//! モノイド

use crate::affine1d::{Affine, AffineTransform};
use std::fmt::Debug;

/// モノイド
pub trait Monoid {
    /// 元の型
    type Val: Debug + Clone + PartialEq;
    /// 単位元
    fn id() -> Self::Val;
    /// 演算
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

/// 各種モノイド
pub mod examples {
    use super::*;

    /// 和
    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        fn id() -> Self::Val {
            0
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }

    /// 積
    pub struct Mul;
    impl Monoid for Mul {
        type Val = isize;
        fn id() -> Self::Val {
            1
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left * right
        }
    }

    /// bit単位の排他的論理和
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

    /// アフィン変換（浮動小数点数）
    struct Affine1d;
    impl Monoid for Affine1d {
        type Val = Affine<f64>;
        fn id() -> Self::Val {
            (1.0, 0.0)
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left.compose(&right)
        }
    }
}
