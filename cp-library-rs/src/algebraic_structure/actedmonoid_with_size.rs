//! 長さを保持した作用付きモノイド

use std::ops::{Add, Mul};

use num::{Bounded, FromPrimitive, Zero};

use crate::algebraic_structure::{
    actedmonoid::{
        examples::{AddMin, AddSum, UpdateMax, UpdateMin, UpdateMinMax, UpdateSum},
        ActedMonoid,
    },
    monoid::Monoid,
    to_acted::ToActed,
};

/// 長さを保持した作用付きモノイド
pub trait ActedMonoidWithSize: ActedMonoid {
    /// 区間長 `size` の「単位区間」を表す集約値を返す
    ///
    /// 例：`Val = (sum, size)` のとき，`e_with_size(size) = (0, size)` のようなもの
    #[inline]
    fn e_with_size(_size: usize) -> Self::Val {
        Self::e()
    }
}

// ========== 区間加算，区間更新 + 区間和系の作用付きモノイドに実装 ==========
impl<T> ActedMonoidWithSize for AddSum<T>
where
    T: Zero + Clone + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialEq,
{
    fn e_with_size(size: usize) -> Self::Val {
        (T::zero(), size)
    }
}

impl<T> ActedMonoidWithSize for UpdateSum<T>
where
    T: Zero + Clone + Add<Output = T> + Mul<Output = T> + FromPrimitive + PartialEq,
{
    fn e_with_size(size: usize) -> Self::Val {
        (T::zero(), size)
    }
}

// ========== その他の作用付きモノイドに実装 ==========
impl<M> ActedMonoidWithSize for ToActed<M>
where
    M: Monoid,
    M::Val: PartialEq,
{
}
impl<T: Bounded + Ord + Clone> ActedMonoidWithSize for UpdateMin<T> {}
impl<T: Bounded + Ord + Clone> ActedMonoidWithSize for UpdateMax<T> {}
impl<T: Bounded + Ord + Clone> ActedMonoidWithSize for UpdateMinMax<T> {}
impl<T> ActedMonoidWithSize for AddMin<T> where T: Zero + Clone + Add<Output = T> + Ord + Bounded {}
