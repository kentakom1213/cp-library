use cp_library_rs::binary_search_slice::*;

#[test]
fn test_lower_bound() {
    let arr = vec![0, 1, 1, 1, 2, 2, 3, 5];

    assert_eq!(arr.lower_bound(0), 0);
    assert_eq!(arr.lower_bound(1), 1);
    assert_eq!(arr.lower_bound(2), 4);
    assert_eq!(arr.lower_bound(3), 6);
    assert_eq!(arr.lower_bound(4), 7);
    assert_eq!(arr.lower_bound(5), 7);
    assert_eq!(arr.lower_bound(10), 8);
}

#[test]
fn test_upper_bound() {
    let arr = vec![0, 1, 1, 1, 2, 2, 3, 5];

    assert_eq!(arr.upper_bound(0), 1);
    assert_eq!(arr.upper_bound(1), 4);
    assert_eq!(arr.upper_bound(2), 6);
    assert_eq!(arr.upper_bound(3), 7);
    assert_eq!(arr.upper_bound(4), 7);
    assert_eq!(arr.upper_bound(5), 8);
    assert_eq!(arr.upper_bound(10), 8);
}
