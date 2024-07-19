#![allow(non_snake_case)]

use cp_library_rs::{data_structure::union_find::UnionFind, get};

fn main() {
    let (N, M) = get!(usize, usize);
    let mut edges = get!(usize, usize, usize; M)
        .into_iter()
        .enumerate()
        .map(|(i, (u, v, w))| (w, u, v, i))
        .collect::<Vec<_>>();

    edges.sort();

    let mut uf = UnionFind::new(N);

    let mut cost = 0;
    let mut ans = vec![];

    for &(w, u, v, id) in &edges {
        if uf.unite(u, v) {
            cost += w;
            ans.push(id);
        }
    }

    println!("{}", cost);
    println!(
        "{}",
        ans.iter().fold(String::new(), |mut s, x| {
            if s.is_empty() {
                s += &format!("{}", x);
            } else {
                s += &format!(" {}", x);
            }
            s
        })
    );
}
