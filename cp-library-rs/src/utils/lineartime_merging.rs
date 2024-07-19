//! ソート済み配列の線形時間マージ

use std::iter::Peekable;

/// ソート済み配列を O(N+M) でマージする
pub fn merge<T, I, J>(A: I, B: J) -> MergeIterator<T, I, J>
where
    T: Ord,
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
{
    MergeIterator {
        itr_a: A.peekable(),
        itr_b: B.peekable(),
    }
}

/// マージ後の値を順に返すイテレータ
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
