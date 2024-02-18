use crate::modint::*;
use crate::segment_tree::Monoid;

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
