use cp_library_rs::{
    debug,
    dijkstra::{dijkstra, path_reconstruction},
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        s: usize,
        t: usize,
        UVW: [(usize, usize, usize); M]
    }

    let mut G = vec![vec![]; N];

    for &(u, v, w) in &UVW {
        G[u].push((v, w));
    }

    let (prev, dist) = dijkstra(&G, s);
    let ans = dist[t];

    let path = path_reconstruction(s, t, &prev);

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
