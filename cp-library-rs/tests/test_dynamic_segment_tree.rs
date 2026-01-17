#![allow(non_snake_case)]

use cp_library_rs::algebraic_structure::actedmonoid::examples::AddSum;
use cp_library_rs::algebraic_structure::to_acted::ToActed;
use cp_library_rs::utils::show_binary_tree::ShowBinaryTree;
use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use rand::{rng, Rng};
use rstest::rstest;

#[test]
fn point_update_and_get() {
    let mut seg = DynamicSegmentTree::<usize, ToActed<Add<isize>>>::new(0, 8);

    // 未生成は id
    assert_eq!(seg.get(0), 0);
    assert_eq!(seg.get(7), 0);

    seg.update(3, 10);
    seg.update(6, -2);

    assert_eq!(seg.get(3), 10);
    assert_eq!(seg.get(6), -2);
    assert_eq!(seg.get(2), 0);

    // get_range（RangeBounds）でも同じ値が取れることを軽く確認
    assert_eq!(seg.get_range(3..4), 10);
    assert_eq!(seg.get_range(6..7), -2);
    assert_eq!(seg.get_range(2..3), 0);
}

#[test]
fn prod_small_ranges() {
    let mut seg = DynamicSegmentTree::<isize, AddSum<isize>>::new(0, 8);
    seg.update(1, (5, 1));
    seg.update(3, (10, 1));
    seg.update(6, (-2, 1));
    // [0, 5, 0, 10, 0, 0, -2, 0]

    // get_range([l, r))
    assert_eq!(seg.get_range(0..8).0, 13);
    assert_eq!(seg.get_range(0..1).0, 0);
    assert_eq!(seg.get_range(1..2).0, 5);
    assert_eq!(seg.get_range(2..4).0, 10);
    assert_eq!(seg.get_range(4..7).0, -2);
    assert_eq!(seg.get_range(7..8).0, 0);

    // 区間更新
    seg.apply(2..5, 10);
    // [0, 5, 10, 20, 10, 0, -2, 0]

    seg.print_as_binary_tree();

    // get_range([l, r))
    assert_eq!(seg.get_range(0..8).0, 43);
    assert_eq!(seg.get_range(0..1).0, 0);
    assert_eq!(seg.get_range(1..2).0, 5);
    assert_eq!(seg.get_range(2..4).0, 30);
    assert_eq!(seg.get_range(4..7).0, 8);
    assert_eq!(seg.get_range(7..8).0, 0);
}

#[test]
fn get_range_bounds_compat() {
    let mut seg = DynamicSegmentTree::<usize, ToActed<Add<isize>>>::new(0, 8);
    seg.update(1, 5);
    seg.update(3, 10);
    seg.update(6, -2);

    // SegmentTree 互換：RangeBounds を受け取る get_range(range)
    assert_eq!(seg.get_range(0..8), 13);
    assert_eq!(seg.get_range(1..=3), 15); // [1,4) のつもり
    assert_eq!(seg.get_range(..), 13);
    assert_eq!(seg.get_range(..3), 5);
    assert_eq!(seg.get_range(4..), -2);

    // 追加：単点取得相当も get_range で確認
    assert_eq!(seg.get_range(1..2), 5);
    assert_eq!(seg.get_range(3..4), 10);
    assert_eq!(seg.get_range(6..7), -2);
}

#[test]
fn get_mut_updates_on_drop() {
    let mut seg = DynamicSegmentTree::<usize, ToActed<Add<isize>>>::new(0, 8);

    // 既存：get_mut で 4 を変更
    {
        let mut v = seg.get_mut(4).expect("in range");
        assert_eq!(*v, 0);
        *v = 7;
        // drop で update される
    }

    assert_eq!(seg.get(4), 7);
    assert_eq!(seg.get_range(0..8), 7);

    // 追加：update を使わず get_mut だけで複数箇所を変更
    {
        let mut v0 = seg.get_mut(0).expect("in range");
        *v0 = 2;
    }
    {
        let mut v7 = seg.get_mut(7).expect("in range");
        *v7 = -3;
    }

    assert_eq!(seg.get(0), 2);
    assert_eq!(seg.get(7), -3);

    // 追加：prod ではなく get_range で全体和を確認
    assert_eq!(seg.get_range(0..8), 2 + 7 - 3);
    // 部分区間も get_range で確認
    assert_eq!(seg.get_range(0..5), 2 + 7);
    assert_eq!(seg.get_range(5..8), -3);
}

#[test]
fn max_right_min_left_sum_monoid() {
    let mut seg = DynamicSegmentTree::<usize, ToActed<Add<isize>>>::new(0, 8);

    // 既存：update で設定
    seg.update(0, 2);
    seg.update(1, 3);
    seg.update(2, 5);
    seg.update(3, 7);
    // 配列は [2,3,5,7,0,0,0,0]

    // 追加：update ではなく get_mut を使った変更を混ぜる（同値に上書き）
    {
        let mut v2 = seg.get_mut(2).expect("in range");
        *v2 = 5;
    }
    {
        let mut v3 = seg.get_mut(3).expect("in range");
        *v3 = 7;
    }

    // max_right：左端 l を固定して，prefix 和が条件を満たす最大の r を探す
    // f(sum) := sum <= 9
    let (s, r) = seg.max_right(0, |x| x <= 9);
    assert_eq!(s, 2 + 3); // 0..2
    assert_eq!(r, 2);
    assert_eq!(seg.get_range(0..r), 5);

    let (s, r) = seg.max_right(1, |x| x <= 8);
    // 1..3 は 3+5=8 まで OK，次の 7 を足すと 15 で NG
    assert_eq!(s, 8);
    assert_eq!(r, 3);
    assert_eq!(seg.get_range(1..r), 8);

    // min_left：右端 r を固定して，suffix 和が条件を満たす最小の l を探す
    // f(sum) := sum <= 10
    let (s, l) = seg.min_left(4, |x| x <= 10);
    // 3..4 は 7 OK，2..4 は 5+7=12 NG なので l=3
    assert_eq!(s, 7);
    assert_eq!(l, 3);
    assert_eq!(seg.get_range(l..4), 7);
}

#[test]
fn show_binary_tree_smoke() {
    let mut seg = DynamicSegmentTree::<usize, ToActed<Add<isize>>>::new(0, 8);

    seg.print_as_binary_tree();

    // 既存：update
    seg.update(3, 10);
    seg.update(6, -2);

    // debug ビルドでは stderr に木が出る（落ちないことだけ確認）
    seg.print_as_binary_tree();

    // 追加：get_mut でも更新してから表示
    {
        let mut v1 = seg.get_mut(1).expect("in range");
        *v1 = 4;
    }
    {
        let mut v6 = seg.get_mut(6).expect("in range");
        *v6 = -2; // 同値上書き
    }

    // 追加：prod を使わず get_range で軽く検証
    assert_eq!(seg.get_range(..), 10 + 4 - 2);

    // debug ビルドでは stderr に木が出る（落ちないことだけ確認）
    seg.print_as_binary_tree();
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
    case(10, 10),
    case(1_000, 1_000),
    case(1_000, 10_000),
    case(10_000, 1_000)
)]
fn test_randomly_range_add(size: usize, query: usize) {
    let (MIN, MAX) = (-1_000_000_000, 1_000_000_000);

    let mut rng = rng();

    let mut arr = vec![0; size];
    let mut seg = DynamicSegmentTree::<_, AddSum<i64>>::new(0, size);

    for _ in 0..query {
        // 区間取得
        let (l, r) = random_range(&mut rng, size);
        let actual = seg.get_range(l..r).0;
        let expected = arr[l..r].iter().sum();
        assert_eq!(actual, expected);

        // 区間加算
        let (l, r) = random_range(&mut rng, size);
        let delta = rng.random_range(MIN..MAX);
        arr[l..r].iter_mut().for_each(|v| *v += delta);
        seg.apply(l..r, delta);
    }
}

#[rstest(
    size,
    query,
    case(10, 10),
    case(1_000, 1_000),
    case(1_000, 10_000),
    case(10_000, 1_000)
)]
fn test_randomly_binary_search(size: usize, query: usize) {
    let A_MAX = 1_000;
    let T_MAX = A_MAX * (size * query / 2) as u64;

    let mut rng = rng();

    let mut arr = vec![0; size];
    let mut seg = DynamicSegmentTree::<_, AddSum<u64>>::new(0, size);

    for _ in 0..query {
        // 二分探索
        let (l, _) = random_range(&mut rng, size);
        let t = rng.random_range(..T_MAX);
        let actual = seg.max_right(l, |s| s.0 <= t);
        let expected = arr[l..]
            .iter()
            .scan(0, |acc, v| {
                *acc += v;
                Some(*acc)
            })
            .take_while(|s| *s <= t)
            .last();
        assert_eq!(actual.0 .0, expected.unwrap_or(0));

        // 区間加算
        let (l, r) = random_range(&mut rng, size);
        let delta = rng.random_range(..A_MAX);
        arr[l..r].iter_mut().for_each(|v| *v += delta);
        seg.apply(l..r, delta);
    }
}
