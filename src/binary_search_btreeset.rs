//! BTreeSetに対する`lower_bound`,`upper_bound`の実装

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};

/// # BinarySearch
/// 二分探索
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> Option<&T>;
    fn upper_bound(&self, x: &T) -> Option<&T>;
}

impl<T: Ord> BinarySearch<T> for BTreeSet<T> {
    /// x以上の値を探索する
    fn lower_bound(&self, x: &T) -> Option<&T> {
        let mut greater_equal = self.range((Included(x), Unbounded));

        greater_equal.next()
    }

    /// xより大きい値を探索する
    fn upper_bound(&self, x: &T) -> Option<&T> {
        let mut greater_equal = self.range((Excluded(x), Unbounded));

        greater_equal.next()
    }
}
