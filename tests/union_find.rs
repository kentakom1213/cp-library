use cp_library_rs::union_find::*;

#[test]
fn test_unionfind() {
    let mut uf = UnionFind::new(8);
    /*
     * 0 1 2 3 4 5 6 7
     */

    uf.unite(1, 2);
    uf.unite(3, 2);
    /*
     * 0 1-2-3 4 5 6 7
     */

    assert_eq!(uf.group_count, 6);

    assert!(uf.issame(1, 3) == true);
    assert!(uf.issame(1, 4) == false);
    assert_eq!(uf.size(1), 3);

    uf.unite(2, 4);
    /*
     * 0 1-2-3-4 5 6 7
     */

    assert_eq!(uf.group_count, 5);
    assert!(uf.issame(4, 1) == true);
    assert_eq!(uf.size(4), 4);

    uf.unite(4, 2);
    uf.unite(0, 0);
    /*
     * 0 1-2-3-4 5 6 7
     */

    assert_eq!(uf.group_count, 5);
    assert!(uf.issame(0, 0) == true);

    uf.unite(0, 7);
    /*
     * 0 1-2-3-4 5 6 7
     * └─────────────┘
     */

    assert_eq!(uf.group_count, 4);
    assert!(uf.issame(0, 7) == true);

    uf.unite(5, 6);
    /*
     * 0 1-2-3-4 5-6 7
     * └─────────────┘
     */

    assert_eq!(uf.group_count, 3);
    assert!(uf.issame(5, 6) == true);
    assert!(uf.issame(5, 7) == false);

    uf.unite(4, 5);
    uf.unite(6, 7);
    /*
     * 0-1-2-3-4-5-6-7
     */

    assert_eq!(uf.group_count, 1);

    uf.unite(0, 7);
    /*
     * 0-1-2-3-4-5-6-7
     */

    assert_eq!(uf.group_count, 1);
}
