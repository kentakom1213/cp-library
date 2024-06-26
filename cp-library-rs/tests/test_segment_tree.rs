use cp_library_rs::{
    monoid::{examples::*, Monoid},
    segment_tree::*,
};

#[test]
fn test_get_point() {
    let segtree = SegmentTree::<Mul>::from(&vec![1, 2, 3, 4, 5]);

    assert_eq!(segtree[0], 1);
    assert_eq!(segtree[3], 4);
}

#[test]
fn test_RSQ() {
    let mut segtree = SegmentTree::<Add>::new(3);

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
    let INF = Min::id();
    let mut segtree = SegmentTree::<Min>::new(3);

    assert_eq!(segtree.get_range(..1), INF);
    *segtree.get_mut(0).unwrap() = 5;
    assert_eq!(segtree.get_range(..1), 5);
}

#[test]
fn test_from_slice() {
    let INF = Max::id();
    let arr = vec![20, 4, 5, 6, 8, 9, 100];
    let mut segtree = SegmentTree::<Max>::from(&arr);

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
    let segtree = SegmentTree::<Add>::from(&vec![0, 1, 2, 3, 4, 5]);

    assert_eq!(segtree.get_range(..), 15);
    assert_eq!(segtree.get_range(..2), 1);
    assert_eq!(segtree.get_range(..6), 15);
    assert_eq!(segtree.get_range(0..), 15);
    assert_eq!(segtree.get_range(..7), 15);
}

#[test]
fn test_debug_print() {
    let arr = vec![20, 4, 5, 6, 8, 9, 100];
    let segtree = SegmentTree::<Max>::from(&arr);

    let dbg = format!("{:?}", &segtree);
    assert_eq!(&dbg, "SegmentTree { [20, 4, 5, 6, 8, 9, 100] }");
}
