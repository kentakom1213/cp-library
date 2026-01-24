//! ## モノイド
//!
//! - [`Monoid::Val`] ： データの型 $`S`$
//! - [`Monoid::e`] ： 単位元を返す関数 $`\varnothing \to S`$
//! - [`Monoid::op`] ： 演算 $`S\times S \to S`$

/// モノイド
///
/// - [`Monoid::Val`] ： データの型 $`S`$
/// - [`Monoid::e`] ： 単位元を返す関数 $`\varnothing \to S`$
/// - [`Monoid::op`] ： 演算 $`S\times S \to S`$
pub trait MonoidCtx {
    /// データの型 （$`S`$）
    type Val: Clone;
    /// 単位元 （$`\varnothing \to S`$）
    fn e(&self) -> Self::Val;
    /// 演算 （$`S \times S \to S`$）
    fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val;
}

// ========== 実装 ==========

pub mod examples {
    use std::ops::{Add, Mul, Rem};

    use num::{One, Zero};

    use crate::algebraic_structure::monoid_with_context::MonoidCtx;

    /// 法が与えられる区間和
    pub struct AddMod<T>(pub T);

    impl<T> MonoidCtx for AddMod<T>
    where
        T: Clone + Zero + Add + Rem<Output = T>,
    {
        type Val = T;
        fn e(&self) -> Self::Val {
            T::zero()
        }
        fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val {
            (left.clone() + right.clone()) % self.0.clone()
        }
    }

    /// 法が与えられる区間積
    pub struct MulMod<T>(pub T);

    impl<T> MonoidCtx for MulMod<T>
    where
        T: Clone + One + Mul + Rem<Output = T>,
    {
        type Val = T;
        fn e(&self) -> Self::Val {
            T::one()
        }
        fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val {
            (left.clone() * right.clone()) % self.0.clone()
        }
    }
}
