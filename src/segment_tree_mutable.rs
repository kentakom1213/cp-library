//! セグメント木

use std::fmt::{self, Debug};
use std::ops::{
    Bound::{Excluded, Included, Unbounded},
    Deref, DerefMut, Index, RangeBounds,
};

/// # SegmentTree (Monoid)
/// - 抽象化セグメント木
pub struct SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    pub size: usize,
    offset: usize,
    e: T,
    op: F,
    data: Vec<T>,
}

impl<T, F> Index<usize> for SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.data[self.offset + idx]
    }
}

impl<T, F> SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    #[inline]
    fn parse_range<R: RangeBounds<usize>>(&self, range: R) -> Option<(usize, usize)> {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        }
        .min(self.size);
        let end = match range.end_bound() {
            Unbounded => self.size,
            Excluded(&v) => v,
            Included(&v) => v + 1,
        }
        .min(self.size);
        if start <= end {
            Some((start, end))
        } else {
            None
        }
    }

    /// セグメント木を初期化する
    pub fn new(n: usize, e: T, op: F) -> Self {
        let offset = n.next_power_of_two();

        Self {
            size: n,
            offset,
            e: e.clone(),
            op,
            data: vec![e; offset << 1],
        }
    }

    /// セグメント木を配列から初期化する
    pub fn build(src: &[T], e: T, op: F) -> Self {
        let mut seg = Self::new(src.len(), e, op);
        for (i, v) in src.iter().enumerate() {
            seg.data[seg.offset + i] = v.clone();
        }
        for i in (0..seg.offset).rev() {
            let lch = i << 1;
            seg.data[i] = (seg.op)(&seg.data[lch], &seg.data[lch + 1]);
        }
        seg
    }

    fn update(&mut self, index: usize, value: T) {
        let mut i = index + self.offset;
        self.data[i] = value;
        while i > 1 {
            i >>= 1;
            let lch = i << 1;
            self.data[i] = (self.op)(&self.data[lch], &self.data[lch + 1]);
        }
    }

    /// 可変な参照を返す
    pub fn get_mut(&mut self, i: usize) -> Option<ValMut<'_, T, F>> {
        if i < self.offset {
            let default = self.index(i).clone();
            Some(ValMut {
                segtree: self,
                idx: i,
                new_val: default,
            })
        } else {
            None
        }
    }

    /// 区間`range`の集約を行う
    pub fn get_range<R: RangeBounds<usize>>(&self, range: R) -> T {
        let parsed = self.parse_range(range);
        if parsed.is_none() {
            return self.e.clone();
        }

        let (start, end) = parsed.unwrap();

        // 全体の値を取得
        if (start, end) == (0, self.size) {
            return self.data[1].clone();
        }

        // 値の取得
        let mut l = self.offset + start;
        let mut r = self.offset + end;
        let (mut res_l, mut res_r) = (self.e.clone(), self.e.clone());

        while l < r {
            if l & 1 == 1 {
                res_l = (self.op)(&res_l, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                res_r = (self.op)(&self.data[r], &res_r);
            }
            l >>= 1;
            r >>= 1;
        }

        (self.op)(&res_l, &res_r)
    }
}

impl<T: Debug, F> std::fmt::Debug for SegmentTree<T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SegmentTree {{ [").ok();
        for i in 0..self.size {
            if i + 1 < self.size {
                write!(f, "{:?}, ", self.data[self.offset + i]).ok();
            } else {
                write!(f, "{:?}", self.data[self.offset + i]).ok();
            }
        }
        write!(f, "] }}")
    }
}

pub struct ValMut<'a, T: 'a, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    segtree: &'a mut SegmentTree<T, F>,
    idx: usize,
    new_val: T,
}

impl<T: Debug, F> fmt::Debug for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ValMut")
            .field(&self.segtree.index(self.idx))
            .finish()
    }
}

impl<T, F> Drop for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn drop(&mut self) {
        self.segtree.update(self.idx, self.new_val.clone());
    }
}

impl<T, F> Deref for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.segtree[self.idx]
    }
}

impl<T, F> DerefMut for ValMut<'_, T, F>
where
    T: Debug + Clone + PartialEq,
    F: Fn(&T, &T) -> T,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_point() {
        let segtree = SegmentTree::build(&vec![1, 2, 3, 4, 5], 0, |a, b| a + b);

        assert_eq!(segtree[0], 1);
        assert_eq!(segtree[3], 4);
    }

    #[test]
    fn test_RSQ() {
        let mut segtree = SegmentTree::new(3, 0, |a, b| a + b);

        // segtree.update(0, 1);
        *segtree.get_mut(0).unwrap() += 1;
        *segtree.get_mut(1).unwrap() += 2;
        *segtree.get_mut(2).unwrap() += 3;
        // [1, 2, 3]

        assert_eq!(segtree.get_range(0..2), 3);
        assert_eq!(segtree.get_range(1..2), 2);
        assert_eq!(segtree.get_range(1..=2), 5);
        assert_eq!(segtree.get_range(..), 6);
    }

    #[test]
    fn test_RMQ() {
        const INF: usize = (1 << 31) - 1;
        let mut segtree = SegmentTree::new(3, INF, |a, b| *a.min(b));

        assert_eq!(segtree.get_range(..1), (1 << 31) - 1);
        *segtree.get_mut(0).unwrap() = 5;
        assert_eq!(segtree.get_range(..1), 5);
    }

    #[test]
    fn test_from_slice() {
        const MINF: isize = -(1 << 31) + 1;
        let arr = vec![20, 4, 5, 6, 8, 9, 100];
        let mut segtree = SegmentTree::build(&arr, MINF, |a, b| *a.max(b));

        assert_eq!(segtree.get_range(0..), 100);
        assert_eq!(segtree.get_range(2..5), 8);

        segtree.update(0, 200);

        assert_eq!(segtree.get_range(..), 200);
        assert_eq!(segtree.get_range(2..5), 8);
        assert_eq!(segtree.get_range(5..10), 100);
        assert_eq!(segtree.get_range(10..20), MINF);
    }

    #[test]
    fn test_debug_print() {
        const MINF: isize = -(1 << 31) + 1;
        let arr = vec![20, 4, 5, 6, 8, 9, 100];
        let segtree = SegmentTree::build(&arr, MINF, |a, b| *a.max(b));

        let dbg = format!("{:?}", &segtree);
        assert_eq!(&dbg, "SegmentTree { [20, 4, 5, 6, 8, 9, 100] }");
    }
}
