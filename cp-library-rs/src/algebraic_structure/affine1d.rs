//! 1次元Affine変換

use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use crate::{
    algebraic_structure::monoid::Monoid,
    linear_algrebra::matrix_exp::Matrix,
    utils::num_traits::{One, Zero},
};

/// 1次元のAffine変換を表す型
pub type Affine<T> = (T, T);

/// Affine変換の実装
pub trait AffineTransform<T> {
    /// 単位元を返す
    fn id() -> Self;
    /// affine変換をマージする
    /// - `self.compose(rhs)`：`self(rhs(x))`
    fn compose(&self, rhs: &Self) -> Self;
    /// スカラ値xに対し，affine変換を適用する
    fn apply(&self, x: T) -> T;
    /// affine変換を累乗する
    fn pow(&self, p: usize) -> Self;
}

impl<T> AffineTransform<T> for Affine<T>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + One + Copy,
{
    fn id() -> Self {
        (T::one(), T::zero())
    }
    fn compose(&self, rhs: &Self) -> Self {
        let &(a1, b1) = rhs;
        let &(a2, b2) = self;
        //   a2 * (a1 * x + b1) + b2
        // = (a2 * a1) * x + (a2 * b1 + b2)
        (a2 * a1, a2 * b1 + b2)
    }
    fn apply(&self, x: T) -> T {
        let &(a, b) = self;
        a * x + b
    }
    fn pow(&self, mut p: usize) -> Self {
        // 繰り返し2乗法
        let &(a, b) = self;
        let mut tmp = Matrix([[a, b], [T::zero(), T::one()]]);
        let mut res = Matrix::id();
        while p > 0 {
            if p & 1 == 1 {
                res = tmp.dot(&res);
            }
            tmp = tmp.dot(&tmp);
            p >>= 1;
        }
        (res.0[0][0], res.0[0][1])
    }
}

// モノイドの実装
impl<T> Monoid for Affine<T>
where
    T: Clone + Debug,
    Affine<T>: AffineTransform<T>,
{
    type Val = Affine<T>;
    fn id() -> Self::Val {
        AffineTransform::id()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        right.compose(left)
    }
}
