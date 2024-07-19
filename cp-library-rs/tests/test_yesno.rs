use cp_library_rs::utils::yesno::YesNo;

#[test]
fn yesno() {
    assert_eq!(true.yesno(), "Yes");
    assert_eq!(false.yesno(), "No");
    assert_eq!((1 + 1 == 2).yesno(), "Yes");
}
