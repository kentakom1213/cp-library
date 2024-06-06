#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use cp_library_rs::{
    extmonoid::ExtMonoid, extmonoid_mod::AffineSum, lazy_segment_tree::LazySegmentTree,
    modint::M998,
};
use proconio::{fastout, input};

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [M998; N],
    }

    let mut seg = LazySegmentTree::<AffineSum<MOD>>::from(&A);

    for _ in 0..Q {
        input! {
            t: usize
        }

        if t == 0 {
            input! {
                l: usize,
                r: usize,
                b: usize,
                c: usize,
            }
            seg.apply(l..r, (b.into(), c.into()));
        } else if t == 1 {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", seg.get(l..r));
        }

        if cfg!(debug_assertions) {
            eprintln!("{:?}", seg);
            eprintln!("seg: {}", seg.show());
        }
    }
}
