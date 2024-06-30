//! 行列累乗

use num_traits::{One, Zero};

/* 行列累乗 */
pub const DIM: usize = 4;
pub type V<T> = [T; DIM];
pub type M<T> = [[T; DIM]; DIM];

pub trait MatrixExp<T> {
    /// 単位行列を返す
    fn e() -> Self;
    /// ベクトル`x`と行列`A`について、`Ax`を返す
    fn dot(&self, other: &Self) -> Self;
    /// 行列の累乗を返す（繰り返し2乗法）
    fn pow(&self, e: usize) -> Self;
    /// 行列のドット積
    fn apply(&self, v: &V<T>) -> V<T>;
}

impl<T> MatrixExp<T> for M<T>
where
    T: One + Zero + Copy,
{
    fn e() -> Self {
        let mut res = [[T::zero(); DIM]; DIM];
        for i in 0..DIM {
            res[i][i] = T::one();
        }
        res
    }
    fn apply(&self, v: &V<T>) -> V<T> {
        let mut res = [T::zero(); DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                res[i] = res[i] + self[i][j] * v[j];
            }
        }
        res
    }
    fn pow(&self, mut p: usize) -> Self {
        let mut res = Self::e();
        let mut tmp = *self;
        while p > 0 {
            if p & 1 == 1 {
                res = tmp.dot(&res);
            }
            tmp = tmp.dot(&tmp);
            p >>= 1;
        }
        res
    }
    fn dot(&self, other: &Self) -> Self {
        let mut res = [[T::zero(); DIM]; DIM];
        for i in 0..DIM {
            for j in 0..DIM {
                for k in 0..DIM {
                    res[i][j] = res[i][j] + self[i][k] * other[k][j];
                }
            }
        }
        res
    }
}
