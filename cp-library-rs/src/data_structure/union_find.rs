//! UnionFind木

use std::mem;

/// UnionFind木
pub struct UnionFind {
    parent: Vec<usize>,
    siz: Vec<usize>,
    /// 連結成分の個数
    count: usize,
}

impl UnionFind {
    /// UnionFindを新規作成
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            siz: vec![1; n],
            count: n,
        }
    }

    /// 根を求める
    pub fn get_root(&mut self, mut x: usize) -> usize {
        // 根を探索
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        // 経路圧縮
        while x != root {
            x = mem::replace(&mut self.parent[x], root);
        }
        root
    }

    /// 同一の集合に所属するか判定
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.get_root(x) == self.get_root(y)
    }

    /// 集合`x,y`を併合する．
    ///
    /// **戻り値**
    /// - すでに併合済みだった場合`false`，そうでない場合`true`を返す
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut parent = self.get_root(x);
        let mut child = self.get_root(y);

        if parent == child {
            return false;
        }

        // 要素数が大きい方を親にすることで、高さを均等に保つ
        if self.siz[parent] < self.siz[child] {
            (parent, child) = (child, parent);
        }

        self.parent[child] = parent;
        self.siz[parent] += self.siz[child];
        self.count -= 1;

        true
    }

    /// 連結成分の大きさを求める
    pub fn get_size(&mut self, x: usize) -> usize {
        let get_root = self.get_root(x);
        self.siz[get_root]
    }

    /// 連結成分の数を返す
    pub fn group_count(&self) -> usize {
        self.count
    }
}
