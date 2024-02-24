//! セグ木用の各種モノイド

use crate::monoid::Monoid;

/// 和
pub struct Add;
impl Monoid for Add {
    type Val = isize;
    const E: Self::Val = 0;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left + right
    }
}

/// 積
pub struct Mul;
impl Monoid for Mul {
    type Val = isize;
    const E: Self::Val = 1;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left * right
    }
}

/// bit単位の排他的論理和
pub struct Xor;
impl Monoid for Xor {
    type Val = usize;
    const E: Self::Val = 0;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        left ^ right
    }
}

/// 最小値
pub struct Min;
impl Monoid for Min {
    type Val = isize;
    const E: Self::Val = (1 << 31) - 1;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        *left.min(right)
    }
}

/// 最大値
pub struct Max;
impl Monoid for Max {
    type Val = isize;
    const E: Self::Val = -((1 << 31) - 1);
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        *left.max(right)
    }
}

/// 最小公倍数
pub struct GCD;
impl Monoid for GCD {
    type Val = usize;
    const E: Self::Val = 0;
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
