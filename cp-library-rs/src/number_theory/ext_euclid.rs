//! 拡張ユークリッド互除法

/// 拡張ユークリッド互除法により，
/// $`ax + by = \gcd(a, b)`$ を満たす $`(x, y, \gcd(a,b))`$
/// を求める．
///
/// **戻り値**
/// - `(x, y, gcd(a, b))`
pub fn ext_gcd(a: isize, b: isize) -> (isize, isize, isize) {
    if b == 0 {
        return (1, 0, a);
    }
    let (q, r) = (a / b, a % b);
    let (xx, yy, d) = ext_gcd(b, r);
    let x = yy;
    let y = xx - q * yy;
    (x, y, d)
}

/// 拡張ユークリッド互除法によりモジュラ逆元を計算する．
/// - $`ax \equiv 1 \mod m`$ を満たす $`x`$ を求める．
/// - $`a`$ と $`m`$ は互いに素である必要がある
///
/// **戻り値**
/// - `a`と`m`が互いに素 → Some($`a^{-1} \mod m`$)
/// - `a`と`m`が互いに素でない → None
pub fn inv(a: isize, m: isize) -> Option<isize> {
    let (x, _, d) = ext_gcd(a, m);
    (d == 1).then_some(x.rem_euclid(m))
}
