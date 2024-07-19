use cp_library_rs::{
    algebraic_structure::monoid::Monoid, data_structure::dynamic_segment_tree::DynamicSegmentTree,
    get,
};

struct Set;

impl Monoid for Set {
    type Val = usize;
    fn id() -> Self::Val {
        0
    }
    fn op(_left: &Self::Val, _right: &Self::Val) -> Self::Val {
        0
    }
}

fn main() {
    let Q = get!(usize);

    let mut tree = DynamicSegmentTree::<usize, Set>::new();

    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, k, v] = &q[..] {
            // 挿入
            *tree.get_mut(k) = v;
        }

        if let &[1, k] = &q[..] {
            let ans = tree.get(&k);
            println!("{ans}");
        }
    }
}
