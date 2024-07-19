use cp_library_rs::data_structure::acc2d::acc2D;

#[test]
fn test_acc2_isize() {
    let arr: Vec<Vec<isize>> = vec![vec![1, -2, 3], vec![4, -5, 6], vec![7, -8, 9]];

    let acc = acc2D(&arr);

    assert_eq!(acc(0, 1, 0, 1), 1);
    assert_eq!(acc(0, 2, 0, 2), -2);
    assert_eq!(acc(0, 3, 1, 2), -15);
    assert_eq!(acc(1, 2, 0, 3), 5);
    assert_eq!(acc(0, 3, 0, 3), 15);
}

#[test]
fn test_acc2D_usize() {
    let arr: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

    let acc = acc2D(&arr);

    assert_eq!(acc(0, 1, 0, 1), 1);
    assert_eq!(acc(0, 2, 0, 2), 12);
    assert_eq!(acc(0, 3, 1, 2), 15);
    assert_eq!(acc(1, 2, 0, 3), 15);
    assert_eq!(acc(0, 3, 0, 3), 45);
}

#[test]
fn test_acc_2D_overflow() {
    let arr: Vec<Vec<usize>> = vec![vec![100, 10, 1], vec![20, 1, 3], vec![1, 5, 1]];

    let acc = acc2D(&arr);

    for t in 0..3 {
        for b in t + 1..=3 {
            for l in 0..3 {
                for r in l + 1..=3 {
                    // 愚直な足し算
                    let mut tmp = 0;
                    for i in t..b {
                        for j in l..r {
                            tmp += arr[i][j];
                        }
                    }
                    assert_eq!(tmp, acc(t, b, l, r));
                }
            }
        }
    }
}
