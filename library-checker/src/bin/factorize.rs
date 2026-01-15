#![allow(non_snake_case)]

use cp_library_rs::{get, number_theory::pollard_rho_algorithm::factorize_pollard_rho};

fn main() {
    let Q = get!(usize);

    for _ in 0..Q {
        let a = get!(usize);

        let factors = factorize_pollard_rho(a);

        print!("{}", factors.len());
        for &v in &factors {
            print!(" {}", v);
        }
        println!();
    }
}
