#![allow(dead_code)]

type Edge = usize;
type Graph = Vec<Vec<Edge>>;
const INF: usize = std::usize::MAX;

/// # SCC (強連結成分分解)
/// - Strongly Conneected Components
pub struct SCC {
    size: usize, // |V|
    fG: Graph, // forwards
    bG: Graph, // backwards
    order: Vec<Edge>,
    visited: Vec<bool>,
    pub scc: Vec<Vec<Edge>>,
    pub components: Vec<usize>,
    pub dag: Graph,
}

impl SCC {
    pub fn new(n: usize) -> Self {
        SCC {
            size: n,
            fG: vec![vec![]; n],
            bG: vec![vec![]; n],
            order: vec![],
            scc: vec![],
            visited: vec![false; n],
            components: vec![INF; n],
            dag: vec![vec![]; n],
        }
    }

    /// グラフに有向辺を追加する
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.fG[u].push(v);
        self.bG[v].push(u);
    }

    /// 順方向にDFSを行う
    fn forward_dfs(&mut self, u: usize) {
        self.visited[v] = true;
        for &v in &self.fG[u] {
            if !self.visited[v] {
                self.forward_dfs(v);
            }
        }
        self.order.push(u);
    }

    /// 逆方向にDFSを行う
    fn backward_dfs(&mut self, u: usize, k: usize) {
        self.visited[u] = true;
        self.components[u] = k;
        for &v in &self.bG[u] {
            if !self.visited[v] {
                self.backward_dfs(v, k);
            }
        }
    }

    /// 強連結成分分解を行う
    pub fn decompose() {
        // 全頂点から順方向にDFSを行う
        for u in 0..self.size {
            if !self.visited[u] {
                self.forward_dfs(u);
            }
        }
        // 逆順にDFSを行う
        let mut k = 0;
        for u in self.forder.iter().rev() {
            if !self.visited[u] {
                self.backward_dfs(u, k);
                k += 1;
            }
        }
    }
}
