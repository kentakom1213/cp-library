//! modを取るモノイド

use crate::modint::*;
use crate::segment_tree_inner::Monoid;

/// あまりをとる和
pub struct ModAdd<const MOD: usize>;
impl<const MOD: usize> Monoid for ModAdd<MOD> {
    type Val = Modint<MOD>;
    const E: Self::Val = Modint::<MOD>(0);
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        *left + *right
    }
}

/// あまりをとる積
pub struct ModMul<const MOD: usize>;
impl<const MOD: usize> Monoid for ModMul<MOD> {
    type Val = Modint<MOD>;
    const E: Self::Val = Modint::<MOD>(1);
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        *left * *right
    }
}

/// アフィン変換
pub struct Affine<const MOD: usize>;
impl<const MOD: usize> Monoid for Affine<MOD> {
    type Val = (Modint<MOD>, Modint<MOD>);
    const E: Self::Val = (Modint::<MOD>(1), Modint::<MOD>(0));
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        let &(a1, b1) = left;
        let &(a2, b2) = right;
        (a2 * a1, a2 * b1 + b2)
    }
}
