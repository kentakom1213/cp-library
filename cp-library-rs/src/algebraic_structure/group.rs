//! 群の実装

/// 群（モノイド + 逆元）
use crate::algebraic_structure::{monoid::Monoid, ordered_monoid::OrderedMonoid};

pub trait Group: Monoid {
    fn inv(val: &Self::Val) -> Self::Val;
}

pub mod examples {
    use super::*;

    #[derive(Debug)]
    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        fn id() -> Self::Val {
            0
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }
    impl Group for Add {
        fn inv(val: &Self::Val) -> Self::Val {
            -val
        }
    }
    impl OrderedMonoid for Add {
        fn lt(left: &Self::Val, right: &Self::Val) -> bool {
            left < right
        }
        fn le(left: &Self::Val, right: &Self::Val) -> bool {
            left <= right
        }
    }

    #[derive(Debug)]
    pub struct UAdd;
    impl Monoid for UAdd {
        type Val = usize;
        fn id() -> Self::Val {
            0
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left.wrapping_add(*right)
        }
    }
    impl Group for UAdd {
        fn inv(val: &Self::Val) -> Self::Val {
            val.wrapping_neg()
        }
    }
    impl OrderedMonoid for UAdd {
        fn lt(left: &Self::Val, right: &Self::Val) -> bool {
            left < right
        }
        fn le(left: &Self::Val, right: &Self::Val) -> bool {
            left <= right
        }
    }

    #[derive(Debug)]
    pub struct Mul;
    impl Monoid for Mul {
        type Val = isize;
        fn id() -> Self::Val {
            1
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }

    #[derive(Debug)]
    pub struct Xor;
    impl Monoid for Xor {
        type Val = usize;
        fn id() -> Self::Val {
            0
        }
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left ^ right
        }
    }
    impl Group for Xor {
        fn inv(val: &Self::Val) -> Self::Val {
            *val
        }
    }
}
