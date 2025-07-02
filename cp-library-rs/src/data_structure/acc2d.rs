//! 2次元累積和

use num_traits::Num;

/// acc2D
/// - 2次元累積和を取る
///
/// **戻り値**
/// - `|r_start, r_end, c_start, c_end|: (usize, usize, usize, usize) -> T`
#[allow(clippy::ptr_arg)]
pub fn acc2D<T: Num + Copy>(array: &Vec<Vec<T>>) -> impl Fn(usize, usize, usize, usize) -> T {
    let (H, W) = (array.len(), array[0].len());
    let mut S = vec![vec![T::zero(); W + 1]; H + 1];
    for i in 0..H {
        for j in 0..W {
            S[i + 1][j + 1] = array[i][j] + S[i][j + 1] + S[i + 1][j] - S[i][j];
        }
    }
    move |r_start: usize, r_end: usize, c_start: usize, c_end: usize| -> T {
        S[r_end][c_end] + S[r_start][c_start] - S[r_end][c_start] - S[r_start][c_end]
    }
}
