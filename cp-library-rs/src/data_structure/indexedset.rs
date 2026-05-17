//! An indexed multiset with order statistics.

use crate::tree::arena::{Arena, ArenaNode, Ptr};
use std::{cmp::Ordering, fmt::Debug, iter::FromIterator};

#[derive(Debug)]
struct Node<T: Ord> {
    key: T,
    priority: u64,
    count: usize,
    len: usize,
    distinct_len: usize,
    left: Option<Ptr>,
    right: Option<Ptr>,
}

impl<T: Ord> ArenaNode for Node<T> {}

impl<T: Ord> Node<T> {
    fn new(key: T, priority: u64) -> Self {
        Self {
            key,
            priority,
            count: 1,
            len: 1,
            distinct_len: 1,
            left: None,
            right: None,
        }
    }
}

/// An ordered multiset that supports order-statistics queries.
///
/// Duplicate keys are stored by count rather than as separate nodes.
pub struct IndexedSet<T: Ord> {
    arena: Arena<Node<T>>,
    root: Option<Ptr>,
    len: usize,
    distinct_len: usize,
    rng_state: u64,
}

impl<T: Ord> Default for IndexedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> IndexedSet<T> {
    /// Creates an empty multiset.
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            root: None,
            len: 0,
            distinct_len: 0,
            rng_state: 0x9e37_79b9_7f4a_7c15,
        }
    }

    /// Returns the total number of elements, including duplicates.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of distinct keys.
    pub fn distinct_len(&self) -> usize {
        self.distinct_len
    }

    /// Returns `true` if the multiset contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns a reference to `key` if it exists.
    pub fn get(&self, key: &T) -> Option<&T> {
        let mut cur = self.root;
        while let Some(ptr) = cur {
            let node = self.arena.get(ptr);
            match key.cmp(&node.key) {
                Ordering::Less => cur = node.left,
                Ordering::Greater => cur = node.right,
                Ordering::Equal => return Some(&node.key),
            }
        }
        None
    }

    /// Returns `true` if `key` exists in the multiset.
    pub fn contains(&self, key: &T) -> bool {
        self.get(key).is_some()
    }

    /// Returns the multiplicity of `key`.
    pub fn count(&self, key: &T) -> usize {
        let mut cur = self.root;
        while let Some(ptr) = cur {
            let node = self.arena.get(ptr);
            match key.cmp(&node.key) {
                Ordering::Less => cur = node.left,
                Ordering::Greater => cur = node.right,
                Ordering::Equal => return node.count,
            }
        }
        0
    }

    /// Inserts one occurrence of `key`.
    ///
    /// Returns the multiplicity of `key` after insertion.
    pub fn insert(&mut self, key: T) -> usize {
        let priority = self.next_priority();
        let (root, count, inserted_new) = self.insert_rec(self.root, key, priority);
        self.root = root;
        self.len += 1;
        if inserted_new {
            self.distinct_len += 1;
        }
        count
    }

    /// Removes one occurrence of `key`.
    ///
    /// Returns `true` if an element was removed.
    pub fn remove(&mut self, key: &T) -> bool {
        self.remove_n(key, 1) == 1
    }

    /// Removes all occurrences of `key`.
    ///
    /// Returns the number of removed elements.
    pub fn remove_all(&mut self, key: &T) -> usize {
        let count = self.count(key);
        if count == 0 {
            return 0;
        }
        self.remove_n(key, count)
    }

    /// Removes up to `n` occurrences of `key`.
    ///
    /// Returns the number of removed elements.
    pub fn remove_n(&mut self, key: &T, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        let (root, removed, removed_node) = self.remove_rec(self.root, key, n);
        self.root = root;
        if removed > 0 {
            self.len -= removed;
            if removed_node {
                self.distinct_len -= 1;
            }
        }
        removed
    }

    /// Returns the minimum element.
    pub fn first(&self) -> Option<&T> {
        let mut cur = self.root?;
        while let Some(left) = self.arena.get(cur).left {
            cur = left;
        }
        Some(&self.arena.get(cur).key)
    }

    /// Returns the maximum element.
    pub fn last(&self) -> Option<&T> {
        let mut cur = self.root?;
        while let Some(right) = self.arena.get(cur).right {
            cur = right;
        }
        Some(&self.arena.get(cur).key)
    }

    /// Returns the smallest element greater than or equal to `key`.
    pub fn first_ge(&self, key: &T) -> Option<&T> {
        let mut cur = self.root;
        let mut candidate = None;
        while let Some(ptr) = cur {
            let node = self.arena.get(ptr);
            if key <= &node.key {
                candidate = Some(ptr);
                cur = node.left;
            } else {
                cur = node.right;
            }
        }
        candidate.map(|ptr| &self.arena.get(ptr).key)
    }

    /// Returns the smallest element strictly greater than `key`.
    pub fn first_gt(&self, key: &T) -> Option<&T> {
        let mut cur = self.root;
        let mut candidate = None;
        while let Some(ptr) = cur {
            let node = self.arena.get(ptr);
            if key < &node.key {
                candidate = Some(ptr);
                cur = node.left;
            } else {
                cur = node.right;
            }
        }
        candidate.map(|ptr| &self.arena.get(ptr).key)
    }

    /// Returns the largest element less than or equal to `key`.
    pub fn last_le(&self, key: &T) -> Option<&T> {
        let mut cur = self.root;
        let mut candidate = None;
        while let Some(ptr) = cur {
            let node = self.arena.get(ptr);
            if key >= &node.key {
                candidate = Some(ptr);
                cur = node.right;
            } else {
                cur = node.left;
            }
        }
        candidate.map(|ptr| &self.arena.get(ptr).key)
    }

    /// Returns the largest element strictly less than `key`.
    pub fn last_lt(&self, key: &T) -> Option<&T> {
        let mut cur = self.root;
        let mut candidate = None;
        while let Some(ptr) = cur {
            let node = self.arena.get(ptr);
            if key > &node.key {
                candidate = Some(ptr);
                cur = node.right;
            } else {
                cur = node.left;
            }
        }
        candidate.map(|ptr| &self.arena.get(ptr).key)
    }

    /// Returns the `n`-th element in sorted order, counting duplicates.
    ///
    /// The index is zero-based.
    pub fn nth(&self, mut n: usize) -> Option<&T> {
        let mut cur = self.root?;
        loop {
            let node = self.arena.get(cur);
            let left_len = self.len_of(node.left);
            if n < left_len {
                cur = node.left?;
            } else if n < left_len + node.count {
                return Some(&node.key);
            } else {
                n -= left_len + node.count;
                cur = node.right?;
            }
        }
    }

    /// Returns the zero-based position of the first occurrence of `key`.
    ///
    /// Positions are counted with duplicates included.
    pub fn position(&self, key: &T) -> Option<usize> {
        let mut cur = self.root;
        let mut skipped = 0;
        while let Some(ptr) = cur {
            let node = self.arena.get(ptr);
            match key.cmp(&node.key) {
                Ordering::Less => cur = node.left,
                Ordering::Greater => {
                    skipped += self.len_of(node.left) + node.count;
                    cur = node.right;
                }
                Ordering::Equal => return Some(skipped + self.len_of(node.left)),
            }
        }
        None
    }

    /// Returns an iterator over the elements in sorted order.
    ///
    /// Duplicate keys are yielded multiple times.
    pub fn iter(&self) -> Iter<'_, T> {
        let mut iter = Iter {
            arena: &self.arena,
            stack: vec![],
            repeat_ptr: None,
            repeat_remaining: 0,
        };
        iter.push_left_edge(self.root);
        iter
    }

    #[inline]
    fn next_priority(&mut self) -> u64 {
        self.rng_state ^= self.rng_state << 7;
        self.rng_state ^= self.rng_state >> 9;
        self.rng_state
    }

    #[inline]
    fn len_of(&self, ptr: Option<Ptr>) -> usize {
        ptr.map_or(0, |ptr| self.arena.get(ptr).len)
    }

    #[inline]
    fn distinct_len_of(&self, ptr: Option<Ptr>) -> usize {
        ptr.map_or(0, |ptr| self.arena.get(ptr).distinct_len)
    }

    fn pull(&mut self, ptr: Ptr) {
        let (left, right, count) = {
            let node = self.arena.get(ptr);
            (node.left, node.right, node.count)
        };
        let len = self.len_of(left) + count + self.len_of(right);
        let distinct_len = self.distinct_len_of(left) + 1 + self.distinct_len_of(right);
        let node = self.arena.get_mut(ptr);
        node.len = len;
        node.distinct_len = distinct_len;
    }

    fn rotate_left(&mut self, root: Ptr) -> Ptr {
        let new_root = self
            .arena
            .get(root)
            .right
            .expect("rotate_left requires right child");
        let beta = self.arena.get(new_root).left;
        self.arena.get_mut(root).right = beta;
        self.arena.get_mut(new_root).left = Some(root);
        self.pull(root);
        self.pull(new_root);
        new_root
    }

    fn rotate_right(&mut self, root: Ptr) -> Ptr {
        let new_root = self
            .arena
            .get(root)
            .left
            .expect("rotate_right requires left child");
        let beta = self.arena.get(new_root).right;
        self.arena.get_mut(root).left = beta;
        self.arena.get_mut(new_root).right = Some(root);
        self.pull(root);
        self.pull(new_root);
        new_root
    }

    fn merge(&mut self, left: Option<Ptr>, right: Option<Ptr>) -> Option<Ptr> {
        match (left, right) {
            (None, other) | (other, None) => other,
            (Some(left_ptr), Some(right_ptr)) => {
                let left_prio = self.arena.get(left_ptr).priority;
                let right_prio = self.arena.get(right_ptr).priority;
                if left_prio > right_prio {
                    let right_child = self.arena.get(left_ptr).right;
                    let merged = self.merge(right_child, Some(right_ptr));
                    self.arena.get_mut(left_ptr).right = merged;
                    self.pull(left_ptr);
                    Some(left_ptr)
                } else {
                    let left_child = self.arena.get(right_ptr).left;
                    let merged = self.merge(Some(left_ptr), left_child);
                    self.arena.get_mut(right_ptr).left = merged;
                    self.pull(right_ptr);
                    Some(right_ptr)
                }
            }
        }
    }

    fn insert_rec(
        &mut self,
        root: Option<Ptr>,
        key: T,
        priority: u64,
    ) -> (Option<Ptr>, usize, bool) {
        let Some(ptr) = root else {
            let new_ptr = self.arena.alloc(Node::new(key, priority));
            return (Some(new_ptr), 1, true);
        };

        let ord = key.cmp(&self.arena.get(ptr).key);
        match ord {
            Ordering::Equal => {
                let node = self.arena.get_mut(ptr);
                node.count += 1;
                let count = node.count;
                self.pull(ptr);
                (Some(ptr), count, false)
            }
            Ordering::Less => {
                let left = self.arena.get(ptr).left;
                let (new_left, count, inserted_new) = self.insert_rec(left, key, priority);
                self.arena.get_mut(ptr).left = new_left;
                let mut root = ptr;
                if let Some(left_ptr) = new_left {
                    if self.arena.get(left_ptr).priority > self.arena.get(root).priority {
                        root = self.rotate_right(root);
                    } else {
                        self.pull(root);
                    }
                } else {
                    self.pull(root);
                }
                (Some(root), count, inserted_new)
            }
            Ordering::Greater => {
                let right = self.arena.get(ptr).right;
                let (new_right, count, inserted_new) = self.insert_rec(right, key, priority);
                self.arena.get_mut(ptr).right = new_right;
                let mut root = ptr;
                if let Some(right_ptr) = new_right {
                    if self.arena.get(right_ptr).priority > self.arena.get(root).priority {
                        root = self.rotate_left(root);
                    } else {
                        self.pull(root);
                    }
                } else {
                    self.pull(root);
                }
                (Some(root), count, inserted_new)
            }
        }
    }

    fn remove_rec(&mut self, root: Option<Ptr>, key: &T, n: usize) -> (Option<Ptr>, usize, bool) {
        let Some(ptr) = root else {
            return (None, 0, false);
        };

        match key.cmp(&self.arena.get(ptr).key) {
            Ordering::Less => {
                let left = self.arena.get(ptr).left;
                let (new_left, removed, removed_node) = self.remove_rec(left, key, n);
                self.arena.get_mut(ptr).left = new_left;
                if removed > 0 {
                    self.pull(ptr);
                }
                (Some(ptr), removed, removed_node)
            }
            Ordering::Greater => {
                let right = self.arena.get(ptr).right;
                let (new_right, removed, removed_node) = self.remove_rec(right, key, n);
                self.arena.get_mut(ptr).right = new_right;
                if removed > 0 {
                    self.pull(ptr);
                }
                (Some(ptr), removed, removed_node)
            }
            Ordering::Equal => {
                let count = self.arena.get(ptr).count;
                if n < count {
                    self.arena.get_mut(ptr).count = count - n;
                    self.pull(ptr);
                    (Some(ptr), n, false)
                } else {
                    let (left, right) = {
                        let node = self.arena.get(ptr);
                        (node.left, node.right)
                    };
                    (self.merge(left, right), count, true)
                }
            }
        }
    }
}

impl<T: Ord> FromIterator<T> for IndexedSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut res = IndexedSet::new();
        for item in iter {
            res.insert(item);
        }
        res
    }
}

/// An in-order iterator over [`IndexedSet`].
pub struct Iter<'a, T: Ord> {
    arena: &'a Arena<Node<T>>,
    stack: Vec<Ptr>,
    repeat_ptr: Option<Ptr>,
    repeat_remaining: usize,
}

impl<'a, T: Ord> Iter<'a, T> {
    fn push_left_edge(&mut self, mut root: Option<Ptr>) {
        while let Some(ptr) = root {
            self.stack.push(ptr);
            root = self.arena.get(ptr).left;
        }
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ptr) = self.repeat_ptr {
            if self.repeat_remaining > 0 {
                self.repeat_remaining -= 1;
                if self.repeat_remaining == 0 {
                    self.repeat_ptr = None;
                }
                return Some(&self.arena.get(ptr).key);
            }
            self.repeat_ptr = None;
        }

        let ptr = self.stack.pop()?;
        let node = self.arena.get(ptr);
        self.push_left_edge(node.right);
        if node.count > 1 {
            self.repeat_ptr = Some(ptr);
            self.repeat_remaining = node.count - 1;
        }
        Some(&node.key)
    }
}

impl<'a, T: Ord> IntoIterator for &'a IndexedSet<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Ord + Debug> Debug for IndexedSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
