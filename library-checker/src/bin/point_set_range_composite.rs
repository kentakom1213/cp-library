#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use cp_library_rs::{
    affine1d::AffineTransform, get, modint::M998, monoid_mod::Affine1dMod,
    segment_tree::SegmentTree,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let AB = get!(usize, usize; N);
    let queries = get!(usize, usize, usize, usize; Q);

    let mut seg = SegmentTree::<Affine1dMod<998244353>>::from(
        &AB.iter()
            .map(|&(a, b)| (M998::new(a), M998::new(b)))
            .collect(),
    );

    for q in &queries {
        match q {
            &(0, p, c, d) => {
                *seg.get_mut(p).unwrap() = (c.into(), d.into());
            }
            &(1, l, r, x) => {
                let f = seg.get_range(l..r);
                println!("{}", f.apply(x.into()));
            }
            _ => (),
        }
    }
}
