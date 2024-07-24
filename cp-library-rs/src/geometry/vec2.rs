//! 幾何ライブラリ

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::num_traits::{Num, PrimInt};

/// 2次元ベクトル
#[derive(Debug, Hash, Clone, Copy)]
pub struct Vec2<T>(pub T, pub T);

impl<T: PrimInt> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        let Self(ax, ay) = self;
        let Self(bx, by) = &other;
        ax == bx && ay == by
    }
}

impl<T: PrimInt> Eq for Vec2<T> {}

impl<T> Add for Vec2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Sub for Vec2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs.clone(), self.1 * rhs)
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Div<Output = T> + Clone,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs.clone(), self.1 / rhs)
    }
}

impl<T: Num + Copy> Vec2<T> {
    /// ドット積
    ///
    /// ```math
    /// \boldsymbol{a}\cdot\boldsymbol{b} = a_x b_x + a_y b_y
    /// ```
    pub fn dot(&self, other: Self) -> T {
        let Self(ax, ay) = *self;
        let Self(bx, by) = other;
        ax * bx + ay * by
    }

    /// ノルムの2乗
    ///
    /// ```math
    /// \|\boldsymbol{a}\|
    /// ```
    pub fn norm2(&self) -> T {
        self.dot(*self)
    }

    /// ベクトル同士の距離の2乗
    ///
    /// ```math
    /// \|\boldsymbol{a} - \boldsymbol{b}\|^2 = (a_x - b_x)^2 + (a_y - b_y)^2
    /// ```
    pub fn dist2(&self, other: Self) -> T {
        let diff = *self - other;
        diff.norm2()
    }

    /// クロス積
    ///
    /// 2次元ベクトルのクロス積はスカラー
    ///
    /// ```math
    /// \boldsymbol{a} \times \boldsymbol{b} = a_x b_y - a_y b_x
    /// ```
    pub fn cross(&self, other: Self) -> T {
        let Self(ax, ay) = *self;
        let Self(bx, by) = other;
        ax * by - ay * bx
    }
}
