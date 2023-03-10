#![allow(non_snake_case)]
#![allow(dead_code)]

use num_traits::identities::Zero;
use std::ops::{Add, AddAssign, Sub};

// BinaryIndexedTree
struct BIT<T> {
    size: usize,
    arr: Vec<T>,
}

impl<T> BIT<T>
where
    T: Copy + Clone + Zero + Add + AddAssign + Sub<Output = T>,
{
    fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::zero(); n + 1],
        }
    }

    fn build(src: &[T]) -> Self {
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

    fn add(&mut self, mut i: usize, x: T) {
        i += 1;
        while i <= self.size {
            self.arr[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, mut i: usize) -> T {
        let mut res = T::zero();
        while i != 0 {
            res += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        res
    }

    fn sum_range(&self, l: usize, r: usize) -> T {
        let to_l = self.sum(l);
        let to_r = self.sum(r);
        to_r - to_l
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

        let sum_5 = bit.sum(5);
        assert_eq!(sum_5, 15);

        bit.add(4, 10);
        bit.add(1, -20);

        let sum_2 = bit.sum(2);
        assert_eq!(sum_2, 0);

        let sum_all = bit.sum(5);
        assert_eq!(sum_all, 5);
    }

    #[test]
    fn test_build() {
        let mut bit = BIT::build(&vec![1, 2, 3, 4, 5]);

        assert_eq!(bit.sum_range(1, 4), 9);
        assert_eq!(bit.sum(5), 15);

        bit.add(2, -3);
        bit.add(3, -4);

        assert_eq!(bit.sum(5), 8);
    }
}
