#![allow(non_snake_case)]
#![allow(clippy::needless_range_loop)]

use cp_library_rs::{
    algebraic_structure::actedmonoid::examples::AddSum,
    data_structure::dynamic_segment_tree_2d::DynamicSegmentTree2D,
};
use rand::{rng, Rng};
use rstest::rstest;

#[test]
fn point_apply_and_get() {
    let mut seg = DynamicSegmentTree2D::<usize, AddSum<i64>>::new((0, 8), (0, 8));

    // 未生成はデフォルト（面積 1 の e_len）
    assert_eq!(seg.get(0, 0).0, 0);
    assert_eq!(seg.get(7, 7).0, 0);

    // 点への加算（面積 1 の矩形 apply）
    seg.apply(3..4, 5..6, 10);
    seg.apply(6..7, 1..2, -2);

    assert_eq!(seg.get(3, 5).0, 10);
    assert_eq!(seg.get(6, 1).0, -2);
    assert_eq!(seg.get(2, 2).0, 0);

    // get_range でも同値
    assert_eq!(seg.get_range(3..4, 5..6).0, 10);
    assert_eq!(seg.get_range(6..7, 1..2).0, -2);
    assert_eq!(seg.get_range(2..3, 2..3).0, 0);
}

#[test]
fn rect_apply_and_query_small() {
    // 4x4
    let mut seg = DynamicSegmentTree2D::<usize, AddSum<i64>>::new((0, 4), (0, 4));

    // 全体和は 0
    assert_eq!(seg.get_range(0..4, 0..4).0, 0);

    // 矩形加算
    // [x:1..3) × [y:0..2) に +5（面積 2*2=4 => +20）
    seg.apply(1..3, 0..2, 5);
    assert_eq!(seg.get_range(0..4, 0..4).0, 20);

    // 部分チェック
    assert_eq!(seg.get_range(1..3, 0..2).0, 20);
    assert_eq!(seg.get_range(0..1, 0..4).0, 0);
    assert_eq!(seg.get_range(3..4, 0..4).0, 0);

    // さらに重ねる
    // [x:2..4) × [y:1..4) に -3（面積 2*3=6 => -18）
    // 重なり領域 [2..3)×[1..2) は +5 と -3 が両方入る
    seg.apply(2..4, 1..4, -3);

    // 期待値を手計算
    // 1つ目の +5: 4セル
    // 2つ目の -3: 6セル
    // 重なりは 1セル（x=2,y=1）なので合算はそのまま（線形）
    // 全体和 = 20 - 18 = 2
    assert_eq!(seg.get_range(0..4, 0..4).0, 2);

    // 点チェック（代表）
    assert_eq!(seg.get(1, 0).0, 5); // +5 のみ
    assert_eq!(seg.get(2, 1).0, 2); // +5-3
    assert_eq!(seg.get(3, 3).0, -3); // -3 のみ
    assert_eq!(seg.get(0, 0).0, 0); // どちらも無し
}

#[test]
fn get_range_bounds_compat_2d() {
    let mut seg = DynamicSegmentTree2D::<usize, AddSum<i64>>::new((0, 8), (0, 8));

    seg.apply(1..3, 2..5, 4); // 面積 2*3=6 => +24
    seg.apply(6..7, 7..8, -2); // 面積 1 => -2

    // x/y とも RangeBounds を受け取る
    assert_eq!(seg.get_range(.., ..).0, 24 - 2);
    assert_eq!(seg.get_range(1..=2, 2..=4).0, 24); // [1,3)×[2,5)
    assert_eq!(seg.get_range(..3, ..).0, 24); // x<3 に全部入る
    assert_eq!(seg.get_range(6.., 7..).0, -2); // 右上の点

    // 単点取得相当
    assert_eq!(seg.get_range(6..7, 7..8).0, -2);
    assert_eq!(seg.get(6, 7).0, -2);
    assert_eq!(seg.get(0, 0).0, 0);
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
    w,
    h,
    query,
    case(8, 8, 200),
    case(20, 20, 500),
    case(30, 10, 500),
    case(10, 30, 500),
    case(100, 100, 1000)
)]
fn test_randomly_rect_add_and_query(w: usize, h: usize, query: usize) {
    let (MIN, MAX) = (-1_000_000_i64, 1_000_000_i64);

    let mut rng = rng();

    // 期待配列（愚直）
    let mut a = vec![vec![0_i64; h]; w];

    // 2D seg
    let mut seg = DynamicSegmentTree2D::<usize, AddSum<i64>>::new((0, w), (0, h));

    for _ in 0..query {
        // 矩形取得
        let (xl, xr) = random_range(&mut rng, w);
        let (yl, yr) = random_range(&mut rng, h);

        let actual = seg.get_range(xl..xr, yl..yr).0;

        let mut expected = 0_i64;
        for x in xl..xr {
            for y in yl..yr {
                expected += a[x][y];
            }
        }
        assert_eq!(actual, expected);

        // 矩形加算
        let (xl, xr) = random_range(&mut rng, w);
        let (yl, yr) = random_range(&mut rng, h);
        let delta = rng.random_range(MIN..MAX);

        for x in xl..xr {
            for y in yl..yr {
                a[x][y] += delta;
            }
        }
        seg.apply(xl..xr, yl..yr, delta);
    }
}
