//! 多重集合（BTreeMapによる実装）

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MultiSet<T> {
    pub map: BTreeMap<T, usize>,
    len: usize,
}

impl<T> MultiSet<T>
where
    T: Ord,
{
    /// MultiSetを初期化する
    pub fn new() -> Self {
        MultiSet {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    /// 要素`x`を追加する
    pub fn insert(&mut self, x: T) {
        *self.map.entry(x).or_insert(0) += 1;
        self.len += 1;
    }

    /// 要素`x`を削除する
    pub fn remove(&mut self, x: &T) -> bool {
        if let Some(v) = self.map.get_mut(x) {
            *v -= 1;
            if *v == 0 {
                self.map.remove(x);
            }
            self.len -= 1;
            return true;
        }
        false
    }

    /// 要素`x`が存在するか判定する
    pub fn contains(&self, x: &T) -> bool {
        self.map.contains_key(x)
    }

    /// 先頭の要素を取得する
    pub fn first(&self) -> Option<&T> {
        self.map.keys().next()
    }

    /// 末尾の要素を取得する
    pub fn last(&self) -> Option<&T> {
        self.map.keys().last()
    }

    /// `x`の個数をカウントする
    pub fn count(&self, x: &T) -> usize {
        match self.map.get(x) {
            Some(&v) => v,
            None => 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
