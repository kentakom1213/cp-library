use cp_library_rs::{
    extmonoid::ExtMonoid,
    lazy_segment_tree::LazySegmentTree,
    modint::{Modint, M998},
};
use proconio::input;

struct RangeSetRangeComposite;

impl ExtMonoid for RangeSetRangeComposite {
    type X = (M998, M998);
    type M = (M998, M998);
    const IX: Self::X = (Modint::<998244353>(1), Modint::<998244353>(0));
    const IM: Self::M = (Modint::<998244353>(1), Modint::<998244353>(0));
    fn operate_x(x: &Self::X, y: &Self::X) -> Self::X {
        let &(a1, b1) = x;
        let &(a2, b2) = y;
        (a2 * a1, a2 * b1 + b2)
    }
    fn operate_m(x: &Self::M, y: &Self::M) -> Self::M {
        *y
    }
    fn aggregate(x: &Self::M, p: usize) -> Self::M {
        // let &(a, b) = x;
        // (a, b * p)
        *x
    }
    fn apply(_: &Self::X, y: &Self::M) -> Self::X {
        *y
    }
}

fn main() {
    input! {
        N: usize,
        Q: usize,
        AB: [(M998, M998); N]
    }

    let mut seg = LazySegmentTree::<RangeSetRangeComposite>::from(&AB);

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
