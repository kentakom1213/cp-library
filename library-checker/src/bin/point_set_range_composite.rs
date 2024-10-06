#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::affine1d::{Affine, AffineTransform},
    data_structure::segment_tree::SegmentTree,
    get,
    number_theory::modint::Modint,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let AB = get!(usize, usize; N);
    let queries = get!(usize, usize, usize, usize; Q);

    let mut seg: SegmentTree<Affine<Modint<998244353>>> =
        AB.iter().map(|&(a, b)| (a.into(), b.into())).collect();

    for &q in &queries {
        match q {
            (0, p, c, d) => {
                *seg.get_mut(p).unwrap() = (c.into(), d.into());
            }
            (1, l, r, x) => {
                let f = seg.get_range(l..r);
                println!("{}", f.apply(x.into()));
            }
            _ => (),
        }
    }
}
