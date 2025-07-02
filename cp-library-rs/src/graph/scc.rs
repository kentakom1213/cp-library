//! 強連結成分分解

type Graph = Vec<Vec<usize>>;

/// ## SCC (強連結成分分解)
/// - Strongly Conneected Components
#[derive(Debug)]
pub struct SCC {
    pub V: usize,
    pub E: usize,
    pub G: Graph,
    rG: Graph,
    pub group_count: usize,
    pub belongs_to: Vec<usize>,
    pub components: Vec<Vec<usize>>,
    pub DAG: Graph,
}

impl SCC {
    const INF: usize = usize::MAX;

    /// 頂点`V`のグラフを構築する
    pub fn new(N: usize) -> Self {
        Self {
            V: N,
            E: 0,
            G: vec![vec![]; N],
            rG: vec![vec![]; N],
            group_count: 0,
            belongs_to: vec![0; N],
            components: vec![],
            DAG: vec![],
        }
    }

    /// uからvへの有向辺を追加
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.E += 1;
        self.G[u].push(v);
        self.rG[v].push(u);
    }

    /// 強連結成分に分解する
    pub fn decompose(&mut self) {
        // 帰りがけ順で順序付け
        let mut order = vec![];
        let mut visited = vec![false; self.V];
        for i in 0..self.V {
            Self::dfs(i, &self.G, &mut order, &mut visited);
        }

        // 連結成分に分解
        let mut group = 0;
        let mut belongs_to = vec![Self::INF; self.V];
        for &i in order.iter().rev() {
            if belongs_to[i] == Self::INF {
                Self::rdfs(i, group, &self.rG, &mut belongs_to);
                group += 1;
            }
        }

        // DAGを構築
        let mut DAG = vec![vec![]; group];
        for i in 0..self.V {
            for &j in &self.G[i] {
                let (u, v) = (belongs_to[i], belongs_to[j]);
                if u != v {
                    DAG[u].push(v);
                }
            }
        }

        // 分解する
        self.components.resize_with(group, Vec::new);

        for (v, &g) in belongs_to.iter().enumerate() {
            self.components[g].push(v);
        }

        self.group_count = group;
        self.belongs_to = belongs_to;
        self.DAG = DAG;
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

    fn rdfs(u: usize, group: usize, rG: &Graph, belongs_to: &mut Vec<usize>) {
        if belongs_to[u] != Self::INF {
            return;
        }
        belongs_to[u] = group;
        for &v in &rG[u] {
            Self::rdfs(v, group, rG, belongs_to);
        }
    }
}
