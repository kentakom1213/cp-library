use cp_library_rs::string::z::z_algorithm;

#[test]
fn test_empty() {
    let s: Vec<char> = vec![];
    assert_eq!(z_algorithm(&s), Vec::<usize>::new());
}

#[test]
fn test_single() {
    let s: Vec<char> = "a".chars().collect();
    assert_eq!(z_algorithm(&s), vec![1]);
}

#[test]
fn test_all_distinct() {
    let s: Vec<char> = "abcdefg".chars().collect();
    assert_eq!(z_algorithm(&s), vec![7, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_all_same() {
    let s: Vec<char> = "aaaaaa".chars().collect();
    assert_eq!(z_algorithm(&s), vec![6, 5, 4, 3, 2, 1]);
}

#[test]
fn test_typical_examples() {
    let s: Vec<char> = "aabcaabxaaaz".chars().collect();
    assert_eq!(z_algorithm(&s), vec![12, 1, 0, 0, 3, 1, 0, 0, 2, 2, 1, 0]);

    let s: Vec<char> = "abacaba".chars().collect();
    assert_eq!(z_algorithm(&s), vec![7, 0, 1, 0, 3, 0, 1]);

    let s: Vec<char> = "abababab".chars().collect();
    assert_eq!(z_algorithm(&s), vec![8, 0, 6, 0, 4, 0, 2, 0]);
}
