#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{
        indexed_monoid::Indexed,
        operation::{Add, Max, Min, Mul},
    },
    data_structure::segment_tree::*,
    debug,
    tree::show_binary_tree::ShowBinaryTree,
};

#[test]
fn test_get_point() {
    let segtree = SegmentTree::<Mul<isize>>::from_vec(vec![1, 2, 3, 4, 5]);

    assert_eq!(segtree[0], 1);
    assert_eq!(segtree[3], 4);
}

#[test]
fn test_RSQ() {
    let mut segtree = SegmentTree::<Add<isize>>::new(3);

    // segtree.update(0, 1);
    *segtree.get_mut(0).unwrap() += 1;
    *segtree.get_mut(1).unwrap() += 2;
    *segtree.get_mut(2).unwrap() += 3;
    // [1, 2, 3]

    assert_eq!(segtree.get_range(0..2), 3);
    assert_eq!(segtree.get_range(1..2), 2);
    assert_eq!(segtree.get_range(1..=2), 5);
    assert_eq!(segtree.get_range(..), 6);
}

#[test]
fn test_RMQ() {
    let INF = i32::MAX;
    let mut segtree = SegmentTree::<Min<_>>::new(3);

    assert_eq!(segtree.get_range(..1), INF);
    *segtree.get_mut(0).unwrap() = 5;
    assert_eq!(segtree.get_range(..1), 5);
}

#[test]
fn test_from_slice() {
    let arr = vec![20, 4, 5, 6, 8, 9, 100];
    let mut segtree = SegmentTree::<Max<_>>::from_vec(arr);

    assert_eq!(segtree.get_range(0..), 100);
    assert_eq!(segtree.get_range(2..5), 8);

    segtree.update(0, 200);
    eprintln!("{:?}", segtree);

    assert_eq!(segtree.get_range(..), 200);
    assert_eq!(segtree.get_range(2..5), 8);
}

#[test]
#[should_panic]
fn test_wrong_range() {
    let segtree = SegmentTree::<Add<isize>>::from_vec(vec![0, 1, 2, 3, 4, 5]);

    assert_eq!(segtree.get_range(..), 15);
    assert_eq!(segtree.get_range(..2), 1);
    assert_eq!(segtree.get_range(..6), 15);
    assert_eq!(segtree.get_range(0..), 15);
    assert_eq!(segtree.get_range(..7), 15);
}

#[test]
fn test_debug_print() {
    let arr = vec![20, 4, 5, 6, 8, 9, 100];
    let segtree = arr.into_iter().collect::<SegmentTree<Add<isize>>>();

    let dbg = format!("{:?}", &segtree);
    assert_eq!(&dbg, "SegmentTree { [20, 4, 5, 6, 8, 9, 100] }");
}

#[test]
fn test_index() {
    let mut seg = SegmentTree::<Indexed<Min<isize>>>::new_with_index(5);

    let res = seg.get_range(..);
    assert_eq!(res, (isize::MAX, 0));

    let res = seg.get_range(1..3);
    assert_eq!(res, (isize::MAX, 1));

    seg.get_mut(0).unwrap().0 = 10;

    let res = seg.get_range(..);
    assert_eq!(res, (10, 0));

    let res = seg.get_range(1..3);
    assert_eq!(res, (isize::MAX, 1));

    seg.get_mut(2).unwrap().0 = 5;

    let res = seg.get_range(..);
    assert_eq!(res, (5, 2));

    let res = seg.get_range(1..3);
    assert_eq!(res, (5, 2));
}

#[test]
#[allow(clippy::absurd_extreme_comparisons)]
fn test_binary_search_right() {
    let mut seg: SegmentTree<Add<usize>> = (0..10).collect();

    debug!(seg);

    assert_eq!(seg.max_right(0, |x| x <= 0), (0, 1));
    assert_eq!(seg.max_right(0, |x| x < 6), (3, 3));
    assert_eq!(seg.max_right(0, |x| x <= 6), (6, 4));
    assert_eq!(seg.max_right(0, |x| x < 10), (6, 4));
    assert_eq!(seg.max_right(0, |x| x <= 10), (10, 5));
    assert_eq!(seg.max_right(5, |x| x < 5), (0, 5));
    assert_eq!(seg.max_right(5, |x| x <= 5), (5, 6));
    assert_eq!(seg.max_right(5, |x| x <= 20), (18, 8));
    assert_eq!(seg.max_right(5, |x| x <= 100), (35, 10));
    assert_eq!(seg.max_right(5, |x| x <= 1000), (35, 10));

    *seg.get_mut(4).unwrap() = 100;

    assert_eq!(seg.max_right(0, |x| x <= 0), (0, 1));
    assert_eq!(seg.max_right(0, |x| x < 6), (3, 3));
    assert_eq!(seg.max_right(0, |x| x <= 6), (6, 4));
    assert_eq!(seg.max_right(0, |x| x < 10), (6, 4));
    assert_eq!(seg.max_right(0, |x| x <= 10), (6, 4));
    assert_eq!(seg.max_right(5, |x| x < 5), (0, 5));
    assert_eq!(seg.max_right(5, |x| x <= 5), (5, 6));
    assert_eq!(seg.max_right(5, |x| x <= 20), (18, 8));
    assert_eq!(seg.max_right(5, |x| x <= 100), (35, 10));
    assert_eq!(seg.max_right(5, |x| x <= 1000), (35, 10));

    *seg.get_mut(7).unwrap() = 0;

    assert_eq!(seg.max_right(0, |x| x <= 0), (0, 1));
    assert_eq!(seg.max_right(0, |x| x < 6), (3, 3));
    assert_eq!(seg.max_right(0, |x| x <= 6), (6, 4));
    assert_eq!(seg.max_right(0, |x| x < 10), (6, 4));
    assert_eq!(seg.max_right(0, |x| x <= 10), (6, 4));
    assert_eq!(seg.max_right(5, |x| x < 5), (0, 5));
    assert_eq!(seg.max_right(5, |x| x <= 5), (5, 6));
    assert_eq!(seg.max_right(5, |x| x <= 20), (19, 9));
    assert_eq!(seg.max_right(5, |x| x <= 100), (28, 10));
    assert_eq!(seg.max_right(5, |x| x <= 1000), (28, 10));
}

#[test]
fn test_binary_search_left() {
    let mut seg: SegmentTree<Add<usize>> = (0..10).collect();

    debug!(seg);

    assert_eq!(seg.min_left(10, |x| x < 4), (0, 10));
    assert_eq!(seg.min_left(10, |x| x <= 10), (9, 9));
    assert_eq!(seg.min_left(10, |x| x < 24), (17, 8));
    assert_eq!(seg.min_left(10, |x| x <= 24), (24, 7));
    assert_eq!(seg.min_left(10, |x| x <= 100), (45, 0));
    assert_eq!(seg.min_left(10, |x| x <= 1000), (45, 0));
    assert_eq!(seg.min_left(5, |x| x < 4), (0, 5));
    assert_eq!(seg.min_left(5, |x| x <= 4), (4, 4));
    assert_eq!(seg.min_left(5, |x| x < 10), (9, 2));
    assert_eq!(seg.min_left(5, |x| x <= 10), (10, 0));
    assert_eq!(seg.min_left(5, |x| x <= 100), (10, 0));
    assert_eq!(seg.min_left(5, |x| x <= 1000), (10, 0));

    *seg.get_mut(4).unwrap() = 100;

    assert_eq!(seg.min_left(10, |x| x < 4), (0, 10));
    assert_eq!(seg.min_left(10, |x| x <= 10), (9, 9));
    assert_eq!(seg.min_left(10, |x| x < 24), (17, 8));
    assert_eq!(seg.min_left(10, |x| x <= 24), (24, 7));
    assert_eq!(seg.min_left(10, |x| x <= 100), (35, 5));
    assert_eq!(seg.min_left(10, |x| x <= 1000), (141, 0));
    assert_eq!(seg.min_left(5, |x| x < 4), (0, 5));
    assert_eq!(seg.min_left(5, |x| x <= 4), (0, 5));
    assert_eq!(seg.min_left(5, |x| x < 10), (0, 5));
    assert_eq!(seg.min_left(5, |x| x <= 10), (0, 5));
    assert_eq!(seg.min_left(5, |x| x <= 100), (100, 4));
    assert_eq!(seg.min_left(5, |x| x <= 1000), (106, 0));

    *seg.get_mut(7).unwrap() = 0;

    assert_eq!(seg.min_left(10, |x| x < 4), (0, 10));
    assert_eq!(seg.min_left(10, |x| x <= 10), (9, 9));
    assert_eq!(seg.min_left(10, |x| x < 24), (23, 6));
    assert_eq!(seg.min_left(10, |x| x <= 24), (23, 6));
    assert_eq!(seg.min_left(10, |x| x <= 100), (28, 5));
    assert_eq!(seg.min_left(10, |x| x <= 1000), (134, 0));
    assert_eq!(seg.min_left(5, |x| x < 4), (0, 5));
    assert_eq!(seg.min_left(5, |x| x <= 4), (0, 5));
    assert_eq!(seg.min_left(5, |x| x < 10), (0, 5));
    assert_eq!(seg.min_left(5, |x| x <= 10), (0, 5));
    assert_eq!(seg.min_left(5, |x| x <= 100), (100, 4));
    assert_eq!(seg.min_left(5, |x| x <= 1000), (106, 0));
}

#[test]
fn test_print_as_binary_tree() {
    let mut seg = SegmentTree::<Add<isize>>::from_vec(vec![1, 2, 3, 4, 5]);

    seg.print_as_binary_tree();

    // 変更
    *seg.get_mut(2).unwrap() = 8;
    println!("> update seg[2] <- 8");
    seg.print_as_binary_tree();

    // 変更
    println!("> update seg[3] <- 10");
    *seg.get_mut(3).unwrap() = 10;

    seg.print_as_binary_tree();
}
