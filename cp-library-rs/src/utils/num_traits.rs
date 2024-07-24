//! 数の構造

#[rustfmt::skip] macro_rules! zero_impl { ($t:ty, $v:expr) => { impl Zero for $t { #[inline] fn zero() -> $t { $v } } } }
#[rustfmt::skip] macro_rules! one_impl { ($t:ty, $v:expr) => { impl One for $t { #[inline] fn one() -> $t { $v } } } }
#[rustfmt::skip] macro_rules! bounded_impl { ($t:ty, $min:expr, $max:expr) => { impl Bounded for $t { #[inline] fn min_value() -> $t { $min } #[inline] fn max_value() -> $t { $max } } } }

pub use traits::*;
#[rustfmt::skip]
mod traits {
    use std::ops::{Add, Div, Mul, Rem, Sub};
    pub trait Zero: Sized + Add<Self, Output = Self> { fn zero() -> Self; fn is_zero(&self) -> bool where Self: PartialEq, { self == &Self::zero() } }
    zero_impl!(u32, 0);
    zero_impl!(usize, 0);
    zero_impl!(i32, 0);
    zero_impl!(isize, 0);
    zero_impl!(f32, 0.0);
    zero_impl!(f64, 0.0);
    
    pub trait One: Sized + Mul<Self, Output = Self> { fn one() -> Self; fn is_one(&self) -> bool where Self: PartialEq, { self == &Self::one() } }
    one_impl!(u32, 1);
    one_impl!(usize, 1);
    one_impl!(i32, 1);
    one_impl!(isize, 1);
    one_impl!(f32, 1.0);
    one_impl!(f64, 1.0);
    
    pub trait NumOps<Rhs = Self, Output = Self>: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> { }
    impl NumOps for u32 {}
    impl NumOps for usize {}
    impl NumOps for i32 {}
    impl NumOps for isize {}
    impl NumOps for f32 {}
    impl NumOps for f64 {}

    pub trait Num: PartialEq + Zero + One + NumOps {}
    impl Num for u32 {}
    impl Num for usize {}
    impl Num for i32 {}
    impl Num for isize {}
    impl Num for f32 {}
    impl Num for f64 {}

    pub trait Bounded { fn min_value() -> Self; fn max_value() -> Self; }
    bounded_impl!(u32, std::u32::MIN, std::u32::MAX);
    bounded_impl!(usize, std::usize::MIN, std::usize::MAX);
    bounded_impl!(i32, std::i32::MIN, std::i32::MAX);
    bounded_impl!(isize, std::isize::MIN, std::isize::MAX);
    bounded_impl!(f32, std::f32::MIN, std::f32::MAX);
    bounded_impl!(f64, std::f64::MIN, std::f64::MAX);

    pub trait PrimInt: Num + Bounded + Eq + PartialOrd + Ord + Clone + Copy {}
    impl PrimInt for u32 {}
    impl PrimInt for usize {}
    impl PrimInt for i32 {}
    impl PrimInt for isize {}
}
