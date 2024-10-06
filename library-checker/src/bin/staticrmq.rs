#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Min, data_structure::sparse_table::SparseTable, get,
};

fn main() {
    let (_N, Q) = get!(usize, usize);
    let A = get!(isize;;);
    let LR = get!(usize, usize; Q);

    let tb = SparseTable::<Min<_>>::build(&A);

    for &(l, r) in &LR {
        println!("{}", tb.get_range(l..r));
    }
}
