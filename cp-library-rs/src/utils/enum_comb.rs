//! 組合せの列挙

use itertools::Itertools;
use superslice::Ext;

// ========== pairs ==========
/// ペアのベクタ型
pub type Pairs<T> = Vec<(T, T)>;

/// ペアを列挙する
#[derive(Debug)]
pub struct ListPairs<T: Clone> {
    stack: Vec<(Vec<T>, Pairs<T>)>,
}

impl<T: Clone> FromIterator<T> for ListPairs<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            stack: vec![(iter.into_iter().collect::<Vec<T>>(), vec![])],
        }
    }
}

impl<T: Clone> Iterator for ListPairs<T> {
    type Item = Pairs<T>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (rem, pairs) = self.stack.pop()?;

            if rem.len() < 2 {
                return Some(pairs);
            }
            for i in (1..rem.len()).rev() {
                let mut new_rem = rem.clone();
                let snd = new_rem.remove(i);
                let fst = new_rem.remove(0);
                let mut new_pairs = pairs.clone();
                new_pairs.push((fst, snd));
                // 新しい要素を追加
                self.stack.push((new_rem, new_pairs));
            }
        }
    }
}

impl ListPairs<usize> {
    /// (0〜n-1)のn個の要素からなる系列
    /// をペアにする組合せを列挙する
    pub fn pairs_usize(n: usize) -> Self {
        (0..n).collect()
    }
}

// ========== comb with rep ==========
/// r 個の n 面ダイスを振った結果を列挙する
pub fn comb_with_rep(n: usize, r: usize) -> impl Iterator<Item = Vec<usize>> {
    let perm: Vec<_> = std::iter::repeat_n(false, r)
        .chain(std::iter::repeat_n(true, n - 1))
        .collect();

    std::iter::once(perm.clone())
        .chain(std::iter::repeat(()).scan(perm, |p, _| p.next_permutation().then_some(p.clone())))
        .map(aggregate_comb)
}

fn aggregate_comb(choose: Vec<bool>) -> Vec<usize> {
    let mut res = vec![];
    if choose[0] {
        res.push(0);
    }
    for (k, &f) in choose.iter().dedup_with_count() {
        if f {
            res.extend(std::iter::repeat_n(0, k - 1));
        } else {
            res.push(k);
        }
    }
    if *choose.last().unwrap() {
        res.push(0);
    }
    res
}
