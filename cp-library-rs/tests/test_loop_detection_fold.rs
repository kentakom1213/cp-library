#![allow(non_snake_case)]

use cp_library_rs::graph::loop_detection_fold::*;
use num_bigint::BigUint;
use std::str::FromStr;

#[test]
fn test_functional_graph() {
    let graph = [1, 2, 3, 2, 4];
    /*
     * 1 --> 2 --> 3 <-> 2,  4 <-> 4
     */

    let next = |u: usize| graph[u];

    // 0から始める
    let from_0 = Loop::build(0, next, |_, _| 1);

    assert_eq!(from_0.get_nth_node_usize(0), 0);
    assert_eq!(from_0.get_nth_node_usize(1), 1);
    assert_eq!(from_0.get_nth_node_usize(2), 2);
    assert_eq!(from_0.get_nth_node_usize(20), 2);
    assert_eq!(from_0.get_nth_node_usize(100), 2);
    assert_eq!(from_0.get_nth_node_usize(101), 3);
    assert_eq!(from_0.get_nth_node_usize(10000), 2);
    assert_eq!(from_0.get_nth_node_usize(100000001), 3);

    // 2から始める
    let from_2 = Loop::build(2, next, |_, _| 1);

    assert_eq!(from_2.get_nth_node_usize(0), 2);
    assert_eq!(from_2.get_nth_node_usize(1), 3);
    assert_eq!(from_2.get_nth_node_usize(2), 2);
    assert_eq!(from_2.get_nth_node_usize(20), 2);
    assert_eq!(from_2.get_nth_node_usize(100), 2);
    assert_eq!(from_2.get_nth_node_usize(101), 3);
    assert_eq!(from_2.get_nth_node_usize(10000), 2);
    assert_eq!(from_2.get_nth_node_usize(100000001), 3);

    // 4から始める
    let from_4 = Loop::build(4, next, |_, _| 1);

    assert_eq!(from_4.get_nth_node_usize(0), 4);
    assert_eq!(from_4.get_nth_node_usize(1), 4);
    assert_eq!(from_4.get_nth_node_usize(2), 4);
    assert_eq!(from_4.get_nth_node_usize(20), 4);
    assert_eq!(from_4.get_nth_node_usize(100), 4);
    assert_eq!(from_4.get_nth_node_usize(101), 4);
    assert_eq!(from_4.get_nth_node_usize(10000), 4);
    assert_eq!(from_4.get_nth_node_usize(100000001), 4);
}

/// テストケース: <https://atcoder.jp/contests/abc030/tasks/abc030_d>
#[test]
fn test_biguint() {
    let graph = [1, 2, 3, 0];

    let next = |u: usize| graph[u];

    // 0から始める
    let from_0 = Loop::build(0, next, |_, _| 1);

    let n = BigUint::from_str("100000000000000000000").unwrap();

    assert_eq!(from_0.get_nth_node_biguint(n), 0);
}

/// テストケース: <https://atcoder.jp/contests/abc179/tasks/abc179_e>
#[test]
fn test_accumulate() {
    let f1: Loop<usize, usize, _, _> = Loop::build(2, |x| x * x % 1001, |s, x| s + x);
    assert_eq!(f1.get_nth_val_usize(0), 0);
    assert_eq!(f1.get_nth_val_usize(1), 2);
    assert_eq!(f1.get_nth_val_usize(6), 1369);

    let f2: Loop<usize, usize, _, _> = Loop::build(2, |x| x * x % 16, |s, x| s + x);
    assert_eq!(f2.get_nth_val_usize(1000), 6);
}
