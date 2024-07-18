#![allow(non_snake_case)]

use cp_library_rs::{debug2D, ford_fulkerson::FordFulkerson, get};

fn main() {
    let (L, R, M) = get!(usize, usize, usize);
    let AB = get!(usize, usize; M);

    let UV: Vec<(usize, usize)> = AB.iter().map(|&(a, b)| (a, L + b)).collect();

    let mut ff = FordFulkerson::new(L + R + 2);

    let (S, T) = (L + R, L + R + 1);

    for &(u, v) in &UV {
        ff.add_edge(u, v, 1);
    }

    for u in 0..L {
        ff.add_edge(S, u, 1);
    }

    for v in L..L + R {
        ff.add_edge(v, T, 1);
    }

    let mf = ff.max_flow(S, T);

    debug2D!(ff.G);

    println!("{mf}");

    for &(u, v) in &UV {
        if ff.get_flow(u, v) == 1 {
            println!("{} {}", u, v - L);
        }
    }
}
