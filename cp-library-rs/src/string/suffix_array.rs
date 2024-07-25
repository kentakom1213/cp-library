//! 接尾辞配列（Manber-Myers法）

macro_rules! cfor {
    ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
        $def
        while $fin {
            $bl
            $incr
        }
    }}
}

/// SuffixArray
pub struct SuffixArray;

impl SuffixArray {
    /// (rank[i], rank[i+k]) により比較する
    fn ord(rank: &[isize], i: usize, k: usize) -> (isize, isize) {
        (rank[i], *rank.get(i + k).unwrap_or(&-1))
    }

    /// 文字列Sの接尾辞配列を構築し，返す
    /// - 計算量： $`O(|S| \log^2 |S|)`$
    pub fn build(S: &str) -> Vec<usize> {
        let N = S.len();
        let mut rank: Vec<_> = S.chars().map(|c| c as isize).collect();
        // 末尾（空文字列）
        rank.push(-1);
        let mut sa: Vec<_> = (0..=N).collect();

        cfor! {let mut k = 1; k <= N; k *= 2;; {
            // rankでソート
            sa.sort_by_key(|&i| Self::ord(&rank, i, k));
            // rankの更新
            let mut tmp = vec![0; N + 1];
            tmp[sa[0]] = 0;
            for i in 1..=N {
                tmp[sa[i]] = tmp[sa[i - 1]] +
                    (Self::ord(&rank, sa[i - 1], k) < Self::ord(&rank, sa[i], k)) as isize;
            }
            rank = tmp;
        }}
        sa
    }
}
