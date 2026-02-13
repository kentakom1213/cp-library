#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid_with_context::examples::{AddMod, MulMod},
    data_structure::segment_tree_ctx::SegmentTreeCtx,
    tree::show_binary_tree::ShowBinaryTree,
};

#[test]
fn test_get_point() {
    let segtree = SegmentTreeCtx::from_vec(vec![1, 2, 3, 4, 5], AddMod(5));

    assert_eq!(segtree[0], 1);
    assert_eq!(segtree[1], 2);
    assert_eq!(segtree[2], 3);
    assert_eq!(segtree[3], 4);
    assert_eq!(segtree[4], 5);
}

#[test]
fn test_RSQ() {
    let mut segtree = SegmentTreeCtx::new(3, AddMod(4));

    // segtree.update(0, 1);
    *segtree.get_mut(0).unwrap() += 1;
    *segtree.get_mut(1).unwrap() += 2;
    *segtree.get_mut(2).unwrap() += 3;
    // [1, 2, 3]

    assert_eq!(segtree.get_range(0..2), 3);
    assert_eq!(segtree.get_range(1..2), 2);
    assert_eq!(segtree.get_range(1..=2), 1);
    assert_eq!(segtree.get_range(..), 2);
}

#[test]
fn test_debug_print() {
    let arr = vec![20, 4, 5, 6, 8, 9, 100];
    let segtree = SegmentTreeCtx::from_vec(arr, MulMod(101));

    let dbg = format!("{:?}", &segtree);
    assert_eq!(&dbg, "SegmentTreeCtx { [20, 4, 5, 6, 8, 9, 100] }");

    segtree.print_as_binary_tree();
}
