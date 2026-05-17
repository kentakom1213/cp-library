//! Weighted disjoint-set union.

use crate::algebraic_structure::abel::Abel;

/// A weighted union-find over an abelian group.
///
/// It maintains relative potentials between vertices in the same component.
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
    /// Creates a weighted union-find with `n` isolated vertices.
    ///
    /// # Complexity
    ///
    /// - $`O(n)`$
    pub fn new(n: usize) -> Self {
        WeightedUnionFind {
            par: (0..n).collect(),
            rank: vec![1; n],
            weight: vec![G::e(); n],
            group_count: n,
        }
    }

    /// Returns the representative of `x`.
    ///
    /// Path compression is applied.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
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

    /// Returns the potential of `x` relative to its component root.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
    pub fn weight(&mut self, x: usize) -> G::Val {
        self.get_root(x); // 経路圧縮
        self.weight[x].clone()
    }

    /// Returns `true` if `x` and `y` are in the same component.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.get_root(x) == self.get_root(y)
    }

    /// Returns the potential difference `weight(y) - weight(x)`.
    ///
    /// Returns `None` if `x` and `y` are in different components.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
    pub fn diff(&mut self, x: usize, y: usize) -> Option<G::Val> {
        if self.is_same(x, y) {
            let res = G::op(&self.weight(y), &G::inv(&self.weight(x)));
            return Some(res);
        }
        None
    }

    /// Merges the components so that `diff(x, y) = weight` holds afterward.
    ///
    /// Returns:
    ///
    /// - `Ok(true)` if a merge happened.
    /// - `Ok(false)` if `x` and `y` were already connected and the constraint was consistent.
    /// - `Err("weight mismatch")` if they were already connected but the constraint was inconsistent.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
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

    /// Returns the size of the component containing `x`.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
    pub fn get_size(&mut self, x: usize) -> usize {
        let get_root = self.get_root(x);
        self.rank[get_root]
    }

    /// Returns the current number of connected components.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    #[inline]
    pub fn group_count(&self) -> usize {
        self.group_count
    }
}
