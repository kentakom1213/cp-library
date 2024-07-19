use cp_library_rs::graph::dijkstra::*;

#[test]
fn test_dijkstra() {
    let graph = vec![
        vec![(1, 2), (3, 4), (4, 5)],
        vec![(0, 2), (1, 6), (3, 3)],
        vec![(1, 6), (3, 2), (5, 4)],
        vec![(0, 4), (1, 3), (2, 2), (4, 2)],
        vec![(0, 5), (3, 2), (5, 6)],
        vec![(2, 4), (4, 6)],
    ];

    let (_, dist) = dijkstra(&graph, 0);
    assert_eq!(dist, vec![0, 2, 6, 4, 5, 10]);
}
