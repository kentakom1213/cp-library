use cp_library_rs::{dynamic_segment_tree::DynamicSegmentTree, monoid::Monoid};
use proconio::{fastout, input};

struct Set;

impl Monoid for Set {
    type Val = usize;
    const E: Self::Val = 0;
    fn op(_left: &Self::Val, _right: &Self::Val) -> Self::Val {
        0
    }
}

#[fastout]
fn main() {
    input! {
        Q: usize
    }

    let mut tree = DynamicSegmentTree::<usize, Set>::new();

    for _ in 0..Q {
        input! {
            t: usize
        }

        if t == 0 {
            input! {
                k: usize,
                v: usize
            }
            // 挿入
            *tree.get_mut(k) = v;
        } else {
            input! {
                k: usize
            }
            let ans = tree.get(&k);
            println!("{ans}");
        }
    }
}
