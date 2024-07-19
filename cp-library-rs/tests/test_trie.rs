use cp_library_rs::tree::trie::*;

#[test]
fn test_trie_node() {
    let mut trie: Trie<usize> = Trie::default();

    // 文字列の挿入
    // trie.insert("powell", 5);
    // trie.insert("kentakomoto", 11);
    // trie.insert("kenta", 5);
    // trie.insert("pow", 3);
    // trie.insert("", 0);
    *trie.get_or_insert_mut("powell") = Some(6);
    *trie.get_or_insert_mut("kenta") = Some(8);

    // デバッグ
    println!("{:#?}", trie);

    // 一覧表示
    let dict = trie.traverse();
    println!("{:?}", dict);

    // 検索
    println!("{:?}", trie.get_mut("pow"));

    *trie.get_mut("powell").unwrap() += 1;

    // 一覧表示
    let dict = trie.traverse();
    println!("{:?}", dict);
}
