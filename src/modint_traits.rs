//! Modintをusizeに注入するトレイト

/// 有限体の実装
pub trait Fp {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Fp for usize {
    fn val(&self) -> usize {
        self % MOD
    }
    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }
    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }
    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }
    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }
    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }
    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }
    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

pub trait FpAssign {
    fn madd_assign(&mut self, other: usize);
    fn msub_assign(&mut self, other: usize);
    fn mmul_assign(&mut self, other: usize);
}

impl FpAssign for usize {
    fn madd_assign(&mut self, other: usize) {
        *self = self.madd(other);
    }
    fn mmul_assign(&mut self, other: usize) {
        *self = self.mmul(other);
    }
    fn msub_assign(&mut self, other: usize) {
        *self = self.msub(other);
    }
}

pub const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;
