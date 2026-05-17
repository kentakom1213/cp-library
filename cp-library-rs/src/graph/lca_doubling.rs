//! Lowest common ancestor by binary lifting.

use crate::utils::consts::Infinity;

type Graph = Vec<Vec<usize>>;

/// LCA data structure based on binary lifting.
///
/// After $`O(n log n)` preprocessing, each query is answered in `O(log n)`$.
pub struct LCA {
    root: usize,
    logn: usize,
    double: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LCA {
    /// Builds the structure for `tree` rooted at `root`.
    ///
    /// # Complexity
    ///
    /// - $`O(n log n)`$
    pub fn new(tree: &Graph, root: usize) -> Self {
        let n = tree.len(); // グラフの頂点数
        let logn = n.next_power_of_two().trailing_zeros() as usize;
        let mut double = vec![vec![0; n]; logn]; // ダブリング配列
        let mut depth = vec![usize::infinity(); n]; // 頂点の根からの距離
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
            if depth[v] != usize::infinity() {
                continue;
            }
            depth[v] = depth[u] + 1;
            par[v] = u;
            Self::dfs(v, par, depth, tree);
        }
    }

    /// Returns the depth of `v` from the root.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn depth(&self, v: usize) -> usize {
        self.depth[v]
    }

    /// Returns the parent of `v`, or `None` if `v` is the root.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn parent(&self, v: usize) -> Option<usize> {
        (v != self.root).then_some(self.double[0][v])
    }

    /// Returns the lowest common ancestor of `u` and `v`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
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

    /// Returns the number of edges on the path between `u` and `v`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let o = self.lca(u, v);
        (self.depth[u] - self.depth[o]) + (self.depth[v] - self.depth[o])
    }

    /// Returns the `k`-th ancestor of `v`.
    ///
    /// Returns `None` if the ancestor does not exist.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
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

    /// Returns the `k`-th vertex on the path from `u` to `v`.
    ///
    /// Here `k = 0` means `u`.
    ///
    /// # Complexity
    ///
    /// - $`O(log n)`$
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
