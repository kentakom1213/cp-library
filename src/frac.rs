//! 比較を実装した分数の実装

// ===== Fraction =====
use std::{cmp::Ordering, ops::Mul};

/// 分数を表す構造体
/// - `Frac(a, b)` := a / b
#[derive(Debug, Clone, Copy)]
pub struct Frac<T>(T, T);

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
        let &Frac(a1, b1) = self;
        let &Frac(a2, b2) = other;
        (a1 * b2).partial_cmp(&(a2 * b1))
    }
}

impl<T> Ord for Frac<T>
where
    T: Clone + Copy + Ord + Mul<Output = T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

// TODO: Add, Mul等の実装

#[cfg(test)]
mod test_frac {
    use super::*;

    #[test]
    fn test_eq() {
        let values = vec![
            Frac(0, 1),
            Frac(8, 2),
            Frac(4, 1),
            Frac(4, 4),
            Frac(5, 5),
            Frac(3, 2),
            Frac(1, 0),
            Frac(2, 0),
        ];

        let eq_matrix = vec![
            vec![1, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0, 1, 1],
        ];

        for i in 0..8 {
            for j in 0..8 {
                assert_eq!((values[i] == values[j]) as u8, eq_matrix[i][j]);
            }
        }
    }

    #[test]
    fn test_ord() {
        let values = vec![
            Frac(0, 1),
            Frac(1, 8),
            Frac(2, 8),
            Frac(4, 4),
            Frac(5, 5),
            Frac(3, 2),
            Frac(8, 2),
            Frac(4, 1),
            Frac(1, 0),
            Frac(2, 0),
        ];

        let mut sorted = values.clone();
        sorted.sort();

        assert_eq!(values, sorted);
    }
}
