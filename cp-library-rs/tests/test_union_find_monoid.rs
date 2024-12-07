use cp_library_rs::{
    algebraic_structure::operation::{Add, MinMax},
    data_structure::union_find_monoid::UnionFindMonoid,
    debug,
};

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
