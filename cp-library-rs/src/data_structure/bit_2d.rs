//! 2次元BIT

use crate::algebraic_structure::monoid::Monoid;

macro_rules! cfor {
    ($def:stmt ; $fin:expr ; $incr:stmt ;; $bl:block) => {{
        $def
        while $fin {
            $bl
            $incr
        }
    }}
}

pub struct BIT2D<M: Monoid> {
    pub H: usize,
    pub W: usize,
    pub data: Vec<Vec<M::Val>>,
}

impl<M: Monoid> BIT2D<M> {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    /// 2次元BITを作成する
    pub fn new(H: usize, W: usize) -> Self {
        Self {
            H,
            W,
            data: vec![vec![M::e(); W + 1]; H + 1],
        }
    }

    /// 位置 (r,c) に値 `v` を加算する
    /// - `(r, c)`: 加算を行うインデックス（`0-indexed`）
    /// - `x`: 加算する値
    pub fn add(&mut self, mut r: usize, mut c: usize, v: M::Val) {
        // 0-indexedに修正
        r += 1;
        c += 1;

        cfor! {let mut i = r; i <= self.H; i += Self::lsb(i) ;; {
            cfor! {let mut j = c; j <= self.W; j += Self::lsb(j) ;; {
                self.data[i][j] = M::op(&self.data[i][j], &v);
            }}
        }}
    }

    /// 左上からの和を求める
    /// - `(r,c)`: 領域 `0<=i<r, 0<=j<c` に対しての総和（`0-indexed`）
    pub fn prefix_sum(&self, r: usize, c: usize) -> M::Val {
        let mut res = M::e();

        cfor! {let mut i = r; i > 0; i -= Self::lsb(i) ;; {
            cfor! {let mut j = c; j > 0; j -= Self::lsb(j) ;; {
                res = M::op(&res, &self.data[i][j]);
            }}
        }}

        res
    }
}
