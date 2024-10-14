use cp_library_rs::number_theory::factorize_query::*;

#[test]
fn test_factorize() {
    let f = FactorTable::new(200_000);

    assert_eq!(f.factorize(200), vec![2, 2, 2, 5, 5]);
    assert_eq!(f.factorize(123450), vec![2, 3, 5, 5, 823]);
    assert_eq!(f.factorize(107311), vec![239, 449]);
    assert_eq!(f.factorize(199999), vec![199999]);
}

#[test]
fn test_factorize_pairs() {
    let f = FactorTable::new(200_000);

    assert_eq!(f.factorize_pairs(200), vec![(2, 3), (5, 2)]);
    assert_eq!(
        f.factorize_pairs(123450),
        vec![(2, 1), (3, 1), (5, 2), (823, 1)]
    );
    assert_eq!(f.factorize_pairs(107311), vec![(239, 1), (449, 1)]);
    assert_eq!(f.factorize_pairs(199999), vec![(199999, 1)]);
}
