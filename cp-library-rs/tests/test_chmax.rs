#![allow(non_snake_case)]

use cp_library_rs::chmax;

#[test]
fn test_chmax() {
    // 2変数
    let mut a = 20;
    let b = 10;
    chmax!(a, b);
    assert_eq!(a, 20);

    // 5変数
    let mut one = 100;
    let two = 30;
    let three = 400;
    let four = 10;
    let five = 5;
    let has_changed = chmax!(one, two, three, four, five,);
    assert!(has_changed);
    assert_eq!(one, 400);
}

#[test]
fn test_chmax_vector() {
    let mut arr = (0..10).collect::<Vec<usize>>();
    println!("{:?}", &arr);

    for i in 0..10 {
        chmax!(arr[0], arr[i]);
    }

    println!("{:?}", &arr);
    assert_eq!(&arr, &vec![9_usize, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
