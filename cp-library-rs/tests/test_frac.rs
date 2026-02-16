#![allow(non_snake_case)]

use cp_library_rs::number_theory::frac::*;
use num::{One, Zero};
use num_integer::gcd;

#[test]
fn test_eq() {
    assert_eq!(Frac::new(8_i64, 2), Frac::new(4, 1));
    assert_eq!(Frac::new(4_i64, 4), Frac::new(5, 5));
    assert_eq!(Frac::new(-1_i64, 2), Frac::new(1, -2));
    assert_ne!(Frac::new(3_i64, 2), Frac::new(4, 3));
}

#[test]
fn test_ord() {
    let values = vec![
        Frac::new(2_i64, 3),
        Frac::new(0, 1),
        Frac::new(-1, 2),
        Frac::new(-2, 1),
        Frac::new(-1, 3),
    ];

    let mut sorted = values.clone();
    sorted.sort();

    assert_eq!(
        sorted,
        vec![
            Frac::new(-2_i64, 1),
            Frac::new(-1, 2),
            Frac::new(-1, 3),
            Frac::new(0, 1),
            Frac::new(2, 3),
        ]
    );
}

#[test]
fn test_normalize() {
    let x = Frac::new(6_i64, -8);
    assert_eq!(x.numer(), -3);
    assert_eq!(x.denom(), 4);
    assert!(x.denom() > 0);
    assert_eq!(gcd(x.numer(), x.denom()), 1);

    let y = Frac::new(-6_i64, -8);
    assert_eq!(y.numer(), 3);
    assert_eq!(y.denom(), 4);

    let z = Frac::new(0_i64, -5);
    assert_eq!(z, Frac::new(0, 1));
    assert_eq!(z.denom(), 1);
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
}

#[test]
#[should_panic(expected = "denominator must be non-zero")]
fn test_new_zero_denom_panics() {
    let _ = Frac::new(1_i64, 0);
}
