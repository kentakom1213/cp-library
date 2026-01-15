//! モノイドのラッパー

use std::marker::PhantomData;

use crate::{
    algebraic_structure::{
        monoid::Monoid,
        operation::{Max, Min},
    },
    data_structure::segment_tree::SegmentTree,
};

use num_traits::Bounded;

/// インデックスを同時に取得できるようにするラッパー
pub struct Indexed<M: Monoid>(PhantomData<M>);

// ========== セグ木の初期化 ==========
impl<T: Ord + Bounded + Clone> Monoid for Indexed<Min<T>> {
    type Val = (T, usize);
    fn id() -> Self::Val {
        (Min::id(), usize::MAX)
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        if left <= right {
            left.clone()
        } else {
            right.clone()
        }
    }
}

impl<T: Ord + Bounded + Clone> SegmentTree<Indexed<Min<T>>> {
    /// セグメント木（インデックス付きで）を初期化する
    /// - 時間計算量:  $`O(N)`$
    pub fn new_with_index(N: usize) -> Self {
        let arr = (0..N).map(|i| (Min::id(), i));
        Self::from_iter(arr)
    }
}

impl<T: Ord + Bounded + Clone> Monoid for Indexed<Max<T>> {
    type Val = (T, usize);
    fn id() -> Self::Val {
        (Max::id(), usize::MAX)
    }
    fn op(left: &Self::Val, right: &Self::Val) -> Self::Val {
        if left >= right {
            left.clone()
        } else {
            right.clone()
        }
    }
}

impl<T: Ord + Bounded + Clone> SegmentTree<Indexed<Max<T>>> {
    /// セグメント木を（インデックス付きで）初期化する
    /// - 時間計算量:  $`O(N)`$
    pub fn new_with_index(N: usize) -> Self {
        let arr = (0..N).map(|i| (Max::id(), i));
        Self::from_iter(arr)
    }
}
