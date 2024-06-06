use cp_library_rs::{
    extmonoid::ExtMonoid,
    extmonoid_mod::AffineUpdateComposite,
    lazy_segment_tree::LazySegmentTree,
    modint::{Modint, M998},
};
use proconio::input;

const M: usize = 998244353;

fn main() {
    input! {
        N: usize,
        Q: usize,
        AB: [(M998, M998); N]
    }

    let mut seg = LazySegmentTree::<AffineUpdateComposite<M>>::from(&AB);

    for _ in 0..Q {
        input! {
            t: usize,
        }

        if t == 0 {
            input! {
                l: usize,
                r: usize,
                c: M998,
                d: M998,
            }
            seg.apply(l..r, (c, d));
        } else {
            input! {
                l: usize,
                r: usize,
                x: M998,
            }
            let (a, b) = seg.get(l..r);
            let ans = a * x + b;
            println!("{ans}");
        }
    }
}
