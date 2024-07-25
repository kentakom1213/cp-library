//! ローリングハッシュ

use crate::number_theory::modint_for_rollinghash::modint::Modint;

/// ローリングハッシュ
///
/// 文字列をハッシュし，連続部分列の一致判定を $`O(1)`$ で行う．
#[derive(Debug)]
pub struct RollingHash {
    pub size: usize,
    power: Vec<Modint>,
    hash: Vec<Modint>,
    base: Modint,
}

impl RollingHash {
    /// 初期化
    pub fn build(arr: &[Modint], base: usize) -> Self {
        let size = arr.len();
        let mut power = vec![Modint(0); size + 1];
        let mut hash = vec![Modint(0); size + 1];

        // hashを初期化
        let (mut h, mut p) = (Modint(0), Modint(1));
        for i in 0..size {
            h = arr[i] + (h * base);
            p *= base;
            hash[i + 1] = h;
            power[i + 1] = p;
        }

        Self {
            size,
            power,
            hash,
            base: base.into(),
        }
    }

    /// 文字列から生成
    pub fn from_str(s: &str, base: usize) -> Self {
        let arr: Vec<Modint> = s.chars().map(Self::ord).map(Modint).collect();
        Self::build(&arr, base)
    }

    /// `l..r`のハッシュを取得
    /// - 計算量: $`O(1)`$
    pub fn get(&self, l: usize, r: usize) -> Modint {
        self.hash[r] - self.hash[l] * self.power[r - l]
    }

    /// `0..size`のハッシュを取得
    /// - 計算量: $`O(1)`$
    pub fn full(&self) -> Modint {
        self.hash[self.size]
    }

    /// a,bからの最長共通接頭辞の長さを調べる
    /// - 計算量: $`O(\log N)`$
    pub fn getLCP(&self, a: usize, b: usize) -> usize {
        let len = self.size.saturating_sub(a.max(b));
        let (mut lo, mut hi) = (0, len + 1);
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.get(a, a + mid) == self.get(b, b + mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo
    }

    /// ハッシュ同士を連結
    /// - 計算量: $`O(1)`$
    pub fn concat(&self, h1: Modint, h2: Modint, h2_len: usize) -> Modint {
        h1 * self.power[h2_len] + h2
    }

    /// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
    #[inline]
    fn ord(c: char) -> usize {
        let a = 'A' as u32;
        let c = c as u32;
        (c - a) as usize
    }
}
