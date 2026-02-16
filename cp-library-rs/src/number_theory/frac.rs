//! 比較を実装した分数の実装

use std::cmp::Ordering;

use num::Integer;
use num_integer::gcd;

/// 分数を表す構造体
/// - `Frac(a, b)` := a / b
#[derive(Debug, Clone, Copy)]
pub struct Frac<T: Integer>(pub T, pub T);

impl<T: Integer + Copy> Frac<T> {
    pub fn new(a: T, b: T) -> Self {
        let c = gcd(a, b);
        Self(a / c, b / c)
    }
}

impl<T> PartialEq for Frac<T>
where
    T: Integer + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        a1 * b2 == a2 * b1
    }
}

impl<T> Eq for Frac<T>
where
    T: Integer + Copy,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> PartialOrd for Frac<T>
where
    T: Integer + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Frac<T>
where
    T: Integer + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        (a1 * b2).cmp(&(a2 * b1))
    }
}
