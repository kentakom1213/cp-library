//! 単純グラフの連結成分分解，2部グラフ判定など

type Graph = Vec<Vec<usize>>;

/// # 単純グラフ
/// 単純グラフに対して
/// - 連結成分分解
/// - 2部グラフ分解
///
/// を行う。
#[derive(Debug)]
pub struct SimpleGraph {
    pub V: usize,
    pub E: usize,
    pub graph: Graph,
    pub edges: Vec<(usize, usize)>,
    pub component_size: Option<usize>,
    pub components: Vec<usize>,
}

impl SimpleGraph {
    const INF: usize = 1_000_000_000_000_000_000;

    /// グラフの構築
    pub fn new(V: usize) -> Self {
        Self {
            V,
            E: 0,
            graph: vec![vec![]; V],
            edges: vec![],
            component_size: None,
            components: vec![Self::INF; V],
        }
    }

    /// 辺の追加
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.E += 1;
        self.edges.push((u, v));
        self.graph[u].push(v);
        self.graph[v].push(u);
    }

    /// 連結成分に分解：O(|V|+|E|)
    pub fn decompose(&mut self) {
        let mut component = 0;
        self.components = vec![Self::INF; self.V];
        for i in 0..self.V {
            if self.components[i] != Self::INF {
                continue;
            }
            self.components[i] = component;
            let mut stack = vec![i];
            while let Some(u) = stack.pop() {
                for &v in &self.graph[u] {
                    if self.components[v] == Self::INF {
                        self.components[v] = component;
                        stack.push(v);
                    }
                }
            }
            component += 1;
        }
        self.component_size = Some(component);
    }

    /// 2部グラフ判定：O(|V|+|E|)
    pub fn bipartite(&mut self) -> Option<Vec<isize>> {
        // 未だ連結成分分解されていない場合
        if self.component_size.is_none() {
            self.decompose();
        }
        let mut res: Vec<isize> = vec![0; self.V];
        for i in 0..self.V {
            let mut stack = vec![i];
            if res[i] != 0 {
                continue;
            }
            res[i] = self.components[i] as isize + 1;
            while let Some(u) = stack.pop() {
                for &v in &self.graph[u] {
                    if res[v] == res[u] {
                        return None;
                    }
                    if res[v] == 0 {
                        res[v] = -res[u];
                        stack.push(v);
                    }
                }
            }
        }
        Some(res)
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
}
