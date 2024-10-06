#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug, get,
    utils::consts::INF,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(usize;;);
    let P = get!(usize;;);

    let mut hld = HeavyLightDecomposition::new(N);

    for (v, &p) in (1..).zip(&P) {
        hld.add_edge(p, v);
    }

    hld.decompose(0);

    // 1点取得 & 区間和 のためのセグ木
    let mut seg: SegmentTree<Add<usize>> = SegmentTree::new(N);

    for v in 0..N {
        let id = hld.get_id(v);
        *seg.get_mut(id).unwrap() += A[v];
    }

    debug!(seg);

    // クエリ処理
    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, u, x] = &q[..] {
            let id = hld.get_id(u);
            *seg.get_mut(id).unwrap() += x;
        }

        if let &[1, u] = &q[..] {
            let res = hld.get_subtree(u, |l, r| seg.get_range(l..r));
            println!("{}", res);
        }
    }
}

/// HL分解
pub struct HeavyLightDecomposition {
    /// 頂点数
    pub N: usize,
    /// 根
    pub root: usize,
    /// グラフ
    pub G: Vec<Vec<usize>>,
    /// 親頂点
    pub parent: Vec<usize>,
    /// subtree_size[i] := `i`を根とする部分木のサイズ
    pub subtree_size: Vec<usize>,
    /// 行きがけ順での番号
    pub in_: Vec<usize>,
    /// 帰りがけ順での番号
    pub out: Vec<usize>,
    /// heavy pathの端点
    pub head: Vec<usize>,
}

impl HeavyLightDecomposition {
    /// `N`頂点の木を初期化する
    pub fn new(N: usize) -> Self {
        Self {
            N,
            root: INF,
            G: vec![vec![]; N],
            parent: vec![INF; N],
            subtree_size: vec![INF; N],
            in_: vec![INF; N],
            out: vec![INF; N],
            head: vec![INF; N],
        }
    }

    /// 辺`(u,v)`を追加する
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.G[u].push(v);
        self.G[v].push(u);
    }

    /// 頂点`root`を根としてHL分解をする
    pub fn decompose(&mut self, root: usize) {
        self.root = root;

        // heavy childの計算
        self.set_heavy_child(INF, root);

        // heavy pathの計算
        self.head[root] = root;
        self.build_heavy_path(INF, root, &mut 0);
    }

    /// 頂点`v`の配列上でのインデックス
    #[inline]
    pub fn get_id(&self, v: usize) -> usize {
        self.in_[v]
    }

    /// 2頂点`u,v`の最小共通祖先 (LCA) を求める
    pub fn get_lca(&self, mut u: usize, mut v: usize) -> usize {
        let mut pu = self.head[u];
        let mut pv = self.head[v];

        while self.head[u] != self.head[v] {
            if self.in_[pu] > self.in_[pv] {
                u = self.parent[pu];
                pu = self.head[u];
            } else {
                v = self.parent[pv];
                pv = self.head[v];
            }
        }

        if self.in_[u] <= self.in_[v] {
            u
        } else {
            v
        }
    }

    /// `u`を根とする部分木の値を集約する
    ///
    /// （モノイド`M`が可環であるときに定義される）
    pub fn get_subtree<T, F>(&self, u: usize, f: F) -> T
    where
        F: Fn(usize, usize) -> T,
    {
        f(self.in_[u], self.out[u])
    }

    /// 再帰的にheavy childを計算し，
    /// heavy childが`G[u][0]`にくるように設定する．
    ///
    /// （これにより，行きがけ順の走査で自然にheavy pathがえられる）
    fn set_heavy_child(&mut self, p: usize, u: usize) {
        self.parent[u] = p;
        self.subtree_size[u] = 1;

        for i in 0..self.G[u].len() {
            let v = self.G[u][i];
            if v == p {
                continue;
            }
            // 再帰的に計算
            self.set_heavy_child(u, v);

            // 部分木のサイズを足す
            self.subtree_size[u] += self.subtree_size[v];

            // G[u][0]にheavy childがくるようにswap
            let v_0 = self.G[u][0];
            if v_0 == p || self.subtree_size[v] > self.subtree_size[v_0] {
                self.G[u].swap(i, 0);
            }
        }
    }

    /// 行きがけ順に走査し，heavy pathの列を構築する
    fn build_heavy_path(&mut self, p: usize, u: usize, id: &mut usize) {
        self.in_[u] = *id;
        *id += 1;

        for i in 0..self.G[u].len() {
            let v = self.G[u][i];
            if v == p {
                continue;
            }
            self.head[v] = if i == 0 {
                // 自分がheavy childの場合
                self.head[u]
            } else {
                v
            };

            self.build_heavy_path(u, v, id);
        }

        self.out[u] = *id;
    }
}
