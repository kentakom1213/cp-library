//! 幾何ライブラリ

#![allow(dead_code)]

use std::ops::{Add, Sub, Mul, Neg};

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
where T: Copy
    + Add<Output = T>
    + Sub<Output = T>
    + Mul<Output = T>
    + Neg
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
        (self.0 - other.0) * (self.0 - other.0)
        + (self.1 - other.1) * (self.1 - other.1)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_sub_mul_vec2() {
        let a = (2, 3);
        let b = (5, -2);

        assert_eq!(a.add(b), (7, 1));
        assert_eq!(a.sub(b), (-3, 5));

        let a2 = a.mul(-2);
        let b2 = b.mul(3);
        assert_eq!(a2.add(b2), (11, -12));
    }

    #[test]
    fn test_dot() {
        let a = (2.0, -5.0);
        let b = (10.0, 4.0);

        assert_eq!(a.dot(b), 0.0);
    }

    #[test]
    fn test_dist2() {
        let zero = (0, 0);
        let a = (1, 2);
        let b = (2, 1);

        let dist_0_a = a.dist2(zero);
        let dist_0_b = b.dist2(zero);
        assert_eq!(dist_0_a, dist_0_b);

        let dist_a_b = a.dist2(b);
        assert_eq!(dist_a_b, 2);
    }

    #[test]
    fn test_collision_line() {
        let ab: Line<isize> = (
            (3, 1),
            (-3, 1)
        );

        let line1: Line<isize> = (
            (1, 2),
            (2, 2)
        );

        let line2: Line<isize> = (
            (1, 2),
            (1, 0)
        );

        assert_eq!(is_collided(ab, line1), false);
        assert_eq!(is_collided(ab, line2), true);
    }
}
