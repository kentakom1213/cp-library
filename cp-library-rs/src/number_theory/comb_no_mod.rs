//! 再帰関数による愚直な組合せ計算

pub fn comb(n: usize, r: usize) -> usize {
    if r == 0 {
        1
    } else if n < r {
        0
    } else {
        n * comb(n - 1, r - 1) / r
    }
}
