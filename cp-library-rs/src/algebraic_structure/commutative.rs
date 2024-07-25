//! 可環モノイド

use std::fmt::Debug;

use crate::utils::num_traits::Bounded;

use super::monoid::{examples::*, Monoid};

/// 可環モノイド
pub trait CommutativeMonoid: Monoid {}

// 実装
impl CommutativeMonoid for Add<isize> {}
impl CommutativeMonoid for Add<usize> {}
impl CommutativeMonoid for Xor {}
impl<T: Ord + Bounded + Clone + Debug> CommutativeMonoid for Min<T> {}
impl<T: Ord + Bounded + Clone + Debug> CommutativeMonoid for Max<T> {}
impl CommutativeMonoid for GCD {}
