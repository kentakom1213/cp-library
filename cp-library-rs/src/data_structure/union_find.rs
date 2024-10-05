//! UnionFind木

use std::mem;

use crate::utils::consts::NEG1;

/// UnionFind木
pub struct UnionFind {
    /// 要素数
    n: usize,
    /// 親の番号を格納する配列
    parent: Vec<usize>,
    /// 連結成分の個数
    count: usize,
}

impl UnionFind {
    /// UnionFindを新規作成
    pub fn new(N: usize) -> Self {
        UnionFind {
            n: N,
            parent: vec![NEG1; N],
            count: N,
        }
    }

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
}
