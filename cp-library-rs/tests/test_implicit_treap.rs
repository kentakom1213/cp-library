use std::ops::Bound;

use cp_library_rs::{
    algebraic_structure::actedmonoid::examples::AddSum, tree::implicit_treap::ImplicitTreap,
};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;

fn pack(x: i64) -> (i64, usize) {
    (x, 1)
}
fn unpack_sum(v: (i64, usize)) -> i64 {
    v.0
}

#[test]
fn basic_insert_remove_get_range_apply() {
    let mut tr = ImplicitTreap::<AddSum<i64>>::default();

    // [1,2,3,4,5]
    for i in 1..=5 {
        tr.push_back(pack(i));
    }
    assert_eq!(tr.len(), 5);
    assert_eq!(unpack_sum(tr.get_range(0..5)), 1 + 2 + 3 + 4 + 5);
    assert_eq!(unpack_sum(tr.get_range(1..4)), 2 + 3 + 4);

    // 区間加算: [1, 12, 13, 14, 5]
    tr.apply(1..4, 10);
    assert_eq!(unpack_sum(tr.get_range(0..5)), 1 + 12 + 13 + 14 + 5);
    assert_eq!(unpack_sum(tr.get_range(1..4)), 12 + 13 + 14);

    // insert
    // i=2 に 7 を挿入: [1, 12, 7, 13, 14, 5]
    tr.insert(2, pack(7));
    assert_eq!(tr.len(), 6);
    assert_eq!(unpack_sum(tr.get_range(0..6)), 1 + 12 + 7 + 13 + 14 + 5);

    // remove
    // i=4 を削除: [1, 12, 7, 13, 5]
    tr.remove(4);
    assert_eq!(tr.len(), 5);
    assert_eq!(unpack_sum(tr.get_range(0..5)), 1 + 12 + 7 + 13 + 5);

    // get（中身は (sum,size) なので sum だけ確認）
    assert_eq!(tr.get(0).map(|v| v.0), Some(1));
    assert_eq!(tr.get(1).map(|v| v.0), Some(12));
    assert_eq!(tr.get(2).map(|v| v.0), Some(7));
    assert_eq!(tr.get(3).map(|v| v.0), Some(13));
    assert_eq!(tr.get(4).map(|v| v.0), Some(5));
    assert_eq!(tr.get(5).map(|v| v.0), None);
}

#[test]
fn randomized_against_vec_model() {
    let mut tr = ImplicitTreap::<AddSum<i64>>::default();
    let mut vec: Vec<i64> = vec![];

    let mut rng = XorShiftRng::seed_from_u64(123456789);

    for _step in 0..2000 {
        let op = rng.random_range(0..5);

        match op {
            0 => {
                // push_back
                let x = rng.random_range(-50..=50);
                tr.push_back(pack(x));
                vec.push(x);
            }
            1 => {
                // insert
                let x = rng.random_range(-50..=50);
                let i = if vec.is_empty() {
                    0
                } else {
                    rng.random_range(0..=vec.len())
                };
                tr.insert(i, pack(x));
                vec.insert(i, x);
            }
            2 => {
                // remove
                if !vec.is_empty() {
                    let i = rng.random_range(0..vec.len());
                    tr.remove(i);
                    vec.remove(i);
                } else {
                    // 空なら何もしない
                    tr.remove(0);
                }
            }
            3 => {
                // apply range add
                if !vec.is_empty() {
                    let l = rng.random_range(0..vec.len());
                    let r = rng.random_range(l..=vec.len());
                    let add = rng.random_range(-20..=20);
                    tr.apply(l..r, add);
                    for v in &mut vec[l..r] {
                        *v += add;
                    }
                }
            }
            _ => {
                // get_range sum
                let n = vec.len();
                let (l, r) = if n == 0 {
                    (0, 0)
                } else {
                    let l = rng.random_range(0..=n);
                    let r = rng.random_range(l..=n);
                    (l, r)
                };
                let got = unpack_sum(tr.get_range(l..r));
                let exp: i64 = vec[l..r].iter().sum();
                assert_eq!(got, exp);
            }
        }

        // 毎ステップで全体和が一致することも確認
        let got_all = unpack_sum(tr.get_range(0..tr.len()));
        let exp_all: i64 = vec.iter().sum();
        assert_eq!(tr.len(), vec.len());
        assert_eq!(got_all, exp_all);
    }
}

#[test]
fn range_bounds_variants() {
    let mut tr = ImplicitTreap::<AddSum<i64>>::default();
    for i in 0..10 {
        tr.push_back(pack(i));
    }
    // .. 末尾
    assert_eq!(unpack_sum(tr.get_range(..)), (0..10).sum::<i64>());
    // ..= （Included）
    assert_eq!(unpack_sum(tr.get_range(..=4)), (0..=4).sum::<i64>());
    // Excluded
    assert_eq!(
        unpack_sum(tr.get_range((Bound::Excluded(2), Bound::Excluded(7)))),
        (3..7).sum::<i64>()
    );

    // apply の RangeBounds も軽く確認
    tr.apply(..=4, 1); // 0..=4 に +1
    let expected: i64 = (0..=4).map(|x| x + 1).sum::<i64>() + (5..10).sum::<i64>();
    assert_eq!(unpack_sum(tr.get_range(..)), expected);
}
