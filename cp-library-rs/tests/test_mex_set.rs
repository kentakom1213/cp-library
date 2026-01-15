#![allow(non_snake_case)]

use cp_library_rs::data_structure::mex_set::*;

#[test]
fn test_insert() {
    let mut mex = MexSet::new();
    /* set: {} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert!(mex.insert(3));
    /* set: {3} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert!(mex.insert(1));
    /* set: {1, 3} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert!(!mex.insert(3));
    /* set: {1, 3} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 100);

    assert!(mex.insert(100));
    /* set: {1, 3, 100} */
    assert_eq!(mex.mex(0), 0);
    assert_eq!(mex.mex(100), 101);

    assert!(mex.insert(0));
    /* set: {0, 1, 3, 100} */
    assert_eq!(mex.mex(0), 2);
    assert_eq!(mex.mex(100), 101);

    assert!(mex.insert(2));
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

    assert!(mex.delete(3));
    /* set: {0, 1, 2, 4, 5} */
    assert_eq!(mex.mex(0), 3);

    assert!(mex.delete(1));
    /* set: {0, 2, 4, 5} */
    assert_eq!(mex.mex(0), 1);

    assert!(!mex.delete(3));
    /* set: {0, 2, 4, 5} */
    assert_eq!(mex.mex(0), 1);

    assert!(mex.delete(0));
    /* set: {2, 4, 5} */
    assert_eq!(mex.mex(0), 0);
}

#[test]
fn test_insert_range() {
    let mut mex = MexSet::new();

    assert!(mex.insert_range(0..=20));
    assert_eq!(mex.mex(0), 21);
    eprintln!("{:?}", mex);

    assert!(!mex.insert_range(2..10));
    assert_eq!(mex.mex(0), 21);

    assert!(mex.insert_range(30..40));
    assert_eq!(mex.mex(0), 21);
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(11..=28));
    assert_eq!(mex.mex(0), 29);
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(29..=29));
    assert_eq!(mex.mex(0), 40);
    eprintln!("{:?}", mex);
}

#[test]
fn test_delete_range() {
    let mut mex = MexSet::new();
    eprintln!("{:?}", mex);

    assert!(!mex.delete(200));
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(..));
    eprintln!("{:?}", mex);

    assert!(!mex.insert_range(2..=4));
    eprintln!("{:?}", mex);

    assert!(!mex.insert(8));
    eprintln!("{:?}", mex);

    assert!(!mex.insert(10));
    eprintln!("{:?}", mex);

    assert!(!mex.insert_range(20..=40));
    eprintln!("{:?}", mex);

    assert!(mex.delete_range(..));
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(1000..2000));
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(0..=5000));
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(..2000));
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(8000..));
    eprintln!("{:?}", mex);

    assert!(mex.delete_range(3000..4000));
    eprintln!("{:?}", mex);

    assert!(mex.delete_range(100100..));
    eprintln!("{:?}", mex);

    assert!(mex.insert_range(..));
    eprintln!("{:?}", mex);
}
