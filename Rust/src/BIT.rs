#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    ops::{
        Bound::{Excluded, Included, Unbounded},
        RangeBounds,
    },
};

/// # Monoid
pub trait Monoid {
    type Val: Debug + Clone + PartialEq;
    const E: Self::Val;
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val;
}

pub trait InversableMonoid: Monoid {
    fn inv(val: &Self::Val) -> Self::Val;
}

pub trait OrderedMonoid: Monoid {
    fn lt(left: &Self::Val, right: &Self::Val) -> bool;
}

/// # BinaryIndexedTree
/// - `0-indexed`なインターフェースを持つBIT
pub struct BIT<T: Monoid> {
    pub size: usize,
    arr: Vec<T::Val>,
}

impl<T: Monoid> BIT<T> {
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![T::E; n + 1],
        }
    }

    pub fn add(&mut self, mut i: usize, x: T::Val) {
        i += 1;
        while i <= self.size {
            self.arr[i] = T::op(&self.arr[i], &x);
            i += Self::lsb(i);
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> T::Val {
        let mut res = T::E;
        while i != 0 {
            res = T::op(&res, &self.arr[i]);
            i -= Self::lsb(i);
        }
        res
    }
}

impl<T: InversableMonoid> BIT<T> {
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v - 1,
        };
        if start <= end {
            Some((start, end))
        } else {
            None
        }
    }

    fn sum<R: RangeBounds<usize>>(&self, range: R) -> T::Val {
        if let Some((i, j)) = self.parse_range(range) {
            T::op(&self.prefix_sum(j), &T::inv(&self.prefix_sum(i)))
        } else {
            T::E
        }
    }
}

impl<T: Monoid> From<&Vec<T::Val>> for BIT<T> {
    fn from(src: &Vec<T::Val>) -> Self {
        let size = src.len();
        let mut arr = vec![T::E; size + 1];
        for i in 1..=size {
            let x = src[i - 1].clone();
            arr[i] = T::op(&arr[i], &x);
            let j = i + Self::lsb(i);
            if j < size + 1 {
                arr[j] = T::op(&arr[j], &arr[i].clone());
            }
        }
        Self { size, arr }
    }
}

impl<T: OrderedMonoid> BIT<T> {
    pub fn lower_bound(&self, w: T::Val) -> usize {
        let mut sum = T::E;
        let mut idx = 0;
        let mut d = self.size.next_power_of_two() / 2;
        while d != 0 {
            if idx + d <= self.size {
                let nxt = T::op(&sum, &self.arr[idx + d]);
                if T::lt(&nxt, &w) {
                    sum = nxt;
                    idx += d;
                }
            }
            d >>= 1;
        }
        idx
    }
}

impl<T: InversableMonoid> Debug for BIT<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BIT {{ [")?;
        for i in 0..self.size - 1 {
            write!(f, "{:?}, ", self.sum(i..i + 1))?;
        }
        write!(f, "{:?}] }}", self.sum(self.size - 1..self.size))
    }
}

pub mod Alg {
    use super::{InversableMonoid, Monoid, OrderedMonoid};

    #[derive(Debug)]
    pub struct Add;
    impl Monoid for Add {
        type Val = isize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }
    impl InversableMonoid for Add {
        fn inv(val: &Self::Val) -> Self::Val {
            -val
        }
    }
    impl OrderedMonoid for Add {
        fn lt(left: &Self::Val, right: &Self::Val) -> bool {
            left < right
        }
    }

    #[derive(Debug)]
    pub struct Mul;
    impl Monoid for Mul {
        type Val = isize;
        const E: Self::Val = 1;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left + right
        }
    }

    #[derive(Debug)]
    pub struct Xor;
    impl Monoid for Xor {
        type Val = usize;
        const E: Self::Val = 0;
        fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
            left ^ right
        }
    }
    impl InversableMonoid for Xor {
        fn inv(val: &Self::Val) -> Self::Val {
            *val
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let mut bit = BIT::<Alg::Add>::new(5);

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
        let mut bit = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 5]);

        assert_eq!(bit.prefix_sum(4), 10);
        assert_eq!(bit.prefix_sum(5), 15);

        bit.add(2, -3);
        bit.add(3, -4);

        assert_eq!(bit.prefix_sum(5), 8);
    }

    #[test]
    fn test_sum() {
        let bit = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 5]);

        assert_eq!(bit.sum(0..5), 15);
        assert_eq!(bit.sum(1..5), 14);
        assert_eq!(bit.sum(2..3), 3);
        assert_eq!(bit.sum(3..2), 0);
    }

    #[test]
    fn test_lowerbound() {
        let bit = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 0, 5]);

        assert_eq!(bit.lower_bound(0), 0);
        assert_eq!(bit.lower_bound(1), 0);
        assert_eq!(bit.lower_bound(2), 1);
        assert_eq!(bit.lower_bound(10), 3);
        assert_eq!(bit.lower_bound(11), 5);
        assert_eq!(bit.lower_bound(100), 6);
    }

    #[test]
    fn test_debugprint() {
        let bit1 = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 5]);
        println!("{:?}", bit1);
    }
}
