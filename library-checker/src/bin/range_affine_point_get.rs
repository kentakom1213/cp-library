use cp_library_rs::{extmonoid_mod::AffineSum, lazy_segment_tree::LazySegmentTree, modint::M998};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [M998; N]
    }

    let mut seg = LazySegmentTree::<AffineSum<998244353>>::from(&A);

    for _ in 0..Q {
        input! {
            t: usize
        }

        if t == 0 {
            input! {
                l: usize,
                r: usize,
                b: M998,
                c: M998,
            }
            seg.apply(l..r, (b.into(), c.into()));
        } else {
            input! {
                i: usize
            }
            println!("{}", seg.get(i..=i));
        }
    }
}
