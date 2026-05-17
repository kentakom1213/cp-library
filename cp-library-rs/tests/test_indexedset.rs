#![allow(non_snake_case)]

use cp_library_rs::data_structure::indexedset::*;

#[test]
fn test_indexing() {
    let mut set = IndexedSet::from_iter([1, 4, 5, 10, 20, 100, 256, 1024, 10, -5, 32]);

    eprintln!("{:?}", set);

    assert_eq!(set.len(), 11);
    assert_eq!(set.distinct_len(), 10);
    assert_eq!(set.count(&10), 2);
    assert_eq!(set.contains(&10), true);
    assert_eq!(set.position(&10), Some(4));
    assert_eq!(set.first_ge(&10), Some(&10));
    assert_eq!(set.first_gt(&10), Some(&20));
    assert_eq!(set.last_le(&10), Some(&10));
    assert_eq!(set.last_lt(&10), Some(&5));

    assert_eq!(set.nth(0), Some(&(-5)));
    assert_eq!(set.nth(1), Some(&(1)));
    assert_eq!(set.nth(2), Some(&(4)));
    assert_eq!(set.nth(3), Some(&(5)));
    assert_eq!(set.nth(4), Some(&(10)));
    assert_eq!(set.nth(5), Some(&(10)));
    assert_eq!(set.nth(6), Some(&(20)));
    assert_eq!(set.nth(7), Some(&(32)));
    assert_eq!(set.nth(8), Some(&(100)));
    assert_eq!(set.nth(9), Some(&(256)));
    assert_eq!(set.nth(10), Some(&(1024)));

    set.insert(512);

    eprintln!("{:?}", set);

    assert_eq!(set.nth(0), Some(&(-5)));
    assert_eq!(set.nth(1), Some(&(1)));
    assert_eq!(set.nth(2), Some(&(4)));
    assert_eq!(set.nth(3), Some(&(5)));
    assert_eq!(set.nth(4), Some(&(10)));
    assert_eq!(set.nth(5), Some(&(10)));
    assert_eq!(set.nth(6), Some(&(20)));
    assert_eq!(set.nth(7), Some(&(32)));
    assert_eq!(set.nth(8), Some(&(100)));
    assert_eq!(set.nth(9), Some(&(256)));
    assert_eq!(set.nth(10), Some(&(512)));
    assert_eq!(set.nth(11), Some(&(1024)));
}

#[test]
fn test_remove_multiset() {
    let mut set = IndexedSet::from_iter([3, 3, 3, 4, 5]);

    assert_eq!(set.remove(&3), true);
    assert_eq!(set.count(&3), 2);
    assert_eq!(set.position(&4), Some(2));

    assert_eq!(set.remove(&3), true);
    assert_eq!(set.remove(&3), true);
    assert_eq!(set.remove(&3), false);
    assert_eq!(set.contains(&3), false);
    assert_eq!(set.nth(0), Some(&4));
}

#[test]
fn test_remove_n_and_remove_all() {
    let mut set = IndexedSet::from_iter([1, 2, 2, 2, 2, 3, 3, 4]);

    assert_eq!(set.remove_n(&2, 0), 0);
    assert_eq!(set.remove_n(&2, 2), 2);
    assert_eq!(set.count(&2), 2);
    assert_eq!(set.len(), 6);
    assert_eq!(set.position(&3), Some(3));

    assert_eq!(set.remove_n(&2, 10), 2);
    assert_eq!(set.contains(&2), false);
    assert_eq!(set.len(), 4);
    assert_eq!(set.position(&3), Some(1));

    assert_eq!(set.remove_all(&3), 2);
    assert_eq!(set.contains(&3), false);
    assert_eq!(set.nth(0), Some(&1));
    assert_eq!(set.nth(1), Some(&4));
    assert_eq!(set.remove_all(&100), 0);
}
