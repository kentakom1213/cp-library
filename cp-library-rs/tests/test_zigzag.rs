use cp_library_rs::zigzag::zigzag;

#[test]
fn test_zigzag_3x3() {
    let arr = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let actual: Vec<usize> = zigzag(3, 3).map(|(i, j)| arr[i][j]).collect();
    let expected = vec![1, 2, 3, 6, 5, 4, 7, 8, 9];

    assert_eq!(&actual, &expected);
}

#[test]
fn test_zigzag_4x5() {
    let arr = vec![
        vec![4, 23, 6, 7, 10],
        vec![9, 12, 2, 3, 6],
        vec![5, 5, 3, 1, 1],
        vec![90, 3, 39, 0, 41],
    ];

    let actual: Vec<i8> = zigzag(4, 5).map(|(i, j)| arr[i][j]).collect();
    let expected = vec![
        4, 23, 6, 7, 10, 6, 3, 2, 12, 9, 5, 5, 3, 1, 1, 41, 0, 39, 3, 90,
    ];

    assert_eq!(&actual, &expected);
}
