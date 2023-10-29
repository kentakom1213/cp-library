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

#[cfg(test)]
mod test {
    use super::Grid;
    use std::collections::VecDeque;

    const INF: usize = usize::MAX;

    #[test]
    fn test_grid_4() {
        let (H, W) = (5, 6);
        let S = "
        .#...#
        ..##..
        .#..##
        ...##.
        ##....
        ";

        let grid: Vec<Vec<char>> = S
            .split_whitespace()
            .map(|row| row.chars().collect())
            .collect();

        // BFS
        let dist = {
            let mut arr = vec![vec![INF; W]; H];
            arr[0][0] = 0;
            let mut deq = VecDeque::new();
            deq.push_back((0, 0));

            while let Some((r, c)) = deq.pop_front() {
                for (nr, nc) in (r, c).get_adj_4(H, W) {
                    if grid[nr][nc] == '.' && arr[nr][nc] == INF {
                        arr[nr][nc] = arr[r][c] + 1;
                        deq.push_back((nr, nc));
                    }
                }
            }

            arr
        };

        assert_eq!(
            &dist,
            &vec![
                vec![0, INF, INF, INF, INF, INF],
                vec![1, 2, INF, INF, INF, INF],
                vec![2, INF, 6, 7, INF, INF],
                vec![3, 4, 5, INF, INF, 10],
                vec![INF, INF, 6, 7, 8, 9],
            ]
        );
    }

    #[test]
    fn test_grid_8() {
        let (H, W) = (5, 6);
        let S = "
        .#...#
        ..##..
        .#..##
        ...##.
        ##....
        ";

        let grid: Vec<Vec<char>> = S
            .split_whitespace()
            .map(|row| row.chars().collect())
            .collect();

        // BFS
        let dist = {
            let mut arr = vec![vec![INF; W]; H];
            arr[0][0] = 0;
            let mut deq = VecDeque::new();
            deq.push_back((0, 0));

            while let Some((r, c)) = deq.pop_front() {
                for (nr, nc) in (r, c).get_adj_8(H, W) {
                    if grid[nr][nc] == '.' && arr[nr][nc] == INF {
                        arr[nr][nc] = arr[r][c] + 1;
                        deq.push_back((nr, nc));
                    }
                }
            }

            arr
        };

        assert_eq!(
            &dist,
            &vec![
                vec![0, INF, 2, 3, 4, INF],
                vec![1, 1, INF, INF, 4, 5],
                vec![2, INF, 2, 3, INF, INF],
                vec![3, 3, 3, INF, INF, 6],
                vec![INF, INF, 4, 4, 5, 6],
            ]
        );
    }
}
