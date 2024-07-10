//! 行列累乗

use num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct Matrix<const D: usize, T>([[T; D]; D]);

impl<const D: usize, T> Matrix<D, T>
where
    T: One + Zero + Copy,
{
    pub fn new(data: [[T; D]; D]) -> Self {
        Self(data)
    }

    /// 単位行列を返す
    pub fn id() -> Self {
        let mut res = [[T::zero(); D]; D];
        for i in 0..D {
            res[i][i] = T::one();
        }
        Self(res)
    }

    /// ベクトル`x`と行列`A`について、`Ax`を返す
    pub fn apply(&self, v: &[T; D]) -> [T; D] {
        let mut res = [T::zero(); D];
        for i in 0..D {
            for j in 0..D {
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
        let mut res = [[T::zero(); D]; D];
        for i in 0..D {
            for j in 0..D {
                for k in 0..D {
                    res[i][j] = res[i][j] + self.0[i][k] * other.0[k][j];
                }
            }
        }
        Self(res)
    }
}
