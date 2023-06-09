#![allow(dead_code)]

type Graph = Vec<Vec<usize>>;

/// ## SCC (強連結成分分解)
/// - Strongly Conneected Components
pub struct SCC {
    pub V: usize,
    pub E: usize,
    pub G: Graph,
    rG: Graph,
    pub group_count: Option<usize>,
    pub components: Option<Vec<usize>>,
    pub DAG: Option<Graph>,
}

impl SCC {
    const INF: usize = std::usize::MAX;

    pub fn new(V: usize) -> Self {
        Self {
            V,
            E: 0,
            G: vec![vec![]; V],
            rG: vec![vec![]; V],
            group_count: None,
            components: None,
            DAG: None,
        }
    }

    /// uからvへの有向辺を追加
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.E += 1;
        self.G[u].push(v);
        self.rG[v].push(u);
    }

    pub fn decompose(&mut self) {
        // 帰りがけ順で順序付け
        let mut order = vec![];
        let mut visited = vec![false; self.V];
        for i in 0..self.V {
            Self::dfs(i, &self.G, &mut order, &mut visited);
        }

        // 連結成分に分解
        let mut group = 0;
        let mut components = vec![Self::INF; self.V];
        for &i in order.iter().rev() {
            if components[i] == Self::INF {
                Self::rdfs(i, group, &self.rG, &mut components);
                group += 1;
            }
        }

        // DAGを構築
        let mut DAG = vec![vec![]; group];
        for i in 0..self.V {
            for &j in &self.G[i] {
                let (u, v) = (components[i], components[j]);
                if u != v {
                    DAG[u].push(v);
                }
            }
        }

        self.group_count = Some(group);
        self.components = Some(components);
        self.DAG = Some(DAG);
    }

    fn dfs(u: usize, G: &Graph, order: &mut Vec<usize>, visited: &mut Vec<bool>) {
        if visited[u] {
            return;
        }
        visited[u] = true;
        for &v in &G[u] {
            Self::dfs(v, G, order, visited);
        }
        order.push(u);
    }

    fn rdfs(u: usize, group: usize, rG: &Graph, components: &mut Vec<usize>) {
        if components[u] != Self::INF {
            return;
        }
        components[u] = group;
        for &v in &rG[u] {
            Self::rdfs(v, group, rG, components);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scc() {
        let V = 6;
        let edges = [(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];
        let mut scc = SCC::new(V);
        edges.iter().for_each(|&(u, v)| scc.add_edge(u, v));

        // 強連結成分分解
        scc.decompose();

        assert_eq!(scc.group_count, Some(4));
        assert_eq!(&scc.components.unwrap(), &vec![3, 1, 2, 3, 1, 0]);
        assert_eq!(&scc.DAG.unwrap(), &vec![vec![2], vec![2], vec![], vec![]]);
    }
}
