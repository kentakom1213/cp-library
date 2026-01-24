//! ダブリングにより、最小共通祖先を求める

use crate::utils::consts::INF;

type Graph = Vec<Vec<usize>>;

/// LCA
/// - 最小共通祖先を求めるクエリに答える
pub struct LCA {
    root: usize,
    logn: usize,
    double: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LCA {
    /// `root`を根に持つ木`tree`で、初期化を行う
    pub fn new(tree: &Graph, root: usize) -> Self {
        let n = tree.len(); // グラフの頂点数
        let logn = n.next_power_of_two().trailing_zeros() as usize;
        let mut double = vec![vec![0; n]; logn]; // ダブリング配列
        let mut depth = vec![INF; n]; // 頂点の根からの距離
        depth[0] = 0;
        Self::dfs(root, &mut double[0], &mut depth, tree);

        // ダブリング
        for i in 1..logn {
            for j in 0..n {
                double[i][j] = double[i - 1][double[i - 1][j]];
            }
        }

        Self {
            root,
            logn,
            double,
            depth,
        }
    }

    fn dfs(u: usize, par: &mut Vec<usize>, depth: &mut Vec<usize>, tree: &Graph) {
        for &v in &tree[u] {
            if depth[v] != INF {
                continue;
            }
            depth[v] = depth[u] + 1;
            par[v] = u;
            Self::dfs(v, par, depth, tree);
        }
    }

    /// 頂点 `v` の深さを求める
    pub fn depth(&self, v: usize) -> usize {
        self.depth[v]
    }

    /// 頂点 `v` の親を求める
    pub fn parent(&self, v: usize) -> Option<usize> {
        (v != self.root).then_some(self.double[0][v])
    }

    /// 頂点 `u`,`v` の最小共通祖先を求める
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        // 常にuを深くする
        if self.depth[u] < self.depth[v] {
            (u, v) = (v, u);
        }

        // LCAまでの距離を同じにする
        for k in 0..self.logn {
            if ((self.depth[u] - self.depth[v]) >> k) & 1 == 1 {
                u = self.double[k][u];
            }
        }

        if u == v {
            return u;
        }

        // 二分探索
        for k in (0..self.logn).rev() {
            if self.double[k][u] != self.double[k][v] {
                u = self.double[k][u];
                v = self.double[k][v];
            }
        }

        self.double[0][u]
    }

    /// 頂点 `u`,`v` の距離を求める
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let o = self.lca(u, v);
        (self.depth[u] - self.depth[o]) + (self.depth[v] - self.depth[o])
    }

    /// 頂点 `v` の k 個上の祖先を求める
    pub fn kth_ancestor(&self, mut v: usize, k: usize) -> Option<usize> {
        if k > self.depth(v) {
            return None;
        }
        for i in 0..self.logn {
            if (k >> i) & 1 == 1 {
                v = self.double[i][v];
            }
        }
        Some(v)
    }

    /// 頂点 `u` から `v` へ向かうパス上の `k` 個目の頂点を求める
    pub fn kth_on_path(&self, u: usize, v: usize, k: usize) -> Option<usize> {
        let o = self.lca(u, v);
        let dist_u_o = self.depth[u] - self.depth[o];
        if k <= dist_u_o {
            self.kth_ancestor(u, k)
        } else {
            let dist_v_o = self.depth[v] - self.depth[o];
            (dist_v_o + dist_u_o)
                .checked_sub(k)
                .and_then(|l| self.kth_ancestor(v, l))
        }
    }
}
