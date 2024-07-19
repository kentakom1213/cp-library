#![allow(non_snake_case)]

use cp_library_rs::{
    get,
    graph::dijkstra::{dijkstra, path_reconstruction},
};

fn main() {
    let (N, M, S, T) = get!(usize, usize, usize, usize);
    let UVW = get!(usize, usize, usize; M);

    let mut G = vec![vec![]; N];

    for &(u, v, w) in &UVW {
        G[u].push((v, w));
    }

    let (prev, dist) = dijkstra(&G, S);
    let ans = dist[T];

    let path = path_reconstruction(S, T, &prev);

    if let Some(path) = path {
        // 経路の表示
        let len = path.len() - 1;
        println!("{} {}", ans, len);
        for i in 0..len {
            println!("{} {}", path[i], path[i + 1]);
        }
    } else {
        println!("-1");
    }
}
