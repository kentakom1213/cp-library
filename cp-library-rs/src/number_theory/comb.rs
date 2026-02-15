//! 階乗を前計算する（Modint構造体に依存）

use std::ops::{Add, Mul, Neg, Sub};

use crate::number_theory::modint::{M107, M998, MOD107, MOD998};

use num_traits::{One, Zero};

/// 確率の値となりうる数
pub trait NumProbability<Rhs = Self, Output = Self>:
    Clone
    + Copy
    + From<usize>
    + Neg<Output = Output>
    + Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Mul<usize, Output = Output>
    + Zero
    + One
    + PartialEq
{
    const MOD: usize;
}
impl NumProbability for M107 {
    const MOD: usize = MOD107 as usize;
}
impl NumProbability for M998 {
    const MOD: usize = MOD998 as usize;
}

/// 二項係数を高速に求める
/// - 前計算:  $`O(N)`$ 時間
/// - クエリ:  $`O(1)`$ 時間
pub struct Comb<N: NumProbability> {
    fac: Vec<N>,
    finv: Vec<N>,
}

impl<N: NumProbability> Comb<N> {
    /// サイズ`max_size`で配列を初期化する
    pub fn new(max_size: usize) -> Self {
        let mut fac = vec![N::one(); max_size];
        let mut finv = vec![N::one(); max_size];
        let mut inv = vec![N::one(); max_size];
        for i in 2..max_size {
            fac[i] = fac[i - 1] * i;
            inv[i] = -N::from(N::MOD / i) * inv[N::MOD % i];
            finv[i] = finv[i - 1] * inv[i];
        }
        Comb { fac, finv }
    }

    /// n 個から r 個選ぶ組合せ
    ///
    /// - 時間計算量: $`O(1)`$
    pub fn comb(&self, n: usize, r: usize) -> N {
        if n < r {
            return 0.into();
        }
        self.fac[n] * self.finv[r] * self.finv[n - r]
    }

    /// n 個から r 個を選び並べる順列
    ///
    /// - 時間計算量: $`O(1)`$
    pub fn perm(&self, n: usize, r: usize) -> N {
        if n < r {
            return 0.into();
        }
        self.fac[n] * self.finv[n - r]
    }

    /// 重複組合せ
    ///
    /// - balls 個の区別しない玉を boxes 個の区別する箱に入れる組合せ
    /// - 時間計算量: $`O(1)`$
    pub fn comb_with_rep(&self, balls: usize, boxes: usize) -> N {
        self.comb(balls + boxes - 1, balls)
    }
}
