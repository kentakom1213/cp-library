//! modを取る作用付きモノイド

use crate::lazy_segment_tree_inner::ExtMonoid;
use crate::modint::*;

/// ## 1次元Affine変換
/// - 区間を`ax + b`で更新（Affine変換）
/// - 区間和を取得
#[derive(Debug)]
pub struct Affine1dMod<const MOD: usize>;
impl<const MOD: usize> ExtMonoid for Affine1dMod<MOD> {
    type X = Modint<MOD>;
    type M = (Modint<MOD>, Modint<MOD>);
    const IX: Self::X = Modint::<MOD>(0);
    const IM: Self::M = (Modint::<MOD>(1), Modint::<MOD>(0));
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
        *x + *y
    }
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
        let &(a1, b1) = x;
        let &(a2, b2) = y;
        //   a2 * (a1 * x + b1) + b2
        // = (a2 * a1) * x + (a2 * b1 + b2)
        (a2 * a1, a2 * b1 + b2)
    }
    fn apply(x: &Self::X, y: &Self::M) -> Self::X {
        let &(a, b) = y;
        a * *x + b
    }
    fn aggregate(x: &Self::M, p: usize) -> Self::M {
        let &(a, b) = x;
        (a, b * p)
    }
}
