//! ミラー・ラビン素数判定法

/// 余りをとる累乗
fn powmod(a: u64, b: u64, m: u64) -> u64 {
    let (mut a, mut b, m) = (a as u128, b as u128, m as u128);
    let mut res = 1;
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    res as u64
}

/// ## ミラーラビン素数判定法
/// 参考: <https://zenn.dev/kaki_xxx/articles/40a92b43200215>
pub fn is_prime_MR(N: u64) -> bool {
    if N <= 2 {
        return N == 2;
    }
    if N % 2 == 0 {
        return false;
    }

    let (mut s, mut d) = (0, N - 1);
    while d % 2 == 0 {
        s += 1;
        d >>= 1;
    }

    // n < 2^64 の場合、以下を調べれば十分
    let A = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &a in &A {
        if a % N == 0 {
            break;
        }
        let mut t = 0;
        let mut x = powmod(a, d, N);
        if x != 1 {
            while t < s {
                if x == N - 1 {
                    break;
                }
                x = ((x as u128).pow(2) % (N as u128)) as u64;
                t += 1;
            }
            if t == s {
                return false;
            }
        }
    }
    true
}
