use crate::consts::INF;

/// EulerTour
#[derive(Debug)]
pub struct EulerTour {
    pub G: Vec<Vec<usize>>,
    pub in_: Vec<usize>,
    pub out: Vec<usize>,
    pub depth: Vec<usize>,
}

impl EulerTour {
    /// 木を初期化する
    pub fn new(N: usize) -> Self {
        Self {
            G: vec![vec![]; N],
            in_: vec![INF; N],
            out: vec![INF; N],
            depth: vec![INF; N],
        }
    }

    /// 辺 `(u,v)` を追加する
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.G[u].push(v);
        self.G[v].push(u);
    }

    /// 順序付けを行う
    pub fn build(&mut self, root: usize) {
        self.dfs(INF, root, &mut 0, &mut 0);
    }

    /// 行きがけ順，帰りがけ順で順序付け
    fn dfs(&mut self, p: usize, u: usize, id: &mut usize, depth: &mut usize) {
        self.in_[u] = *id;
        self.depth[u] = *depth;

        *depth += 1;

        for i in 0..self.G[u].len() {
            let v = self.G[u][i];
            if v == p {
                continue;
            }
            *id += 1;
            self.dfs(u, v, id, depth);
        }

        *depth -= 1;
        *id += 1;

        self.out[u] = *id;
    }
}
