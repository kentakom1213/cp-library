#![allow(non_snake_case)]

use cp_library_rs::data_structure::segment_tree_mutable::*;

#[test]
fn test_get_point() {
    let segtree = SegmentTreeMut::build(&[1, 2, 3, 4, 5], 0, |a, b| a + b);

    assert_eq!(segtree[0], 1);
    assert_eq!(segtree[3], 4);
}

#[test]
fn test_RSQ() {
    let mut segtree = SegmentTreeMut::new(3, 0, |a, b| a + b);

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
    const INF: usize = (1 << 31) - 1;
    let mut segtree = SegmentTreeMut::new(3, INF, |a, b| *a.min(b));

    assert_eq!(segtree.get_range(..1), (1 << 31) - 1);
    *segtree.get_mut(0).unwrap() = 5;
    assert_eq!(segtree.get_range(..1), 5);
}

#[test]
fn test_from_slice() {
    const MINF: isize = -(1 << 31) + 1;
    let arr = vec![20, 4, 5, 6, 8, 9, 100];
    let mut segtree = SegmentTreeMut::build(&arr, MINF, |a, b| *a.max(b));

    assert_eq!(segtree.get_range(0..), 100);
    assert_eq!(segtree.get_range(2..5), 8);

    segtree.update(0, 200);

    assert_eq!(segtree.get_range(..), 200);
    assert_eq!(segtree.get_range(2..5), 8);
    assert_eq!(segtree.get_range(5..10), 100);
    assert_eq!(segtree.get_range(10..20), MINF);
}

#[test]
fn test_debug_print() {
    const MINF: isize = -(1 << 31) + 1;
    let arr = vec![20, 4, 5, 6, 8, 9, 100];
    let segtree = SegmentTreeMut::build(&arr, MINF, |a, b| *a.max(b));

    let dbg = format!("{:?}", &segtree);
    assert_eq!(&dbg, "SegmentTreeMut { [20, 4, 5, 6, 8, 9, 100] }");
}
