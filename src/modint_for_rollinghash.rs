//! Modintの構造体

#[rustfmt::skip]
pub mod modint {
    pub const MOD: usize = (1 << 61) - 1;
    const MOD128: u128 = MOD as u128;
    use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr, num::ParseIntError, iter::{Sum, Product}};
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] pub struct Modint(pub usize);
    impl Modint { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) } }
    impl Neg for Modint { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
    impl Add for Modint { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
    impl Sub for Modint { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
    impl Mul for Modint { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint((self.0 as u128 * rhs.0 as u128 % MOD128) as usize) } }
    impl Div for Modint { type Output = Self; fn div(self, rhs: Self) -> Self { Modint((self.0 as u128 * rhs.inv().0 as u128) as usize) } }
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
    impl Fp for Modint { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0 as u128, rhs as u128); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD128; } a = (a * a) % MOD128; b >>= 1u32; } Modint(res as usize) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
    impl Sum<Modint> for Modint { fn sum<I: Iterator<Item = Modint>>(iter: I) -> Self { iter.fold(Modint(0), |acc, x| acc + x) } }
    impl Product<Modint> for Modint { fn product<I: Iterator<Item = Modint>>(iter: I) -> Self { iter.fold(Modint(1), |acc, x| acc * x) } }
}
use modint::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neg() {
        let x: Modint = 0.into();
        assert_eq!(-x, 0);

        let y = Modint::new(10);
        assert_eq!(-y, MOD - 10);

        let z = Modint::new(MOD + 200);
        assert_eq!(-z, MOD - 200);
    }

    #[test]
    fn test_sub() {
        let x = Modint::new(0);
        let y = 1000000007;
        assert_eq!(x - y, MOD - y);

        let a: Modint = 288230376151711744.into(); // 1 << 58
        let b: usize = 576460752303423488; // 1 << 59
        let c: usize = 1152921504606846976; // 1 << 60
        assert_eq!(
            -a - b - c,
            MOD - (288230376151711744 + 576460752303423488 + 1152921504606846976)
        );

        let zero = Modint::new(0) + 1 - 1;
        assert_eq!(zero.0, 0);
    }

    #[test]
    fn test_pow() {
        let x = Modint::new(2);
        let y: usize = 1000000007;
        assert_eq!(x.pow(y), 35184372088832);

        let a: Modint = MOD.into();
        let b: usize = 1024;
        assert_eq!(a.pow(b), 0);
    }

    #[test]
    fn test_inv() {
        assert_eq!(Modint::new(1).inv(), 1);
        assert_eq!(Modint::new(2).inv(), 1152921504606846976);
        assert_eq!(Modint::new(1000).inv(), 1035323511136948584);
        assert_eq!(Modint::new(MOD - 1).inv(), MOD - 1);
    }

    #[test]
    fn test_add_assign() {
        let arr = vec![1, 2, 3];
        let mut ans = 0;
        for i in 0..3 {
            ans += arr[i];
        }
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_sub_assign() {
        let mut add = Modint::new(0);
        let mut sub = Modint::new(0);
        for i in 0..20 {
            add += i;
            sub -= i;
        }

        assert_eq!(sub, -add);
    }

    #[test]
    fn test_mul_assign() {
        let mut fact = vec![Modint::new(1); 20];

        // 階乗
        for i in 1..20 {
            let prv = fact[i - 1];
            fact[i] *= prv * i;
        }

        assert_eq!(
            &fact,
            &[
                1,
                1,
                2,
                6,
                24,
                120,
                720,
                5040,
                40320,
                362880,
                3628800,
                39916800,
                479001600,
                6227020800,
                87178291200,
                1307674368000,
                20922789888000,
                355687428096000,
                6402373705728000,
                121645100408832000
            ]
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            (0..20).map(|i| Modint::new(2).pow(i)).sum::<Modint>(),
            Modint::new(2).pow(20) - 1
        );
    }

    #[test]
    fn test_product() {
        assert_eq!(
            (0..100).map(|_| 3.into()).product::<Modint>(),
            Modint::new(3).pow(100)
        );
    }
}
