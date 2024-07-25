//! 区間篩

/// 区間 $`[l,r)`$ の素数を列挙する
/// - 計算量 : $`O((\sqrt{r} + (r - l)) \log \log r)`$
pub fn segmented_sieve(l: usize, r: usize) -> Vec<usize> {
    let l = l.max(2);
    let r = r.max(l);
    let MAX = (r as f64).sqrt() as usize + 10;
    let mut divisors = vec![true; MAX];
    (divisors[0], divisors[1]) = (false, false);
    let mut sieve = vec![true; r - l];

    for p in 2..MAX {
        if !divisors[p] {
            continue;
        }
        let mut i = 2;
        while p * i < MAX {
            divisors[p * i] = false;
            i += 1;
        }
        let mut k = (l + p - 1) / p * p;
        while k < r {
            if k > p {
                if l <= k && k < r {
                    sieve[k - l] = false;
                }
            }
            k += p;
        }
    }

    sieve
        .iter()
        .zip(l..)
        .filter_map(|(&is_prime, x)| is_prime.then_some(x))
        .collect()
}
