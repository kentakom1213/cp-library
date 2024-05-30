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
