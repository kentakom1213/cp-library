//! 中国剰余定理
//!
//! Garnerのアルゴリズムによる中国剰余定理の解復元

/// 拡張ユークリッド互除法
/// - `ax + by = gcd(a, b)` を満たす `(x, y, gcd(a,b))` を返す
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

/// 拡張ユークリッド互除法によるモジュラ逆元の計算
/// - `ax ≡ 1 (mod m)` を満たす`x`を求める．
/// - `m`が素数である必要はないが，`a`と`m`は互いに素である必要がある．
pub fn inv(a: isize, m: isize) -> Option<isize> {
    let (x, _, d) = ext_gcd(a, m);
    (d == 1).then_some(x.rem_euclid(m))
}

/// 中国剰余定理
///
/// `rems: [r1, r2, ..., rn], mods: [m1, m2, ..., mn]`に対し，
/// - `x ≡ r1 (mod m1)`
/// - `x ≡ r2 (mod m2)`
/// - ...
/// - `x ≡ rn (mod mn)`
///
/// を満たす`x`を求める．
/// - ただし，任意の`(i,j)`に対し`mi`と`mj`は互いに素である必要がある．
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
