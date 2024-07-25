//! 愚直な2項係数の計算

/// $`\binom{n}{r}`$ の値を愚直に求める
pub fn comb(n: usize, r: usize) -> usize {
    if r == 0 {
        1
    } else if n < r {
        0
    } else {
        n * comb(n - 1, r - 1) / r
    }
}
