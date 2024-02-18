//! グリッド探索の便利ツール

/// グリッドの探索
pub trait Grid<T>
where
    Self: Sized,
{
    /// usizeにおける-1
    const NEG1: T;
    /// 隣接する4方向（上下左右）
    const ADJ4: [(T, T); 4];
    /// 隣接する8方向
    const ADJ8: [(T, T); 8];
    /// 座標`(i,j)`に上下左右で隣接する座標を取得
    /// - グリッドサイズ`HxW`でバリデーション
    fn get_adj_4(&self, H: usize, W: usize) -> Vec<Self>;
    /// 座標`(i,j)`に8方向で隣接する座標を取得
    /// - グリッドサイズ`HxW`でバリデーション
    fn get_adj_8(&self, H: usize, W: usize) -> Vec<Self>;
}

impl Grid<usize> for (usize, usize) {
    const NEG1: usize = 1_usize.wrapping_neg();
    const ADJ4: [(usize, usize); 4] = [(0, Self::NEG1), (Self::NEG1, 0), (0, 1), (1, 0)];
    const ADJ8: [(usize, usize); 8] = [
        (Self::NEG1, Self::NEG1),
        (Self::NEG1, 0),
        (Self::NEG1, 1),
        (0, Self::NEG1),
        (0, 1),
        (1, Self::NEG1),
        (1, 0),
        (1, 1),
    ];
    fn get_adj_4(&self, H: usize, W: usize) -> Vec<(usize, usize)> {
        let mut adj = vec![];
        for &(dr, dc) in &Self::ADJ4 {
            let nr = self.0.wrapping_add(dr);
            let nc = self.1.wrapping_add(dc);
            if nr < H && nc < W {
                adj.push((nr, nc));
            }
        }
        adj
    }
    fn get_adj_8(&self, H: usize, W: usize) -> Vec<(usize, usize)> {
        let mut adj = vec![];
        for &(dr, dc) in &Self::ADJ8 {
            let nr = self.0.wrapping_add(dr);
            let nc = self.1.wrapping_add(dc);
            if nr < H && nc < W {
                adj.push((nr, nc));
            }
        }
        adj
    }
}
