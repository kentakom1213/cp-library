use cp_library_rs::union_find::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }

        if t == 0 {
            uf.unite(u, v);
        } else {
            let res = uf.is_same(u, v);

            println!("{}", res as usize);
        }
    }
}
