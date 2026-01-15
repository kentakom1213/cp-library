//! Modintの構造体

#[rustfmt::skip]
#[allow(clippy::suspicious_arithmetic_impl)]
pub mod modint {
    pub const MOD: usize = (1 << 61) - 1;
    const MOD_MINUS_1_FACTORS: &[usize] = &[2, 3, 5, 7, 11, 13, 31, 41, 61, 151, 331, 1321];
    fn mul(a: usize, b: usize) -> usize { ((a as u128 * b as u128) % MOD as u128) as usize }
    fn calcmod(x: usize) -> usize { let xu = x >> 61; let xd = x & MOD; let res = xu + xd; if res >= MOD { res - MOD } else { res } }
    use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr, num::ParseIntError, iter::{Sum, Product}};
    use rand::{thread_rng, Rng};

    use num_traits::{One, Zero};
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)] pub struct Modint(pub usize);
    impl Modint { pub fn new(n: usize) -> Self { Self(calcmod(n)) }
    fn is_primitive_root(&self) -> bool { if self.is_zero() { return false; } MOD_MINUS_1_FACTORS.iter().all(|&pi| self.pow((MOD - 1) / pi) != 1) }
    /// MOD における原始根をランダムに生成する
    pub fn generate_base() -> Self { let mut rng = thread_rng(); loop { let base_candidate = Self(rng.gen_range(2..MOD)); if base_candidate.is_primitive_root() { return base_candidate; } } } }
    impl Neg for Modint { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
    impl Add for Modint { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
    impl Sub for Modint { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
    impl Mul for Modint { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(mul(self.0, rhs.0)) } }
    impl Div for Modint { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
    impl AddAssign for Modint { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
    impl SubAssign for Modint { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
    impl MulAssign for Modint { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
    impl From<usize> for Modint { fn from(value: usize) -> Self { Modint::new(value) } }
    impl Add<usize> for Modint { type Output = Self; fn add(self, rhs: usize) -> Self { let mut res = self.0 + rhs; if res >= MOD {res -= MOD;} Modint(res) } }
    impl Sub<usize> for Modint { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::new(rhs) } }
    impl Mul<usize> for Modint { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::new(rhs) } }
    impl Div<usize> for Modint { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::new(rhs) } }
    impl AddAssign<usize> for Modint { fn add_assign(&mut self, rhs: usize) { *self += Modint::new(rhs) } }
    impl SubAssign<usize> for Modint { fn sub_assign(&mut self, rhs: usize) { *self -= Modint::new(rhs) } }
    impl MulAssign<usize> for Modint { fn mul_assign(&mut self, rhs: usize) { *self *= Modint::new(rhs) } }
    impl Display for Modint { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
    impl PartialEq<usize> for Modint { fn eq(&self, other: &usize) -> bool { self == &Modint::new(*other) } }
    impl FromStr for Modint { type Err = ParseIntError; fn from_str(s: &str) -> Result<Self, Self::Err> { usize::from_str(s).map(Modint::new) } }
    pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
    impl Fp for Modint { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = mul(res, a); } a = mul(a, a); b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
    impl Sum<Modint> for Modint { fn sum<I: Iterator<Item = Modint>>(iter: I) -> Self { iter.fold(Modint(0), |acc, x| acc + x) } }
    impl Product<Modint> for Modint { fn product<I: Iterator<Item = Modint>>(iter: I) -> Self { iter.fold(Modint(1), |acc, x| acc * x) } }
    impl Zero for Modint { fn is_zero(&self) -> bool { self.0 == 0 } fn zero() -> Self { Modint(0) } }
    impl One for Modint { fn is_one(&self) -> bool where Self: PartialEq, { self.0 == 0 } fn one() -> Self { Modint(1) } }
}
pub use modint::*;
