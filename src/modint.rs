//! modint

#![allow(dead_code)]
#![allow(unused_macros)]

pub const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

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

macro_rules! madd {
    ( $a:expr, $b:expr ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
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

pub const SIZE: usize = 505050;

pub const FACT: [usize; SIZE] = {
    let mut fact = [1; SIZE];
    let mut i = 1;
    while i < fact.len() {
        fact[i] = fact[i - 1] * i % MOD;
        i += 1;
    }
    fact
};

/// 組合せの計算
pub trait Comb: Fp {
    /// 階乗を求める
    fn factorial(&self) -> usize;
    /// 順列を求める
    fn perm(&self, other: usize) -> usize;
    /// 組合せを求める
    fn comb(&self, other: usize) -> usize;
    /// 重複を許す組合せ(combination with repetition)
    fn comb_with_rep(&self, other: usize) -> usize;
}

impl Comb for usize {
    fn factorial(&self) -> usize {
        assert!(*self < SIZE, "Exceeds precomputation size.");
        *FACT.get(*self).unwrap()
    }
    fn perm(&self, other: usize) -> usize {
        self.factorial().mdiv((*self - other).factorial())
    }
    fn comb(&self, other: usize) -> usize {
        if *self < other {
            return 0;
        }
        self.factorial()
            .mdiv(other.factorial())
            .mdiv((*self - other).factorial())
    }
    fn comb_with_rep(&self, other: usize) -> usize {
        (*self + other - 1).comb(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        assert_eq!((MOD - 1).minv(), 998244352);
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

    #[test]
    fn test_madd_macro() {
        let mut arr = vec![1, 2, 3];
        for i in 0..3 {
            madd!(arr[i], arr[i]);
        }
    }

    #[test]
    fn test_madd_assign() {
        let arr = vec![1, 2, 3];
        let mut ans = 0;
        for i in 0..3 {
            ans.madd_assign(arr[i]);
        }
        assert_eq!(ans, 6);
    }

    #[test]
    fn test_const_array() {
        assert_eq!(
            &FACT[..20],
            &[
                1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600,
                237554682, 331032489, 972509923, 586493473, 986189864, 781263551, 868586527
            ]
        )
    }

    #[test]
    fn test_factorial() {
        // 小さい数
        let mut x = 1;
        for i in 1..20 {
            x = x * i % MOD;
            assert_eq!(i.factorial(), x);
        }

        // 大きい数
        assert_eq!(100.factorial(), 35305197);
        assert_eq!(200.factorial(), 245648024);
        assert_eq!(300.factorial(), 310896195);
        assert_eq!(1000.factorial(), 421678599);
        assert_eq!(5000.factorial(), 779957103);
        assert_eq!(8941.factorial(), 906514862);
        assert_eq!(10000.factorial(), 777990065);
        assert_eq!(50000.factorial(), 734256002);
        assert_eq!(50051.factorial(), 973600151);
        assert_eq!(100000.factorial(), 215582594);
        assert_eq!(100001.factorial(), 389935206);
        assert_eq!(500009.factorial(), 978886571);
    }

    #[test]
    fn test_permutation() {
        assert_eq!(5.perm(3), 60);
        assert_eq!(349192.perm(183860), 545014128);
        assert_eq!(408260.perm(141033), 568084541);
        assert_eq!(302385.perm(90762), 363332267);
        assert_eq!(422790.perm(410280), 772920431);
        assert_eq!(472062.perm(319870), 256733754);
        assert_eq!(248533.perm(73060), 733999468);
        assert_eq!(252319.perm(143521), 706025716);
        assert_eq!(191800.perm(6326), 759308997);
        assert_eq!(414251.perm(235179), 672952687);
        assert_eq!(31353.perm(22071), 690599083);
        assert_eq!(95805.perm(21679), 561322527);
        assert_eq!(408083.perm(52108), 542443119);
        assert_eq!(469876.perm(466473), 843578753);
        assert_eq!(107846.perm(22030), 71461903);
        assert_eq!(328957.perm(271541), 670944266);
        assert_eq!(360650.perm(327552), 32074145);
        assert_eq!(473087.perm(139934), 241607404);
        assert_eq!(364381.perm(197801), 278234240);
        assert_eq!(437695.perm(413667), 539537573);
        assert_eq!(189171.perm(172484), 455977866);
    }

    #[test]
    fn test_combination() {
        assert_eq!(259242.comb(87705), 774328561);
        assert_eq!(71363.comb(25183), 536450162);
        assert_eq!(475558.comb(122225), 420488366);
        assert_eq!(368429.comb(306431), 112920167);
        assert_eq!(395820.comb(283172), 799630048);
        assert_eq!(274510.comb(71447), 223324216);
        assert_eq!(429876.comb(132102), 775071833);
        assert_eq!(407521.comb(42009), 244493769);
        assert_eq!(438833.comb(80716), 858062498);
        assert_eq!(470066.comb(39209), 432758784);
        assert_eq!(415740.comb(264741), 820821026);
        assert_eq!(434436.comb(221664), 995889028);
        assert_eq!(130516.comb(121882), 113770683);
        assert_eq!(486996.comb(353909), 300270267);
        assert_eq!(478668.comb(108726), 306915821);
        assert_eq!(414714.comb(232132), 611611130);
        assert_eq!(393359.comb(37171), 823768816);
        assert_eq!(416741.comb(286395), 255449048);
        assert_eq!(296230.comb(255641), 644480874);
        assert_eq!(384285.comb(200334), 59276605);
    }
}
