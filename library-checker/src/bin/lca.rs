#![allow(non_snake_case)]

use std::io::{stdout, BufWriter, Write};

use cp_library_rs::{get, graph::hld::HLD};

fn main() {
    let (N, Q) = get!(usize, usize);
    let P = get!(usize;;);

    let buf = &mut BufWriter::new(stdout().lock());

    let mut hld = HLD::new(N);

    for (u, &v) in (1..).zip(&P) {
        hld.add_edge(u, v);
    }

    hld.decompose(0);

    for _ in 0..Q {
        let (u, v) = get!(usize, usize);

        let p = hld.get_lca(u, v);

        writeln!(buf, "{p}").unwrap();
    }

    buf.flush().unwrap();
}
