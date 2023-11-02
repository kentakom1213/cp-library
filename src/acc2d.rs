//! 2次元累積和

use num_traits::Num;

/// ## acc2D
/// - 2次元累積和を取る
/// ### 戻り値
/// - `|r_start, r_end, c_start, c_end|: (usize, usize, usize, usize) -> T`
pub fn acc2D<T: Num + Copy>(array: &Vec<Vec<T>>) -> impl Fn(usize, usize, usize, usize) -> T {
    let (H, W) = (array.len(), array[0].len());
    let mut S = vec![vec![T::zero(); W + 1]; H + 1];
    for i in 0..H {
        for j in 0..W {
            S[i + 1][j + 1] = S[i][j + 1] + S[i + 1][j] - S[i][j] + array[i][j];
        }
    }
    move |r_start: usize, r_end: usize, c_start: usize, c_end: usize| -> T {
        S[r_end][c_end] - S[r_end][c_start] - S[r_start][c_end] + S[r_start][c_start]
    }
}

#[cfg(test)]
mod test {
    use super::acc2D;

    #[test]
    fn test_acc2_isize() {
        let arr: Vec<Vec<isize>> = vec![vec![1, -2, 3], vec![4, -5, 6], vec![7, -8, 9]];

        let acc = acc2D(&arr);

        assert_eq!(acc(0, 1, 0, 1), 1);
        assert_eq!(acc(0, 2, 0, 2), -2);
        assert_eq!(acc(0, 3, 1, 2), -15);
        assert_eq!(acc(1, 2, 0, 3), 5);
        assert_eq!(acc(0, 3, 0, 3), 15);
    }

    #[test]
    fn test_acc2D_usize() {
        let arr: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let acc = acc2D(&arr);

        assert_eq!(acc(0, 1, 0, 1), 1);
        assert_eq!(acc(0, 2, 0, 2), 12);
        assert_eq!(acc(0, 3, 1, 2), 15);
        assert_eq!(acc(1, 2, 0, 3), 15);
        assert_eq!(acc(0, 3, 0, 3), 45);
    }
}
