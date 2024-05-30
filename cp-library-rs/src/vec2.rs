//! 幾何ライブラリ

use std::ops::{Add, Mul, Neg, Sub};

pub type Pos<T> = (T, T);
pub type Line<T> = (Pos<T>, Pos<T>);

/// ベクトル演算を行う
pub trait Vec2<T> {
    fn mul(&self, scalar: T) -> Self;
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
    /// ドット積
    fn dot(&self, other: Self) -> T;
    /// クロス積
    fn cross(&self, other: Self) -> T;
    /// L2-ノルム（の2乗）
    fn dist2(&self, other: Self) -> T;
}

impl<T> Vec2<T> for Pos<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Neg,
{
    fn mul(&self, scalar: T) -> Self {
        (self.0 * scalar, self.1 * scalar)
    }
    fn add(&self, other: Self) -> Self {
        (self.0 + other.0, self.1 + other.1)
    }
    fn sub(&self, other: Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }
    fn dot(&self, other: Self) -> T {
        self.0 * other.0 + self.1 * other.1
    }
    fn cross(&self, other: Self) -> T {
        (self.0 * other.1) - (other.0 * self.1)
    }
    fn dist2(&self, other: Self) -> T {
        (self.0 - other.0) * (self.0 - other.0) + (self.1 - other.1) * (self.1 - other.1)
    }
}

/// 線分abと線分xyが衝突しているかどうか
pub fn is_collided(ab: Line<isize>, xy: Line<isize>) -> bool {
    let (a, b) = ab;
    let (x, y) = xy;

    // Aから見たとき
    let AX = x.sub(a);
    let AY = y.sub(a);
    let AB = b.sub(a);

    // Xから見たとき
    let XA = a.sub(x);
    let XB = b.sub(x);
    let XY = y.sub(x);

    AB.cross(AX) * AB.cross(AY) < 0 && XY.cross(XA) * XY.cross(XB) < 0
}
