#![allow(non_snake_case)]

use std::{cmp::max, collections::VecDeque};

use cp_library_rs::{
    debug, get,
    graph::{
        dijkstra::{dijkstra, path_reconstruction},
        rerooting::{self, examples::Diameter, Rerooting},
    },
    utils::{consts::INF, iterutil::IterUtil},
};

fn main() {
    let N = get!(usize);
    let edges = get!(usize, usize, isize; N - 1);

    let mut rerooting = Rerooting::<Diameter>::new(N);

    for (u, v, w) in edges {
        rerooting.add_edge2(u, v, w);
    }

    rerooting.build();

    debug!(rerooting.ans);

    let diameter = rerooting.ans.iter().max().unwrap();

    let (mut u, mut v) = (INF, INF);

    for i in 0..N {
        if rerooting.ans[i] == *diameter {
            if u == INF {
                u = i;
            } else {
                v = i;
            }
        }
    }

    assert_ne!(u, INF, "ans: {:?}", rerooting.ans);
    assert_ne!(v, INF, "ans: {:?}", rerooting.ans);

    debug!(u, v);

    // u → v へのパスを構築
    let prev = {
        let mut prev = vec![INF; N];
        let mut seen = vec![false; N];
        seen[u] = true;
        let mut q = VecDeque::from([u]);
        while let Some(u) = q.pop_front() {
            seen[u] = true;
            for e in &rerooting.G[u] {
                let v = e.to;
                if seen[v] {
                    continue;
                }
                prev[v] = u;
                q.push_back(v);
            }
        }
        prev
    };

    debug!(prev, prev[u], prev[v]);

    let path = path_reconstruction(u, v, &prev).unwrap();

    // 出力
    println!("{} {}", diameter, path.len());
    println!("{}", path.iter().join(" "));
}
