#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug, debug2D,
    get, graph::hld::HLD, utils::iterutil::IterUtil,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(usize;;);
    let edges = get!(usize, usize; N - 1);

    let mut hld = HLD::new(N);

    for &(u, v) in &edges {
        hld.add_edge(u, v);
    }

    hld.decompose(0);

    debug!(hld.subtree_size);
    debug2D!(hld.G);
    debug!(hld.parent);
    debug!(hld.in_);
    debug!(hld.out);
    debug!(hld.head);

    // セグ木
    let mut W = vec![0; N];

    for i in 0..N {
        W[hld.get_id(i)] = A[i];
    }

    let mut seg = SegmentTree::<Add<usize>>::from(W);

    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, p, x] = &q[..] {
            let id = hld.get_id(p);
            *seg.get_mut(id).unwrap() += x;
        }

        if let &[1, u, v] = &q[..] {
            let mut ans = 0;

            #[cfg(debug_assertions)]
            eprintln!("{}", hld.get_path(u, v).join_debug(" -> "));

            for (a, b, _, _) in hld.get_path(u, v) {
                let l = hld.get_id(a);
                let r = hld.get_id(b);
                ans += seg.get_range(l..=r);
            }

            println!("{}", ans);
        }
    }
}
