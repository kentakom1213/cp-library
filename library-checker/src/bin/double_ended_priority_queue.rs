#![allow(non_snake_case)]

use cp_library_rs::{data_structure::multiset_splay_tree::MultiSet, get, utils::consts::IINF};

fn main() {
    let (_N, Q) = get!(usize, usize);
    let S = get!(isize;;);

    let mut mset = MultiSet::from_iter(S);

    for _ in 0..Q {
        let q = get!(isize;;);

        match &q[..] {
            &[0, x] => {
                mset.insert(x);
            }
            [1] => {
                let mini = *mset.lower_bound(&-IINF).unwrap();
                println!("{}", mini);
                mset.delete(&mini);
            }
            [2] => {
                let maxi = *mset.lower_bound_rev(&IINF).unwrap();
                println!("{}", maxi);
                mset.delete(&maxi);
            }
            _ => (),
        }
    }
}
