//! 行列累乗

use crate::number_theory::num_traits::{One, Zero};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub R: usize,
    pub C: usize,
    pub arr: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: One + Zero + Copy,
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let R = data.len();
        let C = data[0].len();
        Self { R, C, arr: data }
    }

    /// 単位行列を返す
    pub fn id(&self) -> Self {
        let mut res = vec![vec![T::zero(); self.C]; self.R];
        for i in 0..self.R {
            res[i][i] = T::one();
        }
        Self {
            R: self.R,
            C: self.C,
            arr: res,
        }
    }

    /// ベクトル`x`と行列`A`について、`Ax`を返す
    pub fn apply(&self, v: &[T]) -> Vec<T> {
        assert_eq!(v.len(), self.C);
        let mut res = vec![T::zero(); self.R];
        for i in 0..self.R {
            for j in 0..self.R {
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

    /// 行列`A`,`B`のドット積`AB`を求める
    pub fn dot(&self, other: &Self) -> Self {
        assert_eq!(self.C, other.R);
        let mut res = vec![vec![T::zero(); other.C]; self.R];
        for i in 0..self.R {
            for j in 0..other.C {
                for k in 0..self.C {
                    res[i][j] = res[i][j] + self.arr[i][k] * other.arr[k][j];
                }
            }
        }
        Self {
            R: self.R,
            C: other.C,
            arr: res,
        }
    }
}
