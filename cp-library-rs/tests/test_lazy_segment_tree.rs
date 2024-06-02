use cp_library_rs::{extmonoid::examples::*, extmonoid::ExtMonoid, lazy_segment_tree::*};

#[test]
fn test_RSQ_and_RAQ_hand() {
    let mut seg = LazySegmentTree::<AddSum>::new(10);
    // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    assert_eq!(seg.get(..), 0);
    assert_eq!(seg.get(..5), 0);
    assert_eq!(seg.get(5..), 0);
    assert_eq!(seg.get(3..8), 0);

    seg.apply(0..4, 2);
    // [2, 2, 2, 2, 0, 0, 0, 0, 0, 0]

    assert_eq!(seg.get(..), 8);
    assert_eq!(seg.get(..5), 8);
    assert_eq!(seg.get(5..), 0);
    assert_eq!(seg.get(3..8), 2);

    seg.apply(4.., 5);
    // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

    assert_eq!(seg.get(..), 38);
    assert_eq!(seg.get(..5), 13);
    assert_eq!(seg.get(5..), 25);
    assert_eq!(seg.get(3..8), 22);

    seg.apply(2..=5, -3);
    // [2, 2, -1, -1, 2, 2, 5, 5, 5, 5]

    assert_eq!(seg.get(..), 26);
    assert_eq!(seg.get(..5), 4);
    assert_eq!(seg.get(5..), 22);
    assert_eq!(seg.get(3..8), 13);

    seg.apply(8..10, -10);
    // [2, 2, -1, -1, 2, 2, 5, 5, -5, -5]

    assert_eq!(seg.get(..), 6);
    assert_eq!(seg.get(..5), 4);
    assert_eq!(seg.get(5..), 2);
    assert_eq!(seg.get(3..8), 13);
}

#[test]
fn test_RMQ_and_RUQ_hand() {
    const INF: isize = isize::MAX;
    let mut seg = LazySegmentTree::<UpdateMin>::new(10);
    // [INF, INF, INF, INF, INF, INF, INF, INF, INF, INF]

    assert_eq!(seg.get(..), INF);
    assert_eq!(seg.get(..5), INF);
    assert_eq!(seg.get(5..), INF);
    assert_eq!(seg.get(3..8), INF);

    seg.apply(0..4, 2);
    // [2, 2, 2, 2, INF, INF, INF, INF, INF, INF]

    assert_eq!(seg.get(..), 2);
    assert_eq!(seg.get(..5), 2);
    assert_eq!(seg.get(5..), INF);
    assert_eq!(seg.get(3..8), 2);

    seg.apply(4.., 5);
    // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

    assert_eq!(seg.get(..), 2);
    assert_eq!(seg.get(..5), 2);
    assert_eq!(seg.get(5..), 5);
    assert_eq!(seg.get(3..8), 2);

    seg.apply(2..=5, -3);
    // [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

    assert_eq!(seg.get(..), -3);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -3);
    assert_eq!(seg.get(3..8), -3);

    seg.apply(8..10, -10);
    // [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

    assert_eq!(seg.get(..), -10);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -10);
    assert_eq!(seg.get(3..8), -3);
}

/// テストケース: <https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_I&lang=ja>
#[test]
fn test_RSQ_and_RUQ() {
    let mut seg = LazySegmentTree::<UpdateSum>::new(6);

    seg.apply(1..=3, Some(1));
    seg.apply(2..=4, Some(-2));

    assert_eq!(seg.get(..=5), -5);
    assert_eq!(seg.get(..=1), 1);

    seg.apply(3..=5, Some(3));

    assert_eq!(seg.get(3..=4), 6);
    assert_eq!(seg.get(..=5), 8);
}

#[test]
fn test_from() {
    const INF: isize = AddMin::IX;

    let arr = vec![5, 2, -3, -1, -9, -2, 5, 0, 0, 5];

    let mut seg = LazySegmentTree::<UpdateMin>::from(&arr);
    // [5, 2, -3, -1, -9, -2, 5, 0, 0, 5]

    assert_eq!(seg.get(..), -9);
    assert_eq!(seg.get(..5), -9);
    assert_eq!(seg.get(5..), -2);
    assert_eq!(seg.get(3..8), -9);

    seg.apply(..4, 2);
    // [2, 2, 2, 2, -9, -2, 5, 0, 0, 5]

    assert_eq!(seg.get(..), -9);
    assert_eq!(seg.get(..5), -9);
    assert_eq!(seg.get(5..), -2);
    assert_eq!(seg.get(3..8), -9);

    seg.apply(4.., 5);
    // [2, 2, 2, 2, 5, 5, 5, 5, 5, 5]

    assert_eq!(seg.get(..), 2);
    assert_eq!(seg.get(..5), 2);
    assert_eq!(seg.get(5..), 5);
    assert_eq!(seg.get(3..8), 2);

    seg.apply(2..=5, -3);
    // [2, 2, -3, -3, -3, -3, 5, 5, 5, 5]

    assert_eq!(seg.get(..), -3);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -3);
    assert_eq!(seg.get(3..8), -3);

    seg.apply(8.., -10);
    // [2, 2, -3, -3, -3, -3, 5, 5, -10, -10]

    assert_eq!(seg.get(..), -10);
    assert_eq!(seg.get(..5), -3);
    assert_eq!(seg.get(5..), -10);
    assert_eq!(seg.get(3..8), -3);
}

#[test]
#[should_panic]
fn get_wrong_range() {
    let mut seg = LazySegmentTree::<AddMin>::from(&vec![0, 1, 2, 3, 4, 5]);

    seg.get(..7);
}

#[test]
#[should_panic]
fn set_wrong_range() {
    let mut seg = LazySegmentTree::<AddMin>::from(&vec![0, 1, 2, 3, 4, 5]);

    seg.apply(..7, 0);
}
