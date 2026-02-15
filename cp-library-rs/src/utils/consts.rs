//! 定数

/// usizeにおける`-1`の値
pub const NEG1: usize = 1_usize.wrapping_neg();

/// 競プロ用の最大値
pub trait Infinity {
    fn infinity() -> Self;
}

macro_rules! impl_infinity {
    ($t:ty, $v:expr) => {
        impl Infinity for $t {
            fn infinity() -> $t {
                $v
            }
        }
    };
}

impl_infinity!(usize, 1001001001);
impl_infinity!(isize, 1001001001);
impl_infinity!(u64, 1001001001001001001);
impl_infinity!(i64, 1001001001001001001);
impl_infinity!(u32, 1001001001);
impl_infinity!(i32, 1001001001);
impl_infinity!(f64, 1e20);
