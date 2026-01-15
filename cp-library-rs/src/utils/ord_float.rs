//! 順序付きの浮動小数点数型 `OrdF64` の定義

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

use num_traits::{Bounded, One, Zero};

macro_rules! impl_ord_f64 {
    (f64, $op_trait:ident, $op_func:ident, $op:tt) => {
        impl $op_trait<f64> for OrdF64 {
            type Output = Self;
            fn $op_func(self, rhs: f64) -> Self::Output {
                Self(self.0 $op rhs)
            }
        }
    };
    ($op_trait:ident, $op_func:ident, $op:tt) => {
        impl $op_trait for OrdF64 {
            type Output = Self;
            fn $op_func(self, rhs: Self) -> Self::Output {
                Self(self.0 $op rhs.0)
            }
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OrdF64(pub f64);

impl Eq for OrdF64 {}

impl PartialOrd for OrdF64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for OrdF64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<f64> for OrdF64 {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl Bounded for OrdF64 {
    fn min_value() -> Self {
        Self(f64::MIN)
    }
    fn max_value() -> Self {
        Self(f64::MAX)
    }
}

impl Zero for OrdF64 {
    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
    fn zero() -> Self {
        Self(0.0)
    }
}

impl One for OrdF64 {
    fn one() -> Self {
        Self(1.0)
    }
}

// 演算の定義
impl_ord_f64!(Add, add, +);
impl_ord_f64!(Sub, sub, -);
impl_ord_f64!(Mul, mul, *);
impl_ord_f64!(Div, div, /);
impl_ord_f64!(Rem, rem, %);
impl_ord_f64!(f64, Add, add, +);
impl_ord_f64!(f64, Sub, sub, -);
impl_ord_f64!(f64, Mul, mul, *);
impl_ord_f64!(f64, Div, div, /);
impl_ord_f64!(f64, Rem, rem, %);
