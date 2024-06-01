#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use cp_library_rs::{extmonoid::ExtMonoid, lazy_segment_tree::LazySegmentTree, modint::M998};
use proconio::{fastout, input};

const MOD: usize = 998244353;

struct Affine;

impl ExtMonoid for Affine {
    type X = usize;
    type M = (usize, usize);
    const IX: Self::X = 0;
    const IM: Self::M = (1, 0);
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
        let mut res = *x + *y;
        if res >= MOD {
            res -= MOD;
        }
        res
    }
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
        let &(a1, b1) = x;
        let &(a2, b2) = y;
        //   a2 * (a1 * x + b1) + b2
        // = (a2 * a1) * x + (a2 * b1 + b2)
        (a2 * a1 % MOD, (a2 * b1 % MOD + b2) % MOD)
    }
    fn apply(x: &Self::X, y: &Self::M) -> Self::X {
        let &(a, b) = y;
        (a * *x % MOD + b) % MOD
    }
    fn aggregate(x: &Self::M, p: usize) -> Self::M {
        let &(a, b) = x;
        (a, b * p % MOD)
    }
}

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
    }

    let mut seg = LazySegmentTree::<Affine>::from(&A);

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
        } else {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", seg.get(l..r));
        }
    }
}
