#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{
        affine1d::{Affine, AffineTransform},
        monoid::Monoid,
    },
    data_structure::segment_tree::SegmentTree,
    debug, debug2D, get,
    graph::hld::HLD,
    number_theory::modint::M998,
    utils::iterutil::IterUtil,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let AB = get!(M998, M998; N);
    let edges = get!(usize, usize; N - 1);

    let mut hld = HLD::new(N);

    for &(u, v) in &edges {
        hld.add_edge(u, v);
    }

    hld.decompose(0);

    debug2D!(hld.G);
    debug!(hld.in_);

    let mut Wl = vec![Affine::id(); N];
    let mut Wr = vec![Affine::id(); N];

    let rev = |i| N - i - 1;

    for u in 0..N {
        Wl[hld.get_id(u)] = AB[u];
        Wr[rev(hld.get_id(u))] = AB[u];
    }

    debug!(Wl);
    debug!(Wr);

    let mut segl = SegmentTree::<Affine<M998>>::from(Wl);
    let mut segr = SegmentTree::<Affine<M998>>::from(Wr);

    for _ in 0..Q {
        let q = get!(usize, usize, usize, usize);

        match q {
            (0, p, c, d) => {
                let f = hld.get_id(p);
                let b = rev(f);
                *segl.get_mut(f).unwrap() = (c.into(), d.into());
                *segr.get_mut(b).unwrap() = (c.into(), d.into());
            }
            (1, u, v, x) => {
                let mut resl = Affine::id();
                let mut resr = Affine::id();

                debug!(u, v);
                #[cfg(debug_assertions)]
                eprintln!("{}", hld.get_path(u, v).join_debug(" -> "));

                for (a, b, _, r) in hld.get_path(u, v) {
                    if r {
                        let r = rev(hld.get_id(a)) + 1;
                        let l = rev(hld.get_id(b));
                        let tmp = segr.get_range(l..r);
                        debug!(r, l, tmp);
                        resl = Affine::op(&resl, &tmp);
                    } else {
                        let l = hld.get_id(a);
                        let r = hld.get_id(b) + 1;
                        let tmp = segl.get_range(l..r);
                        debug!(l, r, tmp);
                        resr = Affine::op(&tmp, &resr);
                    }
                }

                debug!(resl, resr);
                let res = Affine::op(&resl, &resr).apply(x.into());

                println!("{}", res);
            }
            _ => (),
        }
    }
}
