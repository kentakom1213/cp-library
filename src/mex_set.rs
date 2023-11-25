//! Mexを管理するデータ構造

use std::collections::BTreeSet;

/// 集合とそのmexを管理する
#[derive(Debug)]
pub struct MexSet {
    pub ranges: BTreeSet<(isize, isize)>,
}

impl MexSet {
    /// MexSetを初期化する
    pub fn new() -> Self {
        let ranges = [(isize::MIN, isize::MIN), (isize::MAX, isize::MAX)]
            .into_iter()
            .collect();
        Self { ranges }
    }

    /// 集合に要素`x`を追加する
    /// ### 戻り値
    /// - `true`: `x`が追加された場合
    /// - `false`: `x`がすでに存在していた場合
    pub fn insert(&mut self, x: isize) -> bool {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        let &(r, rr) = self.ranges.range((x + 1, x + 1)..).next().unwrap();
        if x <= l {
            return false;
        }
        match (l == x - 1, x + 1 == r) {
            (false, false) => {
                self.ranges.insert((x, x));
            }
            (false, true) => {
                self.ranges.remove(&(r, rr));
                self.ranges.insert((x, rr));
            }
            (true, false) => {
                self.ranges.remove(&(ll, l));
                self.ranges.insert((ll, x));
            }
            (true, true) => {
                self.ranges.remove(&(ll, l));
                self.ranges.remove(&(r, rr));
                self.ranges.insert((ll, rr));
            }
        }
        true
    }

    /// 集合から要素`x`を削除する
    /// ### 戻り値
    /// - `true`: `x`が削除された場合
    /// - `false`: `x`がすでに存在していなかった場合
    pub fn delete(&mut self, x: isize) -> bool {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        if l < x {
            return false;
        }
        self.ranges.remove(&(ll, l));
        match (ll == x, x == l) {
            (false, false) => {
                self.ranges.insert((ll, x - 1));
                self.ranges.insert((x + 1, l));
            }
            (false, true) => {
                self.ranges.insert((ll, x - 1));
            }
            (true, false) => {
                self.ranges.insert((x + 1, l));
            }
            (true, true) => (),
        }
        true
    }

    /// **集合に含まれない**`x`以上で最小の整数を調べる
    pub fn mex(&self, x: isize) -> isize {
        let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
        if ll <= x && x <= l {
            l + 1
        } else {
            x
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert() {
        let mut mex = MexSet::new();
        /* set: {} */
        assert_eq!(mex.mex(0), 0);
        assert_eq!(mex.mex(100), 100);

        assert_eq!(mex.insert(3), true);
        /* set: {3} */
        assert_eq!(mex.mex(0), 0);
        assert_eq!(mex.mex(100), 100);

        assert_eq!(mex.insert(1), true);
        /* set: {1, 3} */
        assert_eq!(mex.mex(0), 0);
        assert_eq!(mex.mex(100), 100);

        assert_eq!(mex.insert(3), false);
        /* set: {1, 3} */
        assert_eq!(mex.mex(0), 0);
        assert_eq!(mex.mex(100), 100);

        assert_eq!(mex.insert(100), true);
        /* set: {1, 3, 100} */
        assert_eq!(mex.mex(0), 0);
        assert_eq!(mex.mex(100), 101);

        assert_eq!(mex.insert(0), true);
        /* set: {0, 1, 3, 100} */
        assert_eq!(mex.mex(0), 2);
        assert_eq!(mex.mex(100), 101);

        assert_eq!(mex.insert(2), true);
        /* set: {0, 1, 2, 3, 100} */
        assert_eq!(mex.mex(0), 4);
        assert_eq!(mex.mex(100), 101);
    }

    #[test]
    fn test_delete() {
        let mut mex = MexSet::new();

        // 0~5 を追加
        mex.ranges.insert((0, 5));
        /* set: {0, 1, 2, 3, 4, 5} */

        assert_eq!(mex.mex(0), 6);

        assert_eq!(mex.delete(3), true);
        /* set: {0, 1, 2, 4, 5} */
        assert_eq!(mex.mex(0), 3);

        assert_eq!(mex.delete(1), true);
        /* set: {0, 2, 4, 5} */
        assert_eq!(mex.mex(0), 1);

        assert_eq!(mex.delete(3), false);
        /* set: {0, 2, 4, 5} */
        assert_eq!(mex.mex(0), 1);

        assert_eq!(mex.delete(0), true);
        /* set: {2, 4, 5} */
        assert_eq!(mex.mex(0), 0);
    }
}
