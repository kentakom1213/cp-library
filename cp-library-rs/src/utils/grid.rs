//! グリッド探索の便利ツール

use num::One;
use num_traits::{WrappingAdd, WrappingSub};

/// グリッドの探索
pub trait Grid<T>
where
    Self: Sized,
{
    /// 座標`(i,j)`に上下左右で隣接する座標を取得
    /// （グリッドサイズ`HxW`でバリデーション）
    ///
    /// **探索順**
    ///
    /// > ```text
    /// >    2
    /// >    ↑
    /// > 3 ← → 1
    /// >    ↓
    /// >    4
    /// > ```
    fn get_adj_4(&self, rrange: (T, T), crange: (T, T)) -> Vec<Self>;
    /// 座標`(i,j)`に8方向で隣接する座標を取得
    /// （グリッドサイズ`HxW`でバリデーション）
    ///
    /// **探索順**
    ///
    /// > ```text
    /// > 4  3  2
    /// >   ↖↑↗
    /// > 5 ← → 1
    /// >   ↙↓➘
    /// > 6  7  8
    /// > ```
    fn get_adj_8(&self, rrange: (T, T), crange: (T, T)) -> Vec<Self>;
    /// 右のセルを返す
    fn right(&self) -> (T, T);
    /// 右上のセルを返す
    fn upright(&self) -> (T, T);
    /// 上のセルを返す
    fn up(&self) -> (T, T);
    /// 左上のセルを返す
    fn upleft(&self) -> (T, T);
    /// 左のセルを返す
    fn left(&self) -> (T, T);
    /// 左下のセルを返す
    fn downleft(&self) -> (T, T);
    /// 下のセルを返す
    fn down(&self) -> (T, T);
    /// 右下のセルを返す
    fn downright(&self) -> (T, T);
}

impl<T> Grid<T> for (T, T)
where
    T: Clone + PartialOrd + WrappingAdd + WrappingSub + One,
{
    fn right(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r, c.wrapping_add(&T::one()))
    }
    fn upright(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r.wrapping_sub(&T::one()), c.wrapping_add(&T::one()))
    }
    fn up(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r.wrapping_sub(&T::one()), c)
    }
    fn upleft(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r.wrapping_sub(&T::one()), c.wrapping_sub(&T::one()))
    }
    fn left(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r, c.wrapping_sub(&T::one()))
    }
    fn downleft(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r.wrapping_add(&T::one()), c.wrapping_sub(&T::one()))
    }
    fn down(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r.wrapping_add(&T::one()), c)
    }
    fn downright(&self) -> (T, T) {
        let (r, c) = self.clone();
        (r.wrapping_add(&T::one()), c.wrapping_add(&T::one()))
    }
    fn get_adj_4(&self, rrange: (T, T), crange: (T, T)) -> Vec<Self> {
        [self.right(), self.up(), self.left(), self.down()]
            .into_iter()
            .filter(|(r, c)| (&rrange.0 <= r && r < &rrange.1) && (&crange.0 <= c && c < &crange.1))
            .collect()
    }
    fn get_adj_8(&self, rrange: (T, T), crange: (T, T)) -> Vec<Self> {
        [
            self.right(),
            self.upright(),
            self.up(),
            self.upleft(),
            self.left(),
            self.downleft(),
            self.down(),
            self.downright(),
        ]
        .into_iter()
        .filter(|(r, c)| (&rrange.0 <= r && r < &rrange.1) && (&crange.0 <= c && c < &crange.1))
        .collect()
    }
}
