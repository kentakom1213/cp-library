//! ## 最長共通部分列

use crate::chmax;

/// 最長共通部分列
pub struct LCS<'a, T> {
    pub A: &'a [T],
    pub B: &'a [T],
    pub N: usize,
    pub M: usize,
    pub lcs: usize,
    pub dp: Vec<Vec<usize>>,
}

impl<'a, T: PartialEq> LCS<'a, T> {
    /// 動的計画法により最長共通部分列の長さを求める
    /// - 時間計算量: $`O(NM)`$
    pub fn build(A: &'a [T], B: &'a [T]) -> Self {
        let (N, M) = (A.len(), B.len());
        let mut dp = vec![vec![0; M + 1]; N + 1];

        for (i, a) in A.iter().enumerate() {
            for (j, b) in B.iter().enumerate() {
                if a == b {
                    chmax!(dp[i + 1][j + 1], dp[i][j] + 1);
                }
                chmax!(dp[i + 1][j + 1], dp[i + 1][j]);
                chmax!(dp[i + 1][j + 1], dp[i][j + 1]);
            }
        }

        Self {
            A,
            B,
            N,
            M,
            lcs: dp[N][M],
            dp,
        }
    }

    /// 最長共通部分列を復元する
    /// - 時間計算量:  $`O(N + M)`$
    pub fn reconstruct(&self) -> Vec<&T> {
        let mut res = vec![];
        let mut i = self.N;
        let mut j = self.M;

        while i > 0 && j > 0 {
            if self.A[i - 1] == self.B[j - 1] {
                res.push(&self.A[i - 1]);
                i -= 1;
                j -= 1;
            } else if self.dp[i - 1][j] > self.dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }

        res.reverse();
        res
    }
}
