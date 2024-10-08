#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::{Add, Max},
    data_structure::segment_tree_2d::SegmentTree2D,
};

#[test]
fn test_small() {
    let mut seg = SegmentTree2D::<Add<isize>>::new(4, 4);
    seg.show();

    // 0, 0, 0, 0
    // 0, 0, 0, 0
    // 0, 0, 0, 0
    // 0, 0, 0, 0
    assert_eq!(seg.get_range(..2, ..), 0);
    assert_eq!(seg.get_range(2.., ..), 0);
    assert_eq!(seg.get_range(1..3, ..), 0);
    assert_eq!(seg.get_range(.., ..2), 0);
    assert_eq!(seg.get_range(.., 2..), 0);
    assert_eq!(seg.get_range(1.., 1..), 0);
    assert_eq!(seg.get_range(..3, ..3), 0);
    assert_eq!(seg.get_range(1..3, 1..3), 0);
    assert_eq!(seg.get_range(.., ..), 0);

    // 0, 0, 0, 0
    // 0, 0, 0, 0
    // 0, 0, 2, 0
    // 0, 0, 0, 0
    seg.update(2, 2, 2);
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 0);
    assert_eq!(seg.get_range(2.., ..), 2);
    assert_eq!(seg.get_range(1..3, ..), 2);
    assert_eq!(seg.get_range(.., ..2), 0);
    assert_eq!(seg.get_range(.., 2..), 2);
    assert_eq!(seg.get_range(1.., 1..), 2);
    assert_eq!(seg.get_range(..3, ..3), 2);
    assert_eq!(seg.get_range(1..3, 1..3), 2);
    assert_eq!(seg.get_range(.., ..), 2);

    // 0, 0, 0, 0
    // 0, 0, 0, 8
    // 0, 0, 2, 0
    // 0, 0, 0, 0
    seg.update(1, 3, 8);
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 8);
    assert_eq!(seg.get_range(2.., ..), 2);
    assert_eq!(seg.get_range(1..3, ..), 10);
    assert_eq!(seg.get_range(.., ..2), 0);
    assert_eq!(seg.get_range(.., 2..), 10);
    assert_eq!(seg.get_range(1.., 1..), 10);
    assert_eq!(seg.get_range(..3, ..3), 2);
    assert_eq!(seg.get_range(1..3, 1..3), 2);
    assert_eq!(seg.get_range(.., ..), 10);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 0, 0, 2, 0
    // 0, 0, 0, 0
    seg.update(0, 0, 3);
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 11);
    assert_eq!(seg.get_range(2.., ..), 2);
    assert_eq!(seg.get_range(1..3, ..), 10);
    assert_eq!(seg.get_range(.., ..2), 3);
    assert_eq!(seg.get_range(.., 2..), 10);
    assert_eq!(seg.get_range(1.., 1..), 10);
    assert_eq!(seg.get_range(..3, ..3), 5);
    assert_eq!(seg.get_range(1..3, 1..3), 2);
    assert_eq!(seg.get_range(.., ..), 13);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 6, 0, 2, 0
    // 0, 0, 0, 0
    seg.update(2, 0, 6);
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 11);
    assert_eq!(seg.get_range(2.., ..), 8);
    assert_eq!(seg.get_range(1..3, ..), 16);
    assert_eq!(seg.get_range(.., ..2), 9);
    assert_eq!(seg.get_range(.., 2..), 10);
    assert_eq!(seg.get_range(1.., 1..), 10);
    assert_eq!(seg.get_range(..3, ..3), 11);
    assert_eq!(seg.get_range(1..3, 1..3), 2);
    assert_eq!(seg.get_range(.., ..), 19);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 6, 0, 2, 0
    // 0, 0, 1, 0
    seg.update(3, 2, 1);
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 11);
    assert_eq!(seg.get_range(2.., ..), 9);
    assert_eq!(seg.get_range(1..3, ..), 16);
    assert_eq!(seg.get_range(.., ..2), 9);
    assert_eq!(seg.get_range(.., 2..), 11);
    assert_eq!(seg.get_range(1.., 1..), 11);
    assert_eq!(seg.get_range(..3, ..3), 11);
    assert_eq!(seg.get_range(1..3, 1..3), 2);
    assert_eq!(seg.get_range(.., ..), 20);

    // 3, 0, 0, 0
    // 0, 0, 0, 8
    // 6, 0, 10, 0
    // 0, 0, 1, 0
    seg.update(2, 2, 10);
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 11);
    assert_eq!(seg.get_range(2.., ..), 17);
    assert_eq!(seg.get_range(1..3, ..), 24);
    assert_eq!(seg.get_range(.., ..2), 9);
    assert_eq!(seg.get_range(.., 2..), 19);
    assert_eq!(seg.get_range(1.., 1..), 19);
    assert_eq!(seg.get_range(..3, ..3), 19);
    assert_eq!(seg.get_range(1..3, 1..3), 10);
    assert_eq!(seg.get_range(.., ..), 28);
}

#[test]
fn test_from_array() {
    let arr = vec![
        vec![2, 5, 7, 0, 3, 2],
        vec![9, 8, 2, 2, 1, 8],
        vec![0, 4, 3, 8, 1, 6],
    ];

    let mut seg = SegmentTree2D::<Max<i32>>::from(&arr);

    seg.show();

    // 2, 5, 7, 0, 3, 2
    // 9, 8, 2, 2, 1, 8
    // 0, 4, 3, 8, 1, 6
    assert_eq!(seg.get_range(..2, ..), 9);
    assert_eq!(seg.get_range(2.., ..), 8);
    assert_eq!(seg.get_range(1..3, ..), 9);
    assert_eq!(seg.get_range(.., ..3), 9);
    assert_eq!(seg.get_range(.., 3..), 8);
    assert_eq!(seg.get_range(1.., 4..), 8);
    assert_eq!(seg.get_range(..3, ..4), 9);
    assert_eq!(seg.get_range(1..3, 3..5), 8);
    assert_eq!(seg.get_range(.., ..), 9);

    // 2, 5, 7, 0, 3, 2
    // 7, 8, 2, 2, 1, 8
    // 0, 4, 3, 6, 1, 6
    *seg.get_mut(1, 0).unwrap() = 7;
    *seg.get_mut(2, 3).unwrap() -= 2;
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 8);
    assert_eq!(seg.get_range(2.., ..), 6);
    assert_eq!(seg.get_range(1..3, ..), 8);
    assert_eq!(seg.get_range(.., ..3), 8);
    assert_eq!(seg.get_range(.., 3..), 8);
    assert_eq!(seg.get_range(1.., 4..), 8);
    assert_eq!(seg.get_range(..3, ..4), 8);
    assert_eq!(seg.get_range(1..3, 3..5), 6);
    assert_eq!(seg.get_range(.., ..), 8);

    // 2, 5, 7,10, 3, 2
    // 7, 2, 2, 2, 1, 8
    // 0, 4, 3, 6, 1, 6
    *seg.get_mut(0, 3).unwrap() = 10;
    *seg.get_mut(1, 1).unwrap() -= 6;
    seg.show();

    assert_eq!(seg.get_range(..2, ..), 10);
    assert_eq!(seg.get_range(2.., ..), 6);
    assert_eq!(seg.get_range(1..3, ..), 8);
    assert_eq!(seg.get_range(.., ..3), 7);
    assert_eq!(seg.get_range(.., 3..), 10);
    assert_eq!(seg.get_range(1.., 4..), 8);
    assert_eq!(seg.get_range(..3, ..4), 10);
    assert_eq!(seg.get_range(1..3, 3..5), 6);
    assert_eq!(seg.get_range(.., ..), 10);
}

#[test]
fn test_imos() {
    let mut seg = SegmentTree2D::<Add<isize>>::new(4, 4);

    seg.show();

    {
        let mut p = seg.get_mut(0, 0).unwrap();
        eprintln!("{:?}", p);
        *p += 1;
    }

    seg.show();

    {
        let mut p = seg.get_mut(0, 2).unwrap();
        eprintln!("{:?}", p);
        *p -= 1;
    }

    seg.show();

    {
        let mut p = seg.get_mut(2, 0).unwrap();
        eprintln!("{:?}", p);
        *p -= 1;
    }

    seg.show();

    {
        let mut p = seg.get_mut(2, 2).unwrap();
        eprintln!("{:?}", p);
        *p += 1;
    }

    seg.show();

    assert_eq!(seg.get_range(..=0, ..=0), 1);
    assert_eq!(seg.get_range(..=0, ..=1), 1);
    assert_eq!(seg.get_range(..=0, ..=2), 0);
    assert_eq!(seg.get_range(..=0, ..=3), 0);
    assert_eq!(seg.get_range(..=1, ..=0), 1);
    assert_eq!(seg.get_range(..=1, ..=1), 1);
    assert_eq!(seg.get_range(..=1, ..=2), 0);
    assert_eq!(seg.get_range(..=1, ..=3), 0);
    assert_eq!(seg.get_range(..=2, ..=0), 0);
    assert_eq!(seg.get_range(..=2, ..=1), 0);
    assert_eq!(seg.get_range(..=2, ..=2), 0);
    assert_eq!(seg.get_range(..=2, ..=3), 0);
    assert_eq!(seg.get_range(..=3, ..=0), 0);
    assert_eq!(seg.get_range(..=3, ..=1), 0);
    assert_eq!(seg.get_range(..=3, ..=2), 0);
    assert_eq!(seg.get_range(..=3, ..=3), 0);
}
