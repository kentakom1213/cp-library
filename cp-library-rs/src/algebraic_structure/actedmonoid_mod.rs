//! modを取る作用付きモノイド

use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::algebraic_structure::{
    actedmonoid::ActedMonoid,
    affine1d::{Affine, AffineTransform},
};

use num::One;
use num_traits::Zero;

/// 1次元Affine変換
/// - 区間を $`ax + b`$ で更新（Affine変換）
/// - 区間和を取得
#[derive(Debug)]
pub struct AffineSum<T>(PhantomData<T>);

impl<T> ActedMonoid for AffineSum<T>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + One + Copy + PartialEq + Mul<usize, Output = T>,
{
    type Val = (T, usize);
    type Act = Affine<T>;
    fn e() -> Self::Val {
        (T::zero(), 0)
    }
    fn id() -> Self::Act {
        Self::Act::id_()
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        let (xv, xs) = *x;
        let (yv, ys) = *y;
        (xv + yv, xs + ys)
    }
    fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act {
        y.compose(x)
    }
    fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val {
        let (a, b) = *y;
        let (val, size) = *x;
        (a * val + b * size, size)
    }
}

/// 一次関数のupdate + 関数合成
/// - 区間を $`ax + b`$ で更新
/// - 区間を関数として合成
#[derive(Debug)]
pub struct AffineUpdateComposite<T>(PhantomData<T>);

impl<T> ActedMonoid for AffineUpdateComposite<T>
where
    T: Add<Output = T> + Mul<Output = T> + Zero + One + Copy + PartialEq,
{
    type Val = Affine<T>;
    type Act = Affine<T>;
    fn e() -> Self::Val {
        Self::Val::id_()
    }
    fn id() -> Self::Act {
        Self::Act::id_()
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        y.compose(x)
    }
    fn compose(_x: &Self::Act, y: &Self::Act) -> Self::Act {
        *y
    }
    fn mapping(_x: &Self::Val, y: &Self::Act) -> Self::Val {
        *y
    }
}
