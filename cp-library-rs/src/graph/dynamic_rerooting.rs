//! ### 全方位木DP（クロージャ）

use std::collections::HashMap;

use crate::utils::consts::INF;

/// 全方位木DP
pub struct Rerooting<T, FE, FV>
where
    T: Clone,
    FE: Fn(&T, usize) -> T,
    FV: Fn(&T, usize) -> T,
{
    /// dpテーブル
    pub dp: Vec<Vec<T>>,
    /// 結果を保存する配列
    pub ans: Vec<T>,
    /// グラフ
    pub G: Vec<Vec<usize>>,
    /// 辺の本数
    edge_cnt: usize,
    /// 辺の番号: `(u, v) -> (辺番号, G[u].index(v))`
    pub edge_id: HashMap<(usize, usize), (usize, usize)>,
    /// 単位元
    id: T,
    /// 値をマージする関数
    merge: fn(&T, &T) -> T,
    /// 辺を追加する関数
    put_edge: FE,
    /// 頂点を追加する関数
    put_vertex: FV,
}
impl<T, FE, FV> Rerooting<T, FE, FV>
where
    T: Clone,
    FE: Fn(&T, usize) -> T,
    FV: Fn(&T, usize) -> T,
{
    /// 木を初期化する
    /// - 計算量: `$O(N)$`
    ///
    /// **引数**
    /// - `N`: 頂点数
    /// - `id`: 単位元
    /// - `merge`: 値をマージする関数
    /// - `put_edge`: 辺を追加する関数
    /// - `put_vertex`: 頂点を追加する関数
    pub fn new(N: usize, id: T, merge: fn(&T, &T) -> T, put_edge: FE, put_vertex: FV) -> Self {
        Self {
            dp: vec![vec![]; N],
            ans: vec![id.clone(); N],
            G: vec![vec![]; N],
            edge_cnt: 0,
            edge_id: HashMap::default(),
            id,
            merge,
            put_edge,
            put_vertex,
        }
    }

    /// 有向辺 `(u,v)` を追加する
    /// - 計算量: `$O(1)$`
    pub fn add_edge(&mut self, u: usize, v: usize) {
        let pos = self.G[u].len();
        self.G[u].push(v);

        // 辺番号を記録
        self.edge_id.insert((u, v), (self.edge_cnt, pos));
        self.edge_cnt += 1;
    }

    /// 有向辺 `(u,v)` / `(v,u)` を追加する
    /// - 計算量: `$O(1)$`
    pub fn add_edge2(&mut self, u: usize, v: usize) {
        let pos_u_v = self.G[u].len();
        self.G[u].push(v);
        let pos_v_u = self.G[v].len();
        self.G[v].push(u);

        // 辺番号を記録
        self.edge_id.insert((u, v), (self.edge_cnt, pos_u_v));
        self.edge_id.insert((v, u), (self.edge_cnt, pos_v_u));
        self.edge_cnt += 1;
    }

    /// すべての頂点`v`について，`v`を根として集約した値を求める
    /// - 計算量: `$O(N)$`
    pub fn build(&mut self) {
        // 頂点0に集約
        self.aggregate(INF, 0);
        // rerooting
        self.reroot(INF, 0);
    }

    /// 頂点`u`に対して値を集約する
    /// - 計算量: `$O(N)$`
    pub fn aggregate(&mut self, p: usize, u: usize) -> T {
        let mut res = self.id.clone();
        let deg = self.G[u].len();
        self.dp[u] = vec![self.id.clone(); deg];

        for i in 0..deg {
            let v = self.G[u][i];
            if v == p {
                continue;
            }
            // 再帰的に計算
            let mut val = self.aggregate(u, v);

            // v から u に戻ってくる辺
            let (edge_vu, _) = *self.edge_id.get(&(v, u)).unwrap();
            val = (self.put_edge)(&val, edge_vu);

            res = (self.merge)(&res, &val);

            self.dp[u][i] = val;
        }

        // 頂点 u を付加
        res = (self.put_vertex)(&res, u);

        res
    }

    /// rerootingを行う
    /// - 計算量: `$O(N)$`
    ///
    /// **引数**
    /// - `p`: 親の頂点
    /// - `u`: 現在の頂点
    pub fn reroot(&mut self, p: usize, u: usize) {
        let deg = self.G[u].len();

        // 左右からの累積を保存する配列
        let mut Sl = vec![self.id.clone(); deg + 1];
        let mut Sr = vec![self.id.clone(); deg + 1];
        for i in 0..deg {
            Sl[i + 1] = (self.merge)(&Sl[i], &self.dp[u][i]);
        }
        for i in (0..deg).rev() {
            Sr[i] = (self.merge)(&self.dp[u][i], &Sr[i + 1]);
        }

        // 解の計算
        self.ans[u] = (self.put_vertex)(&Sl[deg], u);

        // 根を移動させる
        for i in 0..deg {
            let v = self.G[u][i];
            if v == p {
                continue;
            }
            let val = (self.put_vertex)(&(self.merge)(&Sl[i], &Sr[i + 1]), u);
            let (edge_uv, _) = *self.edge_id.get(&(u, v)).unwrap();
            let (_, pos_u) = *self.edge_id.get(&(v, u)).unwrap();

            // 親の値を伝搬
            self.dp[v][pos_u] = (self.put_edge)(&val, edge_uv);

            self.reroot(u, v);
        }
    }
}
