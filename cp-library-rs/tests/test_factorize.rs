use cp_library_rs::number_theory::factorize::*;

#[test]
fn test_factorize() {
    assert_eq!(factorize(1024), vec![(2, 10)]);

    assert_eq!(factorize(123456789), vec![(3, 2), (3607, 1), (3803, 1)]);

    assert_eq!(factorize(20021213), vec![(20021213, 1)]);

    assert_eq!(
        factorize(1234567891234567),
        vec![(47, 1), (167, 1), (167953, 1), (936511, 1)]
    );
}
