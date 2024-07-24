//! 行列累乗

use crate::utils::num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct Matrix<const N: usize, T>(pub [[T; N]; N]);

impl<const N: usize, T> Matrix<N, T>
where
    T: One + Zero + Copy,
{
    /// 単位行列を返す
    pub fn id() -> Self {
        let mut res = [[T::zero(); N]; N];
        for i in 0..N {
            res[i][i] = T::one();
        }
        Self(res)
    }

    /// ベクトル`x`と行列`A`について、`Ax`を返す
    pub fn apply(&self, v: &[T; N]) -> [T; N] {
        let mut res = [T::zero(); N];
        for i in 0..N {
            for j in 0..N {
                res[i] = res[i] + self.0[i][j] * v[j];
            }
        }
        res
    }

    /// 行列の累乗を返す（繰り返し2乗法）
    pub fn pow(&self, mut p: usize) -> Self {
        let mut res = Self::id();
        let mut tmp = self.clone();
        while p > 0 {
            if p & 1 == 1 {
                res = tmp.dot(&res);
            }
            tmp = tmp.dot(&tmp);
            p >>= 1;
        }
        res
    }

    /// 行列のドット積
    pub fn dot(&self, other: &Self) -> Self {
        let mut res = [[T::zero(); N]; N];
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    res[i][j] = res[i][j] + self.0[i][k] * other.0[k][j];
                }
            }
        }
        Self(res)
    }
}
