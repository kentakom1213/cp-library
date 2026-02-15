//! Modintの構造体

pub use modint_::*;

/// MOD用の定数：$`998244353`$
pub const MOD998: u64 = 998244353;

/// MOD用の定数：$`10^9 + 7`$
pub const MOD107: u64 = 1000000007;

pub type M998 = Modint<MOD998>;
pub type M107 = Modint<MOD107>;

// 適当な素数
pub type P1 = Modint<938472061>;
pub type P2 = Modint<958472071>;

#[rustfmt::skip]
#[allow(clippy::suspicious_arithmetic_impl)]
pub mod modint_ {
    fn sqrt(n: u64) -> u64 { (n as f64).sqrt() as u64 }
    use std::{fmt::{Debug, Display}, iter::{Product, Sum}, mem::replace, num::ParseIntError, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};
    

    use num_traits::{One, Zero};
    #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)] pub struct Modint<const MOD: u64>(pub u64);
    impl<const MOD: u64> Modint<MOD> { pub fn new(n: u64) -> Self { Self(if n < MOD { n } else { n % MOD }) }
    pub fn from_isize(n: isize) -> Self { Self::new(n.rem_euclid(MOD as isize) as u64) }
    pub fn rational_reconstruction(&self) -> Option<(u64, u64)> { let N = sqrt(MOD / 2); let mut v = (MOD, 0); let mut w = (self.0, 1);
    while w.0 > N { let q = v.0.div_euclid(w.0); let z = (v.0 - q * w.0, v.1 + q * w.1); v = replace(&mut w, z); } (w.0 <= N && w.1 <= N).then_some(w) } }
    impl<const MOD: u64> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
    impl<const MOD: u64> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
    impl<const MOD: u64> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
    impl<const MOD: u64> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(self.0 * rhs.0 % MOD) } }
    impl<const MOD: u64> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
    impl<const MOD: u64> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
    impl<const MOD: u64> SubAssign for Modint<MOD> { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
    impl<const MOD: u64> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
    impl<const MOD: u64> DivAssign for Modint<MOD> { fn div_assign(&mut self, rhs: Self) { self.0 = (*self / rhs).0 } }
    impl<const MOD: u64> From<u64> for Modint<MOD> { fn from(value: u64) -> Self { Modint::new(value) } }
    impl<const MOD: u64> Add<u64> for Modint<MOD> { type Output = Self; fn add(self, rhs: u64) -> Self { self + Modint::new(rhs) } }
    impl<const MOD: u64> Sub<u64> for Modint<MOD> { type Output = Self; fn sub(self, rhs: u64) -> Self { self - Modint::new(rhs) } }
    impl<const MOD: u64> Mul<u64> for Modint<MOD> { type Output = Self; fn mul(self, rhs: u64) -> Self { self * Modint::new(rhs) } }
    impl<const MOD: u64> Div<u64> for Modint<MOD> { type Output = Self; fn div(self, rhs: u64) -> Self { self / Modint::new(rhs) } }
    impl<const MOD: u64> AddAssign<u64> for Modint<MOD> { fn add_assign(&mut self, rhs: u64) { *self += Modint::new(rhs) } }
    impl<const MOD: u64> SubAssign<u64> for Modint<MOD> { fn sub_assign(&mut self, rhs: u64) { *self -= Modint::new(rhs) } }
    impl<const MOD: u64> MulAssign<u64> for Modint<MOD> { fn mul_assign(&mut self, rhs: u64) { *self *= Modint::new(rhs) } }
    impl<const MOD: u64> DivAssign<u64> for Modint<MOD> { fn div_assign(&mut self, rhs: u64) { *self /= Modint::new(rhs) } }
    impl<const MOD: u64> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint::new(value as u64) } }
    impl<const MOD: u64> Add<usize> for Modint<MOD> { type Output = Self; fn add(self, rhs: usize) -> Self { self + Modint::from(rhs) } }
    impl<const MOD: u64> Sub<usize> for Modint<MOD> { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::from(rhs) } }
    impl<const MOD: u64> Mul<usize> for Modint<MOD> { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::from(rhs) } }
    impl<const MOD: u64> Div<usize> for Modint<MOD> { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::from(rhs) } }
    impl<const MOD: u64> AddAssign<usize> for Modint<MOD> { fn add_assign(&mut self, rhs: usize) { *self += Modint::from(rhs) } }
    impl<const MOD: u64> SubAssign<usize> for Modint<MOD> { fn sub_assign(&mut self, rhs: usize) { *self -= Modint::from(rhs) } }
    impl<const MOD: u64> MulAssign<usize> for Modint<MOD> { fn mul_assign(&mut self, rhs: usize) { *self *= Modint::from(rhs) } }
    impl<const MOD: u64> DivAssign<usize> for Modint<MOD> { fn div_assign(&mut self, rhs: usize) { *self /= Modint::from(rhs) } }
    impl<const MOD: u64> Display for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
    impl<const MOD: u64> PartialEq<u64> for Modint<MOD> { fn eq(&self, other: &u64) -> bool { self == &Modint::new(*other) } }
    impl<const MOD: u64> FromStr for Modint<MOD> { type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { let chunk_size = 9; let mut chars = s.chars(); let mut chunk = chars.by_ref().take(chunk_size).collect::<String>(); let mut res = Modint::zero();
    while !chunk.is_empty() { res = res * Modint::new(10).pow(chunk.len() as u64) + chunk.parse::<u64>()?; chunk = chars.by_ref().take(chunk_size).collect::<String>(); } Ok(res) } }
    impl<const MOD: u64> Zero for Modint<MOD> { fn is_zero(&self) -> bool { self.0 == 0 } fn zero() -> Self { Modint(0) } }
    impl<const MOD: u64> One for Modint<MOD> { fn is_one(&self) -> bool where Self: PartialEq, { self.0 == 0 } fn one() -> Self { Modint(1) } }
    pub trait Fp { fn pow(&self, rhs: u64) -> Self; fn powi(&self, rhs: isize) -> Self ; fn inv(&self) -> Self; } 
    impl<const MOD: u64> Fp for Modint<MOD> {
    fn pow(&self, rhs: u64) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) }
    fn powi(&self, rhs: isize) -> Self { match rhs { ..0 => self.pow((-rhs) as u64).inv(), _ => self.pow(rhs as u64) } }
    fn inv(&self) -> Self { self.pow(MOD - 2) } }
    impl<const MOD: u64> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
    impl<const MOD: u64> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
}
