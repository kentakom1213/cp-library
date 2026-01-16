//! modを取る作用付きモノイド

use crate::{
    algebraic_structure::{
        actedmonoid::ActedMonoid,
        affine1d::{Affine, AffineTransform},
    },
    number_theory::modint::*,
};

use num_traits::Zero;

/// 1次元Affine変換
/// - 区間を $`ax + b`$ で更新（Affine変換）
/// - 区間和を取得
#[derive(Debug)]
pub struct AffineSum<const MOD: usize>;
impl<const MOD: usize> ActedMonoid for AffineSum<MOD> {
    type Val = (Modint<MOD>, usize);
    type Act = Affine<Modint<MOD>>;
    fn e() -> Self::Val {
        (Modint::zero(), 0)
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
        (a * val + b * Modint::from(size), size)
    }
}

/// 一次関数のupdate + 関数合成
/// - 区間を $`ax + b`$ で更新
/// - 区間を関数として合成
#[derive(Debug)]
pub struct AffineUpdateComposite<const MOD: usize>;
impl<const MOD: usize> ActedMonoid for AffineUpdateComposite<MOD> {
    type Val = Affine<Modint<MOD>>;
    type Act = Affine<Modint<MOD>>;
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
