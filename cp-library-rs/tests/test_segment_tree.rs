use cp_library_rs::{
    algebraic_structure::{
        indexed_monoid::Indexed,
        operation::{Add, Max, Min, Mul},
    },
    data_structure::segment_tree::*,
    debug,
};

#[test]
fn test_get_point() {
    let segtree = SegmentTree::<Mul<isize>>::from(vec![1, 2, 3, 4, 5]);

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
    let mut segtree = SegmentTree::<Max<_>>::from(arr);

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
    let segtree = SegmentTree::<Add<isize>>::from(vec![0, 1, 2, 3, 4, 5]);

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
