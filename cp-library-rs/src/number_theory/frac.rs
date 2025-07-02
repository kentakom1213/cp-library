//! 比較を実装した分数の実装

use std::{cmp::Ordering, ops::Mul};

/// 分数を表す構造体
/// - `Frac(a, b)` := a / b
#[derive(Debug, Clone, Copy)]
pub struct Frac<T>(pub T, pub T);

impl<T> PartialEq for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn eq(&self, other: &Self) -> bool {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        a1 * b2 == a2 * b1
    }
}

impl<T> Eq for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T> PartialOrd for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        (a1 * b2).cmp(&(a2 * b1))
    }
}

// TODO: Add, Mul等の実装
