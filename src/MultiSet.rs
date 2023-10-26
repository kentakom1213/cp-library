#![allow(dead_code)]

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
        if let Some(v) = self.map.get_mut(&x) {
            *v -= 1;
            if *v == 0 {
                self.map.remove(&x);
            }
            self.len -= 1;
            return true;
        }
        false
    }

    /// 要素`x`が存在するか判定する
    pub fn contains(&self, x: &T) -> bool {
        self.map.contains_key(&x)
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
         * MultiSet { 3, 5, 10, 20, 20, 20 }
         */

        assert_eq!(mset.count(&5), 1);
        assert_eq!(mset.count(&20), 3);
        assert_eq!(mset.count(&1000), 0);
    }
}
