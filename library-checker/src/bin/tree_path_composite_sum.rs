#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::affine1d::{Affine, AffineTransform},
    get,
    graph::rerooting::{Rerooting, TreeMonoid},
    number_theory::modint::M998,
};

fn main() {
    let N = get!(usize);
    let _A = get!(M998;;);
    let UVBC = get!(usize, usize, M998, M998; N - 1);

    // 全方位木DP
    let mut tree = Rerooting::<Composite>::new(N);

    for &(u, v, b, c) in &UVBC {
        tree.add_edge2(u, v, (b, c));
    }
}

struct Composite;

impl TreeMonoid for Composite {
    type T = Affine<M998>;
    type W = Affine<M998>;
    fn id() -> Self::T {
        AffineTransform::id_()
    }
    fn merge(x: &Self::T, y: &Self::T) -> Self::T {
        x.compose(y)
    }
    fn put_edge(x: &Self::T, weight: &Self::W) -> Self::T {
        x.compose(weight)
    }
}
