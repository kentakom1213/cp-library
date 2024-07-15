use cp_library_rs::{debug, suffix_array::SuffixArray};

#[test]
fn test_build() {
    assert_eq!(
        SuffixArray::build("abracadabra"),
        vec![11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]
    );

    assert_eq!(
        SuffixArray::build("momomosumomomomomonouchi"),
        vec![
            24, 21, 22, 23, 8, 10, 12, 0, 14, 2, 16, 4, 18, 9, 11, 13, 1, 15, 3, 17, 5, 19, 6, 20,
            7
        ]
    );

    assert_eq!(
        SuffixArray::build("aaaaaaaaaa"),
        vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    )
}
