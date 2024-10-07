#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug, get,
};

fn main() {
    let (N, Q) = get!(usize, usize);
    let T = get!(String);

    let set = T
        .chars()
        .enumerate()
        .fold(vec![0; N + 5], |mut set, (x, e)| {
            if e == '1' {
                set[x] = 1;
            }
            set
        });

    let mut set = SegmentTree::<Add<usize>>::from(set);

    for _ in 0..Q {
        let (c, k) = get!(usize, usize);

        match c {
            0 => {
                if let Some(mut x) = set.get_mut(k) {
                    if *x == 0 {
                        *x = 1;
                    }
                }
            }
            1 => {
                if let Some(mut x) = set.get_mut(k) {
                    if *x == 1 {
                        *x = 0;
                    }
                }
            }
            2 => {
                println!("{}", set[k]);
            }
            3 => {
                let (cnt, res) = set.max_right(k, |x| x <= 0);

                if res <= N && set.get_range(k..=res) == 1 {
                    println!("{res}");
                } else {
                    println!("-1");
                }
            }
            4 => {
                let (cnt, res) = set.min_left(k + 1, |x| x <= 0);

                if res >= 0 && set.get_range(res.saturating_sub(1)..=k) == 1 {
                    println!("{}", res.saturating_sub(1));
                } else {
                    println!("-1");
                }
            }
            _ => (),
        }
    }
}
