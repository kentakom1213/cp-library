//! modを取る作用付きモノイド

use crate::{
    algebraic_structure::{
        affine1d::{Affine, AffineTransform},
        extmonoid::ExtMonoid,
    },
    number_theory::modint::*,
};

use num_traits::Zero;

/// ## 1次元Affine変換
/// - 区間を $`ax + b`$ で更新（Affine変換）
/// - 区間和を取得
#[derive(Debug)]
pub struct AffineSum<const MOD: usize>;
impl<const MOD: usize> ExtMonoid for AffineSum<MOD> {
    type X = (Modint<MOD>, usize);
    type F = Affine<Modint<MOD>>;
    fn id_x() -> Self::X {
        (Modint::zero(), 0)
    }
    fn id_f() -> Self::F {
        Self::F::id_()
    }
    fn op(x: &Self::X, y: &Self::X) -> Self::X {
        let (xv, xs) = x.clone();
        let (yv, ys) = y.clone();
        (xv + yv, xs + ys)
    }
    fn composition(x: &Self::F, y: &Self::F) -> Self::F {
        y.compose(x)
    }
    fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
        let &(a, b) = y;
        let (val, size) = x.clone();
        (a * val + b * Modint::from(size), size)
    }
}

/// ## 一次関数のupdate + 関数合成
/// - 区間を $`ax + b`$ で更新
/// - 区間を関数として合成
#[derive(Debug)]
pub struct AffineUpdateComposite<const MOD: usize>;
impl<const MOD: usize> ExtMonoid for AffineUpdateComposite<MOD> {
    type X = Affine<Modint<MOD>>;
    type F = Affine<Modint<MOD>>;
    fn id_x() -> Self::X {
        Self::X::id_()
    }
    fn id_f() -> Self::F {
        Self::F::id_()
    }
    fn op(x: &Self::X, y: &Self::X) -> Self::X {
        y.compose(x)
    }
    fn composition(_x: &Self::F, y: &Self::F) -> Self::F {
        *y
    }
    fn mapping(_x: &Self::X, y: &Self::F) -> Self::X {
        *y
    }
}
