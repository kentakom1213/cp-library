use std::marker::PhantomData;

use crate::{
    algebraic_structure::{monoid::Monoid, semilattice::Semilattice},
    data_structure::segment_tree::SegmentTree,
};

/// インデックスを同時に取得できるようにするラッパー
pub struct Indexed<M>(PhantomData<M>);

impl<M> Monoid for Indexed<M>
where
    M: Monoid + Semilattice,
    <M as Monoid>::Val: PartialEq,
{
    type Val = (<M as Monoid>::Val, usize);
    fn id() -> Self::Val {
        (<M as Monoid>::id(), usize::MAX)
    }
    fn op((l_val, l_idx): &Self::Val, (r_val, r_idx): &Self::Val) -> Self::Val {
        let val = <M as Monoid>::op(l_val, r_val);
        if &val == l_val {
            (val, *l_idx)
        } else if &val == r_val {
            (val, *r_idx)
        } else {
            unreachable!()
        }
    }
}

// ========== セグ木の初期化 ==========
impl<M> SegmentTree<Indexed<M>>
where
    M: Monoid + Semilattice,
    <M as Monoid>::Val: PartialEq,
{
    /// セグメント木を初期化する
    /// - 計算量 : $`O(1)`$
    pub fn new_with_index(N: usize) -> Self {
        let arr = (0..N).map(|i| (<M as Monoid>::id(), i));
        Self::from_iter(arr)
    }
}
