use cp_library_rs::mex_set::*;

#[test]
fn test_insert() {
    let mut mex = MexSet::new();
    /* set: {} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert_eq!(mex.insert(3), true);
    /* set: {3} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert_eq!(mex.insert(1), true);
    /* set: {1, 3} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert_eq!(mex.insert(3), false);
    /* set: {1, 3} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert_eq!(mex.insert(100), true);
    /* set: {1, 3, 100} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 101);

    assert_eq!(mex.insert(0), true);
    /* set: {0, 1, 3, 100} */
    assert_eq!(mex.mex(0), 2);
    assert_eq!(mex.mex(100), 101);

    assert_eq!(mex.insert(2), true);
    /* set: {0, 1, 2, 3, 100} */
    assert_eq!(mex.mex(0), 4);
    assert_eq!(mex.mex(100), 101);
}

#[test]
fn test_delete() {
    let mut mex = MexSet::new();

    // 0~5 を追加
    mex.ranges.insert((0, 5));
    /* set: {0, 1, 2, 3, 4, 5} */

    assert_eq!(mex.mex(0), 6);

    assert_eq!(mex.delete(3), true);
    /* set: {0, 1, 2, 4, 5} */
    assert_eq!(mex.mex(0), 3);

    assert_eq!(mex.delete(1), true);
    /* set: {0, 2, 4, 5} */
    assert_eq!(mex.mex(0), 1);

    assert_eq!(mex.delete(3), false);
    /* set: {0, 2, 4, 5} */
    assert_eq!(mex.mex(0), 1);

    assert_eq!(mex.delete(0), true);
    /* set: {2, 4, 5} */
    assert_eq!(mex.mex(0), 0);
}

#[test]
fn test_insert_range() {
    let mut mex = MexSet::new();

    assert_eq!(mex.insert_range(0..=20), true);
    assert_eq!(mex.mex(0), 21);
    eprintln!("{:?}", mex);

    assert_eq!(mex.insert_range(2..10), false);
    assert_eq!(mex.mex(0), 21);

    assert_eq!(mex.insert_range(30..40), true);
    assert_eq!(mex.mex(0), 21);
    eprintln!("{:?}", mex);

    assert_eq!(mex.insert_range(11..=28), true);
    assert_eq!(mex.mex(0), 29);
    eprintln!("{:?}", mex);

    assert_eq!(mex.insert_range(29..=29), true);
    assert_eq!(mex.mex(0), 40);
    eprintln!("{:?}", mex);
}

// #[test]
// fn test_delete_range() {
//     let mut mex = MexSet::new();

//     mex.insert_range(2..=4);
//     mex.insert(8);
//     mex.insert(10);
//     mex.insert_range(20..=40);
//     eprintln!("{:?}", mex);

//     mex.delete_range(..);
//     eprintln!("{:?}", mex);
// }
