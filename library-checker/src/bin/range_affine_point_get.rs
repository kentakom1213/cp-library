#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::extmonoid_mod::AffineSum,
    data_structure::lazy_segment_tree::LazySegmentTree, get, number_theory::modint::M998,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(M998;;);

    let mut seg = LazySegmentTree::<AffineSum<998244353>>::from(&A);

    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, l, r, b, c] = &q[..] {
            seg.apply(l..r, (b.into(), c.into()));
        }

        if let &[1, i] = &q[..] {
            println!("{}", seg.get(i..=i));
        }
    }
}
