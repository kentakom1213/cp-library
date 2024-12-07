use cp_library_rs::{data_structure::union_find::*, debug};

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
