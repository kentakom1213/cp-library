//! 可環モノイド

use super::monoid::{examples::*, Monoid};

/// 可環モノイド
pub trait CommutativeMonoid: Monoid {}

// 実装
impl CommutativeMonoid for Add<isize> {}
impl CommutativeMonoid for Add<usize> {}
impl CommutativeMonoid for Xor {}
impl CommutativeMonoid for Min {}
impl CommutativeMonoid for Max {}
impl CommutativeMonoid for GCD {}
