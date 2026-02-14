#![allow(non_snake_case)]

use cp_library_rs::utils::boolutil::BoolUtil;

#[test]
fn yesno() {
    assert_eq!(true.yesno(), "Yes");
    assert_eq!(false.yesno(), "No");
    assert_eq!((1 + 1 == 2).yesno(), "Yes");
}

#[test]
fn endl() {
    assert_eq!(false.endl(), " ");
    assert_eq!(true.endl(), "\n");
}
