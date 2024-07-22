use cp_library_rs::{
    algebraic_structure::monoid::{examples::Add, Monoid},
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
