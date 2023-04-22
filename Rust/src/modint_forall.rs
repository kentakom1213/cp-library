#![allow(dead_code)]

/// ## Fp
/// 有限体の実装
pub trait Fp
where
    Self: core::ops::Rem<Output = Self>
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + Sized
        + Copy,
{
    const MOD: Self;
    const ADD_E: Self;
    const MUL_E: Self;

    fn mneg(&self) -> Self {
        (Self::MOD - *self % Self::MOD) % Self::MOD
    }

    fn madd(&self, other: Self) -> Self {
        (*self % Self::MOD + other % Self::MOD) % Self::MOD
    }

    fn msub(&self, other: Self) -> Self {
        self.madd(Self::MOD - other % Self::MOD)
    }

    fn mmul(&self, other: Self) -> Self {
        (*self % Self::MOD * other % Self::MOD) % Self::MOD
    }

    fn mpow(&self, other: usize) -> Self {
        let (mut a, mut b) = (*self % Self::MOD, other);
        let mut res = Self::MUL_E;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> Self;

    fn mdiv(&self, other: Self) -> Self {
        self.mmul(other.minv())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    impl Fp for usize {
        const MOD: usize = 998_244_353;
        const ADD_E: usize = 0;
        const MUL_E: usize = 1;
        fn minv(&self) -> usize {
            assert_ne!(*self, 0);
            self.mpow(Self::MOD - 2)
        }
    }

    #[test]
    fn test_madd() {
        let x: usize = 998244355;
        let y: usize = 998244359;
        assert_eq!(x.madd(y), 8);

        let a: usize = 998244353;
        let b: usize = 1000000007;
        let c: usize = 20021213;
        assert_eq!(a.madd(b).madd(c), 21776867);
    }

    #[test]
    fn test_mneg() {
        let x: usize = 0;
        assert_eq!(x.mneg(), 0);
    }

    #[test]
    fn test_msub() {
        let x: usize = 0;
        let y: usize = 1000000007;
        assert_eq!(x.msub(y), 996488699);

        let a: usize = 288230376151711744; // 1 << 58
        let b: usize = 576460752303423488; // 1 << 59
        let c: usize = 1152921504606846976; // 1 << 60
        assert_eq!(a.mneg().msub(b).msub(c), 553154679);
    }

    #[test]
    fn test_mpow() {
        let x: usize = 2;
        let y: usize = 1000000007;
        assert_eq!(x.mpow(y), 132727571);

        let a: usize = 998244353;
        let b: usize = 1024;
        assert_eq!(a.mpow(b), 0);
    }

    #[test]
    fn test_minv() {
        assert_eq!(1.minv(), 1);
        assert_eq!(2.minv(), 499122177);
        assert_eq!(1000.minv(), 981274199);
        assert_eq!((usize::MOD - 1).minv(), 998244352);
    }

    #[test]
    #[should_panic]
    fn test_minv_err() {
        0.minv();
    }

    #[test]
    #[should_panic]
    fn test_mdiv_err() {
        1.mdiv(0);
    }
}
