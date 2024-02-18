//! N人をペアに分ける組合せを全列挙する

/// ペアを列挙する
#[derive(Debug)]
pub struct PairsIterator<T: Clone> {
    stack: Vec<(Vec<T>, Vec<(T, T)>)>,
}

impl<T: Clone> FromIterator<T> for PairsIterator<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            stack: vec![(iter.into_iter().collect::<Vec<T>>(), vec![])],
        }
    }
}

impl<T: Clone> Iterator for PairsIterator<T> {
    type Item = Vec<(T, T)>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some((rem, pairs)) = self.stack.pop() else {
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
                self.stack.push((new_rem, new_pairs));
            }
        }
    }
}

/// (0〜n-1)のn個の要素からなる系列
/// をペアにする組合せを列挙する
pub fn pairs_usize(n: usize) -> PairsIterator<usize> {
    (0..n).collect()
}
