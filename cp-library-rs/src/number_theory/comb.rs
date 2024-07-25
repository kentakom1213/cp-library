//! 二項係数を求める

const MOD: usize = 998_244_353;
// const MOD: usize = 1_000_000_007;

/// # Comb
/// 二項係数を高速に求める
/// - 前計算 : $`O(N)`$
/// - クエリ : $`O(1)`$
pub struct Comb {
    fac: Vec<usize>,
    finv: Vec<usize>,
}

impl Comb {
    /// サイズ`max_size`で配列を初期化する
    pub fn new(max_size: usize) -> Self {
        let mut fac = vec![1; max_size];
        let mut finv = vec![1; max_size];
        let mut inv = vec![1; max_size];
        for i in 2..max_size {
            fac[i] = fac[i - 1] * i % MOD;
            inv[i] = MOD - (MOD / i) * inv[MOD % i] % MOD;
            finv[i] = finv[i - 1] * inv[i] % MOD;
        }

        Comb { fac, finv }
    }

    /// `nCr`を`MOD`で割った値を求める
    pub fn comb(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        self.fac[n] * (self.finv[r] * self.finv[n - r] % MOD) % MOD
    }

    /// `nPr`を`MOD`で割った値を求める
    pub fn perm(&self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        self.fac[n] * self.finv[n - r] % MOD
    }
}
