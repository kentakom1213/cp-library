//! Extended Euclidean algorithm.

/// Returns `(x, y, g)` such that `ax + by = g = gcd(a, b)`.
///
/// # Complexity
///
/// - $`O(log min(a, b))`$
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

/// Returns the modular inverse of `a` modulo `m`.
///
/// Returns `None` if `a` and `m` are not coprime.
///
/// # Complexity
///
/// - $`O(log min(a, m))`$
pub fn inv(a: isize, m: isize) -> Option<isize> {
    let (x, _, d) = ext_gcd(a, m);
    (d == 1).then_some(x.rem_euclid(m))
}
