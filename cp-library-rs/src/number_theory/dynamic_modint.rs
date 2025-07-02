//! modを動的に設定できるModint

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::number_theory::ext_euclid::inv;

/// modを動的に設定できるModint
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub struct Modint2 {
    pub value: usize,
    pub m: usize,
}

impl Modint2 {
    pub fn new(x: usize, m: usize) -> Self {
        Self { value: x % m, m }
    }

    pub fn from_isize(x: isize, m: usize) -> Self {
        Self::new(x.rem_euclid(m as isize) as usize, m)
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

impl Add<usize> for Modint2 {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        self + Modint2::new(rhs, self.m)
    }
}

impl Sub<usize> for Modint2 {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        self - Modint2::new(rhs, self.m)
    }
}

impl Mul<usize> for Modint2 {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        self * Modint2::new(rhs, self.m)
    }
}

impl Div<usize> for Modint2 {
    type Output = Self;
    fn div(self, rhs: usize) -> Self {
        self / Modint2::new(rhs, self.m)
    }
}

impl AddAssign<usize> for Modint2 {
    fn add_assign(&mut self, rhs: usize) {
        *self += Modint2::new(rhs, self.m)
    }
}

impl SubAssign<usize> for Modint2 {
    fn sub_assign(&mut self, rhs: usize) {
        *self -= Modint2::new(rhs, self.m)
    }
}

impl MulAssign<usize> for Modint2 {
    fn mul_assign(&mut self, rhs: usize) {
        *self *= Modint2::new(rhs, self.m)
    }
}

impl DivAssign<usize> for Modint2 {
    fn div_assign(&mut self, rhs: usize) {
        *self /= Modint2::new(rhs, self.m)
    }
}

impl Display for Modint2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq<usize> for Modint2 {
    fn eq(&self, other: &usize) -> bool {
        self == &Modint2::new(*other, self.m)
    }
}

pub trait Fp {
    fn pow(&self, rhs: usize) -> Self;
    /// $`a^{-1}`$ を返す．逆元が存在しない場合はpanicする．
    fn inv(&self) -> Self;
}

impl Fp for Modint2 {
    fn pow(&self, rhs: usize) -> Self {
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
        let x = inv(self.value as isize, self.m as isize).unwrap() as usize;
        Modint2::new(x, self.m)
    }
}
