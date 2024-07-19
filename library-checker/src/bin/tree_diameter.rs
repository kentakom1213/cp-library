#![allow(non_snake_case)]

use cp_library_rs::{
    get,
    graph::rerooting::{examples::Diameter, Rerooting},
};

fn main() {
    let N = get!(usize);
    let edges = get!(usize, usize, isize; N - 1);

    let mut T = Rerooting::<Diameter>::new(N);

    for &(u, v, w) in &edges {
        T.add_edge2(u, v, w);
    }

    T.build();

    let ans = T.ans.iter().max().unwrap();
    println!("{}", ans);
}
