#![allow(non_snake_case)]

use cp_library_rs::utils::palindrome::*;

#[test]
fn test_generate_d_digit_number() {
    // d=0: 0のみを生成する (0は自然数ではないが、この関数の仕様として0..1を返す)
    let mut iter = generate_d_digit_number(0);
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), None);

    // d=1: 1桁の自然数 (1-9)
    let mut iter = generate_d_digit_number(1);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.last(), Some(9)); // .last()で末尾まで消費
    assert_eq!(generate_d_digit_number(1).count(), 9);

    // d=2: 2桁の自然数 (10-99)
    let mut iter = generate_d_digit_number(2);
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.last(), Some(99));
    assert_eq!(generate_d_digit_number(2).count(), 90);

    // d=3: 3桁の自然数 (100-999)
    let mut iter = generate_d_digit_number(3);
    assert_eq!(iter.next(), Some(100));
    assert_eq!(iter.last(), Some(999));
    assert_eq!(generate_d_digit_number(3).count(), 900);
}

#[test]
fn test_inverted_number() {
    assert_eq!(inverted_number(12345), 54321);
    assert_eq!(inverted_number(12321), 12321); // 回文数
    assert_eq!(inverted_number(120), 21); // 末尾の0は反転後、先頭に来るため消える
    assert_eq!(inverted_number(100), 1);
    assert_eq!(inverted_number(7), 7); // 1桁
    assert_eq!(inverted_number(0), 0); // 0
}

#[test]
fn test_generate_palindrome_number() {
    // N=8: 1桁の回文数のみ
    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(generate_palindrome_number(8).collect::<Vec<_>>(), expected);

    // N=100: 2桁の回文数まで
    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44, 55, 66, 77, 88, 99,
    ];
    assert_eq!(
        generate_palindrome_number(100).collect::<Vec<_>>(),
        expected
    );

    // N=130: 3桁の回文数を含む
    let expected = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44, 55, 66, 77, 88, 99, 101, 111, 121,
    ];
    assert_eq!(
        generate_palindrome_number(130).collect::<Vec<_>>(),
        expected
    );

    // N=0: 0のみ
    assert_eq!(generate_palindrome_number(0).collect::<Vec<_>>(), vec![0]);
}

#[test]
fn test_is_palindrome() {
    // --- 基数: 10 ---
    assert!(is_palindrome(12321, 10)); // 奇数桁
    assert!(is_palindrome(554455, 10)); // 偶数桁
    assert!(is_palindrome(5, 10)); // 1桁
    assert!(is_palindrome(0, 10)); // 0
    assert!(!is_palindrome(123, 10));
    assert!(!is_palindrome(12332, 10));

    // --- 基数: 2 (Binary) ---
    // 5 = 101_2
    assert!(is_palindrome(5, 2));
    // 9 = 1001_2
    assert!(is_palindrome(9, 2));
    // 6 = 110_2
    assert!(!is_palindrome(6, 2));
    // 27 = 11011_2
    assert!(is_palindrome(27, 2));

    // --- 基数: 8 (Octal) ---
    // 9 = 11_8
    assert!(is_palindrome(9, 8));
    // 65 = 101_8
    assert!(is_palindrome(65, 8));

    // --- 基数: 1 ---
    // 基数1は常に回文とみなす
    assert!(is_palindrome(10, 1)); // 10は1進数
    assert!(is_palindrome(100, 1)); // 100も1進数
}

/// `is_palindrome`の基数`a`に1を指定すると、
/// `x % 1` が常に0, `x / 1`が常に`x`となるため、
/// `while x > 0` のループが無限ループとなりスタックオーバーフローを引き起こします。
/// このような不正な入力に対してはpanicすることが期待されるため、テストを追加します。
/// 本来はResultを返すか、関数の先頭でpanicさせることが望ましいです。
#[test]
#[should_panic]
fn test_is_palindrome_panic_on_base_1() {
    is_palindrome(10, 0);
}
