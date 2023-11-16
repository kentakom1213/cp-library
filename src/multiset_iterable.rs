//! 多重集合（Setによる実装）

use rustc_hash::FxHashMap;
use std::{collections::BTreeSet, hash::Hash};

#[derive(Debug)]
pub struct MultiSet<T> {
    pub counter: FxHashMap<T, usize>,
    pub items: BTreeSet<(T, usize)>,
}

impl<T> MultiSet<T>
where
    T: Ord + Hash + Copy,
{
    /// MultiSetを初期化する
    pub fn new() -> Self {
        MultiSet {
            counter: FxHashMap::default(),
            items: BTreeSet::new(),
        }
    }

    /// 要素`x`を追加する
    pub fn insert(&mut self, x: T) {
        // カウンターに追加
        let cnt = self.counter.entry(x).or_insert(0);
        // setに追加
        self.items.insert((x, *cnt));
        // カウント
        *cnt += 1;
    }

    /// 要素`x`を削除する
    pub fn remove(&mut self, x: &T) -> bool {
        if let Some(v) = self.counter.get_mut(x) {
            // カウンターをデクリメント
            *v -= 1;
            // setから削除
            self.items.remove(&(*x, *v));
            return true;
        }
        false
    }

    /// 要素`x`が存在するか判定する
    pub fn contains(&self, x: &T) -> bool {
        self.counter.get(x).is_some_and(|cnt| *cnt > 0)
    }

    /// 先頭の要素を取得する
    pub fn first(&self) -> Option<&T> {
        self.items.first().map(|(ref x, _)| x)
    }

    /// 末尾の要素を取得する
    pub fn last(&self) -> Option<&T> {
        self.items.last().map(|(ref x, _)| x)
    }

    /// `x`の個数をカウントする
    pub fn count(&self, x: &T) -> usize {
        match self.counter.get(x) {
            Some(&v) => v,
            None => 0,
        }
    }

    /// 要素をすべて削除する
    pub fn clear(&mut self) {
        self.counter.clear();
        self.items.clear();
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> MultiSet<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter().map(|(ref x, _)| x)
    }
}

impl<T: Ord + Hash + Copy> FromIterator<T> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut multiset = MultiSet::new();
        for x in iter {
            multiset.insert(x);
        }
        multiset
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_usize() {
        let mut mset: MultiSet<usize> = MultiSet::new();

        assert!(mset.is_empty());

        mset.insert(5);
        mset.insert(7);
        mset.insert(7);
        mset.insert(10);
        mset.insert(3);

        assert_eq!(mset.len(), 5);

        // remove value
        assert_eq!(mset.remove(&7), true);
        assert_eq!(mset.remove(&7), true);
        assert_eq!(mset.remove(&0), false);

        // is_contain
        assert_eq!(mset.contains(&5), true);
        assert_eq!(mset.contains(&7), false);
        assert_eq!(mset.contains(&0), false);
        assert_eq!(mset.contains(&1000), false);

        // first element
        assert_eq!(mset.first(), Some(&3));

        assert_eq!(mset.remove(&3), true);
        assert_eq!(mset.first(), Some(&5));
        assert_eq!(mset.contains(&3), false);

        // last element
        assert_eq!(mset.last(), Some(&10));

        // count values
        mset.insert(20);
        mset.insert(20);
        mset.insert(20);
        /*
         * MultiSet { 5, 10, 20, 20, 20 }
         */

        assert_eq!(mset.len(), 5);

        assert_eq!(mset.count(&5), 1);
        assert_eq!(mset.count(&20), 3);
        assert_eq!(mset.count(&1000), 0);

        // clear all elements
        mset.clear();

        assert!(mset.is_empty());
        assert_eq!(mset.len(), 0);
    }

    #[test]
    fn test_iterator() {
        let mut arr = vec![0, 9, 4, 4, 5, 5, 10, 10, 3, 3, 0, 0, 2, 1];

        let mut mset: MultiSet<usize> = arr.iter().cloned().collect();

        assert_eq!(mset.len(), 14);

        arr.sort();

        for (a, b) in mset.iter().zip(arr.iter()) {
            assert_eq!(a, b);
        }
    }
}
