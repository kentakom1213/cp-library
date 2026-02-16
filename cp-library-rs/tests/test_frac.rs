#![allow(non_snake_case)]

use cp_library_rs::number_theory::frac::*;
use num::{One, Zero};

#[test]
fn test_eq() {
    let values = [
        Frac(0, 1),
        Frac(8, 2),
        Frac(4, 1),
        Frac(4, 4),
        Frac(5, 5),
        Frac(3, 2),
        Frac(1, 0),
        Frac(2, 0),
    ];

    let eq_matrix = [
        vec![1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 1, 1],
    ];

    for i in 0..8 {
        for j in 0..8 {
            assert_eq!((values[i] == values[j]) as u8, eq_matrix[i][j]);
        }
    }
}

#[test]
fn test_ord() {
    let values = vec![
        Frac(0, 1),
        Frac(1, 8),
        Frac(2, 8),
        Frac(4, 4),
        Frac(5, 5),
        Frac(3, 2),
        Frac(8, 2),
        Frac(4, 1),
        Frac(1, 0),
        Frac(2, 0),
    ];

    let mut sorted = values.clone();
    sorted.sort();

    assert_eq!(values, sorted);
}

#[test]
fn test_arithmetic() {
    let a = Frac::new(1_i64, 2);
    let b = Frac::new(1_i64, 3);

    assert_eq!(a + b, Frac::new(5, 6));
    assert_eq!(a - b, Frac::new(1, 6));
    assert_eq!(a * b, Frac::new(1, 6));
    assert_eq!(a / b, Frac::new(3, 2));
    assert_eq!(-a, Frac::new(-1, 2));
}

#[test]
fn test_arithmetic_assign() {
    let mut x = Frac::new(3_i64, 4);
    x += Frac::new(1, 4);
    assert_eq!(x, Frac::new(1, 1));

    x -= Frac::new(1, 2);
    assert_eq!(x, Frac::new(1, 2));

    x *= Frac::new(4, 3);
    assert_eq!(x, Frac::new(2, 3));

    x /= Frac::new(2, 5);
    assert_eq!(x, Frac::new(5, 3));
}

#[test]
fn test_from_and_scalar_arithmetic() {
    let x = Frac::from(3_i64);
    assert_eq!(x, Frac::new(3, 1));

    let a = Frac::new(3_i64, 4);
    assert_eq!(a + 2, Frac::new(11, 4));
    assert_eq!(a - 2, Frac::new(-5, 4));
    assert_eq!(a * 2, Frac::new(3, 2));
    assert_eq!(a / 2, Frac::new(3, 8));
}

#[test]
fn test_scalar_arithmetic_assign() {
    let mut x = Frac::new(1_i64, 2);
    x += 1;
    assert_eq!(x, Frac::new(3, 2));

    x -= 2;
    assert_eq!(x, Frac::new(-1, 2));

    x *= 6;
    assert_eq!(x, Frac::new(-3, 1));

    x /= 3;
    assert_eq!(x, Frac::new(-1, 1));
}

#[test]
#[should_panic(expected = "division by zero scalar")]
fn test_scalar_div_by_zero_panics() {
    let _ = Frac::new(1_i64, 2) / 0_i64;
}

#[test]
#[should_panic(expected = "division by zero scalar")]
fn test_scalar_div_assign_by_zero_panics() {
    let mut x = Frac::new(3_i64, 4);
    x /= 0_i64;
}

#[test]
fn test_zero_one_trait() {
    assert!(Frac::<i64>::zero().is_zero());
    assert!(!Frac::<i64>::zero().is_one());
    assert!(Frac::<i64>::one().is_one());
    assert!(!Frac::<i64>::one().is_zero());

    // 非正規値でも panic せず false を返すこと
    assert!(!Frac(1_i64, 0).is_zero());
    assert!(!Frac(1_i64, 0).is_one());
    assert!(!Frac(0_i64, 0).is_zero());
    assert!(!Frac(0_i64, 0).is_one());
}
