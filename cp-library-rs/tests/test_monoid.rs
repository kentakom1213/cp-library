#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{
        monoid::Monoid,
        operation::{Add, GCD},
    },
    utils::consts::NEG1,
};

#[test]
fn test_add() {
    assert_eq!(Add::<usize>::op(&5, &15), 20);
    assert_eq!(Add::<isize>::op(&5, &15), 20);
    assert_eq!(Add::<f64>::op(&0.5, &1.5), 2.0);

    // オーバーフロー
    assert_eq!(Add::<usize>::op(&NEG1, &15), 14);
}

#[test]
#[allow(clippy::upper_case_acronyms)]
fn test_gcd() {
    type GCDI = GCD<isize>;
    type GCDU = GCD<usize>;

    // isize
    assert_eq!(GCDI::op(&GCDI::id(), &GCDI::id()), GCDI::id());
    assert_eq!(GCDI::op(&GCDI::id(), &998244353), 998244353);
    assert_eq!(GCDI::op(&20, &240), 20);
    assert_eq!(GCDI::op(&101, &20021213), 1);

    // usize
    assert_eq!(GCDU::op(&GCDU::id(), &GCDU::id()), GCDU::id());
    assert_eq!(GCDU::op(&GCDU::id(), &998244353), 998244353);
    assert_eq!(GCDU::op(&20, &240), 20);
    assert_eq!(GCDU::op(&101, &20021213), 1);
}
