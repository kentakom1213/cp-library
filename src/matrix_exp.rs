//! 行列累乗

// 定数
pub const MOD: usize = 998244353;

/* 行列累乗 */
pub const DIM: usize = 4;
pub type Vec = [usize; DIM];
pub type Matrix = [[usize; DIM]; DIM];

pub trait MatrixExp {
    /// ## e
    /// 単位行列を返す
    fn e() -> Self;
    /// ## apply
    /// ベクトル`x`と行列`A`について、`Ax`を返す
    fn dot(&self, other: Self) -> Self;
    /// ## pow
    /// 行列の累乗を返す（繰り返し2乗法）
    fn pow(&self, e: usize) -> Self;
    /// ## dot
    /// 行列のドット積
    fn apply(&self, vec: Vec) -> Vec;
}

impl MatrixExp for Matrix {
    fn e() -> Self {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            res[i][i] = 1;
        }
        res
    }
    fn apply(&self, vec: Vec) -> Vec {
        let mut res = [0; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                res[i] += self[i][j] * vec[j] % MOD;
                res[i] %= MOD;
            }
        }
        res
    }
    fn pow(&self, mut e: usize) -> Self {
        let mut res = Self::e();
        let mut tmp = *self;
        while e > 0 {
            if e & 1 == 1 {
                res = tmp.dot(res);
            }
            tmp = tmp.dot(tmp);
            e >>= 1;
        }
        res
    }
    fn dot(&self, other: Self) -> Self {
        let mut res = [[0; DIM]; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                for k in 0..DIM {
                    res[i][j] += self[i][k] * other[k][j] % MOD;
                    res[i][j] %= MOD;
                }
            }
        }
        res
    }
}
