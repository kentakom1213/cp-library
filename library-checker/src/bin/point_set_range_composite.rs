#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use cp_library_rs::{
    algebraic_structure::{
        affine1d::{Affine, AffineTransform},
        monoid::Monoid,
    },
    data_structure::segment_tree::SegmentTree,
    get,
    number_theory::modint::M998,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let AB = get!(usize, usize; N);
    let queries = get!(usize, usize, usize, usize; Q);

    let mut seg: SegmentTree<Affine<M998>> = SegmentTree::from(
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
