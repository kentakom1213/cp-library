#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use cp_library_rs::{
    consts::MOD998, extmonoid::ExtMonoid, extmonoid_mod::AffineSum, get,
    lazy_segment_tree::LazySegmentTree, modint::M998,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(M998;;);

    let mut seg = LazySegmentTree::<AffineSum<MOD998>>::from(&A);

    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, l, r, b, c] = &q[..] {
            seg.apply(l..r, (b.into(), c.into()));
        }

        if let &[1, l, r] = &q[..] {
            println!("{}", seg.get(l..r));
        }

        seg.show();
    }
}
