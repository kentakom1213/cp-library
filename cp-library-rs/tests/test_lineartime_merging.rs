use cp_library_rs::utils::lineartime_merging::*;
use itertools::Itertools;

#[test]
fn test_merge_usize() {
    let a = vec![0, 2, 4, 6, 8, 10];
    let b = vec![1, 3, 5, 7];

    assert_eq!(
        merge(a.iter(), b.iter()).cloned().collect::<Vec<usize>>(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10]
    );

    assert_eq!(
        merge(b.iter(), a.iter()).cloned().collect::<Vec<usize>>(),
        vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10]
    );
}

#[test]
fn test_char() {
    let a = "aabbbcddef";
    let b = "bbccceeeegh";

    assert_eq!(
        merge(a.chars(), b.chars()).collect::<String>(),
        "aabbbbbccccddeeeeefgh".to_string(),
    );

    assert_eq!(
        merge(b.chars(), a.chars()).collect::<String>(),
        "aabbbbbccccddeeeeefgh".to_string(),
    );

    let upper = "ABCDE";
    let lower = "abcde";

    assert_eq!(
        merge(upper.chars(), lower.chars()).collect::<String>(),
        "ABCDEabcde".to_string(),
    );

    assert_eq!(
        merge(
            upper.repeat(2).chars().sorted(),
            lower.repeat(3).chars().sorted()
        )
        .collect::<String>(),
        "AABBCCDDEEaaabbbcccdddeee".to_string(),
    );
}
