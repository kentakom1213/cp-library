//! 中国剰余定理
//!
//! Garnerのアルゴリズムによる中国剰余定理の解復元

use crate::number_theory::ext_euclid::inv;

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
pub fn garner_algorithm(rems: &[u64], mods: &[u64]) -> u64 {
    let mut m = 1;
    let mut x = (rems[0] % mods[0]) as i64;

    for i in 0..rems.len() {
        let (ri, mi) = (rems[i] as i64, mods[i] as i64);
        let Some(inv_m) = inv(m, mi) else {
            panic!("For all (i,j), gcd(mi, mj) must be 1.")
        };
        let t = ((ri - x) * inv_m).rem_euclid(mi);
        x += t * m;
        m *= mi;
    }
    x as u64
}
