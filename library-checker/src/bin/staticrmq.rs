use cp_library_rs::sparse_table::{Alg::Min, SparseTable};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [isize; N],
        LR: [(usize, usize); Q]
    }

    let tb = SparseTable::<Min>::build(&A);

    for &(l, r) in &LR {
        println!("{}", tb.get_range(l..r));
    }
}
