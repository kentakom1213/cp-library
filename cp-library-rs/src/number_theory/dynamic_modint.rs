//! modを動的に設定できるModint

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::number_theory::ext_euclid::inv;

/// modを動的に設定できるModint
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Modint2 {
    pub value: u64,
    pub m: u64,
}

impl Modint2 {
    pub fn new(x: u64, m: u64) -> Self {
        Self { value: x % m, m }
    }

    pub fn from_usize(x: usize, m: u64) -> Self {
        Self::new(x as u64, m)
    }

    pub fn from_isize(x: i64, m: u64) -> Self {
        Self::new(x.rem_euclid(m as i64) as u64, m)
    }
}

impl Neg for Modint2 {
    type Output = Self;
    fn neg(self) -> Self {
        Modint2::new(
            if self.value == 0 {
                0
            } else {
                self.m - self.value
            },
            self.m,
        )
    }
}

impl Add for Modint2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m);
        let mut res = self.value + rhs.value;
        if res >= self.m {
            res -= self.m;
        }
        Modint2::new(res, self.m)
    }
}

impl Sub for Modint2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m);
        self + (-rhs)
    }
}

impl Mul for Modint2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m);
        Modint2::new(self.value * rhs.value % self.m, self.m)
    }
}

impl Div for Modint2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        assert!(self.m == rhs.m);
        self * rhs.inv()
    }
}

impl AddAssign for Modint2 {
    fn add_assign(&mut self, rhs: Self) {
        assert!(self.m == rhs.m);
        self.value = (*self + rhs).value
    }
}

impl SubAssign for Modint2 {
    fn sub_assign(&mut self, rhs: Self) {
        assert!(self.m == rhs.m);
        self.value = (*self - rhs).value
    }
}

impl MulAssign for Modint2 {
    fn mul_assign(&mut self, rhs: Self) {
        assert!(self.m == rhs.m);
        self.value = (*self * rhs).value
    }
}

impl DivAssign for Modint2 {
    fn div_assign(&mut self, rhs: Self) {
        assert!(self.m == rhs.m);
        self.value = (*self / rhs).value
    }
}

impl Add<u64> for Modint2 {
    type Output = Self;
    fn add(self, rhs: u64) -> Self {
        self + Modint2::new(rhs, self.m)
    }
}

impl Sub<u64> for Modint2 {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self {
        self - Modint2::new(rhs, self.m)
    }
}

impl Mul<u64> for Modint2 {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self {
        self * Modint2::new(rhs, self.m)
    }
}

impl Div<u64> for Modint2 {
    type Output = Self;
    fn div(self, rhs: u64) -> Self {
        self / Modint2::new(rhs, self.m)
    }
}

impl AddAssign<u64> for Modint2 {
    fn add_assign(&mut self, rhs: u64) {
        *self += Modint2::new(rhs, self.m)
    }
}

impl SubAssign<u64> for Modint2 {
    fn sub_assign(&mut self, rhs: u64) {
        *self -= Modint2::new(rhs, self.m)
    }
}

impl MulAssign<u64> for Modint2 {
    fn mul_assign(&mut self, rhs: u64) {
        *self *= Modint2::new(rhs, self.m)
    }
}

impl DivAssign<u64> for Modint2 {
    fn div_assign(&mut self, rhs: u64) {
        *self /= Modint2::new(rhs, self.m)
    }
}

impl Add<usize> for Modint2 {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self + Modint2::from_usize(rhs, self.m)
    }
}

impl Sub<usize> for Modint2 {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Modint2::from_usize(rhs, self.m)
    }
}

impl Mul<usize> for Modint2 {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        self * Modint2::from_usize(rhs, self.m)
    }
}

impl Div<usize> for Modint2 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        self / Modint2::from_usize(rhs, self.m)
    }
}

impl AddAssign<usize> for Modint2 {
    fn add_assign(&mut self, rhs: usize) {
        *self += Modint2::from_usize(rhs, self.m)
    }
}

impl SubAssign<usize> for Modint2 {
    fn sub_assign(&mut self, rhs: usize) {
        *self -= Modint2::from_usize(rhs, self.m)
    }
}

impl MulAssign<usize> for Modint2 {
    fn mul_assign(&mut self, rhs: usize) {
        *self *= Modint2::from_usize(rhs, self.m)
    }
}

impl DivAssign<usize> for Modint2 {
    fn div_assign(&mut self, rhs: usize) {
        *self /= Modint2::from_usize(rhs, self.m)
    }
}

impl Display for Modint2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq<u64> for Modint2 {
    fn eq(&self, other: &u64) -> bool {
        self == &Modint2::new(*other, self.m)
    }
}

pub trait Fp {
    fn pow(&self, rhs: u64) -> Self;
    /// $`a^{-1}`$ を返す．逆元が存在しない場合はpanicする．
    fn inv(&self) -> Self;
}

impl Fp for Modint2 {
    fn pow(&self, rhs: u64) -> Self {
        let (mut a, mut b) = (self.value, rhs);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = (res * a) % self.m;
            }
            a = (a * a) % self.m;
            b >>= 1u32;
        }
        Modint2::new(res, self.m)
    }

    fn inv(&self) -> Self {
        let x = inv(self.value as i64, self.m as i64).unwrap() as u64;
        Modint2::new(x, self.m)
    }
}
