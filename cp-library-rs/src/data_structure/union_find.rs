//! Disjoint-set union with optional component values.
//!
//! The base alias [`UnionFind`] behaves like an ordinary DSU.
//! [`UnionFindMonoid`] additionally stores one aggregated value per connected component.

use std::{collections::HashMap, fmt::Debug, mem};

use crate::{algebraic_structure::commutative::CommutativeMonoid, utils::consts::NEG1};

/// A plain disjoint-set union without component values.
pub type UnionFind = UnionFindMonoid<()>;

/// A disjoint-set union carrying one aggregated value per component.
///
/// Component values are combined with a [`CommutativeMonoid`] on merge.
pub struct UnionFindMonoid<M: CommutativeMonoid> {
    /// Number of elements.
    n: usize,
    /// Parent array using union-by-size encoding.
    parent: Vec<usize>,
    /// Component values stored only at roots.
    value: Vec<Option<M::Val>>,
    /// Number of connected components.
    count: usize,
}

impl<M: CommutativeMonoid> UnionFindMonoid<M> {
    /// Returns the representative of the set containing `x`.
    ///
    /// Path compression is applied.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
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

    /// Returns the aggregated value of the component containing `x`.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
    pub fn value(&mut self, x: usize) -> &M::Val {
        let root = self.root(x);
        self.value[root].as_ref().unwrap()
    }

    /// Returns `true` if `x` and `y` belong to the same component.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// Merges the components containing `x` and `y`.
    ///
    /// Returns the new root if a merge happened, or `None` if they were already connected.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
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

    /// Returns the size of the component containing `x`.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(alpha(n))`$
    pub fn get_size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.parent[root].wrapping_neg()
    }

    /// Returns the current number of connected components.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn group_count(&self) -> usize {
        self.count
    }

    /// Enumerates all components as a map from root to member list.
    ///
    /// # Complexity
    ///
    /// - $`O(n alpha(n))`$
    pub fn enum_groups(&mut self) -> HashMap<usize, Vec<usize>> {
        (0..self.n).fold(HashMap::default(), |mut map, i| {
            let root = self.root(i);
            map.entry(root).or_default().push(i);
            map
        })
    }
}

impl UnionFindMonoid<()> {
    /// Creates a plain disjoint-set union of size `n`.
    ///
    /// # Complexity
    ///
    /// - $`O(n)`$
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
