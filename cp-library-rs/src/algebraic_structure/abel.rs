//! ## アーベル群（可換群）

use crate::algebraic_structure::{commutative::CommutativeMonoid, group::Group, operation::*};

/// アーベル群
///
/// 可換な群
pub trait Abel: CommutativeMonoid + Group {}

// 実装
impl Abel for Add<isize> {}
impl Abel for Add<usize> {}
impl Abel for Xor {}
