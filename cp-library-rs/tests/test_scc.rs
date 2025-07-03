use cp_library_rs::graph::scc::*;

#[test]
fn test_scc() {
    let n = 6;
    let edges = [(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];
    let mut scc = SCC::new(n);
    edges.iter().for_each(|&(u, v)| scc.add_edge(u, v));

    // 強連結成分分解
    scc.decompose();

    assert_eq!(scc.group_count, Some(4));
    assert_eq!(scc.belongs_to.as_ref(), Some(&vec![3, 1, 2, 3, 1, 0]));
    assert_eq!(
        scc.DAG.as_ref(),
        Some(&vec![vec![2], vec![2], vec![], vec![]])
    );
    assert_eq!(
        scc.components.as_ref(),
        Some(&vec![vec![5], vec![1, 4], vec![2], vec![0, 3]])
    );
}
