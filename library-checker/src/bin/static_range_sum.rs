#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(usize;;);
    let LR = get!(usize, usize; Q);

    let mut S = vec![0; N + 1];

    for i in 0..N {
        S[i + 1] = S[i] + A[i];
    }

    for &(l, r) in &LR {
        println!("{}", S[r] - S[l]);
    }
}
