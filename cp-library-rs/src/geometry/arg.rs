//! 偏角ソート

use std::cmp::Ordering;

/// 2 次元ベクトルの偏角
/// - Ord は x 軸正の向きを基準にした偏角順
/// - 参考: <https://ngtkana.hatenablog.com/entry/2021/11/13/202103>
#[derive(Debug, Clone, Copy)]
pub struct Arg(pub i64, pub i64);

impl Arg {
    pub fn to_tuple(&self) -> (i64, i64) {
        (self.0, self.1)
    }
}

impl From<(i64, i64)> for Arg {
    fn from((x, y): (i64, i64)) -> Self {
        Self(x, y)
    }
}

impl From<&(i64, i64)> for Arg {
    fn from(&(x, y): &(i64, i64)) -> Self {
        Self(x, y)
    }
}

impl Ord for Arg {
    fn cmp(&self, other: &Self) -> Ordering {
        let &Arg(x0, y0) = self;
        let &Arg(x1, y1) = other;

        ((y0, x0) < (0, 0))
            .cmp(&((y1, x1) < (0, 0)))
            .then_with(|| (x1 * y0).cmp(&(x0 * y1)))
    }
}

impl PartialOrd for Arg {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Arg {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Arg {}
