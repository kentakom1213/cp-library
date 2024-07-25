//! あまりを取る累乗

/// あまりをとる累乗
///
/// **戻り値**
/// - `usize` : $`a^b \mod m`$
///
/// **計算量**
/// - $`O(\log b)`$
pub fn powmod(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % m;
        }
        a %= m;
        a = (a * a) % m;
        b >>= 1;
    }
    res
}
