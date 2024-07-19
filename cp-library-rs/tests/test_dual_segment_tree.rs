use cp_library_rs::{
    algebraic_structure::commutative::examples::{Add, Min},
    data_structure::dual_segment_tree::*,
};

#[test]
fn test_add() {
    let mut seg = DualSegmentTree::<Add>::new(8);
    eprintln!("{:?}", seg);

    // [0, 5) : +1
    seg.apply_range(0..5, 1);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), 1);
    assert_eq!(seg.get_point(1), 1);
    assert_eq!(seg.get_point(2), 1);
    assert_eq!(seg.get_point(3), 1);
    assert_eq!(seg.get_point(4), 1);
    assert_eq!(seg.get_point(5), 0);
    assert_eq!(seg.get_point(6), 0);
    assert_eq!(seg.get_point(7), 0);

    // [3, 6] : +4
    seg.apply_range(3..=6, 4);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), 1);
    assert_eq!(seg.get_point(1), 1);
    assert_eq!(seg.get_point(2), 1);
    assert_eq!(seg.get_point(3), 5);
    assert_eq!(seg.get_point(4), 5);
    assert_eq!(seg.get_point(5), 4);
    assert_eq!(seg.get_point(6), 4);
    assert_eq!(seg.get_point(7), 0);

    // [0, 8) : -10
    seg.apply_range(.., -10);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), -9);
    assert_eq!(seg.get_point(1), -9);
    assert_eq!(seg.get_point(2), -9);
    assert_eq!(seg.get_point(3), -5);
    assert_eq!(seg.get_point(4), -5);
    assert_eq!(seg.get_point(5), -6);
    assert_eq!(seg.get_point(6), -6);
    assert_eq!(seg.get_point(7), -10);

    // [6, 8) : +8
    seg.apply_range(6.., 8);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), -9);
    assert_eq!(seg.get_point(1), -9);
    assert_eq!(seg.get_point(2), -9);
    assert_eq!(seg.get_point(3), -5);
    assert_eq!(seg.get_point(4), -5);
    assert_eq!(seg.get_point(5), -6);
    assert_eq!(seg.get_point(6), 2);
    assert_eq!(seg.get_point(7), -2);
}

#[test]
fn test_min() {
    let mut seg = DualSegmentTree::<Min>::new(8);
    eprintln!("{:?}", seg);

    // [0, 5) : +1
    seg.apply_range(0..5, 1);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), 1);
    assert_eq!(seg.get_point(1), 1);
    assert_eq!(seg.get_point(2), 1);
    assert_eq!(seg.get_point(3), 1);
    assert_eq!(seg.get_point(4), 1);
    assert_eq!(seg.get_point(5), isize::MAX);
    assert_eq!(seg.get_point(6), isize::MAX);
    assert_eq!(seg.get_point(7), isize::MAX);

    // [3, 6] : +4
    seg.apply_range(3..=6, 4);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), 1);
    assert_eq!(seg.get_point(1), 1);
    assert_eq!(seg.get_point(2), 1);
    assert_eq!(seg.get_point(3), 1);
    assert_eq!(seg.get_point(4), 1);
    assert_eq!(seg.get_point(5), 4);
    assert_eq!(seg.get_point(6), 4);
    assert_eq!(seg.get_point(7), isize::MAX);

    // [0, 8) : -10
    seg.apply_range(.., -10);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), -10);
    assert_eq!(seg.get_point(1), -10);
    assert_eq!(seg.get_point(2), -10);
    assert_eq!(seg.get_point(3), -10);
    assert_eq!(seg.get_point(4), -10);
    assert_eq!(seg.get_point(5), -10);
    assert_eq!(seg.get_point(6), -10);
    assert_eq!(seg.get_point(7), -10);

    // [6, 8) : +8
    seg.apply_range(6.., 8);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), -10);
    assert_eq!(seg.get_point(1), -10);
    assert_eq!(seg.get_point(2), -10);
    assert_eq!(seg.get_point(3), -10);
    assert_eq!(seg.get_point(4), -10);
    assert_eq!(seg.get_point(5), -10);
    assert_eq!(seg.get_point(6), -10);
    assert_eq!(seg.get_point(7), -10);
}

#[test]
fn test_add_build() {
    let mut seg = DualSegmentTree::<Add>::build(&vec![0, 6, 4, 3, 7, 1, 5, 2]);
    eprintln!("{:?}", seg);

    // [0, 5) : +1
    seg.apply_range(0..5, 1);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), 1);
    assert_eq!(seg.get_point(1), 7);
    assert_eq!(seg.get_point(2), 5);
    assert_eq!(seg.get_point(3), 4);
    assert_eq!(seg.get_point(4), 8);
    assert_eq!(seg.get_point(5), 1);
    assert_eq!(seg.get_point(6), 5);
    assert_eq!(seg.get_point(7), 2);

    // [3, 6] : +4
    seg.apply_range(3..=6, 4);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), 1);
    assert_eq!(seg.get_point(1), 7);
    assert_eq!(seg.get_point(2), 5);
    assert_eq!(seg.get_point(3), 8);
    assert_eq!(seg.get_point(4), 12);
    assert_eq!(seg.get_point(5), 5);
    assert_eq!(seg.get_point(6), 9);
    assert_eq!(seg.get_point(7), 2);

    // [0, 8) : -10
    seg.apply_range(.., -10);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), -9);
    assert_eq!(seg.get_point(1), -3);
    assert_eq!(seg.get_point(2), -5);
    assert_eq!(seg.get_point(3), -2);
    assert_eq!(seg.get_point(4), 2);
    assert_eq!(seg.get_point(5), -5);
    assert_eq!(seg.get_point(6), -1);
    assert_eq!(seg.get_point(7), -8);

    // [6, 8) : +8
    seg.apply_range(6.., 8);
    eprintln!("{:?}", seg);

    assert_eq!(seg.get_point(0), -9);
    assert_eq!(seg.get_point(1), -3);
    assert_eq!(seg.get_point(2), -5);
    assert_eq!(seg.get_point(3), -2);
    assert_eq!(seg.get_point(4), 2);
    assert_eq!(seg.get_point(5), -5);
    assert_eq!(seg.get_point(6), 7);
    assert_eq!(seg.get_point(7), 0);
}
