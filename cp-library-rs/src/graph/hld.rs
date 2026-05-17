//! Heavy-light decomposition.

use crate::utils::consts::Infinity;

/// Heavy-light decomposition of a rooted tree.
///
/// This structure maps each subtree to a contiguous range and decomposes a path
/// into $`O(log n)`$ heavy-path segments.
pub struct HLD {
    /// Number of vertices.
    pub N: usize,
    /// Root vertex used for decomposition.
    pub root: usize,
    /// Undirected tree adjacency list.
    pub G: Vec<Vec<usize>>,
    /// Parent of each vertex.
    pub parent: Vec<usize>,
    /// `subtree_size[i]` is the size of the subtree rooted at `i`.
    pub subtree_size: Vec<usize>,
    /// Entry time in Euler-like order.
    pub in_: Vec<usize>,
    /// Exit time in Euler-like order.
    pub out: Vec<usize>,
    /// Head vertex of the heavy path containing each vertex.
    pub head: Vec<usize>,
}

impl HLD {
    /// Creates an empty tree with `N` vertices.
    ///
    /// # Complexity
    ///
    /// - $`O(N)`$
    pub fn new(N: usize) -> Self {
        Self {
            N,
            root: usize::infinity(),
            G: vec![vec![]; N],
            parent: vec![usize::infinity(); N],
            subtree_size: vec![usize::infinity(); N],
            in_: vec![usize::infinity(); N],
            out: vec![usize::infinity(); N],
            head: vec![usize::infinity(); N],
        }
    }

    /// Adds an undirected edge `(u, v)`.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.G[u].push(v);
        self.G[v].push(u);
    }

    /// Builds the decomposition with `root` as the tree root.
    ///
    /// # Complexity
    ///
    /// - $`O(N)`$
    pub fn decompose(&mut self, root: usize) {
        self.root = root;

        // heavy childの計算
        self.set_heavy_child(usize::infinity(), root);

        // heavy pathの計算
        self.head[root] = root;
        self.build_heavy_path(usize::infinity(), root, &mut 0);
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

    /// Returns the linearized index of `v`.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    #[inline]
    pub fn get_id(&self, v: usize) -> usize {
        self.in_[v]
    }

    /// Returns the lowest common ancestor of `u` and `v`.
    ///
    /// # Complexity
    ///
    /// - $`O(log N)`$
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

    /// Returns the contiguous range corresponding to the subtree of `u`.
    ///
    /// The subtree is represented as the half-open interval `[in_[u], out[u])`.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    #[inline]
    pub fn get_subtree<T, F>(&self, u: usize) -> (usize, usize) {
        (self.in_[u], self.out[u])
    }

    /// Returns an iterator over heavy-path segments on the path from `u` to `v`.
    ///
    /// Each item of [`PathSegment`] is `(top, bottom, last, rev)`, where:
    ///
    /// - `top` is the endpoint closer to the root
    /// - `bottom` is the endpoint farther from the root
    /// - `last` indicates whether this is the final segment
    /// - `rev` indicates whether the segment direction is reversed with respect to `u -> v`
    ///
    /// # Complexity
    ///
    /// - $`O(log N)`$ segments overall
    pub fn get_path(&self, u: usize, v: usize) -> PathSegment<'_> {
        PathSegment {
            hld: self,
            from: u,
            to: v,
            exhasted: false,
        }
    }
}

/// Iterator over heavy-path segments produced by [`HLD::get_path`].
///
/// ## References
///
/// - <https://ngtkana.hatenablog.com/entry/2024/06/24/200138>
pub struct PathSegment<'a> {
    hld: &'a HLD,
    from: usize,
    to: usize,
    exhasted: bool,
}

impl Iterator for PathSegment<'_> {
    type Item = (usize, usize, bool, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            hld,
            from,
            to,
            exhasted,
        } = *self;

        if exhasted {
            return None;
        }

        let HLD {
            in_, head, parent, ..
        } = hld;

        Some(if head[from] == head[to] {
            self.exhasted = true;
            if in_[from] < in_[to] {
                (from, to, true, false)
            } else {
                (to, from, true, true)
            }
        } else if in_[from] < in_[to] {
            self.to = parent[head[to]];
            (head[to], to, false, false)
        } else {
            self.from = parent[head[from]];
            (head[from], from, false, true)
        })
    }
}
