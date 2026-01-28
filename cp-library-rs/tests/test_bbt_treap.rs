#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::actedmonoid::examples::{AddSum, UpdateSum},
    tree::bbt_treap::BalancedBinaryTree,
    utils::show_binary_tree::ShowBinaryTree,
};
use rand::{
    distr::{Alphanumeric, SampleString},
    prelude::*,
};

#[test]
fn test_random_insert() {
    const ITER: usize = 800;
    const QUERY: usize = 300;
    const SIZE: usize = 300;

    let mut rng = rand::rng();

    // 配列
    let mut arr: [isize; SIZE] = [0; SIZE];

    // セグ木
    let mut seg: BalancedBinaryTree<usize, AddSum<isize>> = BalancedBinaryTree::default();

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx = rng.random_range(0..SIZE);
        let new_val: isize = rng.random_range(-1_000_000_000..1_000_000_000i64) as isize;

        // 配列の更新
        arr[idx] = new_val;

        // セグ木の更新
        seg.remove(&idx);
        seg.insert_unique(idx, (new_val, 1));

        // 表示
        // println!("{:?}", arr);
        // seg.print_as_binary_tree();

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.random_range(0..SIZE), rng.random_range(0..SIZE));
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(arr[l..r].iter().sum::<isize>(), seg.get_range(l..r).0);
        }
    }
}

#[test]
fn random_insert_delete() {
    const ITER: usize = 800;
    const QUERY: usize = 300;
    const SIZE: usize = 300;

    let mut rng = rand::rng();

    // 配列
    let mut arr: [isize; SIZE] = [0; SIZE];

    // セグ木
    let mut seg: BalancedBinaryTree<usize, AddSum<isize>> = BalancedBinaryTree::default();

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_insert = rng.random_range(0..SIZE);
        let idx_delete = rng.random_range(0..SIZE);
        let new_val: isize = rng.random_range(-1_000_000_000..1_000_000_000i64) as isize;

        // 配列の更新
        arr[idx_insert] = new_val;
        arr[idx_delete] = 0;

        // セグ木の更新
        seg.remove(&idx_insert);
        seg.insert_unique(idx_insert, (new_val, 1));
        seg.remove(&idx_delete);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.random_range(0..SIZE), rng.random_range(0..SIZE));
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(arr[l..r].iter().sum::<isize>(), seg.get_range(l..r).0);
        }
    }
}

#[test]
fn random_delete() {
    const ITER: usize = 300;
    const QUERY: usize = 300;

    let mut rng = rand::rng();

    // 配列
    let mut arr: Vec<(isize, isize)> = vec![];

    // セグ木
    let mut seg: BalancedBinaryTree<isize, UpdateSum<isize>> = BalancedBinaryTree::default();

    // ランダムな値を追加
    for _ in 0..ITER {
        let key = rng.random::<i64>() as isize;
        let val = rng.random_range(-1_000_000_000..1_000_000_000i64) as isize;

        let idx_insert = arr.partition_point(|&(k, _)| k < key);

        // 同じキーのときの処理
        if idx_insert < arr.len() && arr[idx_insert].0 == key {
            continue;
        }

        // 配列に追加
        arr.insert(idx_insert, (key, val));

        // セグ木に追加
        seg.remove(&key);
        seg.insert_unique(key, (val, 1));
    }

    // println!("{:?}", arr);
    // print_as_binary_tree(&seg);

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_delete = rng.random_range(0..arr.len());
        let (key, _arr_delete_val) = arr.remove(idx_delete);

        // セグ木の更新
        let seg_delete_val = seg.remove(&key);

        // 削除されたか
        assert!(seg_delete_val);

        // 表示
        // println!("{:?}", arr);
        // print_as_binary_tree(&seg);

        // 区間取得クエリ
        for _ in 0..QUERY {
            // ランダムな区間
            let (mut l, mut r) = (rng.random::<i64>() as isize, rng.random::<i64>() as isize);
            if l > r {
                (l, r) = (r, l);
            }

            assert_eq!(
                arr.iter()
                    .filter(|&&(k, _)| l <= k && k < r)
                    .map(|&(_, v)| v)
                    .sum::<isize>(),
                seg.get_range(l..r).0
            );
        }
    }
}

#[test]
fn random_delete_str() {
    const ITER: usize = 200;
    const QUERY: usize = 200;
    const SIZE: usize = 10;

    let mut rng = rand::rng();

    // 配列
    let mut arr: Vec<(String, i64)> = vec![];

    // セグ木
    let mut seg: BalancedBinaryTree<String, AddSum<i64>> = BalancedBinaryTree::default();

    // ランダムな値を追加
    for _ in 0..ITER {
        let key = Alphanumeric.sample_string(&mut rng, SIZE);
        let val = rng.random_range(-1_000_000_000..1_000_000_000i64);

        let idx_insert = arr.partition_point(|(k, _)| k < &key);

        // 同じキーのときの処理
        if idx_insert < arr.len() && arr[idx_insert].0 == key {
            continue;
        }

        // 配列に追加
        arr.insert(idx_insert, (key.clone(), val));

        // セグ木に追加
        seg.remove(&key);
        seg.insert_unique(key, (val, 1));
    }

    println!("{:?}", arr);
    seg.print_as_binary_tree();

    for _ in 0..ITER {
        // 一点更新クエリ
        // ランダムな値
        let idx_delete = rng.random_range(0..arr.len());
        let (key, _arr_delete_val) = arr.remove(idx_delete);

        // セグ木の更新
        let seg_delete_val = seg.remove(&key);

        // 削除されたか
        assert!(seg_delete_val);

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
                    .sum::<i64>(),
                seg.get_range(l..r).0
            );
        }
    }
}
