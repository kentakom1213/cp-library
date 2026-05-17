//! Dijkstra's shortest path algorithm.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use num_traits::{Bounded, Zero};

/// Computes single-source shortest paths on a graph with non-negative edge weights.
///
/// The graph is represented as an adjacency list of `(to, weight)` pairs.
///
/// # Returns
///
/// - `prev`: predecessor of each vertex in one shortest path tree
/// - `dist`: shortest distance from `s` to each vertex
///
/// # Complexity
///
/// - $`O((V + E) log V)`$
#[allow(clippy::ptr_arg)]
pub fn dijkstra<T: Copy + Zero + Bounded + Ord>(
    graph: &Vec<Vec<(usize, T)>>,
    s: usize,
) -> (Vec<Option<usize>>, Vec<T>) {
    let n: usize = graph.len();
    let mut prev = vec![None; n];
    let mut dist = vec![T::max_value(); n];
    let mut pq: BinaryHeap<Reverse<(T, usize)>> = BinaryHeap::new();
    // 初期化
    dist[s] = T::zero();
    pq.push(Reverse((dist[s], s)));
    // 更新
    while let Some(Reverse((cost, u))) = pq.pop() {
        if dist[u] < cost {
            continue;
        }
        for &(v, weight) in &graph[u] {
            if dist[v] > dist[u] + weight {
                prev[v] = Some(u);
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    (prev, dist)
}

/// Restores a shortest path from `s` to `t` from the predecessor array.
///
/// Returns `None` if `t` is unreachable from `s`, or if `prev` is inconsistent with `s`
/// being the source.
///
/// # Complexity
///
/// - $`O(length of path)`$
#[allow(clippy::ptr_arg)]
pub fn path_reconstruction(s: usize, t: usize, prev: &Vec<Option<usize>>) -> Option<Vec<usize>> {
    if prev[s].is_some() || prev[t].is_none() {
        return None;
    }
    // 経路復元
    let mut cur = t;
    let mut path = vec![t];
    while cur != s {
        cur = prev[cur]?;
        path.push(cur);
    }
    path.reverse();
    Some(path)
}
