#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::extmonoid::examples::*, data_structure::lazy_segment_tree::*,
    utils::show_binary_tree::ShowBinaryTree,
};

#[test]
fn test_print_as_binary_tree() {
    let mut seg = LazySegmentTree::<AddSum<i32>>::from_vec(vec![(0, 1); 5]);

    seg.print_as_binary_tree();

    // 更新
    seg.apply(..3, 3);
    println!("> add 0..3, 3");
    seg.print_as_binary_tree();

    // 更新
    seg.apply(3.., -4);
    println!("> add 3.., -4");
    seg.print_as_binary_tree();

    // 更新
    seg.apply(2..=2, 100);
    println!("> add 2..=2, 100");
    seg.print_as_binary_tree();

    // 更新
    seg.apply(.., -200);
    println!("> add .., -200");
    seg.print_as_binary_tree();
}

#[test]
fn test_RSQ_and_RAQ_hand() {
    let mut seg = LazySegmentTree::<AddSum<isize>>::from_vec(vec![(0, 1); 10]);
    seg.print_as_binary_tree();
    // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    assert_eq!(seg.get(..), (0, 10));
    assert_eq!(seg.get(..5), (0, 5));
    assert_eq!(seg.get(5..), (0, 5));
    assert_eq!(seg.get(3..8), (0, 5));

    seg.apply(0..4, 2);
    seg.print_as_binary_tree();
    // [2, 2, 2, 2, 0, 0, 0, 0, 0, 0]

    assert_eq!(seg.get(..), (8, 10));
    assert_eq!(seg.get(..5), (8, 5));
    assert_eq!(seg.get(5..), (0, 5));
    assert_eq!(seg.get(3..8), (2, 5));

    seg.apply(4.., 5);
    seg.print_as_binary_tree();
    // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

    assert_eq!(seg.get(..), (38, 10));
    assert_eq!(seg.get(..5), (13, 5));
    assert_eq!(seg.get(5..), (25, 5));
    assert_eq!(seg.get(3..8), (22, 5));

    seg.apply(2..=5, -3);
    seg.print_as_binary_tree();
    // [2, 2, -1, -1, 2, 2, 5, 5, 5, 5]

    assert_eq!(seg.get(..), (26, 10));
    assert_eq!(seg.get(..5), (4, 5));
    assert_eq!(seg.get(5..), (22, 5));
    assert_eq!(seg.get(3..8), (13, 5));

    seg.apply(8..10, -10);
    seg.print_as_binary_tree();
    // [2, 2, -1, -1, 2, 2, 5, 5, -5, -5]

    assert_eq!(seg.get(..), (6, 10));
    assert_eq!(seg.get(..5), (4, 5));
    assert_eq!(seg.get(5..), (2, 5));
    assert_eq!(seg.get(3..8), (13, 5));
}

#[test]
fn test_RMQ_and_RUQ_hand() {
    const INF: isize = isize::MAX;
    let mut seg = LazySegmentTree::<UpdateMin<isize>>::new(10);
    // [INF, INF, INF, INF, INF, INF, INF, INF, INF, INF]

    assert_eq!(seg.get(..), INF);
    assert_eq!(seg.get(..5), INF);
    assert_eq!(seg.get(5..), INF);
    assert_eq!(seg.get(3..8), INF);

    seg.apply(0..4, 2);
    // [2, 2, 2, 2, INF, INF, INF, INF, INF, INF]

    assert_eq!(seg.get(..), 2);
    assert_eq!(seg.get(..5), 2);
    assert_eq!(seg.get(5..), INF);
    assert_eq!(seg.get(3..8), 2);

    seg.apply(4.., 5);
    // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

    assert_eq!(seg.get(..), 2);
    assert_eq!(seg.get(..5), 2);
    assert_eq!(seg.get(5..), 5);
    assert_eq!(seg.get(3..8), 2);

    seg.apply(2..=5, -3);
    // [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

    assert_eq!(seg.get(..), -3);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -3);
    assert_eq!(seg.get(3..8), -3);

    seg.apply(8..10, -10);
    // [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

    assert_eq!(seg.get(..), -10);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -10);
    assert_eq!(seg.get(3..8), -3);
}

/// テストケース: <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I&lang=ja>
#[test]
fn test_RSQ_and_RUQ() {
    let mut seg = LazySegmentTree::<UpdateSum<i8>>::from_vec(vec![(0, 1); 8]);

    seg.apply(1..=3, Some(1));
    seg.apply(2..=4, Some(-2));

    assert_eq!(seg.get(..=5), (-5, 6));
    assert_eq!(seg.get(..=1), (1, 2));

    seg.apply(3..=5, Some(3));

    assert_eq!(seg.get(3..=4), (6, 2));
    assert_eq!(seg.get(..=5), (8, 6));
}

#[test]
fn test_from() {
    let arr = vec![5, 2, -3, -1, -9, -2, 5, 0, 0, 5];

    let mut seg = LazySegmentTree::<UpdateMin<i32>>::from_vec(arr);
    // [5, 2, -3, -1, -9, -2, 5, 0, 0, 5]

    assert_eq!(seg.get(..), -9);
    assert_eq!(seg.get(..5), -9);
    assert_eq!(seg.get(5..), -2);
    assert_eq!(seg.get(3..8), -9);

    seg.apply(..4, 2);
    // [2, 2, 2, 2, -9, -2, 5, 0, 0, 5]

    assert_eq!(seg.get(..), -9);
    assert_eq!(seg.get(..5), -9);
    assert_eq!(seg.get(5..), -2);
    assert_eq!(seg.get(3..8), -9);

    seg.apply(4.., 5);
    // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

    assert_eq!(seg.get(..), 2);
    assert_eq!(seg.get(..5), 2);
    assert_eq!(seg.get(5..), 5);
    assert_eq!(seg.get(3..8), 2);

    seg.apply(2..=5, -3);
    // [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

    assert_eq!(seg.get(..), -3);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -3);
    assert_eq!(seg.get(3..8), -3);

    seg.apply(8.., -10);
    // [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

    assert_eq!(seg.get(..), -10);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -10);
    assert_eq!(seg.get(3..8), -3);
}

#[test]
#[should_panic]
fn get_wrong_range() {
    let mut seg = LazySegmentTree::<AddMin<usize>>::from_vec(vec![0, 1, 2, 3, 4, 5]);

    seg.get(..7);
}

#[test]
#[should_panic]
fn set_wrong_range() {
    let mut seg = LazySegmentTree::<AddMin<usize>>::from_vec(vec![0, 1, 2, 3, 4, 5]);

    seg.apply(..7, 0);
}

#[test]
fn test_lazy_segment_tree_binary_search() {
    let mut seg = LazySegmentTree::<AddSum<usize>>::from_vec(vec![(0, 1); 6]);

    seg.apply(0..3, 2);
    seg.apply(2..5, 1);

    // [2, 2, 3, 1, 1, 0]

    let (sum0, x0) = seg.max_right(0, |x| x.0 <= 8);
    assert_eq!(sum0, (8, 4));
    assert_eq!(x0, 4);

    let (sum1, x1) = seg.max_right(1, |x| x.0 <= 6);
    assert_eq!(sum1, (6, 3));
    assert_eq!(x1, 4);

    let (sum2, x2) = seg.min_left(5, |x| x.0 <= 6);
    assert_eq!(sum2, (5, 3));
    assert_eq!(x2, 2);

    let (sum3, x3) = seg.max_right(6, |x| x.0 <= 100);
    assert_eq!(sum3, (0, 0));
    assert_eq!(x3, 6);

    let (sum4, x4) = seg.min_left(0, |x| x.0 <= 100);
    assert_eq!(sum4, (0, 0));
    assert_eq!(x4, 0);

    let (sum5, x5) = seg.min_left(6, |x| x.0 <= 100);
    assert_eq!(sum5, (9, 6));
    assert_eq!(x5, 0);
}
