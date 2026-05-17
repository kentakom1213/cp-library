//! Bellman-Ford shortest path algorithm.

use crate::utils::consts::Infinity;

/// Computes single-source shortest paths, allowing negative edge weights.
///
/// Distances are encoded as follows:
///
/// - unreachable: [`i64::infinity()`]
/// - finite shortest path: the shortest distance
/// - affected by a reachable negative cycle: `-i64::infinity()`
///
/// # Returns
///
/// - `has_negative`: whether the graph contains any negative cycle
/// - `dist`: distance from `start` to each vertex
/// - `prev`: predecessor array for relaxed shortest paths
///
/// # Complexity
///
/// - $`O(NM)`$
pub fn bellman_ford(
    N: usize,
    start: usize,
    edges: &Vec<(usize, usize, i64)>,
) -> (bool, Vec<i64>, Vec<usize>) {
    let mut dist = vec![2 * i64::infinity(); N];
    dist[start] = 0;
    let mut prev = vec![usize::infinity(); N];

    for _ in 0..N {
        for &(u, v, w) in edges {
            // 緩和
            if dist[v] > dist[u] + w {
                dist[v] = dist[u] + w;
                prev[v] = u;
            }
        }
    }

    // 負閉路検出
    let mut has_negative = false;

    for &(u, v, w) in edges {
        if dist[v] > dist[u] + w {
            has_negative = true;
            // 始点から到達できる場合
            if dist[u] < i64::infinity() {
                dist[v] = -i64::infinity();
            }
        }
    }

    // 到達できない頂点をINFに
    dist.iter_mut().for_each(|d| {
        if *d > i64::infinity() {
            *d = i64::infinity();
        }
    });

    // 影響範囲を調べる（もう一度ベルマンフォード）
    for _ in 0..N {
        for &(u, v, w) in edges {
            if dist[u] == i64::infinity() || dist[v] == i64::infinity() {
                continue;
            }
            // 緩和
            if dist[v] > dist[u] + w {
                dist[v] = -i64::infinity();
            }
        }
    }

    (has_negative, dist, prev)
}
