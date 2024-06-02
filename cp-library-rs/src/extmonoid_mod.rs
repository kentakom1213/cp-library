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
    type M = Affine<Modint<MOD>>;
    const IX: Self::X = Self::X::ZERO;
    const IM: Self::M = Self::M::I;
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
        *x + *y
    }
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
        y.compose(x)
    }
    fn apply(x: &Self::X, y: &Self::M) -> Self::X {
        y.apply(*x)
    }
    fn aggregate(x: &Self::M, p: usize) -> Self::M {
        let &(a, b) = x;
        (a, b * p)
    }
}
