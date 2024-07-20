#![allow(non_snake_case)]

use cp_library_rs::{
    debug, get, linear_algrebra::dynamic_matrix_exp::Matrix, number_theory::modint::M998,
    utils::iterutil::IterUtil,
};

fn main() {
    let (N, M, _K) = get!(usize, usize, usize);
    let A = Matrix::new(get!(M998;; N));
    let B = Matrix::new(get!(M998;; M));

    debug!(A);
    debug!(B);

    let C = A.dot(&B);

    debug!(C);

    for i in 0..N {
        println!("{}", C.arr[i].iter().join(" "));
    }
}
