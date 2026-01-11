use cp_library_rs::utils::show_binary_tree::ShowBinaryTree;
use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::dynamic_segment_tree::DynamicSegmentTree,
};

#[test]
fn point_update_and_get() {
    let mut seg = DynamicSegmentTree::<usize, Add<isize>>::new(0, 8);

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
    let mut seg = DynamicSegmentTree::<isize, Add<isize>>::new(0, 8);
    seg.update(1, 5);
    seg.update(3, 10);
    seg.update(6, -2);

    // prod([l, r))
    assert_eq!(seg.prod(0, 8), 13);
    assert_eq!(seg.prod(0, 1), 0);
    assert_eq!(seg.prod(1, 2), 5);
    assert_eq!(seg.prod(2, 4), 10);
    assert_eq!(seg.prod(4, 7), -2);
    assert_eq!(seg.prod(7, 8), 0);

    // 追加：prod ではなく get_range（RangeBounds）でも取得できることを確認
    assert_eq!(seg.get_range(0..8), 13);
    assert_eq!(seg.get_range(0..1), 0);
    assert_eq!(seg.get_range(1..2), 5);
    assert_eq!(seg.get_range(2..4), 10);
    assert_eq!(seg.get_range(4..7), -2);
    assert_eq!(seg.get_range(7..8), 0);

    // 同一区間で一致することを確認
    assert_eq!(seg.get_range(2..4), seg.prod(2, 4));
    assert_eq!(seg.get_range(4..7), seg.prod(4, 7));
}

#[test]
fn get_range_bounds_compat() {
    let mut seg = DynamicSegmentTree::<usize, Add<isize>>::new(0, 8);
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
    let mut seg = DynamicSegmentTree::<usize, Add<isize>>::new(0, 8);

    // 既存：get_mut で 4 を変更
    {
        let mut v = seg.get_mut(4).expect("in range");
        assert_eq!(*v, 0);
        *v = 7;
        // drop で update される
    }

    assert_eq!(seg.get(4), 7);
    assert_eq!(seg.prod(0, 8), 7);

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
    let mut seg = DynamicSegmentTree::<usize, Add<isize>>::new(0, 8);

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
    let mut seg = DynamicSegmentTree::<usize, Add<isize>>::new(0, 8);

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
