//! ポラード・ロー法による素因数分解

use crate::miller_rabin_test::is_prime_MR;

/// `a`,`b`の最大公約数を求める
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ## pollard_rho
/// ポラード・ロー法を適用し、約数を見つける
pub fn pollard_rho(N: usize) -> usize {
    if N % 2 == 0 {
        return 2;
    }
    if is_prime_MR(N) {
        return N;
    }
    let f = |x: usize| -> usize { (((x as u128).pow(2) + 1) % N as u128) as usize };
    let mut step = 0;
    loop {
        step += 1;
        let mut x = step;
        let mut y = f(x);
        loop {
            let p = gcd(N + y - x, N);
            if p == 0 || p == N {
                break;
            }
            if p != 1 {
                return p;
            }
            x = f(x);
            y = f(f(y));
        }
    }
}

/// ## factorize
/// ポラード・ロー法による高速素因数分解
/// `O(n^(1/4))`
pub fn factorize(N: usize) -> Vec<usize> {
    if N == 1 {
        return vec![];
    }
    let p = pollard_rho(N);
    if p == N {
        return vec![N];
    }
    let mut left = factorize(p);
    let mut right = factorize(N / p);
    left.append(&mut right);
    left.sort();
    left
}
