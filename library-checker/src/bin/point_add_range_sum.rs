#![allow(non_snake_case)]

use cp_library_rs::{get, monoid_examples::Add, segment_tree::SegmentTree};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(isize;;);

    let mut seg = SegmentTree::<Add>::from(&A);

    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, p, x] = &q[..] {
            *seg.get_mut(p).unwrap() += x as isize;
        }

        if let &[1, l, r] = &q[..] {
            let ans = seg.get_range(l..r);

            println!("{ans}");
        }
    }
}
