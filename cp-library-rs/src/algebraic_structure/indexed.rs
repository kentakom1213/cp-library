/// インデックスを同時に取得できるようにするラッパー
pub struct Indexed<M: Monoid>(PhantomData<M>);
impl<M> Monoid for Indexed<M>
where
    M: Monoid,
    M::Val: PartialEq,
{
    type Val = (M::Val, usize);
    fn id() -> Self::Val {
        (M::id(), INF)
    }
    fn op((l_val, l_idx): &Self::Val, (r_val, r_idx): &Self::Val) -> Self::Val {
        let val = M::op(l_val, r_val);
        if &val == l_val {
            (val, *l_idx)
        } else if &val == r_val {
            (val, *r_idx)
        } else {
            unreachable!()
        }
    }
}
