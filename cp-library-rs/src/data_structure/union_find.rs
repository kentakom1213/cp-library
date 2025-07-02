//! ## UnionFind木
//!
//! モノイドを乗せるUnionFind木．

use std::{collections::HashMap, fmt::Debug, mem};

use crate::{algebraic_structure::commutative::CommutativeMonoid, utils::consts::NEG1};

/// UnionFind木
pub type UnionFind = UnionFindMonoid<()>;

/// UnionFind木（モノイド）
pub struct UnionFindMonoid<M: CommutativeMonoid> {
    /// 要素数
    n: usize,
    /// 親の番号を格納する配列
    parent: Vec<usize>,
    /// 値
    value: Vec<Option<M::Val>>,
    /// 連結成分の個数
    count: usize,
}

impl<M: CommutativeMonoid> UnionFindMonoid<M> {
    /// 根を求める
    pub fn root(&mut self, mut x: usize) -> usize {
        // 根を探索
        let mut root = x;
        while self.parent[root] < self.n {
            root = self.parent[root];
        }
        // 経路圧縮
        while self.parent[x] < self.n {
            x = mem::replace(&mut self.parent[x], root);
        }
        root
    }

    /// ノード`x`が属する集合の値を取得
    pub fn value(&mut self, x: usize) -> &M::Val {
        let root = self.root(x);
        self.value[root].as_ref().unwrap()
    }

    /// 同一の集合に所属するか判定
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// 集合`x,y`を併合する．
    ///
    /// **戻り値**
    /// - すでに併合済みだった場合`None`，そうでない場合親となった要素の番号を返す
    pub fn unite(&mut self, x: usize, y: usize) -> Option<usize> {
        let mut parent = self.root(x);
        let mut child = self.root(y);

        if parent == child {
            return None;
        }

        // 要素数が大きい方を親にすることで、高さを均等に保つ
        if self.parent[parent] > self.parent[child] {
            (parent, child) = (child, parent);
        }

        self.parent[parent] = self.parent[parent].wrapping_add(self.parent[child]);
        self.parent[child] = parent;
        self.count -= 1;

        // 値のマージ
        let child_val = self.value[child].take();
        let parent_val = self.value[parent].take();
        self.value[parent] = child_val.zip(parent_val).map(|(c, p)| M::op(&c, &p));

        Some(parent)
    }

    /// 連結成分の大きさを求める
    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.parent[root].wrapping_neg()
    }

    /// 連結成分の数を返す
    pub fn group_count(&self) -> usize {
        self.count
    }

    /// {代表元: 集合} のマップを返す
    ///
    /// - 計算量: $`O(N)`$
    pub fn enum_groups(&mut self) -> HashMap<usize, Vec<usize>> {
        (0..self.n).fold(HashMap::default(), |mut map, i| {
            let root = self.root(i);
            map.entry(root).or_insert_with(Vec::new).push(i);
            map
        })
    }
}

impl UnionFindMonoid<()> {
    /// 新しいUnionFind木を生成する
    pub fn new(n: usize) -> Self {
        UnionFindMonoid {
            n,
            parent: vec![NEG1; n],
            value: vec![None; n],
            count: n,
        }
    }
}

impl<M: CommutativeMonoid> From<Vec<M::Val>> for UnionFindMonoid<M> {
    fn from(value: Vec<M::Val>) -> Self {
        let N = value.len();
        UnionFindMonoid {
            n: N,
            parent: vec![NEG1; N],
            value: value.into_iter().map(Some).collect(),
            count: N,
        }
    }
}

impl<M: CommutativeMonoid> FromIterator<M::Val> for UnionFindMonoid<M> {
    fn from_iter<T: IntoIterator<Item = M::Val>>(iter: T) -> Self {
        UnionFindMonoid::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<M> Debug for UnionFindMonoid<M>
where
    M: CommutativeMonoid,
    M::Val: Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut uf = UnionFindMonoid::<M> {
            n: self.n,
            parent: self.parent.clone(),
            value: self.value.clone(),
            count: self.count,
        };
        let groups = uf.enum_groups();

        f.debug_map().entries(groups).finish()
    }
}
