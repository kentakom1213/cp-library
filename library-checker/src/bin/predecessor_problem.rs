#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add,
    data_structure::multiset_splay_tree::{Multiset, NodeOps},
    debug, get,
};
fn main() {
    let (N, Q) = get!(usize, usize);
    let T = get!(String);

    let mut set = T
        .chars()
        .enumerate()
        .fold(Multiset::new(), |mut set, (x, e)| {
            if e == '1' {
                set.insert(x);
            }
            set
        });

    // set.print_as_tree();

    for _ in 0..Q {
        let (c, x) = get!(usize, usize);

        match c {
            0 => {
                if set.count(&x) == 0 {
                    set.insert(x);
                }
            }
            1 => {
                set.remove(&x);
            }
            2 => {
                println!("{}", set.count(&x));
            }
            3 => {
                if let Some(nxt) = set.range(x..).next() {
                    println!("{}", nxt.key());
                } else {
                    println!("-1");
                }
            }
            4 => {
                if let Some(prv) = set.range(..=x).next_back() {
                    println!("{}", prv.key());
                } else {
                    println!("-1");
                }
            }
            _ => (),
        }

        // set.print_as_tree();
    }
}
