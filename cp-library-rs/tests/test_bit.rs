use cp_library_rs::data_structure::bit::*;

#[test]
fn test_new() {
    let mut bit = BIT::<Alg::Add>::new(5);

    bit.add(0, 20);
    bit.add(2, -5);

    let sum_5 = bit.prefix_sum(5);
    assert_eq!(sum_5, 15);

    bit.add(4, 10);
    bit.add(1, -20);

    let sum_2 = bit.prefix_sum(2);
    assert_eq!(sum_2, 0);

    let sum_all = bit.prefix_sum(5);
    assert_eq!(sum_all, 5);
}

#[test]
fn test_build() {
    let mut bit = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 5]);

    assert_eq!(bit.prefix_sum(4), 10);
    assert_eq!(bit.prefix_sum(5), 15);

    bit.add(2, -3);
    bit.add(3, -4);

    assert_eq!(bit.prefix_sum(5), 8);
}

#[test]
fn test_sum() {
    let bit = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 5]);

    assert_eq!(bit.sum(0..5), 15);
    assert_eq!(bit.sum(1..5), 14);
    assert_eq!(bit.sum(2..3), 3);
    assert_eq!(bit.sum(3..2), 0);
    assert_eq!(bit.sum(0..=5), 15);
    assert_eq!(bit.sum(1..=3), 9);
}

#[test]
fn test_new_usize() {
    let mut bit = BIT::<Alg::UAdd>::new(5);

    bit.add(0, 20);
    bit.add(2, 5_usize.wrapping_neg());

    let sum_5 = bit.prefix_sum(5);
    assert_eq!(sum_5, 15);

    bit.add(4, 10);
    bit.add(1, 20_usize.wrapping_neg());

    let sum_2 = bit.prefix_sum(2);
    assert_eq!(sum_2, 0);

    let sum_all = bit.prefix_sum(5);
    assert_eq!(sum_all, 5);
}

#[test]
fn test_lower_bound() {
    let bit = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 0, 5]);

    assert_eq!(bit.lower_bound(0), 0);
    assert_eq!(bit.lower_bound(1), 0);
    assert_eq!(bit.lower_bound(2), 1);
    assert_eq!(bit.lower_bound(10), 3);
    assert_eq!(bit.lower_bound(11), 5);
    assert_eq!(bit.lower_bound(100), 6);
}

#[test]
fn test_upper_bound() {
    let bit = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 0, 5]);

    assert_eq!(bit.upper_bound(0), 0);
    assert_eq!(bit.upper_bound(1), 1);
    assert_eq!(bit.upper_bound(2), 1);
    assert_eq!(bit.upper_bound(10), 5);
    assert_eq!(bit.upper_bound(11), 5);
    assert_eq!(bit.upper_bound(100), 6);
}

#[test]
fn test_debugprint() {
    let bit1 = BIT::<Alg::Add>::from(&vec![1, 2, 3, 4, 5]);
    println!("{:?}", bit1);
}
