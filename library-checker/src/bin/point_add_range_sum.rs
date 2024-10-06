#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, get,
};

fn main() {
    let (_N, Q) = get!(usize, usize);
    let A = get!(isize;;);

    let mut seg = SegmentTree::<Add<isize>>::from(A);

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
