#[allow(dead_code)]

type Graph = Vec<Vec<usize>>;

/// # Monoid
pub trait Monoid {
    type Val: Clone + PartialEq;
    const E: Self::Val;
    fn op(u: &Self::Val, v: &Self::Val) -> Self::Val;
    fn apply(val: &Self::Val) -> Self::Val;
    fn modulo(val: &Self::Val, M: usize) -> Self::Val;
}

/// # 木DP（任意mod）
struct TreeDPMod<T: Monoid> {
    pub N: usize,
    pub G: Vec<Vec<usize>>,
    dp: Vec<T::Val>,
    M: usize,
}

impl<T: Monoid> TreeDPMod<T> {    
    pub fn new(N: usize, M: usize) -> Self {
        Self {
            N,
            G: vec![vec![]; N],
            dp: vec![T::E; N],
            M,
        }
    }

    /// 辺`u`-`v`を追加する
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.G[u].push(v);
        self.G[v].push(u);
    }

    /// 頂点`start`に値を集約する
    pub fn aggregate(&mut self, start: usize) -> T::Val {
        let NEG1 = 1_usize.wrapping_neg();
        Self::dfs(
            NEG1,
            start,
            &self.G,
            &mut self.dp,
            self.M,
        );

        self.dp[start].clone()
    }

    fn dfs(p: usize, u: usize, G: &Graph, dp: &mut Vec<T::Val>, M: usize) {
        // 葉であるときの処理
        if G[u].len() == 1 && G[u][0] == p {
            dp[u] = T::E;
            return;
        }

        // 子要素を集約する
        let mut acc = T::E; // 子要素の累積
        for &v in &G[u] {
            if v == p {
                continue;
            }
            Self::dfs(u, v, G, dp, M);
            acc = T::op(
                &acc,
                &dp[v]
            );
            acc = T::modulo(&acc, M);
        }
        dp[u] = T::apply(&acc);
    }
}
