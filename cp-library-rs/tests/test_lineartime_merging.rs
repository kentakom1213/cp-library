#![allow(non_snake_case)]

use cp_library_rs::utils::lineartime_merging::*;
use itertools::Itertools;

#[test]
fn test_merge_usize() {
    let a = vec![0, 2, 4, 6, 8, 10];
    let b = vec![1, 3, 5, 7];

    assert_eq!(
        a.iter()
            .merge_linear(b.iter())
            .cloned()
            .collect::<Vec<usize>>(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10]
    );

    assert_eq!(
        b.iter()
            .merge_linear(a.iter())
            .cloned()
            .collect::<Vec<usize>>(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10]
    );
}

#[test]
fn test_char() {
    let a = "aabbbcddef";
    let b = "bbccceeeegh";

    assert_eq!(
        a.chars().merge_linear(b.chars()).collect::<String>(),
        "aabbbbbccccddeeeeefgh".to_string(),
    );

    assert_eq!(
        b.chars().merge_linear(a.chars()).collect::<String>(),
        "aabbbbbccccddeeeeefgh".to_string(),
    );

    let upper = "ABCDE";
    let lower = "abcde";

    assert_eq!(
        upper
            .chars()
            .merge_linear(lower.chars())
            .collect::<String>(),
        "ABCDEabcde".to_string(),
    );

    assert_eq!(
        upper
            .repeat(2)
            .chars()
            .sorted()
            .merge_linear(lower.repeat(3).chars().sorted())
            .collect::<String>(),
        "AABBCCDDEEaaabbbcccdddeee".to_string(),
    );
}
