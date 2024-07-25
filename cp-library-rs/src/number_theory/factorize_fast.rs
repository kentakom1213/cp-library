//! ## 前計算ありの素因数分解
//!
//! $`N`$ までの数の素因数分解を
//! - 前計算 : $`O(N \log\log N)`$
//! - クエリ : $`O(\log N)`$
//!
//! で行う。

/// 前計算ありの素因数分解
pub struct FactorTable {
    pub n: usize,
    pub sieve: Vec<usize>,
}

impl FactorTable {
    /// 前計算を行う
    /// - $`O(N \log\log N)`$ で篩を作成
    pub fn new(n: usize) -> Self {
        let mut facs = FactorTable {
            n,
            sieve: vec![1; n + 1],
        };
        for i in 2..=n {
            for j in 1.. {
                if i * j > n {
                    break;
                }
                if facs.sieve[i * j] == 1 {
                    facs.sieve[i * j] = i;
                }
            }
        }
        facs
    }

    /// 素因数分解を行い，素因数のベクタを返す
    ///
    /// **戻り値**
    /// - 素因数のリスト
    ///
    /// **計算量**
    /// - $`O(\log x)`$
    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        assert!(1 <= x && x <= self.n);
        let mut factors = vec![];
        while x > 1 {
            factors.push(self.sieve[x]);
            x /= self.sieve[x];
        }
        factors
    }

    /// 素因数分解を行い，(素因数, 指数) のベクタを返す
    ///
    /// **戻り値**
    /// - (素因数, 指数) のベクタ
    ///
    /// **計算量**
    /// - $`O(\log x)`$
    pub fn factorize_pairs(&self, mut x: usize) -> Vec<(usize, usize)> {
        assert!(1 <= x && x <= self.n);
        let mut pairs: Vec<(usize, usize)> = vec![];
        while x > 1 {
            let p = self.sieve[x];
            if !pairs.is_empty() && pairs.last().unwrap().0 == p {
                pairs.last_mut().unwrap().1 += 1
            } else {
                pairs.push((p, 1));
            }
            x /= self.sieve[x];
        }
        pairs
    }
}
