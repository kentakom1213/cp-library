#![allow(non_snake_case)]

use cp_library_rs::graph::lca_doubling::*;

#[test]
fn test_depth_parent_lca() {
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

    assert_eq!(lca.depth(0), 0);
    assert_eq!(lca.depth(1), 1);
    assert_eq!(lca.depth(2), 1);
    assert_eq!(lca.depth(3), 1);
    assert_eq!(lca.depth(4), 2);
    assert_eq!(lca.depth(5), 2);
    assert_eq!(lca.depth(6), 3);
    assert_eq!(lca.depth(7), 3);

    assert_eq!(lca.parent(0), None);
    assert_eq!(lca.parent(1), Some(0));
    assert_eq!(lca.parent(2), Some(0));
    assert_eq!(lca.parent(3), Some(0));
    assert_eq!(lca.parent(4), Some(1));
    assert_eq!(lca.parent(5), Some(1));
    assert_eq!(lca.parent(6), Some(5));
    assert_eq!(lca.parent(7), Some(5));

    assert_eq!(lca.lca(4, 6), 1);
    assert_eq!(lca.lca(4, 7), 1);
    assert_eq!(lca.lca(4, 3), 0);
    assert_eq!(lca.lca(5, 2), 0);
    assert_eq!(lca.lca(5, 7), 5);
    assert_eq!(lca.lca(4, 4), 4);
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

#[test]
fn test_kth_ancestor() {
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

    assert_eq!(lca.kth_ancestor(0, 0), Some(0));
    assert_eq!(lca.kth_ancestor(0, 1), None);

    assert_eq!(lca.kth_ancestor(7, 0), Some(7));
    assert_eq!(lca.kth_ancestor(7, 1), Some(5));
    assert_eq!(lca.kth_ancestor(7, 2), Some(1));
    assert_eq!(lca.kth_ancestor(7, 3), Some(0));
    assert_eq!(lca.kth_ancestor(7, 4), None);

    assert_eq!(lca.kth_ancestor(6, 2), Some(1));

    assert_eq!(lca.kth_ancestor(4, 1), Some(1));
    assert_eq!(lca.kth_ancestor(4, 2), Some(0));
}

#[test]
fn test_kth_on_path() {
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

    assert_eq!(lca.kth_on_path(4, 7, 0), Some(4));
    assert_eq!(lca.kth_on_path(4, 7, 1), Some(1));
    assert_eq!(lca.kth_on_path(4, 7, 2), Some(5));
    assert_eq!(lca.kth_on_path(4, 7, 3), Some(7));
    assert_eq!(lca.kth_on_path(4, 7, 4), None);

    assert_eq!(lca.kth_on_path(7, 4, 0), Some(7));
    assert_eq!(lca.kth_on_path(7, 4, 1), Some(5));
    assert_eq!(lca.kth_on_path(7, 4, 2), Some(1));
    assert_eq!(lca.kth_on_path(7, 4, 3), Some(4));
    assert_eq!(lca.kth_on_path(7, 4, 4), None);

    assert_eq!(lca.kth_on_path(6, 3, 0), Some(6));
    assert_eq!(lca.kth_on_path(6, 3, 1), Some(5));
    assert_eq!(lca.kth_on_path(6, 3, 2), Some(1));
    assert_eq!(lca.kth_on_path(6, 3, 3), Some(0));
    assert_eq!(lca.kth_on_path(6, 3, 4), Some(3));
    assert_eq!(lca.kth_on_path(6, 3, 5), None);

    assert_eq!(lca.kth_on_path(3, 6, 0), Some(3));
    assert_eq!(lca.kth_on_path(3, 6, 1), Some(0));
    assert_eq!(lca.kth_on_path(3, 6, 2), Some(1));
    assert_eq!(lca.kth_on_path(3, 6, 3), Some(5));
    assert_eq!(lca.kth_on_path(3, 6, 4), Some(6));
    assert_eq!(lca.kth_on_path(3, 6, 5), None);
}
