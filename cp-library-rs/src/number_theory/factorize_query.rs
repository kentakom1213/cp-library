//! ## 前計算ありの素因数分解
//!
//! $`N`$ までの数の素因数分解を
//! - 前計算: $`O(N \log\log N)`$ 時間
//! - クエリ: $`O(\log N)`$ 時間
//!
//! で行う。

/// 前計算ありの素因数分解
pub struct FactorTable {
    pub n: usize,
    pub sieve: Vec<usize>,
}

impl FactorTable {
    /// 前計算を行う
    /// - $`O(N \log\log N)`$ 時間で篩を作成
    pub fn new(n: usize) -> Self {
        let mut sieve = vec![0; n + 1];
        if n >= 1 {
            sieve[1] = 1;
        }
        for i in 2..=n {
            if sieve[i] == 0 {
                // i は素数
                sieve[i] = i;
                // i*i から始める（溢れ注意）
                if i <= n / i {
                    let mut j = i * i;
                    while j <= n {
                        if sieve[j] == 0 {
                            sieve[j] = i;
                        }
                        j += i;
                    }
                }
            }
        }
        Self { n, sieve }
    }

    /// 素因数分解を行い，素因数のベクタを返す
    ///
    /// **戻り値**
    /// - 素因数のリスト
    ///
    /// - 時間計算量: $`O(\log x)`$
    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        assert!(1 <= x && x <= self.n);
        let mut factors = vec![];
        while x > 1 {
            let p = self.sieve[x];
            factors.push(p);
            x /= p;
        }
        factors
    }

    /// 素因数分解を行い，(素因数, 指数) のベクタを返す
    ///
    /// **戻り値**
    /// - (素因数, 指数) のベクタ
    ///
    /// - 時間計算量: $`O(\log x)`$
    pub fn factorize_pairs(&self, mut x: usize) -> Vec<(usize, usize)> {
        assert!(1 <= x && x <= self.n);
        let mut pairs: Vec<(usize, usize)> = vec![];
        while x > 1 {
            let p = self.sieve[x];
            let mut e = 0;
            while x % p == 0 {
                x /= p;
                e += 1;
            }
            pairs.push((p, e));
        }
        pairs
    }
}
