//! 数の性質

use std::ops::{Add, Mul};

pub trait Zero: Sized + Eq + Add<Self, Output = Self> {
    fn zero() -> Self;
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

pub trait One: Sized + Eq + Mul<Self, Output = Self> {
    fn one() -> Self;
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self == &Self::one()
    }
}
