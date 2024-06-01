#![allow(non_snake_case)]

use cp_library_rs::{bit_2d::BIT2D, monoid::examples::Add};

#[test]
fn test_small() {
    let mut seg = BIT2D::<Add>::new(4, 4);

    // 0, 0, 0, 0
    // 0, 0, 0, 0
    // 0, 0, 0, 0
    // 0, 0, 0, 0
    assert_eq!(seg.prefix_sum(2, 4), 0);
    assert_eq!(seg.prefix_sum(4, 2), 0);
    assert_eq!(seg.prefix_sum(3, 3), 0);
    assert_eq!(seg.prefix_sum(4, 4), 0);

    // 0, 0, 0, 0
    // 0, 0, 0, 0
    // 0, 0, 2, 0
    // 0, 0, 0, 0
    seg.add(2, 2, 2);

    assert_eq!(seg.prefix_sum(2, 4), 0);
    assert_eq!(seg.prefix_sum(4, 2), 0);
    assert_eq!(seg.prefix_sum(3, 3), 2);
    assert_eq!(seg.prefix_sum(4, 4), 2);

    // 0, 0, 0, 0
    // 0, 0, 0, 8
    // 0, 0, 2, 0
    // 0, 0, 0, 0
    seg.add(1, 3, 8);

    assert_eq!(seg.prefix_sum(2, 4), 8);
    assert_eq!(seg.prefix_sum(4, 2), 0);
    assert_eq!(seg.prefix_sum(3, 3), 2);
    assert_eq!(seg.prefix_sum(4, 4), 10);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 0, 0, 2, 0
    // 0, 0, 0, 0
    seg.add(0, 0, 3);

    assert_eq!(seg.prefix_sum(2, 4), 11);
    assert_eq!(seg.prefix_sum(4, 2), 3);
    assert_eq!(seg.prefix_sum(3, 3), 5);
    assert_eq!(seg.prefix_sum(4, 4), 13);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 6, 0, 2, 0
    // 0, 0, 0, 0
    seg.add(2, 0, 6);

    assert_eq!(seg.prefix_sum(2, 4), 11);
    assert_eq!(seg.prefix_sum(4, 2), 9);
    assert_eq!(seg.prefix_sum(3, 3), 11);
    assert_eq!(seg.prefix_sum(4, 4), 19);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 6, 0, 2, 0
    // 0, 0, 1, 0
    seg.add(3, 2, 1);

    assert_eq!(seg.prefix_sum(2, 4), 11);
    assert_eq!(seg.prefix_sum(4, 2), 9);
    assert_eq!(seg.prefix_sum(3, 3), 11);
    assert_eq!(seg.prefix_sum(4, 4), 20);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 6, 0, 12, 0
    // 0, 0, 1, 0
    seg.add(2, 2, 10);

    assert_eq!(seg.prefix_sum(2, 4), 11);
    assert_eq!(seg.prefix_sum(4, 2), 9);
    assert_eq!(seg.prefix_sum(3, 3), 21);
    assert_eq!(seg.prefix_sum(4, 4), 30);
}

#[test]
fn test_imos() {
    let mut bit = BIT2D::<Add>::new(4, 4);

    bit.add(0, 0, 1);
    bit.add(0, 2, -1);
    bit.add(2, 0, -1);
    bit.add(2, 2, 1);

    assert_eq!(bit.prefix_sum(0 + 1, 0 + 1), 1);
    assert_eq!(bit.prefix_sum(0 + 1, 1 + 1), 1);
    assert_eq!(bit.prefix_sum(0 + 1, 2 + 1), 0);
    assert_eq!(bit.prefix_sum(0 + 1, 3 + 1), 0);
    assert_eq!(bit.prefix_sum(1 + 1, 0 + 1), 1);
    assert_eq!(bit.prefix_sum(1 + 1, 1 + 1), 1);
    assert_eq!(bit.prefix_sum(1 + 1, 2 + 1), 0);
    assert_eq!(bit.prefix_sum(1 + 1, 3 + 1), 0);
    assert_eq!(bit.prefix_sum(2 + 1, 0 + 1), 0);
    assert_eq!(bit.prefix_sum(2 + 1, 1 + 1), 0);
    assert_eq!(bit.prefix_sum(2 + 1, 2 + 1), 0);
    assert_eq!(bit.prefix_sum(2 + 1, 3 + 1), 0);
    assert_eq!(bit.prefix_sum(3 + 1, 0 + 1), 0);
    assert_eq!(bit.prefix_sum(3 + 1, 1 + 1), 0);
    assert_eq!(bit.prefix_sum(3 + 1, 2 + 1), 0);
    assert_eq!(bit.prefix_sum(3 + 1, 3 + 1), 0);
}
