//! 2次元グリッドをジグザグに走査する

/// `H x W`領域をジグザグに操作する
///
/// ↓3x3領域の例
/// ```text
/// 123
/// 654
/// 789
/// ```
pub fn zigzag(H: usize, W: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..H).zip(1..).step_by(2).flat_map(move |(t, b)| {
        (0..W)
            .map(move |j| (t, j))
            .chain((0..if b < H { W } else { 0 }).rev().map(move |j| (b, j)))
    })
}
