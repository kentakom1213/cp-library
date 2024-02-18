//! 座標圧縮

/// # 座標圧縮
#[derive(Debug)]
pub struct Compression<'a, T> {
    pub size: usize,
    pub sorted_array: Vec<&'a T>,
}

impl<'a, T: Ord> Compression<'a, T> {
    /// スライス`array`で配列を初期化する
    pub fn new(array: &'a [T]) -> Self {
        array.iter().collect()
    }

    /// 圧縮後の`val`の番号を返す
    pub fn idx(&self, val: &T) -> Option<usize> {
        let idx = self.sorted_array.binary_search(&val);
        if let Ok(idx) = idx {
            Some(idx)
        } else {
            None
        }
    }

    /// 圧縮前の要素`idx`を返す
    pub fn val(&self, idx: usize) -> Option<&T> {
        if let Some(&val) = self.sorted_array.get(idx) {
            Some(val)
        } else {
            None
        }
    }
}

impl<'a, T: Ord> FromIterator<&'a T> for Compression<'a, T> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut comp: Vec<&'a T> = iter.into_iter().collect();
        comp.sort();
        comp.dedup();
        Self {
            size: comp.len(),
            sorted_array: comp,
        }
    }
}
