#![allow(non_snake_case)]

use cp_library_rs::{
    consts::INF,
    debug,
    euler_tour::EulerTour,
    get,
    monoid_examples::Add,
    segment_tree::SegmentTree,
    sparse_table::{Semilattice, SparseTable},
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let mut A = get!(isize;;);
    let edges = get!(usize, usize; N - 1);

    // 根頂点を追加
    let R = N;

    // オイラーツアー
    let mut tree = EulerTour::new(N + 1);

    tree.add_edge(R, 0);

    for &(u, v) in &edges {
        tree.add_edge(u, v);
    }

    tree.build(R);

    debug!(tree);

    // 親を計算
    let par = {
        let mut p = vec![INF; N + 1];
        let mut q = vec![R];
        while let Some(u) = q.pop() {
            for &v in &tree.G[u] {
                if p[v] != INF {
                    continue;
                }
                p[v] = u;
                q.push(v);
            }
        }
        p
    };
    debug!(par);

    // LCA用のSparseTable
    let mut arr = vec![(INF, INF); 2 * N + 2];

    for v in 0..N {
        let (f, b) = tree.get_id(v);
        let d = tree.depth[v];
        arr[f] = (d, v);
        arr[b] = (d - 1, par[v]);
    }

    debug!(arr);

    let lca = SparseTable::<LCA>::build(&arr);

    // 区間処理用のセグ木
    let mut seg = SegmentTree::<Add>::new(2 * N + 2);

    for v in 0..N {
        let (f, b) = tree.get_id(v);
        *seg.get_mut(f).unwrap() += A[v];
        *seg.get_mut(b).unwrap() -= A[v];
    }

    // クエリ処理
    for _ in 0..Q {
        let q = get!(usize;;);

        match &q[..] {
            &[0, p, x] => {
                let (f, b) = tree.get_id(p);
                *seg.get_mut(f).unwrap() += x as isize;
                *seg.get_mut(b).unwrap() -= x as isize;
                A[p] += x as isize;
            }
            &[1, u, v] => {
                // lca
                let mut fu = tree.in_[u];
                let mut fv = tree.in_[v];
                if fu > fv {
                    (fu, fv) = (fv, fu);
                }
                debug!(fu, fv);
                let (d, c) = lca.get_range(fu..=fv);
                debug!(u, v, c, d);

                let to_u = seg.get_range(..tree.out[u]);
                let to_v = seg.get_range(..tree.out[v]);
                let to_c = seg.get_range(..tree.out[c]);

                debug!(to_u, to_v, to_c, A[c]);

                let ans = to_u + to_v - 2 * to_c + A[c];
                println!("{}", ans);
            }
            _ => (),
        }
    }
}

/// 区間最小値
#[derive(Debug)]
pub struct LCA;
impl Semilattice for LCA {
    type Val = (usize, usize);
    const E: Self::Val = (usize::MAX, usize::MAX);
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        *x.min(y)
    }
}
