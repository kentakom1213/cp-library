//! ダブリングにより、最小共通祖先を求める

#![allow(dead_code)]

const INF: usize = 1001001001001001001;
type Graph = Vec<Vec<usize>>;

/// # LCA
/// 最小共通祖先を求めるクエリに答える
pub struct LCA {
    double: Vec<Vec<usize>>,
    depth: Vec<usize>,
}

impl LCA {
    /// `root`を根に持つ木`tree`で、初期化を行う
    pub fn new(tree: &Graph, root: usize) -> Self {
        let V = tree.len();  // グラフの頂点数
        let logV = {          // log_2(グラフの頂点数)
            let mut logv = 0;
            while (V >> logv) > 0 {
                logv += 1;
            }
            logv
        };
        let mut double = vec![vec![0; V]; logV];  // ダブリング配列
        let mut depth = vec![INF; V];             // 頂点の根からの距離
        depth[0] = 0;
        Self::dfs(root, &mut double[0], &mut depth, tree);

        // ダブリング
        for i in 1..logV {
            for j in 0..V {
                double[i][j] = double[i-1][double[i-1][j]];
            }
        }

        Self { double, depth }
    }

    fn dfs(u: usize, par: &mut Vec<usize>, depth: &mut Vec<usize>, tree: &Graph) {
        for &v in &tree[u] {
            if depth[v] != INF { continue; }
            depth[v] = depth[u] + 1;
            par[v] = u;
            Self::dfs(v, par, depth, tree);
        }
    }

    /// 頂点`u`,`v`の最小共通祖先を求める
    pub fn get_lca(&self, mut u: usize, mut v: usize) -> usize {
        // 常にuを深くする
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // LCAまでの距離を同じにする
        for k in 0..self.double.len() {
            if ((self.depth[u] - self.depth[v]) >> k) & 1 == 1 {
                u = self.double[k][u];
            }
        }

        if u == v {
            return u;
        }

        // 二分探索
        for k in ( 0 .. self.double.len() ).rev() {
            if self.double[k][u] != self.double[k][v] {
                u = self.double[k][u];
                v = self.double[k][v];
            }
        }
        
        self.double[0][u]
    }

    /// 頂点`u`,`v`の距離を求める
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let o = self.get_lca(u, v);
        (self.depth[u] - self.depth[o]) + (self.depth[v] - self.depth[o])
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lca() {
        let tree = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![],
            vec![],
            vec![],
            vec![6, 7],
            vec![],
            vec![],
        ];
        /*
         *       (root)
         *         0
         *       / | \
         *      1  2  3
         *     / \
         *    4   5
         *       / \
         *      6   7
         */

        let lca = LCA::new(&tree, 0);

        assert_eq!(lca.get_lca(4, 6), 1);
        assert_eq!(lca.get_lca(4, 7), 1);
        assert_eq!(lca.get_lca(4, 3), 0);
        assert_eq!(lca.get_lca(5, 2), 0);
        assert_eq!(lca.get_lca(5, 7), 5);
        assert_eq!(lca.get_lca(4, 4), 4);
    }

    #[test]
    fn test_dist() {
        let tree = vec![
            vec![1, 2, 3],
            vec![4, 5],
            vec![],
            vec![],
            vec![],
            vec![6, 7],
            vec![],
            vec![],
        ];
        /*
         *       (root)
         *         0
         *       / | \
         *      1  2  3
         *     / \
         *    4   5
         *       / \
         *      6   7
         */

        let lca = LCA::new(&tree, 0);

        // 根からの距離
        assert_eq!(lca.dist(0, 7), 3);
        assert_eq!(lca.dist(0, 3), 1);
        assert_eq!(lca.dist(0, 5), 2);
        assert_eq!(lca.dist(0, 0), 0);
        
        // 根以外の頂点同士の距離
        assert_eq!(lca.dist(1, 2), 2);
        assert_eq!(lca.dist(3, 7), 4);
        assert_eq!(lca.dist(4, 1), 1);
        assert_eq!(lca.dist(2, 5), 3);
        assert_eq!(lca.dist(7, 7), 0);
        assert_eq!(lca.dist(3, 3), 0);
    }
}
