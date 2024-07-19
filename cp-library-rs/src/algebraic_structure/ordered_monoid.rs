//! 順序付きモノイド

use super::monoid::Monoid;

/// モノイドに対する順序の実装
pub trait OrderedMonoid: Monoid {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool;
    fn le(left: &Self::Val, right: &Self::Val) -> bool;
}
