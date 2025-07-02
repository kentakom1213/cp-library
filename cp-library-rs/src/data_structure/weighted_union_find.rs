//! 重み付きUnionFind

use crate::algebraic_structure::abel::Abel;

/// 重み付きUnionFind
pub struct WeightedUnionFind<G: Abel> {
    par: Vec<usize>,
    rank: Vec<usize>,
    weight: Vec<G::Val>,
    group_count: usize,
}

impl<G: Abel> WeightedUnionFind<G>
where
    G::Val: Eq,
{
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
    pub fn get_root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        let r = self.get_root(self.par[x]);
        let parent = self.weight[self.par[x]].clone();
        let child = self.weight.get_mut(x).unwrap();
        *child = G::op(child, &parent);
        self.par[x] = r; // 経路圧縮
        r
    }

    /// 重みを求める
    pub fn weight(&mut self, x: usize) -> G::Val {
        self.get_root(x); // 経路圧縮
        self.weight[x].clone()
    }

    /// 同一の集合に所属するか判定
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.get_root(x) == self.get_root(y)
    }

    /// 重みの差を求める
    ///
    /// 同じグループにいない場合にはNoneを返す
    pub fn diff(&mut self, x: usize, y: usize) -> Option<G::Val> {
        if self.is_same(x, y) {
            let res = G::op(&self.weight(y), &G::inv(&self.weight(x)));
            return Some(res);
        }
        None
    }

    /// 集合`x,y`を`self.diff(x, y) = weight`となるように併合する．
    ///
    /// **戻り値**
    /// - すでに`x,y`が併合済みだった場合
    ///   - `self.diff(x, y) == weight` の場合 → `Some(false)`
    ///   - `self.diff(x, y) != weight` の場合 → `Err(())`
    /// - `x,y`が併合済みでない場合 → `Ok(true)`
    pub fn unite(&mut self, mut x: usize, mut y: usize, mut weight: G::Val) -> Result<bool, &str> {
        // すでにmerge済みの場合
        if let Some(w) = self.diff(x, y) {
            return if w == weight {
                Ok(false)
            } else {
                Err("weight mismatch")
            };
        }

        // x, yそれぞれについて重み差分を補正
        weight = G::op(&weight, &self.weight(x));
        weight = G::op(&weight, &G::inv(&self.weight(y)));

        x = self.get_root(x);
        y = self.get_root(y);

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

        Ok(true)
    }

    /// `x`が属する集合の大きさを求める
    pub fn get_size(&mut self, x: usize) -> usize {
        let get_root = self.get_root(x);
        self.rank[get_root]
    }

    /// 全体の要素数を求める
    #[inline]
    pub fn group_count(&self) -> usize {
        self.group_count
    }
}
