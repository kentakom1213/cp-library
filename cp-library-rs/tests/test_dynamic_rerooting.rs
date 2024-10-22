use cp_library_rs::{
    debug,
    graph::dynamic_rerooting::{Rerooting, TreeMonoid},
};

#[test]
fn test_aggregate() {
    struct TM {
        weights: Vec<usize>,
    }
    impl TreeMonoid for TM {
        type T = usize;
        fn id(&self) -> Self::T {
            0
        }
        fn merge(&self, x: &Self::T, y: &Self::T) -> Self::T {
            *x.max(y)
        }
        fn put_edge(&self, x: &Self::T, i: usize) -> Self::T {
            *x + self.weights[i]
        }
    }

    // 木の構築
    let n = 8;
    let edge = vec![
        (0, 1, 2),
        (1, 2, 3),
        (1, 3, 1),
        (1, 4, 4),
        (4, 5, 1),
        (4, 6, 3),
        (6, 7, 2),
    ];

    let monoid = Box::new(TM {
        weights: edge.iter().map(|x| x.2).collect(),
    });

    let mut rerooting = Rerooting::new(n, monoid);

    // 辺の追加
    for &(u, v, _) in &edge {
        rerooting.add_edge2(u, v);
    }

    debug!(rerooting.edge_id);

    // 集約
    rerooting.build();

    debug!(rerooting.dp);
    debug!(rerooting.ans);

    assert_eq!(rerooting.ans, vec![11, 9, 12, 10, 7, 8, 10, 12])
}
