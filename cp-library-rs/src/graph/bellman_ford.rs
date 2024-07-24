//! ベルマン・フォード法

use crate::utils::consts::{IINF, INF};

/// ベルマン・フォード法
/// - 重み付きグラフの単一始点最短路を求める
/// - 重みが負の場合にも対応
/// - 各頂点`v`の距離`dist[v]`について
///   - vに到達不可能な場合     → [`IINF`]
///   - vへの最短路が求まる場合 → `(vへの最短路長)`
///   - vへの最短路がいくらでも小さくできる場合
///                            → `-IINF`
///
/// 引数
/// - `N` : 頂点数
/// - `start` : 始点
/// - `edges` : 有向辺 `(in, out, weight)` の集合
///
/// 戻り値
/// - `has_negative` : (グラフ全体で)負閉路を含むか
/// - `dist` : 各頂点への最短路
/// - `prev` : 各頂点の最短路について，その頂点の直前に経由した頂点
///
/// 計算量
/// - `$O(NM)$`
pub fn bellman_ford(
    N: usize,
    start: usize,
    edges: &Vec<(usize, usize, isize)>,
) -> (bool, Vec<isize>, Vec<usize>) {
    let mut dist = vec![2 * IINF; N];
    dist[start] = 0;
    let mut prev = vec![INF; N];

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
            if dist[u] < IINF {
                dist[v] = -IINF;
            }
        }
    }

    // 到達できない頂点をINFに
    for u in 0..N {
        if dist[u] > IINF {
            dist[u] = IINF;
        }
    }

    // 影響範囲を調べる（もう一度ベルマンフォード）
    for _ in 0..N {
        for &(u, v, w) in edges {
            if dist[u] == IINF || dist[v] == IINF {
                continue;
            }
            // 緩和
            if dist[v] > dist[u] + w {
                dist[v] = -IINF;
            }
        }
    }

    (has_negative, dist, prev)
}
