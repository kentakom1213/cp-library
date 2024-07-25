//! 中国剰余定理
//!
//! Garnerのアルゴリズムによる中国剰余定理の解復元

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

/// 中国剰余定理による整数の復元
///
/// 中国剰余定理により，
///
/// $`\mathrm{rems} = [r_1, r_2, \ldots, r_n], \mathrm{mods} = [m_1, m_2, \ldots, m_n]`$ に対し，
/// - $`x \equiv r_1 \mod m_1`$
/// - $`x \equiv r_2 \mod m_2`$
/// - $`\vdots`$
/// - $`x \equiv r_n \mod m_n`$
///
/// を満たす $`x`$ を求める．
///
/// ただし，任意の $`(i,j)`$ に対し $`m_i`$ と $`m_j`$ は互いに素である必要がある．
pub fn garner_algorithm(rems: &[usize], mods: &[usize]) -> usize {
    let mut m = 1;
    let mut x = (rems[0] % mods[0]) as isize;

    for i in 0..rems.len() {
        let (ri, mi) = (rems[i] as isize, mods[i] as isize);
        let Some(inv_m) = inv(m, mi) else {
            panic!("For all (i,j), gcd(mi, mj) must be 1.")
        };
        let t = ((ri - x) * inv_m).rem_euclid(mi);
        x += t * m;
        m *= mi;
    }
    x as usize
}
