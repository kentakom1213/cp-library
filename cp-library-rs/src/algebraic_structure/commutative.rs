//! 可環モノイド

use super::monoid::Monoid;

/// 可環モノイド
pub trait CommutativeMonoid: Monoid {}

pub mod examples {
    use std::fmt::Debug;

    use crate::algebraic_structure::monoid::Monoid;

    use super::CommutativeMonoid;

    /// 整数の和
    #[derive(Debug)]
    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        fn id() -> Self::Val {
            0
        }
        fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
            arg1 + arg2
        }
    }
    impl CommutativeMonoid for Add {}

    /// bit単位の排他的論理和
    pub struct Xor;
    impl Monoid for Xor {
        type Val = usize;
        fn id() -> Self::Val {
            0
        }
        fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
            arg1 ^ arg2
        }
    }
    impl CommutativeMonoid for Xor {}

    /// chmin操作
    #[derive(Debug)]
    pub struct Min;
    impl Monoid for Min {
        type Val = isize;
        fn id() -> Self::Val {
            isize::MAX
        }
        fn op(arg1: &Self::Val, arg2: &Self::Val) -> Self::Val {
            *arg1.min(arg2)
        }
    }
    impl CommutativeMonoid for Min {}
}
