use num_traits::PrimInt;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub struct Edge<T> {
    /// 逆向きの辺の番号
    rev: usize,
    from: usize,
    to: usize,
    cap: T,
    original_cap: T,
}

impl<T: Clone> Edge<T> {
    pub fn new(rev: usize, from: usize, to: usize, cap: T) -> Self {
        Self {
            rev,
            from,
            to,
            cap: cap.clone(),
            original_cap: cap,
        }
    }
}

type Graph<T> = Vec<Vec<Edge<T>>>;

/// FordFulkerson法を実行する
#[derive(Debug)]
pub struct FordFulkerson<T> {
    N: usize,
    /// 残余ネットワーク
    pub G: Graph<T>,
    /// 辺の番号
    edge_id: FxHashMap<(usize, usize), usize>,
}

impl<T: PrimInt> FordFulkerson<T> {
    pub fn new(n: usize) -> Self {
        Self {
            N: n,
            G: vec![vec![]; n],
            edge_id: FxHashMap::default(),
        }
    }

    /// 有向辺を追加する
    pub fn add_edge(&mut self, from: usize, to: usize, cap: T) {
        let rev = self.G[to].len();
        self.G[from].push(Edge::new(rev, from, to, cap));
        // 逆向き辺を追加
        let rrev = self.G[from].len() - 1;
        self.G[to].push(Edge::new(rrev, to, from, T::zero()));
        // 辺のidを追加
        self.edge_id.insert((from, to), rrev);
    }

    /// uからtへのフロー追加路を見つける
    fn dfs(u: usize, t: usize, f: T, G: &mut Graph<T>, visit: &mut [bool]) -> T {
        if u == t {
            return f;
        }
        visit[u] = true;
        for i in 0..G[u].len() {
            let v = G[u][i].to;
            let cap = G[u][i].cap;
            if visit[v] || cap <= T::zero() {
                continue;
            }
            let d = Self::dfs(v, t, f.min(cap), G, visit);
            if d > T::zero() {
                // 前向き辺の更新
                G[u][i].cap = cap - d;
                // 後ろ向き辺の更新
                let r = G[u][i].rev;
                G[v][r].cap = G[v][r].cap + d;

                return d;
            }
        }
        T::zero()
    }

    /// 最大流を求める
    pub fn max_flow(&mut self, s: usize, t: usize) -> T {
        let mut flow = T::zero();
        let mut visit = vec![false; self.N];
        loop {
            visit.fill(false);
            // フロー追加路を見つける
            let f = Self::dfs(s, t, T::max_value(), &mut self.G, &mut visit);
            if f.is_zero() {
                break;
            }
            flow = flow + f;
        }
        flow
    }

    /// 現在の状態での辺(u,v)の流量を求める
    pub fn get_flow(&self, u: usize, v: usize) -> T {
        let i = self.edge_id[&(u, v)];
        let cap = self.G[u][i].cap;
        let original = self.G[u][i].original_cap;
        original - cap
    }
}
