//! ## 最長共通部分列

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// 最長共通部分列
pub struct LCS<'a, T> {
    pub A: &'a [T],
    pub B: &'a [T],
    pub la: usize,
    pub lb: usize,
    pub lcs: usize,
    pub dp: Vec<Vec<usize>>,
}

impl<'a, T: PartialEq + Clone> LCS<'a, T> {
    /// 動的計画法により最長共通部分列の長さを求める
    /// - 計算量： $`O(NM)`$
    pub fn build(A: &'a [T], B: &'a [T]) -> Self {
        let (la, lb) = (A.len(), B.len());
        let mut dp = vec![vec![0; lb + 1]; la + 1];

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
            la,
            lb,
            lcs: dp[la][lb],
            dp,
        }
    }

    /// 最長共通部分列を復元する
    /// - 計算量 : $`O(NM)`$
    pub fn reconstruct(&self) -> Vec<T> {
        // 復元する配列
        let mut res: Vec<T> = vec![];
        let (mut cur, mut col) = (0, 0);

        'outer: for i in 0..self.la {
            for j in col..self.lb {
                if cur == self.dp[i][j] && self.dp[i][j] < self.dp[i + 1][j + 1] {
                    res.push(self.A[i].clone());
                    cur += 1;
                    col = j + 1;
                }
                if cur == self.lcs {
                    // LCSの長さに達したら終了
                    break 'outer;
                }
            }
        }

        res
    }
}
