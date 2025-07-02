//! 数論変換

use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, MulAssign, Sub},
};

use crate::{
    number_theory::modint::{Fp, M998},
    utils::num_traits::Zero,
};

/// FFTに必要な関数
pub trait NTTFriendly<Rhs = Self, Output = Self>:
    Clone
    + Copy
    + Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + MulAssign<Rhs>
    + Zero
    + From<u32>
    + Fp
{
    /// M = 2^k * m + 1 を満たすような k
    fn order() -> u32;
    /// M = 2^k * m + 1 を満たすような m
    fn rem() -> u32;
    /// 原始根
    fn root() -> Self;
    /// 2^m 乗根
    fn root_pow2m(a: u32) -> Self {
        let p = Self::rem() << (Self::order() - a);
        Self::root().pow(p)
    }
}

impl NTTFriendly for M998 {
    fn order() -> u32 {
        23
    }
    fn rem() -> u32 {
        119
    }
    fn root() -> Self {
        Self(3)
    }
}

/// 高速フーリエ変換の実装
pub struct FFT<T: NTTFriendly>(PhantomData<T>);

impl<T: NTTFriendly> FFT<T> {
    /// 入力された配列をフーリエ変換する
    pub fn fft(X: &[T]) -> Result<Vec<T>, &'static str> {
        let (i, X) = Self::extend_array(X)?;
        let w = T::root_pow2m(i);
        Ok(Self::fft_core(X, w))
    }

    /// 入力された配列をフーリエ逆変換する
    pub fn ifft(F: &[T]) -> Result<Vec<T>, &'static str> {
        let (i, F) = Self::extend_array(F)?;
        let w = T::root_pow2m(i);
        let winv = w.inv();
        let mut res = Self::fft_core(F, winv);
        let n = res.len() as u32;
        // 逆変換後の配列を正規化
        let inv_n = T::from(n).inv();
        res.iter_mut().for_each(|v| *v *= inv_n);
        Ok(res)
    }

    /// フーリエ変換，フーリエ逆変換の共通部分
    ///
    /// - `w`: 回転演算子
    fn fft_core(X: Vec<T>, w: T) -> Vec<T> {
        let n = X.len();

        if n == 1 {
            return X.to_vec();
        }

        let (X_even, X_odd): (Vec<_>, Vec<_>) = (0..n / 2)
            .map(|i| {
                let l = X[i];
                let r = X[i + n / 2];
                (l + r, w.pow(i as u32) * (l - r))
            })
            .unzip();

        // 再帰的にFFT
        let new_w = w.pow(2);
        let Y_even = Self::fft_core(X_even, new_w);
        let Y_odd = Self::fft_core(X_odd, new_w);

        // マージ
        Y_even
            .into_iter()
            .zip(Y_odd)
            .flat_map(|(e, o)| [e, o])
            .collect()
    }

    /// 長さが 2 べきになるように配列を生成する
    ///
    /// **Arguments**
    /// - `array`: 配列
    ///
    /// **Returns**
    /// - `(i, res)`: 配列の長さを 2^i に拡張した結果
    fn extend_array(array: &[T]) -> Result<(u32, Vec<T>), &'static str> {
        let n = array.len();
        // 2^i >= n となるような最小の i
        let mut i = 0;
        let mut n_ = 1;
        while n_ < n {
            i += 1;
            n_ *= 2;
        }

        if i > T::order() {
            return Err("The prime p does not have enough factors of 2 in (p - 1).");
        }

        // 配列を生成
        let mut res = array.to_vec();

        // 残りをゼロ埋め
        res.extend((0..n_ - n).map(|_| T::zero()));

        Ok((i, res))
    }
}
