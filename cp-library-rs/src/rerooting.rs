//! 全方位木DP

use crate::consts::INF;

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
pub struct Rerooting<T, I, M, P> {
    /// dpテーブル
    dp: Vec<Vec<T>>,
    /// 結果を保存する配列
    pub ans: Vec<T>,
    /// グラフ
    G: Graph<T>,
    /// 単位元を返す関数
    id: I,
    /// 型同士の二項演算
    merge: M,
    /// 頂点の値を追加する関数
    put_edge: P,
}

impl<T, I, M, P> Rerooting<T, I, M, P>
where
    T: Clone,
    I: Fn() -> T,
    M: Fn(&T, &T) -> T,
    P: Fn(&T, &T) -> T,
{
    /// 木を初期化する
    pub fn new(N: usize, id: I, merge: M, put_edge: P) -> Self {
        Self {
            dp: vec![vec![]; N],
            ans: vec![id(); N],
            G: vec![vec![]; N],
            id,
            merge,
            put_edge,
        }
    }

    /// 重み`w`の有向辺 `(u,v)` を追加する
    pub fn add_edge(&mut self, u: usize, v: usize, w: T) {
        self.G[u].push(Edge { to: v, weight: w });
    }

    /// 重み`w`の有向辺 `(u,v)` / `(v,u)` を追加する
    pub fn add_edge2(&mut self, u: usize, v: usize, w: T) {
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
        self.bfs(INF, 0, &(self.id)());
    }

    /// 頂点`u`に対して値を集約する
    fn dfs(&mut self, p: usize, u: usize) -> T {
        let mut res = (self.id)();
        let deg = self.G[u].len();
        self.dp[u] = vec![(self.id)(); deg];

        for i in 0..deg {
            let Edge { to: v, weight } = self.G[u][i].clone();
            if v == p {
                continue;
            }
            // 再帰的に計算
            let mut val = self.dfs(u, v);
            val = (self.put_edge)(&val, &weight);
            res = (self.merge)(&res, &val);
            self.dp[u][i] = val;
        }

        res
    }

    /// rerootingを行う
    /// （実際にはdfsで処理）
    fn bfs(&mut self, p: usize, u: usize, dp_p: &T) {
        let deg = self.G[u].len();

        // 部分木の集約値を保存
        for i in 0..deg {
            let Edge { to: v, .. } = self.G[u][i];
            if v == p {
                self.dp[u][i] = dp_p.clone();
            }
        }

        // 左右からの累積を保存する配列
        let mut Sl = vec![(self.id)(); deg + 1];
        let mut Sr = vec![(self.id)(); deg + 1];
        for i in 0..deg {
            Sl[i + 1] = (self.merge)(&Sl[i], &self.dp[u][i]);
        }
        for i in (0..deg).rev() {
            Sr[i] = (self.merge)(&self.dp[u][i], &Sr[i + 1]);
        }

        // 解の計算
        self.ans[u] = Sl[deg].clone();

        // 根を移動させる
        for i in 0..deg {
            let Edge { to: v, weight } = self.G[u][i].clone();
            if v == p {
                continue;
            }
            let mut val = (self.merge)(&Sl[i], &Sr[i + 1]);
            val = (self.put_edge)(&val, &weight);
            self.bfs(u, v, &val);
        }
    }
}
