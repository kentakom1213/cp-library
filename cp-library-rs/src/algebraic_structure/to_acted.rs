//! モノイドを作用付きモノイドに変換

use crate::algebraic_structure::{actedmonoid::ActedMonoid, monoid::Monoid};

/// モノイドを作用付きモノイドに変換する
pub struct ToActed<M>(M)
where
    M: Monoid,
    M::Val: PartialEq;

impl<M> ActedMonoid for ToActed<M>
where
    M: Monoid,
    M::Val: PartialEq,
{
    type Val = M::Val;
    type Act = ();
    fn e() -> Self::Val {
        M::e()
    }
    fn id() -> Self::Act {
        ()
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        M::op(x, y)
    }
    fn compose(_: &Self::Act, _: &Self::Act) -> Self::Act {
        ()
    }
    fn mapping(x: &Self::Val, _: &Self::Act) -> Self::Val {
        x.clone()
    }
}
