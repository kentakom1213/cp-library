//! 全方位木DP（動的ディスパッチ）

use std::collections::HashMap;

use crate::utils::consts::INF;

/// 木DPの操作
pub trait TreeMonoid {
    /// 値の型
    type T: Clone;
    /// 単位元を返す関数
    fn id(&self) -> Self::T;
    /// 値同士の合成
    fn merge(&self, x: &Self::T, y: &Self::T) -> Self::T;
    /// 辺番号`i`の辺を付加する
    fn put_edge(&self, x: &Self::T, i: usize) -> Self::T {
        x.clone()
    }
    /// 頂点番号`i`の頂点を付加する
    fn put_vertex(&self, x: &Self::T, i: usize) -> Self::T {
        x.clone()
    }
}

/// 全方位木DP
pub struct Rerooting<T: Clone> {
    /// dpテーブル
    pub dp: Vec<Vec<T>>,
    /// 結果を保存する配列
    pub ans: Vec<T>,
    /// グラフ
    pub graph: Vec<Vec<usize>>,
    /// 辺の本数
    edge_cnt: usize,
    /// 辺の本数のカウント
    pub edge_id: HashMap<(usize, usize), usize>,
    /// 操作
    monoid: Box<dyn TreeMonoid<T = T>>,
}
impl<T: Clone> Rerooting<T> {
    /// 木を初期化する
    pub fn new(N: usize, monoid: Box<dyn TreeMonoid<T = T>>) -> Self {
        Self {
            dp: vec![vec![]; N],
            ans: vec![monoid.id(); N],
            graph: vec![vec![]; N],
            edge_cnt: 0,
            edge_id: HashMap::default(),
            monoid,
        }
    }

    /// 有向辺 `(u,v)` を追加する
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.graph[u].push(v);

        // 辺番号を記録
        self.edge_id.insert((u, v), self.edge_cnt);
        self.edge_cnt += 1;
    }

    /// 有向辺 `(u,v)` / `(v,u)` を追加する
    pub fn add_edge2(&mut self, u: usize, v: usize) {
        self.graph[u].push(v);
        self.graph[v].push(u);

        // 辺番号を記録
        self.edge_id.insert((u, v), self.edge_cnt);
        self.edge_id.insert((v, u), self.edge_cnt);
        self.edge_cnt += 1;
    }

    /// すべての頂点`v`について，`v`を根として集約した値を求める
    pub fn build(&mut self) {
        // 頂点0に集約
        self.aggregate(INF, 0);
        // rerooting
        self.reroot(INF, 0);
    }

    /// 頂点`u`に対して値を集約する
    pub fn aggregate(&mut self, p: usize, u: usize) -> T {
        let mut res = self.monoid.id();
        let deg = self.graph[u].len();
        self.dp[u] = vec![self.monoid.id(); deg];

        for i in 0..deg {
            let v = self.graph[u][i];
            if v == p {
                continue;
            }
            // 再帰的に計算
            let mut val = self.aggregate(u, v);

            // v から u に戻ってくる辺
            let edge_vu = *self.edge_id.get(&(v, u)).unwrap();
            val = self.monoid.put_edge(&val, edge_vu);

            res = self.monoid.merge(&res, &val);

            self.dp[u][i] = val;
        }

        // 頂点 u を付加
        res = self.monoid.put_vertex(&res, u);

        res
    }

    /// rerootingを行う
    ///
    /// - `agg_p`: 親の集約値
    pub fn reroot(&mut self, p: usize, u: usize) {
        let deg = self.graph[u].len();

        // 左右からの累積を保存する配列
        let mut Sl = vec![self.monoid.id(); deg + 1];
        let mut Sr = vec![self.monoid.id(); deg + 1];
        for i in 0..deg {
            Sl[i + 1] = self.monoid.merge(&Sl[i], &self.dp[u][i]);
        }
        for i in (0..deg).rev() {
            Sr[i] = self.monoid.merge(&self.dp[u][i], &Sr[i + 1]);
        }

        // 解の計算
        self.ans[u] = self.monoid.put_vertex(&Sl[deg], u);

        // 根を移動させる
        for i in 0..deg {
            let v = self.graph[u][i];
            if v == p {
                continue;
            }
            let val = self
                .monoid
                .put_vertex(&self.monoid.merge(&Sl[i], &Sr[i + 1]), u);
            let uv = *self.edge_id.get(&(u, v)).unwrap();

            // 親の値を伝搬
            self.dp[v][0] = self.monoid.put_edge(&val, uv);

            self.reroot(u, v);
        }
    }
}
