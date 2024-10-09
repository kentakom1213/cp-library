use cp_library_rs::data_structure::multiset_splay_tree::Multiset;
use rand::*;
use std::collections::BTreeMap;

#[test]
fn test_random_insert_delete() {
    const QUERY_SIZE: usize = 10_000;

    let mut map = BTreeMap::<u8, usize>::new();
    let mut multiset = Multiset::<u8>::new();

    for _ in 0..QUERY_SIZE {
        let x = random();

        // mapに追加
        *map.entry(x).or_insert(0) += 1;

        // multisetに挿入
        multiset.insert(x);

        assert_eq!(map[&x], multiset.count(&x));

        let y = random();

        // mapから削除
        map.get_mut(&y).filter(|v| **v > 0).map(|v| *v -= 1);

        // multisetから削除
        multiset.remove(&y);

        assert_eq!(map.get(&y).unwrap_or(&0), &multiset.count(&y));
    }
}
