//! modを取るモノイド

use crate::affine1d::AffineTransform;
use crate::modint::*;
use crate::monoid::Monoid;
use num_traits::{One, Zero};

/// あまりをとる和
pub struct ModAdd<const MOD: usize>;
impl<const MOD: usize> Monoid for ModAdd<MOD> {
    type Val = Modint<MOD>;
    fn id() -> Self::Val {
        Self::Val::zero()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        *left + *right
    }
}

/// あまりをとる積
pub struct ModMul<const MOD: usize>;
impl<const MOD: usize> Monoid for ModMul<MOD> {
    type Val = Modint<MOD>;
    fn id() -> Self::Val {
        Self::Val::one()
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        *left * *right
    }
}

/// アフィン変換
pub struct Affine1dMod<const MOD: usize>;
impl<const MOD: usize> Monoid for Affine1dMod<MOD> {
    type Val = (Modint<MOD>, Modint<MOD>);
    fn id() -> Self::Val {
        Self::Val::I
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        right.compose(left)
    }
}
