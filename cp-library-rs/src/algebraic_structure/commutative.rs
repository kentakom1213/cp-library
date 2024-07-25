//! 可環モノイド

use std::fmt::Debug;

use crate::utils::num_traits::Bounded;

use super::monoid::{examples::*, Monoid};

/// 可環モノイド
///
/// 任意の要素 $`x,y\in S`$ に対し，
///
/// ```math
/// x \times y = y \times x
/// ```
///
/// が成立する．
pub trait CommutativeMonoid: Monoid {}

// 実装
impl CommutativeMonoid for Add<isize> {}
impl CommutativeMonoid for Add<usize> {}
impl CommutativeMonoid for Xor {}
impl<T: Ord + Bounded + Clone + Debug> CommutativeMonoid for Min<T> {}
impl<T: Ord + Bounded + Clone + Debug> CommutativeMonoid for Max<T> {}
impl CommutativeMonoid for GCD {}
