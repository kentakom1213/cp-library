//! 数の性質

use std::ops::{Add, Mul};

pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
    fn is_zero(&self) -> bool
    where
        Self: PartialEq,
    {
        self == &Self::zero()
    }
}

pub trait One: Sized + Mul<Self, Output = Self> {
    fn one() -> Self;
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self == &Self::one()
    }
}

impl Zero for usize {
    fn zero() -> Self {
        0
    }
}

impl One for usize {
    fn one() -> Self {
        1
    }
}

impl Zero for isize {
    fn zero() -> Self {
        0
    }
}

impl One for isize {
    fn one() -> Self {
        1
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.0
    }
}

impl One for f64 {
    fn one() -> Self {
        1.0
    }
}
