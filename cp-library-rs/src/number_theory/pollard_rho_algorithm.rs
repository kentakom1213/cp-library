//! ポラード・ロー法による素因数分解

use crate::number_theory::miller_rabin_test::is_prime_MR;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ポラード・ロー法を適用し、約数を見つける
pub fn pollard_rho(N: u64) -> u64 {
    if N % 2 == 0 {
        return 2;
    }
    if is_prime_MR(N) {
        return N;
    }
    let tmp = &mut 0;
    let f = |x: u64, tmp: &mut u128| -> u64 { (((x as u128).pow(2) + *tmp) % N as u128) as u64 };
    let mut step = 0;
    loop {
        *tmp += 1;
        step += 1;
        let mut x = step;
        let mut y = f(x, tmp);
        loop {
            let p = gcd(N + y - x, N);
            if p == 0 || p == N {
                break;
            }
            if p != 1 {
                return p;
            }
            x = f(x, tmp);
            y = f(f(y, tmp), tmp);
        }
    }
}

/// ポラード・ロー法により素因数分解を行う
///
/// - 時間計算量: $`O(n^{1/4})`$
pub fn factorize_pollard_rho(N: u64) -> Vec<u64> {
    if N == 1 {
        return vec![];
    }
    let p = pollard_rho(N);
    if p == N {
        return vec![N];
    }
    let mut left = factorize_pollard_rho(p);
    let mut right = factorize_pollard_rho(N / p);
    left.append(&mut right);
    left.sort();
    left
}
