#![allow(non_snake_case)]

use cp_library_rs::{
    consts::MOD998, extmonoid_mod::AffineUpdateComposite, get, lazy_segment_tree::LazySegmentTree,
    modint::M998,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let AB = get!(M998, M998; N);

    let mut seg = LazySegmentTree::<AffineUpdateComposite<MOD998>>::from(&AB);

    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, l, r, c, d] = &q[..] {
            seg.apply(l..r, (c.into(), d.into()));
        }

        if let &[1, l, r, x] = &q[..] {
            let (a, b) = seg.get(l..r);
            let ans = a * x + b;
            println!("{ans}");
        }
    }
}
