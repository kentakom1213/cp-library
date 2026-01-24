#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid_with_context::MonoidCtx,
    graph::rerooting::{RerootingDP, TreeMonoid},
};

#[test]
fn test_diameter() {
    struct Diam;
    impl MonoidCtx for Diam {
        type Val = usize;
        fn e(&self) -> Self::Val {
            0
        }
        fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.max(right)
        }
    }
    impl TreeMonoid for Diam {
        type T = usize;
        fn put_edge(&self, x: &Self::T, _i: usize) -> Self::Val {
            *x + 1
        }
        fn put_vertex(&self, x: &Self::Val, _v: usize) -> Self::T {
            *x
        }
    }

    let mut dp = RerootingDP::new(6, Diam);

    //       0
    //      / \
    //     1   2
    //        / \
    //       3   4
    //            \
    //             5

    dp.add_edge(0, 1, 0, 0);
    dp.add_edge(0, 2, 1, 1);
    dp.add_edge(2, 3, 2, 2);
    dp.add_edge(2, 4, 3, 3);
    dp.add_edge(4, 5, 4, 4);

    let ans = dp.build(0);

    assert_eq!(ans, vec![3, 4, 2, 3, 3, 4]);
}

#[test]
fn test_diameter_weighted() {
    struct Diam {
        w: Vec<usize>,
    }
    impl MonoidCtx for Diam {
        type Val = usize;
        fn e(&self) -> Self::Val {
            0
        }
        fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val {
            *left.max(right)
        }
    }
    impl TreeMonoid for Diam {
        type T = usize;
        fn put_edge(&self, x: &Self::T, i: usize) -> Self::Val {
            *x + self.w[i]
        }
        fn put_vertex(&self, x: &Self::Val, _v: usize) -> Self::T {
            *x
        }
    }

    let mut dp = RerootingDP::new(
        6,
        Diam {
            w: vec![7, 5, 10, 3, 2],
        },
    );

    //       0
    //   (7)/ \(5)
    //     1   2
    //        / \
    //   (10)/   \(3)
    //      3     4
    //             \(2)
    //              5

    dp.add_edge(0, 1, 0, 0);
    dp.add_edge(0, 2, 1, 1);
    dp.add_edge(2, 3, 2, 2);
    dp.add_edge(2, 4, 3, 3);
    dp.add_edge(4, 5, 4, 4);

    let ans = dp.build(0);

    assert_eq!(ans, vec![15, 22, 12, 22, 15, 17]);
}
