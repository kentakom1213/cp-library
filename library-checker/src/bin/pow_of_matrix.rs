#![allow(non_snake_case)]

use cp_library_rs::{
    get, linear_algrebra::dynamic_matrix_exp::Matrix, number_theory::modint::M998,
    utils::iterutil::IterUtil,
};

fn main() {
    let (N, K) = get!(usize, usize);
    let A = get!(M998;; N);

    let A = Matrix::new(A);

    let res = A.pow(K);

    for i in 0..N {
        println!("{}", res.arr[i].iter().join(" "));
    }
}
