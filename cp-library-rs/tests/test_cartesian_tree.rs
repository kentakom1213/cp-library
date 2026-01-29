use cp_library_rs::data_structure::cartesian_tree::CartesianTree;

fn inorder_from(
    root: Option<usize>,
    left: &[Option<usize>],
    right: &[Option<usize>],
    out: &mut Vec<usize>,
) {
    if let Some(v) = root {
        inorder_from(left[v], left, right, out);
        out.push(v);
        inorder_from(right[v], left, right, out);
    }
}

fn assert_tree_invariants<T: Ord>(arr: &[T], ct: &CartesianTree<T>) {
    let n = arr.len();
    assert_eq!(ct.n, n);

    // 根は親なし
    assert!(ct.parent[ct.root].is_none());

    // 親子の整合性 + max-heap 性質（親の値 >= 子の値）
    for v in 0..n {
        if let Some(l) = ct.left[v] {
            assert_eq!(ct.parent[l], Some(v));
            assert!(arr[v] >= arr[l]);
        }
        if let Some(r) = ct.right[v] {
            assert_eq!(ct.parent[r], Some(v));
            assert!(arr[v] >= arr[r]);
        }
    }

    // inorder が 0..n-1
    let mut ord = vec![];
    inorder_from(Some(ct.root), &ct.left, &ct.right, &mut ord);
    assert_eq!(ord, (0..n).collect::<Vec<_>>());
}

#[test]
fn test_small_handmade() {
    // n=1
    let arr = vec![10_i32];
    let ct = CartesianTree::build_max(&arr);
    assert_eq!(ct.root, 0);
    assert_tree_invariants(&arr, &ct);

    // strictly increasing -> root is last
    let arr = vec![1_i32, 2, 3, 4, 5];
    let ct = CartesianTree::build_max(&arr);
    assert_eq!(ct.root, 4);
    assert_tree_invariants(&arr, &ct);

    // strictly decreasing -> root is first
    let arr = vec![5_i32, 4, 3, 2, 1];
    let ct = CartesianTree::build_max(&arr);
    assert_eq!(ct.root, 0);
    assert_tree_invariants(&arr, &ct);

    // duplicates（同値は右側優先）: 最大値が複数なら右端が root
    let arr = vec![2_i32, 5, 5, 1];
    let ct = CartesianTree::build_max(&arr);
    assert_eq!(ct.root, 2); // 5 が 1,2 にあり右側(2)が root
    assert_tree_invariants(&arr, &ct);

    // 全部同じ：右端が root，左に鎖状になる
    let arr = vec![7_i32, 7, 7, 7];
    let ct = CartesianTree::build_max(&arr);
    assert_eq!(ct.root, 3);
    assert_tree_invariants(&arr, &ct);
    assert_eq!(ct.left[3], Some(2));
    assert_eq!(ct.left[2], Some(1));
    assert_eq!(ct.left[1], Some(0));
    assert_eq!(ct.right[0], None);
}
