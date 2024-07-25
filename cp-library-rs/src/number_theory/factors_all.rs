//! 高速な約数列挙

/// 約数列挙を行う
/// - `1 ~ N`までの数の約数を高速に列挙する
/// - 計算量 : $`O(N \log\log N)`$
pub fn factors_all(n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![]; n + 1];
    for i in 1..=n {
        for j in 1.. {
            if i * j > n {
                break;
            }
            res[i * j].push(i);
        }
    }
    res
}
