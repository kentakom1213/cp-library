#![allow(non_snake_case)]

use cp_library_rs::{get, pollard_rho_algorithm::factorize};

fn main() {
    let Q = get!(usize);

    for _ in 0..Q {
        let a = get!(usize);

        let factors = factorize(a);

        print!("{}", factors.len());
        for &v in &factors {
            print!(" {}", v);
        }
        println!();
    }
}
