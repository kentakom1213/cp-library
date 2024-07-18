#![allow(non_snake_case)]

use std::collections::BTreeSet;

use cp_library_rs::get;

fn main() {
    let (N, Q) = get!(usize, usize);
    let T = get!(String);

    let mut set = T
        .chars()
        .enumerate()
        .fold(BTreeSet::new(), |mut set, (x, e)| {
            if e == '1' {
                set.insert(x as isize);
            }
            set
        });

    for _ in 0..Q {
        let (c, k) = get!(usize, isize);

        match c {
            0 => {
                if set.get(&k).is_none() {
                    set.insert(k);
                }
            }
            1 => {
                set.remove(&k);
            }
            2 => {
                println!("{}", set.contains(&k) as usize);
            }
            3 => {
                let res = *set.range(k..).next().unwrap_or(&(-1));
                println!("{res}");
            }
            4 => {
                let res = *set.range(..=k).next_back().unwrap_or(&(-1));
                println!("{res}");
            }
            _ => (),
        }
    }
}
