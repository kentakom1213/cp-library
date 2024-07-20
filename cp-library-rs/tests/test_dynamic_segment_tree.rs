use cp_library_rs::{
    algebraic_structure::monoid::examples::Add,
    data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use rand::{
    distributions::{Alphanumeric, DistString},
    prelude::*,
};

#[test]
fn test_random_insert() {
    const ITER: usize = 1000;
    const QUERY: usize = 400;
    const SIZE: usize = 400;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: [isize; SIZE] = [0; SIZE];

    // セグ木
    let mut seg: DynamicSegmentTree<usize, Add<isize>> = DynamicSegmentTree::new();

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx = rng.gen_range(0..SIZE);
        let new_val: isize = rng.gen_range(-1_000_000_000..1_000_000_000);

        // 配列の更新
        arr[idx] = new_val;

        // セグ木の更新
        *seg.get_mut(idx) = new_val;

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.gen_range(0..SIZE), rng.gen_range(0..SIZE));
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(arr[l..r].iter().sum::<isize>(), seg.get_range(l..r));
        }
    }
}

#[test]
fn random_insert_delete() {
    const ITER: usize = 1000;
    const QUERY: usize = 400;
    const SIZE: usize = 400;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: [isize; SIZE] = [0; SIZE];

    // セグ木
    let mut seg: DynamicSegmentTree<usize, Add<isize>> = DynamicSegmentTree::new();

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_insert = rng.gen_range(0..SIZE);
        let idx_delete = rng.gen_range(0..SIZE);
        let new_val: isize = rng.gen_range(-1_000_000_000..1_000_000_000);

        // 配列の更新
        arr[idx_insert] = new_val;
        arr[idx_delete] = 0;

        // セグ木の更新
        seg.insert(idx_insert, new_val);
        seg.remove(&idx_delete);

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.gen_range(0..SIZE), rng.gen_range(0..SIZE));
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(arr[l..r].iter().sum::<isize>(), seg.get_range(l..r));
        }
    }
}

#[test]
fn random_delete() {
    const ITER: usize = 400;
    const QUERY: usize = 400;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: Vec<(isize, isize)> = vec![];

    // セグ木
    let mut seg: DynamicSegmentTree<isize, Add<isize>> = DynamicSegmentTree::new();

    // ランダムな値を追加
    for _ in 0..ITER {
        let key = rng.gen();
        let val = rng.gen_range(-1_000_000_000..1_000_000_000);

        let idx_insert = arr.partition_point(|&(k, _)| k < key);

        // 同じキーのときの処理
        if idx_insert < arr.len() && arr[idx_insert].0 == key {
            continue;
        }

        // 配列に追加
        arr.insert(idx_insert, (key, val));

        // セグ木に追加
        *seg.get_mut(key) = val;
    }

    // println!("{:?}", arr);
    // print_as_binary_tree(&seg);

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_delete = rng.gen_range(0..arr.len());
        let (key, arr_delete_val) = arr.remove(idx_delete);

        // セグ木の更新
        let seg_delete_val = seg.remove(&key).unwrap();

        // 削除した値は等しいか
        assert_eq!(arr_delete_val, seg_delete_val);

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.gen(), rng.gen());
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(
                arr.iter()
                    .filter(|&&(k, _)| l <= k && k < r)
                    .map(|&(_, v)| v)
                    .sum::<isize>(),
                seg.get_range(l..r)
            );
        }
    }
}

#[test]
fn random_delete_str() {
    const ITER: usize = 200;
    const QUERY: usize = 200;
    const SIZE: usize = 10;

    let mut rng = rand::thread_rng();

    // 配列
    let mut arr: Vec<(String, isize)> = vec![];

    // セグ木
    let mut seg: DynamicSegmentTree<String, Add<isize>> = DynamicSegmentTree::new();

    // ランダムな値を追加
    for _ in 0..ITER {
        let key = Alphanumeric.sample_string(&mut rng, SIZE);
        let val = rng.gen_range(-1_000_000_000..1_000_000_000);

        let idx_insert = arr.partition_point(|(k, _)| k < &key);

        // 同じキーのときの処理
        if idx_insert < arr.len() && arr[idx_insert].0 == key {
            continue;
        }

        // 配列に追加
        arr.insert(idx_insert, (key.clone(), val));

        // セグ木に追加
        seg.insert(key, val);
    }

    println!("{:?}", arr);
    seg.print_as_binary_tree();

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_delete = rng.gen_range(0..arr.len());
        let (key, arr_delete_val) = arr.remove(idx_delete);

        // セグ木の更新
        let seg_delete_val = seg.remove(&key).unwrap();

        // 削除した値は等しいか
        assert_eq!(arr_delete_val, seg_delete_val);

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let mut l = Alphanumeric.sample_string(&mut rng, SIZE);
            let mut r = Alphanumeric.sample_string(&mut rng, SIZE);
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(
                arr.iter()
                    .filter(|(k, _)| &l <= k && k < &r)
                    .map(|&(_, v)| v)
                    .sum::<isize>(),
                seg.get_range(l..r)
            );
        }
    }
}
