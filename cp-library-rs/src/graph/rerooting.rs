//! 全方位木DP

use crate::algebraic_structure::monoid_with_context::MonoidCtx;

pub trait TreeMonoid: MonoidCtx {
    /// DP の値
    type T: Clone;
    /// 辺番号 i の辺を付加する
    fn put_edge(&self, x: &Self::T, i: usize) -> Self::Val;
    /// 頂点番号 v の頂点を付加する
    fn put_vertex(&self, x: &Self::Val, v: usize) -> Self::T;
}

/// 辺重みを持つグラフ
pub type Graph = Vec<Vec<Edge>>;

/// 辺の構造体
#[derive(Clone, Debug)]
pub struct Edge {
    pub to: usize,
    /// 辺のインデックス
    pub idx: usize,
    /// 逆辺のインデックス
    pub ridx: usize,
}

/// 全方位木DP
pub struct RerootingDP<M: TreeMonoid> {
    n: usize,
    g: Graph,
    root: usize,
    monoid: M,
}

impl<M: TreeMonoid> RerootingDP<M> {
    /// 空のグラフを初期化する
    pub fn new(n: usize, monoid: M) -> Self {
        Self {
            n,
            g: vec![vec![]; n],
            root: 0,
            monoid,
        }
    }

    /// 辺 (u,v) を追加する
    pub fn add_edge(&mut self, u: usize, v: usize, idx: usize, ridx: usize) {
        self.g[u].push(Edge { to: v, idx, ridx });
        self.g[v].push(Edge {
            to: u,
            idx: ridx,
            ridx: idx,
        });
    }

    /// 全方位木DP を行う
    pub fn build(&mut self, root: usize) -> Vec<M::T> {
        self.root = root;

        let (par, order) = self.rooted_order(root);
        let agg = self.aggregate(root, &par, &order);

        self.propagate(root, &par, &order, &agg)
    }

    // ========== internal ==========

    /// root = r で根付けし，親 `par` と DFS 順 `order` を返す
    fn rooted_order(&self, r: usize) -> (Vec<usize>, Vec<usize>) {
        let n = self.n;
        let mut par = vec![usize::MAX; n];
        par[r] = r;

        let mut order = Vec::with_capacity(n);
        let mut st = vec![r];
        while let Some(u) = st.pop() {
            order.push(u);
            for e in &self.g[u] {
                let v = e.to;
                if par[v] != usize::MAX {
                    continue;
                }
                par[v] = u;
                st.push(v);
            }
        }
        (par, order)
    }

    /// 根に集約する
    fn aggregate(&self, r: usize, par: &[usize], order: &[usize]) -> Vec<M::T> {
        let n = self.n;

        let mut agg = vec![self.monoid.put_vertex(&self.monoid.e(), 0); n];

        for &u in order.iter().rev() {
            let mut prod = self.monoid.e();
            for e in &self.g[u] {
                let v = e.to;
                if u != r && v == par[u] {
                    // 親方向からは集約しない
                    continue;
                }
                let val = self.monoid.put_edge(&agg[v], e.idx);
                prod = self.monoid.op(&prod, &val);
            }
            agg[u] = self.monoid.put_vertex(&prod, u);
        }

        agg
    }

    /// 根から伝播する
    fn propagate(&self, r: usize, par: &[usize], order: &[usize], agg: &[M::T]) -> Vec<M::T> {
        let n = self.n;

        // 親側の部分木の集約値
        let mut par_dp = vec![self.monoid.put_vertex(&self.monoid.e(), r); n];
        let mut ans = vec![self.monoid.put_vertex(&self.monoid.e(), 0); n];

        for &u in order {
            let deg = self.g[u].len();

            let mut vals: Vec<M::Val> = Vec::with_capacity(deg);
            for e in &self.g[u] {
                let v = e.to;
                let t = if u != r && v == par[u] {
                    &par_dp[u]
                } else {
                    &agg[v]
                };
                vals.push(self.monoid.put_edge(t, e.idx));
            }

            // 先頭からの累積
            let mut pre: Vec<M::Val> = vec![self.monoid.e(); deg + 1];
            for i in 0..deg {
                pre[i + 1] = self.monoid.op(&pre[i], &vals[i]);
            }
            // 末尾からの累積
            let mut suf = vec![self.monoid.e(); deg + 1];
            for i in (0..deg).rev() {
                suf[i] = self.monoid.op(&vals[i], &suf[i + 1]);
            }

            // ans[u] := put_vertex(隣接項の積, u)
            ans[u] = self.monoid.put_vertex(&pre[deg], u);

            // 子へ伝播
            for i in 0..deg {
                let v = self.g[u][i].to;
                if u != r && v == par[u] {
                    continue;
                }
                let left = &pre[i];
                let right = &suf[i + 1];
                let total_except_i = self.monoid.op(left, right);

                // 親側の集約値を反映
                par_dp[v] = self.monoid.put_vertex(&total_except_i, u);
            }
        }

        ans
    }
}
