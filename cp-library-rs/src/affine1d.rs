//! 1次元Affine変換

use std::ops::{Add, Mul};

pub trait RingId {
    const ZERO: Self;
    const ONE: Self;
}

impl RingId for usize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl RingId for isize {
    const ZERO: Self = 0;
    const ONE: Self = 1;
}

impl RingId for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

/// Affine変換を表す型
pub type Affine<T> = (T, T);

pub trait AffineTransform<T> {
    const I: Self;
    /// affine変換をマージする
    ///
    /// - `self.compose(rhs)`：`self(rhs(x))`
    fn compose(&self, rhs: &Self) -> Self;
    /// スカラ値xに対し，affine変換を適用する
    fn apply(&self, x: T) -> T;
    /// affine変換を累乗する
    fn pow(&self, p: usize) -> Self;
}

impl<T> AffineTransform<T> for Affine<T>
where
    T: Add<Output = T> + Mul<Output = T> + RingId + Copy,
{
    const I: Self = (T::ONE, T::ZERO);
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
        let mut tmp = [[a, b], [T::ZERO, T::ONE]];
        let mut res = [[T::ONE, T::ZERO], [T::ZERO, T::ONE]];
        while p > 0 {
            if p & 1 == 1 {
                res = dot(&tmp, &res);
            }
            tmp = dot(&tmp, &tmp);
            p >>= 1;
        }
        (res[0][0], res[0][1])
    }
}

type M2x2<T> = [[T; 2]; 2];

fn dot<T>(x: &M2x2<T>, y: &M2x2<T>) -> M2x2<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    let &[[x11, x12], [x21, x22]] = x;
    let &[[y11, y12], [y21, y22]] = y;
    [
        [x11 * y11 + x12 * y21, x11 * y12 + x12 * y22],
        [x21 * y11 + x22 * y21, x21 * y12 + x22 * y22],
    ]
}
