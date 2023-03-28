#![allow(non_snake_case)]
#![allow(dead_code)]

use num_traits::identities::Zero;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// # BinaryIndexedTree
/// - `0-indexed`なインターフェースを持つBIT
pub struct BIT<T> {
    pub size: usize,
    arr: Vec<T>,
}

impl<T> BIT<T>
where
    T: Copy + Clone + Zero + Add + AddAssign + Sub<Output = T> + SubAssign + PartialOrd,
{
    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::zero(); n + 1],
        }
    }

    pub fn build(src: &[T]) -> Self {
        let size = src.len();
        let mut arr = vec![T::zero(); size + 1];
        for i in 1..=size {
            let x = src[i - 1];
            arr[i] += x;
            let tmp = arr[i];
            let j = i + (i & i.wrapping_neg());
            if j < size + 1 {
                arr[j] += tmp;
            }
        }
        Self { size, arr }
    }

    pub fn add(&mut self, mut i: usize, x: T) {
        i += 1;
        while i <= self.size {
            self.arr[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> T {
        let mut res = T::zero();
        while i != 0 {
            res += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        res
    }

    pub fn sum(&self, l: usize, r: usize) -> T {
        let to_l = self.prefix_sum(l);
        let to_r = self.prefix_sum(r);
        to_r - to_l
    }

    /// ## lower_bound
    /// - `0..x`の和が`w`以上になる最小の`x`を求める
    /// - 0-indexed
    pub fn lower_bound(&self, w: T) -> usize {
        let mut sum = T::zero();
        let mut idx = 0;
        let depth = self.size.ilog2() as usize;
        for i in (0..=depth).rev() {
            let k = idx + (1 << i);
            if k <= self.size && sum + self.arr[k] < w {
                sum += self.arr[k];
                idx += 1 << i;
            }
        }
        idx
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let mut bit = BIT::<isize>::new(5);

        bit.add(0, 20);
        bit.add(2, -5);

        let sum_5 = bit.prefix_sum(5);
        assert_eq!(sum_5, 15);

        bit.add(4, 10);
        bit.add(1, -20);

        let sum_2 = bit.prefix_sum(2);
        assert_eq!(sum_2, 0);

        let sum_all = bit.prefix_sum(5);
        assert_eq!(sum_all, 5);
    }

    #[test]
    fn test_build() {
        let mut bit = BIT::build(&vec![1, 2, 3, 4, 5]);

        assert_eq!(bit.sum(1, 4), 9);
        assert_eq!(bit.prefix_sum(5), 15);

        bit.add(2, -3);
        bit.add(3, -4);

        assert_eq!(bit.prefix_sum(5), 8);
    }

    #[test]
    fn test_lowerbound() {
        let bit = BIT::build(&vec![1, 2, 3, 4, 5]);

        assert_eq!(bit.lower_bound(0), 0);
        assert_eq!(bit.lower_bound(1), 0);
        assert_eq!(bit.lower_bound(2), 1);
        assert_eq!(bit.lower_bound(10), 3);
        assert_eq!(bit.lower_bound(11), 4);
        assert_eq!(bit.lower_bound(100), 5);
    }
}
