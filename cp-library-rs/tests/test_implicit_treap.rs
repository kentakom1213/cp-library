use std::ops::Bound;

use cp_library_rs::{
    algebraic_structure::{
        actedmonoid::examples::AddSum,
        actedmonoid_mod::AffineUpdateComposite,
        affine1d::{Affine, AffineTransform},
        operation::Add,
        to_acted::ToActed,
    },
    data_structure::implicit_treap::ImplicitTreap,
    number_theory::modint::{M998, MOD998},
};
use rand::{rng, Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rstest::rstest;

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

/// [0, n) のランダムな区間 [l, r) を返す
fn random_range<R: Rng + ?Sized>(rng: &mut R, n: usize) -> (usize, usize) {
    let a = rng.random_range(0..=n);
    let b = rng.random_range(0..=n);
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

#[rstest(
    size,
    query,
    case(1_000, 1_000),
    case(1_000, 1_000),
    case(5_000, 5_000)
)]
fn range_reverse(size: usize, query: usize) {
    const MIN: i64 = -1_000_000_000;
    const MAX: i64 = 1_000_000_000;

    let mut rng = rng();

    let mut ar: Vec<i64> = vec![];
    let mut tr = ImplicitTreap::<ToActed<Add<i64>>>::default();

    // ランダムに初期化
    for _ in 0..size {
        let x = rng.random_range(MIN..MAX);
        ar.push(x);
        tr.push_back(x);
    }

    // クエリ
    for _ in 0..query {
        let (l, r) = random_range(&mut rng, size);

        // 合成
        let actually = tr.get_range(l..r);
        let expected: i64 = ar[l..r].iter().sum();
        assert_eq!(actually, expected);

        // 区間の反転
        ar[l..r].reverse();
        tr.reverse(l..r);
    }

    let finally = (0..size).map(|i| tr.get(i)).collect::<Vec<_>>();
    assert_eq!(finally, ar);
}

#[rstest(
    size,
    query,
    case(1_000, 1_000),
    case(1_000, 1_000),
    case(5_000, 5_000)
)]
fn range_reverse_affine(size: usize, query: usize) {
    let mut rng = rng();

    let mut ar: Vec<Affine<M998>> = vec![];
    let mut tr = ImplicitTreap::<AffineUpdateComposite<M998>>::default();

    // ランダムに初期化
    for _ in 0..size {
        let a = rng.random_range(1..MOD998);
        let b = rng.random_range(0..MOD998);
        ar.push((a.into(), b.into()));
        tr.push_back((a.into(), b.into()));
    }

    // クエリ
    for _ in 0..query {
        let (l, r) = random_range(&mut rng, size);

        // 合成
        let actually = tr.get_range(l..r);
        let expected = ar[l..r]
            .iter()
            .fold((1.into(), 0.into()), |acc, f| f.compose(&acc));
        assert_eq!(actually, expected);

        // 区間の反転
        ar[l..r].reverse();
        tr.reverse(l..r);
    }

    let finally = (0..size).map(|i| tr.get(i)).collect::<Vec<_>>();
    assert_eq!(finally, ar);
}

// ========== binary search tests (max_right / min_left) ==========

fn model_max_right_addsum(vec: &[i64], l: usize, limit: i64) -> (i64, usize) {
    let mut s = 0_i64;
    let mut x = l;
    while x < vec.len() {
        let ns = s + vec[x];
        if ns <= limit {
            s = ns;
            x += 1;
        } else {
            break;
        }
    }
    (s, x)
}

fn model_min_left_addsum(vec: &[i64], r: usize, limit: i64) -> (i64, usize) {
    let mut s = 0_i64;
    let mut x = r;
    while x > 0 {
        let ns = vec[x - 1] + s;
        if ns <= limit {
            s = ns;
            x -= 1;
        } else {
            break;
        }
    }
    (s, x)
}

#[test]
fn binary_search_basic_addsum() {
    // 単調性が必要なので，非負で作る
    let mut tr = ImplicitTreap::<AddSum<i64>>::default();
    let mut vec = vec![];

    for &x in &[3_i64, 1, 4, 1, 5, 9, 2, 6] {
        tr.push_back(pack(x));
        vec.push(x);
    }

    // max_right
    // l=2 から sum <= 6 で伸ばす：4 + 1 = 5 まで，次の 5 は入らない
    let (got_sum, got_x) = tr.max_right(2, |v| unpack_sum(v) <= 6);
    let (exp_sum, exp_x) = model_max_right_addsum(&vec, 2, 6);
    assert_eq!(unpack_sum(got_sum), exp_sum);
    assert_eq!(got_x, exp_x);

    // min_left
    // r=7 までで suffix の sum <= 10：6 + 2 = 8 まで，次の 9 は入らない
    let (got_sum, got_x) = tr.min_left(7, |v| unpack_sum(v) <= 10);
    let (exp_sum, exp_x) = model_min_left_addsum(&vec, 7, 10);
    assert_eq!(unpack_sum(got_sum), exp_sum);
    assert_eq!(got_x, exp_x);

    // 空区間も許す（f(e)=true が前提）
    let (got_sum, got_x) = tr.max_right(tr.len(), |v| unpack_sum(v) <= 0);
    assert_eq!(unpack_sum(got_sum), 0);
    assert_eq!(got_x, tr.len());
}

#[test]
fn binary_search_randomized_addsum_monotone_only() {
    // max_right / min_left の前提（単調性）を満たすため，
    // 値と加算をすべて非負に制限する
    let mut tr = ImplicitTreap::<AddSum<i64>>::default();
    let mut vec: Vec<i64> = vec![];

    let mut rng = XorShiftRng::seed_from_u64(20260201);

    for _step in 0..3000 {
        let op = rng.random_range(0..6);

        match op {
            0 => {
                // push_back
                let x = rng.random_range(0..=20);
                tr.push_back(pack(x));
                vec.push(x);
            }
            1 => {
                // insert
                let x = rng.random_range(0..=20);
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
                    tr.remove(0);
                }
            }
            3 => {
                // apply range add（非負）
                if !vec.is_empty() {
                    let l = rng.random_range(0..vec.len());
                    let r = rng.random_range(l..=vec.len());
                    let add = rng.random_range(0..=10);
                    tr.apply(l..r, add);
                    for v in &mut vec[l..r] {
                        *v += add;
                    }
                }
            }
            4 => {
                // max_right を検証
                let n = vec.len();
                let l = if n == 0 { 0 } else { rng.random_range(0..=n) };
                let limit = rng.random_range(0..=200);

                let (got_sum, got_x) = tr.max_right(l, |v| unpack_sum(v) <= limit);
                let (exp_sum, exp_x) = model_max_right_addsum(&vec, l, limit);

                assert_eq!(unpack_sum(got_sum), exp_sum);
                assert_eq!(got_x, exp_x);
            }
            _ => {
                // min_left を検証
                let n = vec.len();
                let r = if n == 0 { 0 } else { rng.random_range(0..=n) };
                let limit = rng.random_range(0..=200);

                let (got_sum, got_x) = tr.min_left(r, |v| unpack_sum(v) <= limit);
                let (exp_sum, exp_x) = model_min_left_addsum(&vec, r, limit);

                assert_eq!(unpack_sum(got_sum), exp_sum);
                assert_eq!(got_x, exp_x);
            }
        }

        // ついでに全体和一致も軽く見る
        let got_all = unpack_sum(tr.get_range(0..tr.len()));
        let exp_all: i64 = vec.iter().sum();
        assert_eq!(tr.len(), vec.len());
        assert_eq!(got_all, exp_all);
    }
}
