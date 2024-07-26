//! 重み付きUnionFind

use crate::algebraic_structure::abel::Abel;

/// 重み付きUnionFind
pub struct WeightedUnionFind<G: Abel> {
    par: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<G::Val>,
    pub group_count: usize,
}

impl<G: Abel> WeightedUnionFind<G> {
    /// UnionFindを構築
    pub fn new(n: usize) -> Self {
        WeightedUnionFind {
            par: (0..n).collect(),
            rank: vec![1; n],
            weight: vec![G::id(); n],
            group_count: n,
        }
    }

    /// 根を求める
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        let r = self.root(self.par[x]);
        let parent = self.weight[self.par[x]].clone();
        let child = self.weight.get_mut(x).unwrap();
        *child = G::op(child, &parent);
        self.par[x] = r; // 経路圧縮
        r
    }

    /// 重みを求める
    pub fn weight(&mut self, x: usize) -> G::Val {
        self.root(x); // 経路圧縮
        self.weight[x].clone()
    }

    /// 同一の集合に所属するか判定
    pub fn issame(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// 重みの差を求める
    /// - 同じグループにいない場合にはNone
    pub fn diff(&mut self, x: usize, y: usize) -> Option<G::Val> {
        if self.issame(x, y) {
            let res = G::op(&self.weight(y), &G::inv(&self.weight(x)));
            return Some(res);
        }
        None
    }

    /// 要素を結合
    pub fn unite(&mut self, mut x: usize, mut y: usize, mut weight: G::Val) -> bool {
        // x,yそれぞれについて重み差分を補正
        weight = G::op(&weight, &self.weight(x));
        weight = G::op(&weight, &G::inv(&self.weight(y)));

        x = self.root(x);
        y = self.root(y);

        if x == y {
            return false;
        }

        // 要素数が大きい方を子にすることで、高さを均等に保つ
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
            weight = G::inv(&weight);
        }

        self.par[y] = x;
        self.rank[x] += self.rank[y];
        self.group_count -= 1;

        // 重みの更新
        self.weight[y] = weight;

        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.rank[root]
    }
}
