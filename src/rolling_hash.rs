//! ローリングハッシュ

use crate::modint_for_rollinghash::modint::Modint;

/// # RollingHash
/// 文字列の比較を高速に行う
/// - 計算量: `O(n+m)`
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
            hash[i + 1] = h.into();
            power[i + 1] = p.into();
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
    /// - 計算量: `O(1)`
    pub fn get(&self, l: usize, r: usize) -> Modint {
        self.hash[r] - self.hash[l] * self.power[r - l]
    }

    /// `0..size`のハッシュを取得
    /// - 計算量: `O(1)`
    pub fn full(&self) -> Modint {
        self.hash[self.size]
    }

    /// a,bからの最長共通接頭辞の長さを調べる
    /// - 計算量: `O(log N)`
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
    /// - 計算量: `O(1)`
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pattern_match() {
        let base = 20021213;

        let target = RollingHash::from_str("momomosumomomomomonouchi", base);
        let ptn1 = RollingHash::from_str("sumomo", base);
        let ptn2 = RollingHash::from_str("momo", base);
        let (tlen, p1len, p2len) = (24, 6, 4);

        // "sumomo"を検索
        let mut res1 = vec![];
        for i in 0..tlen - p1len {
            if target.get(i, i + p1len) == ptn1.full() {
                res1.push(i);
            }
        }

        assert_eq!(res1, vec![6]);

        // "momo"を検索
        let mut res2 = vec![];
        for i in 0..tlen - p2len {
            if target.get(i, i + p2len) == ptn2.full() {
                res2.push(i);
            }
        }

        assert_eq!(res2, vec![0, 2, 8, 10, 12, 14]);
    }

    #[test]
    fn test_concat() {
        const BASE: usize = 998244353;

        let a = "abc";
        let b = "str";
        let c = "abcstr";

        let hash_a = RollingHash::from_str(a, BASE);
        let hash_b = RollingHash::from_str(b, BASE);
        let hash_c = RollingHash::from_str(c, BASE);

        assert_eq!(
            hash_a.concat(hash_a.full(), hash_b.full(), 3),
            hash_c.full()
        );
        assert_ne!(
            hash_a.concat(hash_b.full(), hash_a.full(), 3),
            hash_c.full()
        );
        assert_eq!(
            hash_a.concat(hash_a.get(0, 3), hash_a.get(3, 3), 0),
            hash_a.full()
        );
    }

    #[test]
    fn test_LCP() {
        let rh1 = RollingHash::from_str(&"humpbump", 2023);

        assert_eq!(rh1.getLCP(0, 4), 0);
        assert_eq!(rh1.getLCP(1, 5), 3);

        let rh2 = RollingHash::from_str(&"strangeorange", 19);

        assert_eq!(rh2.getLCP(2, 8), 5);
        assert_eq!(rh2.getLCP(3, 9), 4);
    }
}
