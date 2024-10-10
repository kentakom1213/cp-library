//! 包除原理

/// 高次元配列に対して包除原理を適用する
///
/// ---
///
/// 2次元累積和の実装例
///
/// ```rust
/// use cp_library_rs::in_ex;
///
/// // 3次元累積和
/// let a = [[1, 2], [3, 4]];
/// let mut s = [[0; 3]; 3];
///
/// for i in 0..2 {
///     for j in 0..2 {
///         s[i+1][j+1] = a[i][j] - in_ex!(s; i+1,i; j+1,j);
///     }
/// }
///
/// assert_eq!(s, [[0, 0, 0], [0, 1, 3], [0, 4, 10]]);
/// ```
#[macro_export]
macro_rules! in_ex {
    ($arr:expr ; $include:expr , $exclude:expr) => {
        ($arr[$include]) - ($arr[$exclude])
    };
    ($arr:expr ; $in0:expr , $ex0:expr $(; $in_i:expr , $ex_i:expr)+) => {
        in_ex!($arr[$in0] $(; $in_i, $ex_i)*)
        - in_ex!($arr[$ex0] $(; $in_i, $ex_i)*)
    };
}
