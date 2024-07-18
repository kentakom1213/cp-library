use cp_library_rs::{get, union_find::UnionFind};

fn main() {
    let (n, q) = get!(usize, usize);

    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        let (t, u, v) = get!(usize, usize, usize);

        if t == 0 {
            uf.unite(u, v);
        } else {
            let res = uf.is_same(u, v);

            println!("{}", res as usize);
        }
    }
}
