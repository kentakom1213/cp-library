use cp_library_rs::number_theory::frac::*;

#[test]
fn test_eq() {
    let values = vec![
        Frac(0, 1),
        Frac(8, 2),
        Frac(4, 1),
        Frac(4, 4),
        Frac(5, 5),
        Frac(3, 2),
        Frac(1, 0),
        Frac(2, 0),
    ];

    let eq_matrix = vec![
        vec![1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 1, 1],
    ];

    for i in 0..8 {
        for j in 0..8 {
            assert_eq!((values[i] == values[j]) as u8, eq_matrix[i][j]);
        }
    }
}

#[test]
fn test_ord() {
    let values = vec![
        Frac(0, 1),
        Frac(1, 8),
        Frac(2, 8),
        Frac(4, 4),
        Frac(5, 5),
        Frac(3, 2),
        Frac(8, 2),
        Frac(4, 1),
        Frac(1, 0),
        Frac(2, 0),
    ];

    let mut sorted = values.clone();
    sorted.sort();

    assert_eq!(values, sorted);
}
