//! ビット列を管理する

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut, Index},
};

/// ビット列を高速に処理する
#[derive(Clone)]
pub struct BitSet<const SIZE: usize> {
    bits: Vec<u64>,
}

impl<const SIZE: usize> BitSet<SIZE> {
    /// ⌈size / 64⌉個のu64
    const ARRAY_SIZE: usize = (SIZE + 64 - 1) / 64;

    /// あまりのビット
    const REM_BIT: usize = SIZE % 64;

    /// 一時的な値
    const TMP_BOOL: [bool; 2] = [false, true];

    /// Bitsetを初期化する
    /// - `size`: ビットの数
    pub fn new() -> Self {
        Self {
            bits: vec![0; Self::ARRAY_SIZE],
        }
    }

    /// `idx`bit目を1に設定
    pub fn set(&mut self, index: usize) {
        let arr_idx = index / 64;
        let bit_idx = index % 64;
        self.bits[arr_idx] |= 1 << bit_idx;
    }

    /// `idx`bit目を0に設定
    pub fn unset(&mut self, index: usize) {
        let arr_idx = index / 64;
        let bit_idx = index % 64;
        self.bits[arr_idx] &= !(1 << bit_idx);
    }

    /// `idx`bit目を反転
    pub fn flip(&mut self, index: usize) {
        if self[index] {
            self.unset(index);
        } else {
            self.set(index);
        }
    }

    /// すべてのbitが0になっているかを判定する
    pub fn any(&self) -> bool {
        self.bits.iter().all(|&b64| b64 == 0)
    }

    /// すべてのbitが1になっているかを判定する
    pub fn all(&self) -> bool {
        // あまりだけ個別に判定
        let filter = !0_u64 >> (64 - Self::REM_BIT);
        self.bits[Self::ARRAY_SIZE - 1] ^ filter == 0
            && self
                .bits
                .iter()
                .take(Self::ARRAY_SIZE - 1)
                .all(|&b64| b64 == !0)
    }

    /// あるbitを更新する
    fn update(&mut self, index: usize, new_val: bool) {
        if new_val {
            self.set(index);
        } else {
            self.unset(index);
        }
    }

    /// 1であるビットの数を求める
    pub fn count_ones(&self) -> usize {
        self.bits
            .iter()
            .map(|b64| b64.count_ones() as usize)
            .sum::<usize>()
    }

    /// あるbitの可変参照を取得する
    /// - `index`: 取得するbitのインデックス
    pub fn get_mut(&mut self, index: usize) -> Option<BitMut<'_, SIZE>> {
        if index < SIZE {
            let default = self[index];
            Some(BitMut {
                bitset: self,
                idx: index,
                new_val: default,
            })
        } else {
            None
        }
    }
}

impl<const SIZE: usize> Index<usize> for BitSet<SIZE> {
    type Output = bool;
    fn index(&self, index: usize) -> &Self::Output {
        let arr_idx = index / 64;
        let bit_idx = index % 64;
        if self.bits[arr_idx] >> bit_idx & 1 == 0 {
            &Self::TMP_BOOL[0]
        } else {
            &Self::TMP_BOOL[1]
        }
    }
}

/// bitsetの更新を行う
pub struct BitMut<'a, const SIZE: usize> {
    bitset: &'a mut BitSet<SIZE>,
    idx: usize,
    new_val: bool,
}

impl<const SIZE: usize> Deref for BitMut<'_, SIZE> {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.new_val
    }
}

impl<const SIZE: usize> DerefMut for BitMut<'_, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new_val
    }
}

impl<const SIZE: usize> Drop for BitMut<'_, SIZE> {
    fn drop(&mut self) {
        self.bitset.update(self.idx, self.new_val);
    }
}

impl<const SIZE: usize> Debug for BitSet<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bit_str = format!("{:b}", self.bits[Self::ARRAY_SIZE - 1]);
        // ゼロ埋め
        bit_str = "0".repeat(Self::REM_BIT - bit_str.len()) + &bit_str;
        bit_str = self.bits[..Self::ARRAY_SIZE - 1]
            .iter()
            .rev()
            .map(|b64| format!(",{:0>64b}", b64))
            .fold(bit_str, |acc, b64| acc + &b64);
        write!(f, "BitSet {{ {:?} }}", bit_str)
    }
}
