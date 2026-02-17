//! 上位 K 個を保持するモノイド

#![allow(clippy::type_complexity)]

use std::cmp::Ordering;

use crate::algebraic_structure::monoid_with_context::MonoidCtx;

/// 上位 K 個を保持するモノイド
pub struct KBest<const K: usize, T> {
    /// 比較関数: ge(a, b) := a ≥ b
    ge: Box<dyn Fn(&T, &T) -> bool>,
}

impl<const K: usize, T> KBest<K, T> {
    /// 比較関数 ge を受取り，モノイド KBest を返す．
    /// - 比較関数: `ge(a, b) := a ≥ b`
    /// - ge は推移律を満たす必要がある
    pub fn new(ge: impl Fn(&T, &T) -> bool + 'static) -> Self {
        Self { ge: Box::new(ge) }
    }
}

impl<const K: usize, T: Clone> MonoidCtx for KBest<K, T> {
    type Val = [Option<T>; K];
    fn e(&self) -> Self::Val {
        std::array::from_fn(|_| None)
    }
    fn op(&self, left: &Self::Val, right: &Self::Val) -> Self::Val {
        let mut res = Vec::with_capacity(K * 2);
        res.extend_from_slice(left);
        res.extend_from_slice(right);
        res.sort_unstable_by(|a, b| match (a, b) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater,
            (Some(_), None) => Ordering::Less,
            (Some(x), Some(y)) => {
                let xy = (self.ge)(x, y);
                let yx = (self.ge)(y, x);
                if xy && yx {
                    Ordering::Equal
                } else if xy {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        });
        std::array::from_fn(|i| res[i].clone())
    }
}
