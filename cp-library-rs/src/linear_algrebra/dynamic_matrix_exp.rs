//! 行列累乗

use crate::utils::num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    N: usize,
    arr: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: One + Zero + Copy,
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        assert_eq!(data.len(), data[0].len());
        let N = data.len();
        Self { N, arr: data }
    }

    /// 単位行列を返す
    pub fn id(&self) -> Self {
        let mut res = vec![vec![T::zero(); self.N]; self.N];
        for i in 0..self.N {
            res[i][i] = T::one();
        }
        Self {
            N: self.N,
            arr: res,
        }
    }

    /// ベクトル`x`と行列`A`について、`Ax`を返す
    pub fn apply(&self, v: &[T]) -> Vec<T> {
        let mut res = vec![T::zero(); self.N];
        for i in 0..self.N {
            for j in 0..self.N {
                res[i] = res[i] + self.arr[i][j] * v[j];
            }
        }
        res
    }

    /// 行列の累乗を返す（繰り返し2乗法）
    pub fn pow(&self, mut p: usize) -> Self {
        let mut res = self.id();
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
        let mut res = vec![vec![T::zero(); self.N]; self.N];
        for i in 0..self.N {
            for j in 0..self.N {
                for k in 0..self.N {
                    res[i][j] = res[i][j] + self.arr[i][k] * other.arr[k][j];
                }
            }
        }
        Self {
            N: self.N,
            arr: res,
        }
    }
}
