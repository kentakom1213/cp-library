use cp_library_rs::multiset::*;

#[test]
fn test_usize() {
    let mut mset: MultiSet<usize> = MultiSet::new();

    assert!(mset.is_empty());

    mset.insert(5);
    mset.insert(7);
    mset.insert(7);
    mset.insert(10);
    mset.insert(3);

    // remove value
    assert_eq!(mset.remove(&7), true);
    assert_eq!(mset.remove(&7), true);
    assert_eq!(mset.remove(&0), false);

    // is_contain
    assert_eq!(mset.contains(&5), true);
    assert_eq!(mset.contains(&7), false);
    assert_eq!(mset.contains(&0), false);
    assert_eq!(mset.contains(&1000), false);

    // first element
    assert_eq!(mset.first(), Some(&3));

    assert_eq!(mset.remove(&3), true);
    assert_eq!(mset.first(), Some(&5));
    assert_eq!(mset.contains(&3), false);

    // last element
    assert_eq!(mset.last(), Some(&10));

    // count values
    mset.insert(20);
    mset.insert(20);
    mset.insert(20);
    /*
     * MultiSet { 3, 5, 10, 20, 20, 20 }
     */

    assert_eq!(mset.count(&5), 1);
    assert_eq!(mset.count(&20), 3);
    assert_eq!(mset.count(&1000), 0);
}
