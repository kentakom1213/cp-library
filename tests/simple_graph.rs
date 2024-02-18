use cp_library_rs::simple_graph::*;

#[test]
fn test_decompose() {
    let mut graph = SimpleGraph::new(5);
    assert_eq!(graph.component_size, None);
    // 0 -- 1 -- 2 | 3 | 4
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.decompose();
    assert_eq!(graph.component_size, Some(3));
    assert_eq!(&graph.components, &vec![0, 0, 0, 1, 2]);
    // 4 -- 0 -- 1 -- 2 | 3
    graph.add_edge(4, 0);
    graph.decompose();
    assert_eq!(graph.component_size, Some(2));
    assert_eq!(&graph.components, &vec![0, 0, 0, 1, 0]);
}

#[test]
fn test_bipartite() {
    let mut graph = SimpleGraph::new(7);
    // 0 -- 1
    // |    |
    // 2 -- 3 -- 4
    // 5 -- 6
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);
    graph.add_edge(5, 6);
    let bigraph = graph.bipartite().unwrap();
    assert_eq!(&bigraph, &vec![1, -1, -1, 1, -1, 2, -2]);
}
