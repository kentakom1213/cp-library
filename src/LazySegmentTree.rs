#![allow(dead_code)]
#![allow(unused_variables)]

type T = isize;

/// ## LazySegmentTreeRAQ
/// - 遅延セグメント木
/// - 区間加算、区間取得をサポートする
/// - 対応している型はisizeのみ
#[derive(Debug)]
pub struct LazySegmentTreeRAQ {
    size: usize,
    offset: usize,
    data: Vec<T>,
    lazy: Vec<T>,
}

impl LazySegmentTreeRAQ {
    /// 新規作成
    pub fn new(n: usize) -> Self {
        let offset = n.next_power_of_two();
        Self {
            size: n,
            offset,
            data: vec![T::default(); offset << 1],
            lazy: vec![T::default(); offset << 1],
        }
    }

    /// 一点加算
    /// - 0-indexed
    pub fn set_point(&mut self, mut idx: usize, val: T) {
        // 1-indexedに変換
        idx += self.offset;
        // 根に到達するまで更新
        while idx > 0 {
            // 加算
            self.data[idx] += val;
            idx >>= 1; // 2で割る
        }
    }

    /// 区間加算
    /// - [left, right)
    pub fn set_range(&mut self, left: usize, right: usize, val: T) {}

    /// 区間取得
    /// - [left, right)
    pub fn get_range(&self, mut left: usize, mut right: usize) -> T {
        left += self.offset;
        right += self.offset;
        let mut res = T::default(); // 解を保存

        while left < right {
            if left & 1 == 1 {
                res += self.data[left];
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                res += self.data[right];
            }
            left >>= 1;
            right >>= 1;
        }
        res
    }
}

#[cfg(test)]
mod test_lazy_segtree {
    use super::*;

    #[test]
    fn test_build_segtree() {
        let mut lazy_segtree = LazySegmentTreeRAQ::new(4);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 一点加算
        lazy_segtree.set_point(0, 5);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 区間取得
        println!("[0, 1) -> {:?}", lazy_segtree.get_range(0, 1));
        println!("[1, 3) -> {:?}", lazy_segtree.get_range(1, 3));
        println!("[1, 4) -> {:?}", lazy_segtree.get_range(1, 4));
        println!("[0, 4) -> {:?}", lazy_segtree.get_range(0, 4));

        // 一点加算
        lazy_segtree.set_point(3, -2);
        println!("{:?}", &lazy_segtree.data[4..8]);

        // 区間取得
        println!("[0, 1) -> {:?}", lazy_segtree.get_range(0, 1));
        println!("[1, 3) -> {:?}", lazy_segtree.get_range(1, 3));
        println!("[1, 4) -> {:?}", lazy_segtree.get_range(1, 4));
        println!("[0, 4) -> {:?}", lazy_segtree.get_range(0, 4));
    }
}
