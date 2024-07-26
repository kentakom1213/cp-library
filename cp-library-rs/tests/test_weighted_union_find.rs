use cp_library_rs::{
    algebraic_structure::monoid::examples::Add, data_structure::weighted_union_find::*,
};

#[test]
fn test_weighted_unionfind() {
    // 問題例:
    // https://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=7971476#3

    let mut uf = WeightedUnionFind::<Add<isize>>::new(8);

    uf.unite(1, 3, 6);
    uf.unite(4, 6, 4);
    uf.unite(2, 5, 5);

    assert_eq!(uf.diff(1, 6), None);

    uf.unite(1, 2, 4);

    assert_eq!(uf.diff(3, 5), Some(3));
    assert_eq!(uf.diff(5, 3), Some(-3));
    assert_eq!(uf.diff(6, 2), None);

    uf.unite(2, 3, 2);
    uf.unite(3, 6, 6);

    assert_eq!(uf.diff(4, 5), Some(1));
    assert_eq!(uf.diff(5, 4), Some(-1));
    assert_eq!(uf.diff(6, 5), Some(-3));
    assert_eq!(uf.diff(5, 6), Some(3));
}
