//! 比較を実装した分数の実装

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use num::{Integer, One, Zero};
use num_integer::gcd;

/// 分数を表す構造体
/// - `Frac(a, b)` := a / b
#[derive(Debug, Clone, Copy)]
pub struct Frac<T: Integer>(T, T);

impl<T: Integer + Copy> Frac<T> {
    pub fn new(a: T, b: T) -> Self {
        assert!(!b.is_zero(), "denominator must be non-zero");
        let (mut a, mut b) = (a, b);
        if b < T::zero() {
            a = T::zero() - a;
            b = T::zero() - b;
        }
        let c = gcd(a, b);
        Self(a / c, b / c)
    }

    /// 分子
    pub fn numer(&self) -> T {
        self.0
    }

    /// 分母
    pub fn denom(&self) -> T {
        self.1
    }

    /// 整数に変換する
    pub fn as_integer(&self) -> Option<T> {
        let &Self(a, b) = self;
        (a % b == T::zero()).then_some(a / b)
    }
}

impl<T> Zero for Frac<T>
where
    T: Integer + Copy,
{
    fn zero() -> Self {
        Self(T::zero(), T::one())
    }
    fn is_zero(&self) -> bool {
        !self.1.is_zero() && self.0.is_zero()
    }
}

impl<T> One for Frac<T>
where
    T: Integer + Copy,
{
    fn one() -> Self {
        Self(T::one(), T::one())
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        !self.1.is_zero() && self.0 == self.1
    }
}

impl<T> From<T> for Frac<T>
where
    T: Integer + Copy,
{
    fn from(value: T) -> Self {
        Self::new(value, T::one())
    }
}

impl<T> Add for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Frac(a, b) = self;
        let Frac(c, d) = rhs;
        Self::new(a * d + c * b, b * d)
    }
}

impl<T> Sub for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let Frac(a, b) = self;
        let Frac(c, d) = rhs;
        Self::new(a * d - c * b, b * d)
    }
}

impl<T> Mul for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let Frac(a, b) = self;
        let Frac(c, d) = rhs;
        Self::new(a * c, b * d)
    }
}

impl<T> Div for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let Frac(a, b) = self;
        let Frac(c, d) = rhs;
        assert!(!c.is_zero(), "division by zero fraction");
        Self::new(a * d, b * c)
    }
}

impl<T> AddAssign for Frac<T>
where
    T: Integer + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign for Frac<T>
where
    T: Integer + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign for Frac<T>
where
    T: Integer + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign for Frac<T>
where
    T: Integer + Copy,
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T> Neg for Frac<T>
where
    T: Integer + Copy + Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        let Frac(a, b) = self;
        Self::new(-a, b)
    }
}

impl<T> Add<T> for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl<T> Sub<T> for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        self - Self::from(rhs)
    }
}

impl<T> Mul<T> for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let Frac(a, b) = self;
        Self::new(a * rhs, b)
    }
}

impl<T> Div<T> for Frac<T>
where
    T: Integer + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        assert!(!rhs.is_zero(), "division by zero scalar");
        let Frac(a, b) = self;
        Self::new(a, b * rhs)
    }
}

impl<T> AddAssign<T> for Frac<T>
where
    T: Integer + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign<T> for Frac<T>
where
    T: Integer + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for Frac<T>
where
    T: Integer + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign<T> for Frac<T>
where
    T: Integer + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs;
    }
}

impl<T> PartialEq for Frac<T>
where
    T: Integer + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        a1 * b2 == a2 * b1
    }
}

impl<T> Eq for Frac<T>
where
    T: Integer + Copy,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> PartialOrd for Frac<T>
where
    T: Integer + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Frac<T>
where
    T: Integer + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        (a1 * b2).cmp(&(a2 * b1))
    }
}
