//! Arena allocation utilities for tree-like data structures.

/// Marker trait for node types stored in an [`Arena`].
pub trait ArenaNode {}

/// An index-based handle into an [`Arena`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ptr(pub usize);

/// A simple append-only arena.
///
/// This type stores nodes contiguously and returns lightweight [`Ptr`] handles.
#[derive(Default)]
pub struct Arena<N: ArenaNode> {
    nodes: Vec<N>,
    id: usize,
}

impl<N: ArenaNode> Arena<N> {
    /// Creates an empty arena.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            id: 0,
        }
    }

    /// Creates an empty arena with capacity for `n` nodes.
    ///
    /// # Complexity
    ///
    /// - $`O(n)`$ allocation
    pub fn with_capacity(n: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(n),
            id: 0,
        }
    }

    /// Allocates a new node and returns its handle.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(1)`$
    pub fn alloc(&mut self, node: N) -> Ptr {
        let id = self.nodes.len();
        self.nodes.push(node);
        Ptr(id)
    }

    /// Returns an immutable reference to the node pointed to by `ptr`.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn get(&self, ptr: Ptr) -> &N {
        &self.nodes[ptr.0]
    }

    /// Returns a mutable reference to the node pointed to by `ptr`.
    ///
    /// # Complexity
    ///
    /// - $`O(1)`$
    pub fn get_mut(&mut self, ptr: Ptr) -> &mut N {
        &mut self.nodes[ptr.0]
    }
}

impl<N: ArenaNode + Default> Arena<N> {
    /// Allocates a default-initialized node.
    ///
    /// # Complexity
    ///
    /// - Amortized $`O(1)`$
    pub fn alloc_default(&mut self) -> Ptr {
        self.alloc(N::default())
    }
}
