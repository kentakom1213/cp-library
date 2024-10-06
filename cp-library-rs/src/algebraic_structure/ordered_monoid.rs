//! モノイドに対する順序の実装

use super::{monoid::Monoid, operation::*};

/// 順序付きモノイド
pub trait OrderedMonoid: Monoid {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool;
    fn le(left: &Self::Val, right: &Self::Val) -> bool;
}

// 実装
impl OrderedMonoid for Add<isize> {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool {
        left < right
    }
    fn le(left: &Self::Val, right: &Self::Val) -> bool {
        left <= right
    }
}

impl OrderedMonoid for Add<usize> {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool {
        left < right
    }
    fn le(left: &Self::Val, right: &Self::Val) -> bool {
        left <= right
    }
}
