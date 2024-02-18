use cp_library_rs::binary_search_btreeset::*;
use std::collections::BTreeSet;

#[test]
fn test_lower_bound() {
    let mut set = BTreeSet::new();
    set.insert(5);
    set.insert(9);
    set.insert(200);
    set.insert(200);
    set.insert(-5);
    /*
     * BTreeSet{ -5, 5, 9, 200, 200, }
     */

    assert_eq!(set.lower_bound(&4), Some(&5));
    assert_eq!(set.lower_bound(&5), Some(&5));
    assert_eq!(set.lower_bound(&8), Some(&9));
    assert_eq!(set.lower_bound(&100), Some(&200));
    assert_eq!(set.lower_bound(&200), Some(&200));
    assert_eq!(set.lower_bound(&201), None);
}

#[test]
fn test_upper_bound() {
    let mut set = BTreeSet::new();
    set.insert(5);
    set.insert(9);
    set.insert(200);
    set.insert(200);
    set.insert(-5);
    /*
     * BTreeSet{ -5, 5, 9, 200, 200, }
     */

    assert_eq!(set.upper_bound(&4), Some(&5));
    assert_eq!(set.upper_bound(&5), Some(&9));
    assert_eq!(set.upper_bound(&8), Some(&9));
    assert_eq!(set.lower_bound(&100), Some(&200));
    assert_eq!(set.upper_bound(&200), None);
    assert_eq!(set.upper_bound(&201), None);
}
