//! 2次元累積和

use num_traits::Num;

/// acc3D
/// - 3次元累積和を取る
///
/// **戻り値**
/// - `|lx, rx, ly, ry, lz, rz|: (usize, usize, usize, usize, usize, usize) -> T`
pub fn acc3D<T: Num + Copy>(
    array: &Vec<Vec<Vec<T>>>,
) -> impl Fn(usize, usize, usize, usize, usize, usize) -> T {
    let (X, Y, Z) = (array.len(), array[0].len(), array[0][0].len());
    let mut S = vec![vec![vec![T::zero(); Z + 1]; Y + 1]; X + 1];

    for i in 0..X {
        for j in 0..Y {
            for k in 0..Z {
                S[i + 1][j + 1][k + 1] = array[i][j][k]
                    + (S[i + 1][j + 1][k] + S[i + 1][j][k + 1] + S[i][j + 1][k + 1])
                    + S[i][j][k]
                    - (S[i + 1][j][k] + S[i][j + 1][k] + S[i][j][k + 1]);
            }
        }
    }

    move |lx, rx, ly, ry, lz, rz| -> T {
        S[rx][ry][rz] + (S[lx][ly][rz] + S[lx][ry][lz] + S[rx][ly][lz])
            - (S[lx][ry][rz] + S[rx][ly][rz] + S[rx][ry][lz])
            - S[lx][ly][lz]
    }
}
