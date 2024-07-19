use cp_library_rs::data_structure::indexedset::*;

#[test]
fn test_indexing() {
    let mut set = IndexedSet::from_iter([1, 4, 5, 10, 20, 100, 256, 1024, 10, -5, 32]);

    eprintln!("{:?}", set);

    assert_eq!(set.get_by_index(0), Some(&(-5)));
    assert_eq!(set.get_by_index(1), Some(&(1)));
    assert_eq!(set.get_by_index(2), Some(&(4)));
    assert_eq!(set.get_by_index(3), Some(&(5)));
    assert_eq!(set.get_by_index(4), Some(&(10)));
    assert_eq!(set.get_by_index(5), Some(&(20)));
    assert_eq!(set.get_by_index(6), Some(&(32)));
    assert_eq!(set.get_by_index(7), Some(&(100)));
    assert_eq!(set.get_by_index(8), Some(&(256)));
    assert_eq!(set.get_by_index(9), Some(&(1024)));

    set.insert(512);

    eprintln!("{:?}", set);

    assert_eq!(set.get_by_index(0), Some(&(-5)));
    assert_eq!(set.get_by_index(1), Some(&(1)));
    assert_eq!(set.get_by_index(2), Some(&(4)));
    assert_eq!(set.get_by_index(3), Some(&(5)));
    assert_eq!(set.get_by_index(4), Some(&(10)));
    assert_eq!(set.get_by_index(5), Some(&(20)));
    assert_eq!(set.get_by_index(6), Some(&(32)));
    assert_eq!(set.get_by_index(7), Some(&(100)));
    assert_eq!(set.get_by_index(8), Some(&(256)));
    assert_eq!(set.get_by_index(9), Some(&(512)));
    assert_eq!(set.get_by_index(10), Some(&(1024)));
}
