//! N人をペアに分ける組合せを全列挙する

/// ペアを列挙する
#[derive(Debug)]
pub struct PairsIterator<T: Clone> {
    remainings: Vec<Vec<T>>,
    pairs: Vec<Vec<(T, T)>>,
}

impl<T: Clone> FromIterator<T> for PairsIterator<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            remainings: vec![iter.into_iter().collect::<Vec<T>>()],
            pairs: vec![vec![]],
        }
    }
}

impl<T: Clone> Iterator for PairsIterator<T> {
    type Item = Vec<(T, T)>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (Some(rem), Some(pairs)) = (self.remainings.pop(), self.pairs.pop()) else {
                return None;
            };
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
                self.remainings.push(new_rem);
                self.pairs.push(new_pairs);
            }
        }
    }
}

/// (0〜n-1)のn個の要素からなる系列
/// をペアにする組合せを列挙する
pub fn pairs_usize(n: usize) -> PairsIterator<usize> {
    (0..n).collect()
}

#[cfg(test)]
mod test_enum_pairs {
    use super::{pairs_usize, PairsIterator};

    #[test]
    fn test_enum_pairs() {
        let mut res = vec![];
        dfs((0..6).collect(), vec![], &mut res);

        assert_eq!(
            res,
            vec![
                vec![(0, 1), (2, 3), (4, 5)],
                vec![(0, 1), (2, 4), (3, 5)],
                vec![(0, 1), (2, 5), (3, 4)],
                vec![(0, 2), (1, 3), (4, 5)],
                vec![(0, 2), (1, 4), (3, 5)],
                vec![(0, 2), (1, 5), (3, 4)],
                vec![(0, 3), (1, 2), (4, 5)],
                vec![(0, 3), (1, 4), (2, 5)],
                vec![(0, 3), (1, 5), (2, 4)],
                vec![(0, 4), (1, 2), (3, 5)],
                vec![(0, 4), (1, 3), (2, 5)],
                vec![(0, 4), (1, 5), (2, 3)],
                vec![(0, 5), (1, 2), (3, 4)],
                vec![(0, 5), (1, 3), (2, 4)],
                vec![(0, 5), (1, 4), (2, 3)],
            ]
        );
    }

    fn dfs(rem: Vec<usize>, pairs: Vec<(usize, usize)>, res: &mut Vec<Vec<(usize, usize)>>) {
        if rem.len() < 2 {
            res.push(pairs.clone());
            return;
        }
        for i in 1..rem.len() {
            let mut new_rem = rem.clone();
            let snd = new_rem.remove(i);
            let fst = new_rem.remove(0);
            let mut new_pairs = pairs.clone();
            new_pairs.push((fst, snd));
            // 再帰呼出し
            dfs(new_rem, new_pairs, res);
        }
    }

    #[test]
    fn test_iter_pairs_4() {
        let mut pairs4 = vec![];
        dfs((0..4).collect(), vec![], &mut pairs4);

        assert_eq!(pairs_usize(4).collect::<Vec<_>>(), pairs4);
    }

    #[test]
    fn test_iter_pairs_12() {
        let mut pairs12 = vec![];
        dfs((0..12).collect(), vec![], &mut pairs12);

        assert_eq!(pairs_usize(12).collect::<Vec<_>>(), pairs12);
    }
}
