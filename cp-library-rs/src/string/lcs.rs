//! 最長共通部分列

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

/// ## LCS
/// 最長共通部分列を得る
/// 計算量：O(NM)
pub fn LCS<T: std::cmp::PartialEq>(A: &[T], B: &[T]) -> usize {
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

    dp[la][lb]
}

/// ## LCS with Vector
/// 最長共通部分列を得る
/// 計算量：O(NM)
pub fn LCS_with_Vec<T: std::cmp::PartialEq + Copy>(A: &[T], B: &[T]) -> Vec<T> {
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

    let mut res: Vec<T> = vec![];
    let (mut cur, mut col) = (0, 0);
    'outer: for i in 0..la {
        for j in col..lb {
            if cur == dp[i][j] && dp[i][j] < dp[i + 1][j + 1] {
                res.push(A[i]);
                cur += 1;
                col = j + 1;
            }
            if cur == dp[la][lb] {
                // LCSの長さに達したら終了
                break 'outer;
            }
        }
    }

    res
}
