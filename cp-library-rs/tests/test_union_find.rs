#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::{Add, MinMax},
    data_structure::union_find::*,
    debug,
};

#[test]
fn test_unionfind() {
    let mut uf = UnionFind::new(8);
    debug!(uf);
    /*
     * 0 1 2 3 4 5 6 7
     */

    uf.unite(1, 2);
    uf.unite(3, 2);
    debug!(uf);
    /*
     * 0 1-2-3 4 5 6 7
     */

    assert_eq!(uf.group_count(), 6);

    assert!(uf.is_same(1, 3) == true);
    assert!(uf.is_same(1, 4) == false);
    assert_eq!(uf.get_size(1), 3);

    uf.unite(2, 4);
    debug!(uf);
    /*
     * 0 1-2-3-4 5 6 7
     */

    assert_eq!(uf.group_count(), 5);
    assert!(uf.is_same(4, 1) == true);
    assert_eq!(uf.get_size(4), 4);

    uf.unite(4, 2);
    uf.unite(0, 0);
    debug!(uf);
    /*
     * 0 1-2-3-4 5 6 7
     */

    assert_eq!(uf.group_count(), 5);
    assert!(uf.is_same(0, 0) == true);

    uf.unite(0, 7);
    debug!(uf);
    /*
     * 0 1-2-3-4 5 6 7
     * └─────────────┘
     */

    assert_eq!(uf.group_count(), 4);
    assert!(uf.is_same(0, 7) == true);

    uf.unite(5, 6);
    debug!(uf);
    /*
     * 0 1-2-3-4 5-6 7
     * └─────────────┘
     */

    assert_eq!(uf.group_count(), 3);
    assert!(uf.is_same(5, 6) == true);
    assert!(uf.is_same(5, 7) == false);

    uf.unite(4, 5);
    uf.unite(6, 7);
    debug!(uf);
    /*
     * 0-1-2-3-4-5-6-7
     */

    assert_eq!(uf.group_count(), 1);

    uf.unite(0, 7);
    debug!(uf);
    /*
     * 0-1-2-3-4-5-6-7
     */

    assert_eq!(uf.group_count(), 1);
}

#[test]
fn test_union_find_monoid() {
    let mut uf = (0..6).collect::<UnionFindMonoid<Add<usize>>>();
    debug!(uf);

    // 0 1-2-3 4 5
    uf.unite(1, 2);
    uf.unite(3, 2);
    debug!(uf);

    assert_eq!(uf.value(0), &0);
    assert_eq!(uf.value(1), &6);
    assert_eq!(uf.value(2), &6);
    assert_eq!(uf.value(3), &6);
    assert_eq!(uf.value(4), &4);
    assert_eq!(uf.value(5), &5);

    // 0 1-2-3 4-5
    uf.unite(4, 5);
    debug!(uf);

    assert_eq!(uf.value(0), &0);
    assert_eq!(uf.value(1), &6);
    assert_eq!(uf.value(2), &6);
    assert_eq!(uf.value(3), &6);
    assert_eq!(uf.value(4), &9);
    assert_eq!(uf.value(5), &9);

    // 0 1-2-3-4-5
    uf.unite(3, 5);
    debug!(uf);

    assert_eq!(uf.value(0), &0);
    assert_eq!(uf.value(1), &15);
    assert_eq!(uf.value(2), &15);
    assert_eq!(uf.value(3), &15);
    assert_eq!(uf.value(4), &15);
    assert_eq!(uf.value(5), &15);

    // 0 1-2-3-4-5
    uf.unite(1, 5);
    debug!(uf);

    assert_eq!(uf.value(0), &0);
    assert_eq!(uf.value(1), &15);
    assert_eq!(uf.value(2), &15);
    assert_eq!(uf.value(3), &15);
    assert_eq!(uf.value(4), &15);
    assert_eq!(uf.value(5), &15);

    // 0-1-2-3-4-5
    uf.unite(0, 1);
    debug!(uf);

    assert_eq!(uf.value(0), &15);
    assert_eq!(uf.value(1), &15);
    assert_eq!(uf.value(2), &15);
    assert_eq!(uf.value(3), &15);
    assert_eq!(uf.value(4), &15);
    assert_eq!(uf.value(5), &15);
}

#[test]
fn test_union_find_range() {
    let mut uf: UnionFindMonoid<MinMax<usize>> = (0..6).map(|i| (i, i)).collect();

    assert_eq!(uf.value(0), &(0, 0));
    assert_eq!(uf.value(1), &(1, 1));
    assert_eq!(uf.value(2), &(2, 2));
    assert_eq!(uf.value(3), &(3, 3));
    assert_eq!(uf.value(4), &(4, 4));
    assert_eq!(uf.value(5), &(5, 5));

    // 0 1-2 3 4 5
    uf.unite(1, 2);

    assert_eq!(uf.value(0), &(0, 0));
    assert_eq!(uf.value(1), &(1, 2));
    assert_eq!(uf.value(2), &(1, 2));
    assert_eq!(uf.value(3), &(3, 3));
    assert_eq!(uf.value(4), &(4, 4));
    assert_eq!(uf.value(5), &(5, 5));

    // 0 1-2 3 4-5
    uf.unite(4, 5);

    assert_eq!(uf.value(0), &(0, 0));
    assert_eq!(uf.value(1), &(1, 2));
    assert_eq!(uf.value(2), &(1, 2));
    assert_eq!(uf.value(3), &(3, 3));
    assert_eq!(uf.value(4), &(4, 5));
    assert_eq!(uf.value(5), &(4, 5));

    // 0-1-2 3 4-5
    uf.unite(0, 2);

    assert_eq!(uf.value(0), &(0, 2));
    assert_eq!(uf.value(1), &(0, 2));
    assert_eq!(uf.value(2), &(0, 2));
    assert_eq!(uf.value(3), &(3, 3));
    assert_eq!(uf.value(4), &(4, 5));
    assert_eq!(uf.value(5), &(4, 5));

    // 0-1-2 3-4-5
    uf.unite(3, 5);

    assert_eq!(uf.value(0), &(0, 2));
    assert_eq!(uf.value(1), &(0, 2));
    assert_eq!(uf.value(2), &(0, 2));
    assert_eq!(uf.value(3), &(3, 5));
    assert_eq!(uf.value(4), &(3, 5));
    assert_eq!(uf.value(5), &(3, 5));

    // 0-1-2-3-4-5
    uf.unite(0, 5);

    assert_eq!(uf.value(0), &(0, 5));
    assert_eq!(uf.value(1), &(0, 5));
    assert_eq!(uf.value(2), &(0, 5));
    assert_eq!(uf.value(3), &(0, 5));
    assert_eq!(uf.value(4), &(0, 5));
    assert_eq!(uf.value(5), &(0, 5));
}
