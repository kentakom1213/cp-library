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

#[test]
fn test_cyclic_simple_rect() {
    let arr = vec![vec![5, 1, 0, 3], vec![2, 1, 1, 4]];

    // 累積和
    let acc = Acc2D::new(&arr);

    // 普通の区間和
    assert_eq!(acc.sum(.., ..), 17);
    assert_eq!(acc.sum(..2, ..2), 9);
    assert_eq!(acc.sum(.., ..2), 9);
    assert_eq!(acc.sum(..2, ..1), 7);

    // トーラス上の区間和
    let sum_naive = |t, l, h, w| -> isize {
        let mut sum = 0;
        let (H, W) = (arr.len(), arr[0].len());
        let (t, l) = (t % H, l % W);
        for i in t..t + h {
            for j in l..l + w {
                let row: &Vec<isize> = &arr[i % H];
                sum += &row[j % W];
            }
        }
        sum
    };

    assert_eq!(acc.sum_cyclic(0, 0, 3, 3), sum_naive(0, 0, 3, 3));
    assert_eq!(acc.sum_cyclic(0, 0, 30, 30), sum_naive(0, 0, 30, 30));
    assert_eq!(acc.sum_cyclic(1, 1, 3, 3), sum_naive(1, 1, 3, 3));
    assert_eq!(acc.sum_cyclic(2, 2, 2, 2), sum_naive(2, 2, 2, 2));
    assert_eq!(acc.sum_cyclic(1, 3, 2, 3), sum_naive(1, 3, 2, 3));
    assert_eq!(acc.sum_cyclic(1, 1, 2, 4), sum_naive(1, 1, 2, 4));
}
