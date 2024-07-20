#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::examples::Add, data_structure::segment_tree::SegmentTree, debug,
    get, graph::euler_tour::EulerTour,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(isize;;);
    let P = get!(usize;;);

    // 根頂点（頂点0の親）を追加
    let R = N;

    let mut tree = EulerTour::new(N + 1);

    tree.add_edge(R, 0);

    for (v, &p) in (1..).zip(&P) {
        tree.add_edge(p, v);
    }

    tree.build(R);

    debug!(tree);

    // 1点取得 & 区間和 のためのセグ木
    let mut seg: SegmentTree<Add<isize>> = SegmentTree::new(2 * N);

    for v in 0..N {
        let f = tree.in_[v];
        *seg.get_mut(f).unwrap() += A[v];
    }

    debug!(seg);

    // クエリ処理
    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, u, x] = &q[..] {
            let f = tree.in_[u];
            *seg.get_mut(f).unwrap() += x as isize;
        }

        if let &[1, u] = &q[..] {
            let f = tree.in_[u];
            let b = tree.out[u];
            let res = seg.get_range(f..b);
            println!("{}", res);
        }
    }
}
