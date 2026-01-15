#![allow(non_snake_case)]

use cp_library_rs::{
    debug,
    graph::{
        dynamic_rerooting::Rerooting as RerootingDyn, rerooting::examples::Diameter,
        rerooting::Rerooting,
    },
};

#[test]
fn test_aggregate() {
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

    let mut rerooting_dyn =
        RerootingDyn::new(n, 0, |a, b| *a.max(b), |a, i| a + edge[i].2, |a, _| *a);
    let mut rerooting = Rerooting::<Diameter>::new(n);

    // 辺の追加
    for &(u, v, w) in &edge {
        rerooting_dyn.add_edge2(u, v);
        rerooting.add_edge2(u, v, w);
    }

    debug!(rerooting_dyn.edge_id);

    // 集約
    rerooting_dyn.build();
    rerooting.build();

    debug!(rerooting.dp);
    debug!(rerooting.ans);

    debug!(rerooting_dyn.dp);
    debug!(rerooting_dyn.ans);

    assert_eq!(rerooting_dyn.ans, vec![11, 9, 12, 10, 7, 8, 10, 12])
}
