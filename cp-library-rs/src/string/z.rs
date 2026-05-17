//! Z-algorithm.

/// Computes the Z-array of `s`.
///
/// The returned array `z` satisfies:
///
/// ```text
/// z[i] = lcp(s, s[i..])
/// ```
///
/// where `lcp` is the length of the longest common prefix.
///
/// # Complexity
///
/// - $`O(|s|)`$
///
/// ## References
///
/// - <https://qiita.com/Pro_ktmr/items/16904c9570aa0953bf05>
pub fn z_algorithm<T: PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    if n == 0 {
        return vec![];
    }

    let mut z = vec![0; n];
    z[0] = n;

    let mut i = 1;
    let mut j = 0;

    while i < n {
        while i + j < n && s[j] == s[i + j] {
            j += 1;
        }
        z[i] = j;

        if j == 0 {
            i += 1;
            continue;
        }

        let mut k = 1;
        while k < j && k + z[k] < j {
            z[i + k] = z[k];
            k += 1;
        }
        i += k;
        j -= k;
    }

    z
}
