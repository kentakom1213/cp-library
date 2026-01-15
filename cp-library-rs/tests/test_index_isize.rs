#![allow(non_snake_case)]

use cp_library_rs::utils::index_isize::IndexIsize;

#[test]
fn test_vector() {
    let mut arr = vec![0, 1, 2, 3, 4];

    assert_eq!(*arr.iget(5), 0);
    assert_eq!(*arr.iget(4), 4);
    assert_eq!(*arr.iget(3), 3);
    assert_eq!(*arr.iget(2), 2);
    assert_eq!(*arr.iget(1), 1);
    assert_eq!(*arr.iget(0), 0);
    assert_eq!(*arr.iget(-1), 4);
    assert_eq!(*arr.iget(-2), 3);
    assert_eq!(*arr.iget(-3), 2);
    assert_eq!(*arr.iget(-4), 1);
    assert_eq!(*arr.iget(-5), 0);
    assert_eq!(*arr.iget(-6), 4);
    assert_eq!(*arr.iget(-7), 3);
    assert_eq!(*arr.iget(-8), 2);

    *arr.iget_mut(-1) = 100;
    arr.push(5);

    assert_eq!(*arr.iget(5), 5);
    assert_eq!(*arr.iget(4), 100);
    assert_eq!(*arr.iget(3), 3);
    assert_eq!(*arr.iget(2), 2);
    assert_eq!(*arr.iget(1), 1);
    assert_eq!(*arr.iget(0), 0);
    assert_eq!(*arr.iget(-1), 5);
    assert_eq!(*arr.iget(-2), 100);
    assert_eq!(*arr.iget(-3), 3);
    assert_eq!(*arr.iget(-4), 2);
    assert_eq!(*arr.iget(-5), 1);
    assert_eq!(*arr.iget(-6), 0);
    assert_eq!(*arr.iget(-7), 5);
    assert_eq!(*arr.iget(-8), 100);
}

#[test]
fn test_array() {
    let mut arr = [0, 1, 2, 3, 4];

    assert_eq!(*arr.iget(5), 0);
    assert_eq!(*arr.iget(4), 4);
    assert_eq!(*arr.iget(3), 3);
    assert_eq!(*arr.iget(2), 2);
    assert_eq!(*arr.iget(1), 1);
    assert_eq!(*arr.iget(0), 0);
    assert_eq!(*arr.iget(-1), 4);
    assert_eq!(*arr.iget(-2), 3);
    assert_eq!(*arr.iget(-3), 2);
    assert_eq!(*arr.iget(-4), 1);
    assert_eq!(*arr.iget(-5), 0);
    assert_eq!(*arr.iget(-6), 4);
    assert_eq!(*arr.iget(-7), 3);
    assert_eq!(*arr.iget(-8), 2);

    *arr.iget_mut(-1) = 100;

    assert_eq!(*arr.iget(5), 0);
    assert_eq!(*arr.iget(4), 100);
    assert_eq!(*arr.iget(3), 3);
    assert_eq!(*arr.iget(2), 2);
    assert_eq!(*arr.iget(1), 1);
    assert_eq!(*arr.iget(0), 0);
    assert_eq!(*arr.iget(-1), 100);
    assert_eq!(*arr.iget(-2), 3);
    assert_eq!(*arr.iget(-3), 2);
    assert_eq!(*arr.iget(-4), 1);
    assert_eq!(*arr.iget(-5), 0);
    assert_eq!(*arr.iget(-6), 100);
    assert_eq!(*arr.iget(-7), 3);
    assert_eq!(*arr.iget(-8), 2);
}

#[test]
fn test_slice() {
    let arr = (0..100).collect::<Vec<_>>();

    assert_eq!(**&arr[..].iget(-1), 99);
    assert_eq!(**&arr[10..20].iget(-1), 19);
    assert_eq!(**&arr[10..20].iget(-10), 10);
    assert_eq!(**&arr[10..20].iget(-20), 10);
    assert_eq!(**&arr[51..63].iget(-1), 62);
    assert_eq!(**&arr[51..63].iget(-12), 51);
    assert_eq!(**&arr[51..63].iget(-13), 62);
}
