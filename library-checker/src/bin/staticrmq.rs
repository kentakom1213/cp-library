#![allow(non_snake_case)]

use cp_library_rs::{
    get,
    sparse_table::{Alg::Min, SparseTable},
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(isize;;);
    let LR = get!(usize, usize; Q);

    let tb = SparseTable::<Min>::build(&A);

    for &(l, r) in &LR {
        println!("{}", tb.get_range(l..r));
    }
}
