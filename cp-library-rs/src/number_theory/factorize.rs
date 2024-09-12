//! 素因数分解

/// 非負整数 $`n`$ を素因数分解し、`(素因数,指数)`のベクタを返す
/// - 計算量 : $`O(\sqrt{n})`$
pub fn factorize_pairs(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut cnt = 0;
        while n % i == 0 {
            n /= i;
            cnt += 1;
        }
        if cnt >= 1 {
            res.push((i, cnt));
        }
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}
