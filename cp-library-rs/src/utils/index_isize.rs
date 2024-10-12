//! isizeによる巡回的な添字アクセス

/// `isize`による巡回的な添字アクセス
pub trait IndexIsize {
    type T;
    /// 添字 `idx` を `0` 以上 `array.len()` 未満の値に変換する
    fn iidx(&self, idx: isize) -> usize;
    /// `array[array.iidx(idx)]` への**不変**参照を取得する
    fn iget(&self, idx: isize) -> &Self::T;
    /// `array[array.iidx(idx)]` への**可変**参照を取得する
    fn iget_mut(&mut self, idx: isize) -> &mut Self::T;
}

impl<T> IndexIsize for Vec<T> {
    type T = T;
    fn iidx(&self, idx: isize) -> usize {
        idx.rem_euclid(self.len() as isize) as usize
    }
    fn iget(&self, idx: isize) -> &Self::T {
        &self[self.iidx(idx)]
    }
    fn iget_mut(&mut self, idx: isize) -> &mut Self::T {
        let idx = self.iidx(idx);
        &mut self[idx]
    }
}

impl<T> IndexIsize for [T] {
    type T = T;
    fn iidx(&self, idx: isize) -> usize {
        idx.rem_euclid(self.len() as isize) as usize
    }
    fn iget(&self, idx: isize) -> &Self::T {
        &self[self.iidx(idx)]
    }
    fn iget_mut(&mut self, idx: isize) -> &mut Self::T {
        let idx = self.iidx(idx);
        &mut self[idx]
    }
}

impl<T, const N: usize> IndexIsize for [T; N] {
    type T = T;
    fn iidx(&self, idx: isize) -> usize {
        idx.rem_euclid(N as isize) as usize
    }
    fn iget(&self, idx: isize) -> &Self::T {
        &self[self.iidx(idx)]
    }
    fn iget_mut(&mut self, idx: isize) -> &mut Self::T {
        let idx = self.iidx(idx);
        &mut self[idx]
    }
}
