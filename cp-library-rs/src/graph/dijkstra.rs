//! ダイクストラ法

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::utils::num_traits::{Bounded, Zero};

/// Dijkstra法
/// - グラフ`graph`が与えられたとき、スタート地点`s`から各頂点への最短路を求める
///
/// 戻り値
/// - `prev`: 各頂点への最短路において、その頂点の直前の頂点
/// - `dist`: スタート地点`s`から各頂点への最短距離
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

/// 経路復元
/// - スタート地点`s`からゴール地点`t`までの最短路を復元する
///
/// 戻り値
/// - `Some(path)`: スタート地点`s`からゴール地点`t`までの最短路
/// - `None`: sがダイクストラ法のスタート地点でない場合 | 最短路が存在しない場合
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
