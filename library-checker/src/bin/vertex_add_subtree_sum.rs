#![allow(non_snake_case)]

use cp_library_rs::{bit::Alg::Add, debug, euler_tour::EulerTour, get, segment_tree::SegmentTree};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(isize;;);
    let P = get!(usize;;);
    let queries = get!(usize, usize; Q);

    // 根頂点（頂点0の親）を追加
    let R = N;

    let mut tree = EulerTour::new(N + 1);

    tree.add_edge(R, 0);

    for (v, &p) in (1..).zip(&P) {
        tree.add_edge(p, v);
    }

    tree.build(R);

    debug!(tree);

    // セグ木
    let mut seg = SegmentTree::<Add>::new(N);

    for i in 0..N {
        let f = tree.in_[i];
        *seg.get_mut(f).unwrap() += A[i];
    }
}
