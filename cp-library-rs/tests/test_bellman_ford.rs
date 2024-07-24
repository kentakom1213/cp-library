use cp_library_rs::{
    graph::bellman_ford::bellman_ford,
    utils::consts::{IINF, INF},
};

#[test]
fn test_dist() {
    let edges = vec![
        (0, 3, -1000000000),
        (0, 1, 0),
        (1, 2, -1),
        (2, 1, 0),
        (2, 3, 1000000000),
    ];

    let (has_neg, dist, _) = bellman_ford(4, 0, &edges);

    assert_eq!(has_neg, true);
    assert_eq!(dist, vec![0, -IINF, -IINF, -IINF]);
}

#[test]
fn test_prev() {
    let edges = vec![
        (0, 1, -1),
        (0, 2, 2),
        (0, 3, 3),
        (1, 4, 1),
        (3, 5, 5),
        (4, 2, 1),
        (4, 5, -3),
    ];

    let (has_neg, dist, prev) = bellman_ford(6, 0, &edges);

    assert_eq!(has_neg, false);
    assert_eq!(dist, vec![0, -1, 1, 3, 0, -3]);
    assert_eq!(prev, vec![INF, 0, 4, 0, 1, 4]);
}
