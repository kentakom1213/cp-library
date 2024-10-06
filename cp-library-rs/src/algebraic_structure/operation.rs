use std::{fmt::Debug, marker::PhantomData, ops::Rem};

use crate::utils::num_traits::Zero;

/// 区間最小値
#[derive(Debug)]
pub struct Min<T>(PhantomData<T>);

/// 最大値
#[derive(Debug, Clone)]
pub struct Max<T>(PhantomData<T>);

/// 和
#[derive(Debug, Clone)]
pub struct Add<T>(PhantomData<T>);

/// 積
#[derive(Debug, Clone)]
pub struct Mul<T>(PhantomData<T>);

/// bit単位の排他的論理和
#[derive(Debug, Clone)]
pub struct Xor;

/// 最大公約数
#[derive(Debug)]
pub struct GCD<T>(PhantomData<T>);
impl<T: Clone + PartialEq + Zero + Rem<T, Output = T>> GCD<T> {
    /// `a`,`b`の最大公約数を求める
    pub fn gcd(a: &T, b: &T) -> T {
        if b.is_zero() {
            a.clone()
        } else {
            Self::gcd(b, &a.clone().rem(b.clone()))
        }
    }
}
