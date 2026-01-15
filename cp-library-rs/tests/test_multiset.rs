#![allow(non_snake_case)]

use cp_library_rs::data_structure::multiset::*;

#[test]
fn test_usize() {
    let mut mset: MultiSet<usize> = MultiSet::new();

    assert!(mset.is_empty());

    mset.insert(5);
    mset.insert(7);
    mset.insert(7);
    mset.insert(10);
    mset.insert(3);

    assert_eq!(mset.len(), 5);

    // remove value
    assert!(mset.remove(&7));
    assert!(mset.remove(&7));
    assert!(!mset.remove(&0));

    // is_contain
    assert!(mset.contains(&5));
    assert!(!mset.contains(&7));
    assert!(!mset.contains(&0));
    assert!(!mset.contains(&1000));

    // first element
    assert_eq!(mset.first(), Some(&3));

    assert!(mset.remove(&3));
    assert_eq!(mset.first(), Some(&5));
    assert!(!mset.contains(&3));

    // last element
    assert_eq!(mset.last(), Some(&10));

    // count values
    mset.insert(20);
    mset.insert(20);
    mset.insert(20);
    /*
     * MultiSet { 5, 10, 20, 20, 20 }
     */

    assert_eq!(mset.len(), 5);

    assert_eq!(mset.count(&5), 1);
    assert_eq!(mset.count(&20), 3);
    assert_eq!(mset.count(&1000), 0);

    // clear all elements
    mset.clear();

    assert!(mset.is_empty());
    assert_eq!(mset.len(), 0);
}

#[test]
fn test_iterator() {
    let mut arr = [0, 9, 4, 4, 5, 5, 10, 10, 3, 3, 0, 0, 2, 1];

    let mset: MultiSet<usize> = arr.iter().cloned().collect();

    assert_eq!(mset.len(), 14);

    arr.sort();

    for (a, b) in mset.iter().zip(arr.iter()) {
        assert_eq!(a, b);
    }
}
