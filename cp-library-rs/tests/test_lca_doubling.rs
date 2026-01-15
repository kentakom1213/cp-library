#![allow(non_snake_case)]

use cp_library_rs::graph::lca_doubling::*;

#[test]
fn test_lca() {
    let tree = vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![],
        vec![],
        vec![],
        vec![6, 7],
        vec![],
        vec![],
    ];
    /*
     *       (root)
     *         0
     *       / | \
     *      1  2  3
     *     / \
     *    4   5
     *       / \
     *      6   7
     */

    let lca = LCA::new(&tree, 0);

    assert_eq!(lca.get_lca(4, 6), 1);
    assert_eq!(lca.get_lca(4, 7), 1);
    assert_eq!(lca.get_lca(4, 3), 0);
    assert_eq!(lca.get_lca(5, 2), 0);
    assert_eq!(lca.get_lca(5, 7), 5);
    assert_eq!(lca.get_lca(4, 4), 4);
}

#[test]
fn test_dist() {
    let tree = vec![
        vec![1, 2, 3],
        vec![4, 5],
        vec![],
        vec![],
        vec![],
        vec![6, 7],
        vec![],
        vec![],
    ];
    /*
     *       (root)
     *         0
     *       / | \
     *      1  2  3
     *     / \
     *    4   5
     *       / \
     *      6   7
     */

    let lca = LCA::new(&tree, 0);

    // 根からの距離
    assert_eq!(lca.dist(0, 7), 3);
    assert_eq!(lca.dist(0, 3), 1);
    assert_eq!(lca.dist(0, 5), 2);
    assert_eq!(lca.dist(0, 0), 0);

    // 根以外の頂点同士の距離
    assert_eq!(lca.dist(1, 2), 2);
    assert_eq!(lca.dist(3, 7), 4);
    assert_eq!(lca.dist(4, 1), 1);
    assert_eq!(lca.dist(2, 5), 3);
    assert_eq!(lca.dist(7, 7), 0);
    assert_eq!(lca.dist(3, 3), 0);
}
