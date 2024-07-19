use cp_library_rs::data_structure::bitset::*;

#[test]
fn test_get_mut() {
    let mut bitset = BitSet::<20>::new();

    println!("{:?}", bitset);
    assert_eq!(bitset.any(), true);
    assert_eq!(bitset.all(), false);

    *bitset.get_mut(10).unwrap() = true;

    println!("{:?}", bitset);
    assert_eq!(bitset.any(), false);
    assert_eq!(bitset.all(), false);

    *bitset.get_mut(10).unwrap() = false;

    println!("{:?}", bitset);
    assert_eq!(bitset.any(), true);
    assert_eq!(bitset.all(), false);

    assert!(bitset.get_mut(50).is_none());

    *bitset.get_mut(0).unwrap() = true;

    println!("{:?}", bitset);
    assert_eq!(bitset.any(), false);
    assert_eq!(bitset.all(), false);
}

#[test]
fn test_set_unset_flip() {
    let mut bitset = BitSet::<100>::new();

    // set
    println!("{:?}", bitset);
    assert_eq!(bitset.any(), true);
    assert_eq!(bitset.all(), false);
    assert_eq!(bitset.count_ones(), 0);

    for i in 0..99 {
        bitset.set(i);

        println!("{:?}", bitset);
        assert_eq!(bitset.any(), false);
        assert_eq!(bitset.all(), false);
        assert_eq!(bitset.count_ones(), i + 1);
    }

    bitset.set(99);
    assert_eq!(bitset.any(), false);
    assert_eq!(bitset.all(), true);
    assert_eq!(bitset.count_ones(), 100);

    // unset
    for i in (50..100).rev() {
        bitset.unset(i);

        println!("{:?}", bitset);
        assert_eq!(bitset.any(), false);
        assert_eq!(bitset.all(), false);
        assert_eq!(bitset.count_ones(), i);
    }

    // flip
    for i in 25..50 {
        bitset.flip(i);

        println!("{:?}", bitset);
        assert_eq!(bitset.any(), false);
        assert_eq!(bitset.all(), false);
        assert_eq!(bitset.count_ones(), 75 - i - 1);
    }

    for i in 50..75 {
        bitset.flip(i);

        println!("{:?}", bitset);
        assert_eq!(bitset.any(), false);
        assert_eq!(bitset.all(), false);
        assert_eq!(bitset.count_ones(), i - 25 + 1);
    }
}

// #[test]
// fn test_right_shift() {
//     let mut bitset = BitSet::<150>::new();

//     println!("{:?}", bitset);

//     for i in 40..60 {
//         bitset.set(i);
//     }

//     for i in 80..100 {
//         bitset.set(i);
//     }

//     for i in 120..140 {
//         bitset.set(i);
//     }

//     println!("{:?}", bitset);

//     let rh100 = bitset.clone() >> 100;

//     println!("{:?}", rh100);

//     let rh50 = bitset.clone() >> 50;

//     println!("{:?}", rh50);
// }
