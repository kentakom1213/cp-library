//! modを取る作用付きモノイド

use crate::affine1d::{Affine, AffineTransform, RingId};
use crate::extmonoid::ExtMonoid;
use crate::modint::*;

impl<const MOD: usize> RingId for Modint<MOD> {
    const ZERO: Self = Modint::<MOD>(0);
    const ONE: Self = Modint::<MOD>(1);
}

/// ## 1次元Affine変換
/// - 区間を`ax + b`で更新（Affine変換）
/// - 区間和を取得
#[derive(Debug)]
pub struct AffineSum<const MOD: usize>;
impl<const MOD: usize> ExtMonoid for AffineSum<MOD> {
    type X = Modint<MOD>;
    type F = Affine<Modint<MOD>>;
    fn id_x() -> Self::X {
        Self::X::ZERO
    }
    fn id_f() -> Self::F {
        Self::F::I
    }
    fn op(x: &Self::X, y: &Self::X) -> Self::X {
        *x + *y
    }
    fn composition(x: &Self::F, y: &Self::F) -> Self::F {
        y.compose(x)
    }
    fn mapping(x: &Self::X, y: &Self::F) -> Self::X {
        y.apply(*x)
    }
    fn aggregate(x: &Self::F, p: usize) -> Self::F {
        let &(a, b) = x;
        (a, b * p)
    }
}

/// ## 一次関数のupdate + 関数合成
/// - 区間を`ax + b`で更新
/// - 区間を関数として合成
#[derive(Debug)]
pub struct AffineUpdateComposite<const MOD: usize>;
impl<const MOD: usize> ExtMonoid for AffineUpdateComposite<MOD> {
    type X = Affine<Modint<MOD>>;
    type F = Affine<Modint<MOD>>;
    fn id_x() -> Self::X {
        Self::X::I
    }
    fn id_f() -> Self::F {
        Self::F::I
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
    fn aggregate(x: &Self::F, p: usize) -> Self::F {
        x.pow(p)
    }
}
