use cp_library_rs::usize_tools::*;

#[test]
fn test_abs_diff() {
    assert_eq!(5.abs_diff(4), 1);
    assert_eq!(1.abs_diff(9), 8);
    assert_eq!(9238712.abs_diff(98370918237), 98361679525);
    assert_eq!(1092387.abs_diff(9238728), 8146341);
}

#[test]
fn test_sqrt() {
    assert_eq!(0.sqrt(), 0);
    assert_eq!(4.sqrt(), 2);
    assert_eq!(5.sqrt(), 2);
    assert_eq!(100.sqrt(), 10);
    assert_eq!(101.sqrt(), 10);
    assert_eq!(12390879108273.sqrt(), 3520068);
    assert_eq!(99121929823792.sqrt(), 9955999);
    assert_eq!(1001001001001001001.sqrt(), 1000500375);
}
