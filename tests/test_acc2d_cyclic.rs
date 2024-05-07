use cp_library_rs::acc2d_cyclic::Acc2D;

#[test]
fn test_cyclic_simple() {
    let arr = vec![vec![5, 1, 0], vec![2, 1, 1], vec![0, 2, 8]];

    // 累積和
    let acc = Acc2D::new(&arr);

    // 普通の区間和
    assert_eq!(acc.sum(.., ..), 20);
    assert_eq!(acc.sum(..2, ..2), 9);
    assert_eq!(acc.sum(.., ..2), 11);
    assert_eq!(acc.sum(..2, ..1), 7);

    // トーラス上の区間和
    assert_eq!(acc.sum_cyclic(0, 0, 3, 3), 20);
    assert_eq!(acc.sum_cyclic(0, 0, 30, 30), 2000);
    assert_eq!(acc.sum_cyclic(1, 1, 3, 3), 20);
    assert_eq!(acc.sum_cyclic(2, 2, 2, 2), 13);
    assert_eq!(acc.sum_cyclic(1, 3, 2, 3), 14);
    assert_eq!(acc.sum_cyclic(1, 1, 2, 4), 17);
}
