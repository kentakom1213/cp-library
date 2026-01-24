//! 全方位木DP

use crate::algebraic_structure::commutative::CommutativeMonoid;

pub trait TreeMonoid: CommutativeMonoid {
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
pub struct RerootingDP<'a, M: TreeMonoid> {
    n: usize,
    g: Graph,
    root: usize,
    monoid: &'a M,
}

impl<'a, M: TreeMonoid> RerootingDP<'a, M> {
    /// 空のグラフを初期化する
    pub fn new(n: usize, monoid: &'a M) -> Self {
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
    pub fn build(&self, root: usize) -> Vec<M::Val> {
        let mut sub = vec![None; self.n];

        todo!();

        sub.into_iter().map(|x| x.unwrap()).collect()
    }
}
