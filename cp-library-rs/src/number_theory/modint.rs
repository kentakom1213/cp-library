//! Modintの構造体

pub use modint::*;

/// MOD用の定数：$`998244353`$
pub const MOD998: u32 = 998244353;

/// MOD用の定数：$`10^9 + 7`$
pub const MOD107: u32 = 1000000007;

pub type M998 = Modint<MOD998>;
pub type M107 = Modint<MOD107>;

// 適当な素数
pub type P1 = Modint<938472061>;
pub type P2 = Modint<958472071>;

#[rustfmt::skip]
pub mod modint {
    macro_rules! impl_ops {
        ($t:ty, $op_trait:ident, $op_func:ident, $op:tt) => {
            impl<const MOD: u32> $op_trait<$t> for Modint<MOD> { type Output = Self; fn $op_func(self, rhs: $t) -> Self { self $op Modint::from(rhs) } }
        };
        (assign, $t:ty, $op_trait:ident, $op_func:ident, $op:tt) => {
            impl<const MOD: u32> $op_trait<$t> for Modint<MOD> { fn $op_func(&mut self, rhs: $t) { *self = *self $op Modint::from(rhs) } }
        };
        (partial_eq, $t:ty) => {
            impl<const MOD: u32> PartialEq<$t> for Modint<MOD> { fn eq(&self, other: &$t) -> bool { self == &Modint::from(*other) } }
        };
        (all, $t:ty) => {
            impl_ops!($t, Add, add, +);
            impl_ops!($t, Sub, sub, -);
            impl_ops!($t, Mul, mul, *);
            impl_ops!($t, Div, div, /);
            impl_ops!(assign, $t, AddAssign, add_assign, +);
            impl_ops!(assign, $t, SubAssign, sub_assign, -);
            impl_ops!(assign, $t, MulAssign, mul_assign, *);
            impl_ops!(assign, $t, DivAssign, div_assign, /);
            impl_ops!(partial_eq, $t);
        };
    }

    fn sqrt(n: u32) -> usize { (n as f64).sqrt() as usize }
    use std::{fmt::{Debug, Display}, iter::{Product, Sum}, mem::replace, num::ParseIntError, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign}, str::FromStr};

    use crate::utils::num_traits::{One, Zero};
    #[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)] pub struct Modint<const MOD: u32>(pub u32);
    impl<const MOD: u32> Modint<MOD> { pub fn new(n: u32) -> Self { Self(if n < MOD { n } else { n % MOD }) }
    pub fn rational_reconstruction(&self) -> Option<(usize, usize)> { let N = sqrt(MOD / 2); let mut v = (MOD as usize, 0); let mut w = (self.0 as usize, 1);
    while w.0 > N { let q = v.0.div_euclid(w.0); let z = (v.0 - q * w.0, v.1 + q * w.1); v = replace(&mut w, z); } (w.0 <= N && w.1 <= N).then_some(w) } }

    impl<const MOD: u32> Neg for Modint<MOD> { type Output = Self; fn neg(self) -> Self { Modint(if self.0 == 0 { 0 } else { MOD - self.0 }) } }
    impl<const MOD: u32> Add for Modint<MOD> { type Output = Self; fn add(self, rhs: Self) -> Self { let mut res = self.0 + rhs.0; if res >= MOD { res -= MOD; } Modint(res) } }
    impl<const MOD: u32> Sub for Modint<MOD> { type Output = Self; fn sub(self, rhs: Self) -> Self { self + (- rhs) } }
    impl<const MOD: u32> Mul for Modint<MOD> { type Output = Self; fn mul(self, rhs: Self) -> Self { Modint((self.0 as u64 * rhs.0 as u64 % MOD as u64) as u32) } }
    impl<const MOD: u32> Div for Modint<MOD> { type Output = Self; fn div(self, rhs: Self) -> Self { self * rhs.inv() } }
    impl<const MOD: u32> AddAssign for Modint<MOD> { fn add_assign(&mut self, rhs: Self) { self.0 = (*self + rhs).0 } }
    impl<const MOD: u32> SubAssign for Modint<MOD> { fn sub_assign(&mut self, rhs: Self) { self.0 = (*self - rhs).0 } }
    impl<const MOD: u32> MulAssign for Modint<MOD> { fn mul_assign(&mut self, rhs: Self) { self.0 = (*self * rhs).0 } }
    impl<const MOD: u32> DivAssign for Modint<MOD> { fn div_assign(&mut self, rhs: Self) { self.0 = (*self / rhs).0 } }

    impl<const MOD: u32> From<u32> for Modint<MOD> { fn from(value: u32) -> Self { Modint::new(value) } }
    impl<const MOD: u32> From<i32> for Modint<MOD> { fn from(value: i32) -> Self { Modint(value.rem_euclid(MOD as i32) as u32) } }
    impl<const MOD: u32> From<u64> for Modint<MOD> { fn from(value: u64) -> Self { Modint((value % MOD as u64) as u32) } }
    impl<const MOD: u32> From<i64> for Modint<MOD> { fn from(value: i64) -> Self { Modint(value.rem_euclid(MOD as i64) as u32) } }
    impl<const MOD: u32> From<usize> for Modint<MOD> { fn from(value: usize) -> Self { Modint((value % MOD as usize) as u32) } }
    impl<const MOD: u32> From<isize> for Modint<MOD> { fn from(value: isize) -> Self { Modint(value.rem_euclid(MOD as isize) as u32) } }

    // impl_ops!(all, u32);
    // impl_ops!(all, i32);
    // impl_ops!(all, u64);
    // impl_ops!(all, i64);
    impl_ops!(all, usize);
    // impl_ops!(all, isize);

    impl<const MOD: u32> Display for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) } }
    impl<const MOD: u32> FromStr for Modint<MOD> { type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> { let chunk_size = 17; let mut chars = s.chars(); let mut chunk = chars.by_ref().take(chunk_size).collect::<String>(); let mut res = Modint::zero();
    while !chunk.is_empty() { res = res * Modint::new(10).pow(chunk.len() as u32) + chunk.parse::<usize>()?; chunk = chars.by_ref().take(chunk_size).collect::<String>(); } Ok(res) } }
    // impl<const MOD: u32> Debug for Modint<MOD> { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { match self.rational_reconstruction() { Some((n, d)) => if d > 1 { write!(f, "Modint({n}/{d})") } else { write!(f, "Modint({n})") } _ => write!(f, "Modint({})", self.0) } } }
    impl<const MOD: u32> Zero for Modint<MOD> { fn zero() -> Self { Modint(0) } }
    impl<const MOD: u32> One for Modint<MOD> { fn one() -> Self { Modint(1) } }
    pub trait Fp { fn pow(&self, rhs: u32) -> Self; fn inv(&self) -> Self; }
    impl<const MOD: u32> Fp for Modint<MOD> { fn pow(&self, rhs: u32) -> Self { let (mut a, mut b) = (*self, rhs); let mut res = Modint::one(); while b > 0 { if b & 1 == 1 { res *= a; } a *= a; b >>= 1u32; } Modint::from(res) } fn inv(&self) -> Self { self.pow(MOD - 2) } }
    impl<const MOD: u32> Sum<Modint<MOD>> for Modint<MOD> { fn sum<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(0), |acc, x| acc + x) } }
    impl<const MOD: u32> Product<Modint<MOD>> for Modint<MOD> { fn product<I: Iterator<Item = Modint<MOD>>>(iter: I) -> Self { iter.fold(Modint::<MOD>(1), |acc, x| acc * x) } }
}
