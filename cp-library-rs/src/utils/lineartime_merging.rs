//! ソート済み配列の線形時間マージ

use std::iter::Peekable;

pub trait Merge: Iterator
where
    Self::Item: Ord,
    Self: Sized,
{
    /// ソート済み配列をマージする
    ///
    /// - 計算量 : $`O(N + M)`$
    fn merge_linear(self, other: Self) -> MergeIterator<Self::Item, Self, Self> {
        MergeIterator {
            itr_a: self.peekable(),
            itr_b: other.peekable(),
        }
    }
}

impl<I: Iterator> Merge for I where I::Item: Ord {}

/// 2つのソート済みイテレータを，整列順に返す
pub struct MergeIterator<T, I, J>
where
    T: Ord,
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    itr_a: Peekable<I>,
    itr_b: Peekable<J>,
}

impl<T, I, J> Iterator for MergeIterator<T, I, J>
where
    T: Ord,
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.itr_a.peek(), self.itr_b.peek()) {
            (Some(a), Some(b)) => {
                if a <= b {
                    self.itr_a.next()
                } else {
                    self.itr_b.next()
                }
            }
            (Some(_), _) => self.itr_a.next(),
            _ => self.itr_b.next(),
        }
    }
}
