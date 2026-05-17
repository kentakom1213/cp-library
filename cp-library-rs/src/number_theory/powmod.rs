//! Modular exponentiation.

/// Returns `a^b mod m`.
///
/// # Complexity
///
/// - $`O(log b)`$
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
