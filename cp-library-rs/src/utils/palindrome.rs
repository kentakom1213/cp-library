//! 回文数に関するユーティリティ

use crate::utils::usize_pow::PowUsize;

/// d 桁の自然数を昇順に出力するイテレータ
/// - 時間計算量: O(10^d) (whole)
pub fn generate_d_digit_number(d: usize) -> impl Iterator<Item = usize> {
    if d == 0 {
        0..1
    } else {
        10.pow(d - 1)..10.pow(d)
    }
}

/// n を 10 進数で表したとき，桁を逆順に並べ替えた数を返す
pub fn inverted_number(mut n: usize) -> usize {
    let mut r = 0;
    while n > 0 {
        r = r * 10 + n % 10;
        n /= 10;
    }
    r
}

/// N 以下の回文数を昇順に出力するイテレータ
/// - 時間計算量: O(√N) (whole)
pub fn generate_palindrome_number(N: usize) -> impl Iterator<Item = usize> {
    let n_length = (N as f32).log10().ceil() as usize;

    (0..=n_length / 2)
        .flat_map(|d| {
            (generate_d_digit_number(d).map(move |k| k * 10.pow(d) + inverted_number(k))).chain(
                generate_d_digit_number(d).flat_map(move |k| {
                    (0..=9).map(move |m| (k * 10 + m) * 10.pow(d) + inverted_number(k))
                }),
            )
        })
        // 最初の0は除外
        .skip(1)
        .take_while(move |k| k <= &N)
}

/// 自然数 `n` を `a` 進数とみなしたとき，回文数であるか判定する．
/// - `a = 0` のとき panic する
/// - `a = 1` のとき `true` を返す．
pub fn is_palindrome(mut n: usize, a: usize) -> bool {
    match a {
        0 => panic!("Base cannot be 0"),
        1 => return true,
        _ => {}
    }
    let mut ps = vec![];
    while n > 0 {
        ps.push(n % a);
        n /= a;
    }
    ps.iter()
        .zip(ps.iter().rev())
        .take(ps.len() / 2)
        .all(|(x, y)| x == y)
}
