use cp_library_rs::{
    algebraic_structure::monoid::{
        examples::{Add, GCD},
        Monoid,
    },
    utils::consts::{MOD998, NEG1},
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
fn test_gcd() {
    assert_eq!(GCD::op(&GCD::id(), &GCD::id()), GCD::id());
    assert_eq!(GCD::op(&GCD::id(), &MOD998), MOD998);
    assert_eq!(GCD::op(&20, &240), 20);
    assert_eq!(GCD::op(&101, &20021213), 1);
}
