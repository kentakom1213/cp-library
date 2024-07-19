use cp_library_rs::utils::coordinate_compression::*;

#[test]
fn test_compression_i32() {
    let arr = vec![6, 7, 100, 10, 4, 100, 20, 0, 300, 0];
    let comp = Compression::new(&arr);

    println!("{:?}", &comp);

    // idxのテスト
    assert_eq!(comp.idx(&0), Some(0));
    assert_eq!(comp.idx(&5), None);
    assert_eq!(comp.idx(&20), Some(5));
    assert_eq!(comp.idx(&300), Some(7));
    assert_eq!(comp.idx(&400), None);

    // valのテスト
    assert_eq!(comp.val(0), Some(&0));
    assert_eq!(comp.val(5), Some(&20));
    assert_eq!(comp.val(10), None);
}

#[test]
fn test_compression_val() {
    let arr = vec!["a", "zoo", "hello", "nagoya", "newyork", "a", "zoo"];
    let comp = Compression::new(&arr);

    println!("{:?}", &comp);

    // idxのテスト
    assert_eq!(comp.idx(&"a"), Some(0));
    assert_eq!(comp.idx(&"zoo"), Some(4));
    assert_eq!(comp.idx(&"akita"), None);

    // valのテスト
    assert_eq!(comp.val(2), Some(&"nagoya"));
    assert_eq!(comp.val(3), Some(&"newyork"));
    assert_eq!(comp.val(10), None);
}
