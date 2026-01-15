#![allow(non_snake_case)]

use cp_library_rs::number_theory::factors_all::*;

#[test]
fn test_factors_all() {
    let facs = factors_all(10);
    assert_eq!(
        facs,
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 2, 4],
            vec![1, 5],
            vec![1, 2, 3, 6],
            vec![1, 7],
            vec![1, 2, 4, 8],
            vec![1, 3, 9],
            vec![1, 2, 5, 10],
        ]
    );
}
