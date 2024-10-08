#![allow(non_snake_case)]

use cp_library_rs::{get, string::suffix_array::SuffixArray};

fn main() {
    let S = get!(String);

    let SA = SuffixArray::build(&S);

    for i in 1..SA.len() {
        if i == SA.len() - 1 {
            println!("{}", SA[i]);
        } else {
            print!("{} ", SA[i]);
        }
    }
}
