//! 前計算ありの高速素因数分解

/// # 前計算ありの高速素因数分解
/// `N`までの数の素因数分解を
/// - 前計算: `O(NloglogN)`
/// - クエリ: `O(logN)`
/// で行う。
pub struct FactorTable {
    n: usize,
    sieve: Vec<usize>,
}

impl FactorTable {
    /// 前計算を行う
    /// - `O(NloglogN)`で篩を作成
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

    /// 素因数分解を`O(logn)`で行う
    /// ### 戻り値
    /// - `Vec<usize>`: 素因数のリスト
    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        assert!(1 <= x && x <= self.n);
        let mut factors = vec![];
        while x > 1 {
            factors.push(self.sieve[x]);
            x /= self.sieve[x];
        }
        factors
    }

    /// 素因数分解を`O(logn)`で行う
    /// ### 戻り値
    /// - `Vec<(usize, usize)>`: (素因数, その個数)
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
