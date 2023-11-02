use num_bigint::BigUint;
use num_traits::{FromPrimitive, ToPrimitive, Zero};
use std::ops::{Add, Mul, Sub};
use std::{collections::HashMap, hash::Hash};

pub struct Loop<T, V, F, G>
where
    F: Fn(T) -> T,
    G: Fn(T) -> V,
{
    /// ノードの移動を行う関数
    pub next: F,
    /// ノードから値を取り出す関数
    pub get_val: G,
    /// 始点となるノード
    pub begin: T,
    /// ループの長さ
    pub loop_len: usize,
    /// ループ開始時の値
    pub loop_begin: T,
    /// ループに到達するまでの移動回数
    pub loop_begin_idx: usize,
    /// ループ開始時までの累積
    pub before_loop_sum: V,
    /// ループ内での累積
    pub loop_sum: V,
    /// ループの途中の値
    vals: HashMap<T, (usize, V)>,
}

impl<T, V, F, G> Loop<T, V, F, G>
where
    T: Copy + Hash + Eq,
    V: Copy + Zero + Add<Output = V> + Sub<Output = V> + Mul<usize, Output = V>,
    F: Fn(T) -> T,
    G: Fn(T) -> V,
{
    /// ループを検出する
    pub fn build(begin: T, next: F, get_val: G) -> Self {
        // 初期化
        let mut cur: T = begin;
        let mut idx: usize = 0;
        let mut sum: V = V::zero();
        let mut vals: HashMap<T, (usize, V)> = HashMap::new();

        // ループ検出
        while vals.get(&cur).is_none() {
            vals.insert(cur, (idx, sum));
            sum = sum + get_val(cur);
            cur = next(cur);
            idx += 1;
        }

        // ループの値を取り出す
        let loop_begin = cur;
        let (loop_begin_idx, before_loop_sum) = vals[&loop_begin];
        let loop_len = idx - loop_begin_idx;
        let loop_sum = sum - before_loop_sum;

        // 返す
        Self {
            next,
            get_val,
            begin,
            loop_len,
            loop_begin,
            loop_begin_idx,
            before_loop_sum,
            loop_sum,
            vals,
        }
    }

    fn accumulate(&self, begin: T, n: usize) -> (T, V) {
        let mut res = V::zero();
        let mut cur = begin;
        for _ in 0..n {
            res = res + (self.get_val)(cur);
            cur = (self.next)(cur);
        }
        (cur, res)
    }

    /// self.beginからn個後の頂点を取り出す
    pub fn get_nth_node_usize(&self, n: usize) -> T {
        if n < self.loop_begin_idx {
            self.accumulate(self.begin, n).0
        } else {
            let loop_rem = (n - self.loop_begin_idx) % self.loop_len;
            self.accumulate(self.loop_begin, loop_rem).0
        }
    }

    /// self.beginからn個後の値を取り出す
    pub fn get_nth_val_usize(&self, n: usize) -> V {
        if n < self.loop_begin_idx {
            self.accumulate(self.begin, n).1
        } else {
            let loop_rep = (n - self.loop_begin_idx) / self.loop_len;
            let loop_rem = (n - self.loop_begin_idx) % self.loop_len;
            self.before_loop_sum
                + self.loop_sum * loop_rep
                + self.accumulate(self.loop_begin, loop_rem).1
        }
    }

    /// self.beginからn個後の値を取り出す
    pub fn get_nth_node_biguint(&self, n: BigUint) -> T {
        let loop_begin_idx = BigUint::from_usize(self.loop_begin_idx).unwrap();
        if n < loop_begin_idx {
            let n_usize = n.to_usize().unwrap();
            self.accumulate(self.begin, n_usize).0
        } else {
            let loop_len = BigUint::from_usize(self.loop_len).unwrap();
            let loop_rem = (n - loop_begin_idx) % loop_len;
            let loop_rem = loop_rem.to_usize().unwrap();
            self.accumulate(self.loop_begin, loop_rem).0
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_functional_graph() {
        let graph = vec![1, 2, 3, 2, 4];
        /*
         * 1 --> 2 --> 3 <-> 2,  4 <-> 4
         */

        let next = |u: usize| graph[u];

        // 0から始める
        let from_0 = Loop::build(0, next, |_| 1);

        assert_eq!(from_0.get_nth_node_usize(0), 0);
        assert_eq!(from_0.get_nth_node_usize(1), 1);
        assert_eq!(from_0.get_nth_node_usize(2), 2);
        assert_eq!(from_0.get_nth_node_usize(20), 2);
        assert_eq!(from_0.get_nth_node_usize(100), 2);
        assert_eq!(from_0.get_nth_node_usize(101), 3);
        assert_eq!(from_0.get_nth_node_usize(10000), 2);
        assert_eq!(from_0.get_nth_node_usize(100000001), 3);
        assert_eq!(from_0.get_nth_node_usize(10000000000000000), 2);

        // 2から始める
        let from_2 = Loop::build(2, next, |_| 1);

        assert_eq!(from_2.get_nth_node_usize(0), 2);
        assert_eq!(from_2.get_nth_node_usize(1), 3);
        assert_eq!(from_2.get_nth_node_usize(2), 2);
        assert_eq!(from_2.get_nth_node_usize(20), 2);
        assert_eq!(from_2.get_nth_node_usize(100), 2);
        assert_eq!(from_2.get_nth_node_usize(101), 3);
        assert_eq!(from_2.get_nth_node_usize(10000), 2);
        assert_eq!(from_2.get_nth_node_usize(100000001), 3);
        assert_eq!(from_2.get_nth_node_usize(10000000000000000), 2);

        // 4から始める
        let from_4 = Loop::build(4, next, |_| 1);

        assert_eq!(from_4.get_nth_node_usize(0), 4);
        assert_eq!(from_4.get_nth_node_usize(1), 4);
        assert_eq!(from_4.get_nth_node_usize(2), 4);
        assert_eq!(from_4.get_nth_node_usize(20), 4);
        assert_eq!(from_4.get_nth_node_usize(100), 4);
        assert_eq!(from_4.get_nth_node_usize(101), 4);
        assert_eq!(from_4.get_nth_node_usize(10000), 4);
        assert_eq!(from_4.get_nth_node_usize(100000001), 4);
        assert_eq!(from_4.get_nth_node_usize(10000000000000000), 4);
    }

    /// テストケース: <https://atcoder.jp/contests/abc030/tasks/abc030_d>
    #[test]
    fn test_biguint() {
        let graph = vec![1, 2, 3, 0];

        let next = |u: usize| graph[u];

        // 0から始める
        let from_0 = Loop::build(0, next, |_| 1);

        let n = BigUint::from_str("100000000000000000000").unwrap();

        assert_eq!(from_0.get_nth_node_biguint(n), 0);
    }

    /// テストケース: <https://atcoder.jp/contests/abc179/tasks/abc179_e>
    #[test]
    fn test_accumulate() {
        let f1 = Loop::build(2, |x| x * x % 1001, |x| x);
        assert_eq!(f1.get_nth_val_usize(6), 1369);

        let f2 = Loop::build(2, |x| x * x % 16, |x| x);
        assert_eq!(f2.get_nth_val_usize(1000), 6);

        let f3 = Loop::build(10, |x| x * x % 99959, |x| x);
        assert_eq!(f3.get_nth_val_usize(10000000000), 492443256176507);
    }
}
