//! Modintの構造体

#[rustfmt::skip]
pub mod modint {
    use std::{fmt::Display,ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr, num::ParseIntError, iter::{Sum, Product}};
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)] pub struct Modint<const MOD: usize>(pub usize);
    impl<const MOD: usize> Modint<MOD> { pub fn new(n: usize) -> Self { Self(if n < MOD { n } else { n % MOD }) } }
    impl<const MOD: usize> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
    impl<const MOD: usize> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
    impl<const MOD: usize> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
    impl<const MOD: usize> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint(self.0 * rhs.0 % MOD) } }
    impl<const MOD: usize> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
    impl<const MOD: usize> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
    impl<const MOD: usize> SubAssign for Modint<MOD> { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
    impl<const MOD: usize> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
    impl<const MOD: usize> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint::new(value) } }
    impl<const MOD: usize> Add<usize> for Modint<MOD> { type Output = Self; fn add(self, rhs: usize) -> Self { let mut res = self.0 + rhs; if res >= MOD {res -= MOD;} Modint(res) } }
    impl<const MOD: usize> Sub<usize> for Modint<MOD> { type Output = Self; fn sub(self, rhs: usize) -> Self { self - Modint::new(rhs) } }
    impl<const MOD: usize> Mul<usize> for Modint<MOD> { type Output = Self; fn mul(self, rhs: usize) -> Self { self * Modint::new(rhs) } }
    impl<const MOD: usize> Div<usize> for Modint<MOD> { type Output = Self; fn div(self, rhs: usize) -> Self { self / Modint::new(rhs) } }
    impl<const MOD: usize> AddAssign<usize> for Modint<MOD> { fn add_assign(&mut self, rhs: usize) { *self += Modint::new(rhs) } }
    impl<const MOD: usize> SubAssign<usize> for Modint<MOD> { fn sub_assign(&mut self, rhs: usize) { *self -= Modint::new(rhs) } }
    impl<const MOD: usize> MulAssign<usize> for Modint<MOD> { fn mul_assign(&mut self, rhs: usize) { *self *= Modint::new(rhs) } }
    impl<const MOD: usize> Display for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
    impl<const MOD: usize> PartialEq<usize> for Modint<MOD> { fn eq(&self, other: &usize) -> bool { self == &Modint::new(*other) } }
    impl<const MOD: usize> FromStr for Modint<MOD> { type Err = ParseIntError; fn from_str(s: &str) -> Result<Self, Self::Err> { usize::from_str(s).map(Modint::new) } }
    pub trait Fp { fn pow(&self, rhs: usize) -> Self; fn inv(&self) -> Self; }
    impl<const MOD: usize> Fp for Modint<MOD> { fn pow(&self, rhs: usize) -> Self { let (mut a, mut b) = (self.0, rhs); let mut res = 1; while b > 0 { if b & 1 == 1 { res = (res * a) % MOD; } a = (a * a) % MOD; b >>= 1u32; } Modint(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
    impl<const MOD: usize> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
    impl<const MOD: usize> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
}
use modint::*;

pub type Mod998 = Modint<998244353>;
pub type Mod1e9 = Modint<1000000007>;

#[cfg(test)]
mod test {
    use super::*;

    const MOD998: usize = 998244353;

    #[test]
    fn test_add() {
        let x: Mod998 = 998244355.into();
        let y: usize = 998244359;
        assert_eq!(x + y, 8 + MOD998);

        let a: Mod998 = MOD998.into();
        let b = 1000000007;
        let c = 20021213;
        assert_eq!(a + b + c, 21776867);
        assert_eq!(a + b + c, (21776867 + MOD998));
    }

    #[test]
    fn test_neg() {
        let x: Mod998 = 0.into();
        assert_eq!(-x, 0);

        let y = Mod998::new(10);
        assert_eq!(-y, MOD998 - 10);

        let z = Mod998::new(MOD998 + 200);
        assert_eq!(-z, MOD998 - 200);
    }

    #[test]
    fn test_sub() {
        let x = Mod998::new(0);
        let y = 1000000007;
        assert_eq!(x - y, 996488699);

        let a: Mod998 = 288230376151711744.into(); // 1 << 58
        let b: usize = 576460752303423488; // 1 << 59
        let c: usize = 1152921504606846976; // 1 << 60
        assert_eq!(-a - b - c, 553154679);

        let zero = Mod998::new(0) + 1 - 1;
        assert_eq!(zero.0, 0);
    }

    #[test]
    fn test_pow() {
        let x = Mod998::new(2);
        let y: usize = 1000000007;
        assert_eq!(x.pow(y), 132727571);

        let a: Mod998 = MOD998.into();
        let b: usize = 1024;
        assert_eq!(a.pow(b), 0);
    }

    #[test]
    fn test_inv() {
        assert_eq!(Mod998::new(1).inv(), 1);
        assert_eq!(Mod998::new(2).inv(), 499122177);
        assert_eq!(Mod998::new(1000).inv(), 981274199);
        assert_eq!(Mod998::new(998244352).inv(), 998244352);
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
        let mut add = Mod1e9::new(0);
        let mut sub = Mod1e9::new(0);
        for i in 0..20 {
            add += i;
            sub -= i;
        }

        assert_eq!(sub, -add);
    }

    #[test]
    fn test_mul_assign() {
        let mut fact = vec![Mod998::new(1); 20];

        // 階乗
        for i in 1..20 {
            let prv = fact[i - 1];
            fact[i] *= prv * i;
        }

        assert_eq!(
            &fact,
            &[
                1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
                237554682, 331032489, 972509923, 586493473, 986189864, 781263551, 868586527
            ]
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            (0..20)
                .map(|i| Mod1e9::new(2).pow(i))
                .sum::<Mod1e9>(),
            Mod1e9::new(2).pow(20) - 1
        );
    }

    #[test]
    fn test_product() {
        assert_eq!(
            (0..100)
                .map(|_| 3.into())
                .product::<Mod1e9>(),
            Mod1e9::new(3).pow(100)
        );
    }
}
