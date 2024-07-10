use cp_library_rs::{monoid_examples::Add, segment_tree::SegmentTree};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [isize; N],
    }

    let mut seg = SegmentTree::<Add>::from(&A);

    for _ in 0..Q {
        input! {
            t: usize,
        }

        if t == 0 {
            input! {
                p: usize,
                x: isize,
            }

            *seg.get_mut(p).unwrap() += x;
        } else {
            input! {
                l: usize,
                r: usize,
            }

            let ans = seg.get_range(l..r);

            println!("{ans}");
        }
    }
}
