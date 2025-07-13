//! `usize` への `pow` メソッドの実装

/// `usize` に `pow` メソッドの実装
pub trait PowUsize {
    fn pow(&self, exp: Self) -> Self;
}

impl PowUsize for usize {
    fn pow(&self, exp: Self) -> Self {
        (*self as u64).pow(exp as u32) as usize
    }
}
