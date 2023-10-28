//! なもりグラフの分解

#![allow(dead_code)]

use std::collections::VecDeque;

pub type Graph = Vec<Vec<usize>>;

/// # なもりグラフ
/// - なもりグラフ（木に辺を1本加えたグラフ）を分解する
#[derive(Debug)]
pub struct Namori {
    pub N: usize,
    pub graph: Graph,
    pub forest: Graph,
    pub on_cycle: Vec<bool>,
}

impl Namori {
    /// 頂点数`N`のグラフを作成する
    pub fn new(N: usize) -> Self {
        Self {
            N,
            graph: vec![vec![]; N],
            forest: vec![vec![]; N],
            on_cycle: vec![true; N],
        }
    }

    /// 頂点`u`,`v`に辺を張る
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.graph[u].push(v);
        self.graph[v].push(u);
    }

    /// なもりグラフを分解し、
    /// - サイクルを取り除いた森
    /// - サイクル上の頂点
    /// 
    /// を求める
    pub fn decompose(&mut self) {
        // 葉を調べる
        let mut degree = vec![0; self.N];
        let mut leafs = VecDeque::new();
        let mut visited = vec![false; self.N];
        for i in 0..self.N {
            degree[i] = self.graph[i].len(); // 次数を調べる
            // 次数が1の頂点を格納
            if degree[i] == 1 {
                leafs.push_back(i);
                visited[i] = true;
            }
        }
        // 葉を辿って木に分解
        while let Some(u) = leafs.pop_front() {
            self.on_cycle[u] = false;
            for &v in &self.graph[u] {
                if visited[v] {
                    continue;
                }
                degree[v] -= 1;
                // 森に追加
                self.forest[u].push(v);
                self.forest[v].push(u);
                if degree[v] <= 1 {
                    leafs.push_back(v);
                    visited[v] = true;
                }
            }
        }
    }
}
