//! 全方位木DP

use crate::consts::INF;

pub trait TreeMonoid {
    /// データの型
    type T: Clone;
    /// 単位元を返す関数
    fn id() -> Self::T;
    /// 値同士の合成
    fn merge(x: &Self::T, y: &Self::T) -> Self::T;
    /// 辺の作用
    fn put_edge(x: &Self::T, weight: &Self::T) -> Self::T;
}

pub mod examples {
    use super::TreeMonoid;

    pub struct Diameter;

    impl TreeMonoid for Diameter {
        type T = isize;
        fn id() -> Self::T {
            0
        }
        fn merge(x: &Self::T, y: &Self::T) -> Self::T {
            *x.max(y)
        }
        fn put_edge(x: &Self::T, weight: &Self::T) -> Self::T {
            x + weight
        }
    }
}

/// グラフ
pub type Graph<T> = Vec<Vec<Edge<T>>>;
/// 辺の構造体
#[derive(Clone)]
pub struct Edge<T> {
    to: usize,
    /// 辺重み
    weight: T,
}
/// 全方位木DP
pub struct Rerooting<M: TreeMonoid> {
    /// dpテーブル
    dp: Vec<Vec<M::T>>,
    /// 結果を保存する配列
    pub ans: Vec<M::T>,
    /// グラフ
    G: Graph<M::T>,
}
impl<M: TreeMonoid> Rerooting<M> {
    /// 木を初期化する
    pub fn new(N: usize) -> Self {
        Self {
            dp: vec![vec![]; N],
            ans: vec![M::id(); N],
            G: vec![vec![]; N],
        }
    }
    /// 重み`w`の有向辺 `(u,v)` を追加する
    pub fn add_edge(&mut self, u: usize, v: usize, w: M::T) {
        self.G[u].push(Edge { to: v, weight: w });
    }
    /// 重み`w`の有向辺 `(u,v)` / `(v,u)` を追加する
    pub fn add_edge2(&mut self, u: usize, v: usize, w: M::T) {
        self.G[u].push(Edge {
            to: v,
            weight: w.clone(),
        });
        self.G[v].push(Edge { to: u, weight: w });
    }
    /// すべての頂点`v`について，`v`を根として集約した値を求める
    pub fn build(&mut self) {
        // 頂点0に集約
        self.dfs(INF, 0);
        // rerooting
        self.bfs(INF, 0, &M::id());
    }
    /// 頂点`u`に対して値を集約する
    fn dfs(&mut self, p: usize, u: usize) -> M::T {
        let mut res = M::id();
        let deg = self.G[u].len();
        self.dp[u] = vec![M::id(); deg];
        for i in 0..deg {
            let Edge { to: v, weight } = self.G[u][i].clone();
            if v == p {
                continue;
            }
            // 再帰的に計算
            let mut val = self.dfs(u, v);
            val = M::put_edge(&val, &weight);
            res = M::merge(&res, &val);
            self.dp[u][i] = val;
        }
        res
    }
    /// rerootingを行う
    /// （実際にはdfsで処理）
    fn bfs(&mut self, p: usize, u: usize, dp_p: &M::T) {
        let deg = self.G[u].len();
        // 部分木の集約値を保存
        for i in 0..deg {
            let Edge { to: v, .. } = self.G[u][i];
            if v == p {
                self.dp[u][i] = dp_p.clone();
            }
        }
        // 左右からの累積を保存する配列
        let mut Sl = vec![M::id(); deg + 1];
        let mut Sr = vec![M::id(); deg + 1];
        for i in 0..deg {
            Sl[i + 1] = M::merge(&Sl[i], &self.dp[u][i]);
        }
        for i in (0..deg).rev() {
            Sr[i] = M::merge(&self.dp[u][i], &Sr[i + 1]);
        }
        // 解の計算
        self.ans[u] = Sl[deg].clone();
        // 根を移動させる
        for i in 0..deg {
            let Edge { to: v, weight } = self.G[u][i].clone();
            if v == p {
                continue;
            }
            let mut val = M::merge(&Sl[i], &Sr[i + 1]);
            val = M::put_edge(&val, &weight);
            self.bfs(u, v, &val);
        }
    }
}
