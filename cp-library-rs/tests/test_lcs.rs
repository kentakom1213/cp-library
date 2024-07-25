use cp_library_rs::string::lcs::*;

#[test]
fn test_LCS_isize() {
    let a: Vec<isize> = vec![0, 1, 2, 3, 4, 5];
    let b: Vec<isize> = vec![-1, 1, 3, 5, 7, 9];

    let lcs = LCS::build(&a, &b);

    assert_eq!(lcs.lcs, 3);
    assert_eq!(lcs.reconstruct(), vec![1, 3, 5]);
}

#[test]
fn test_LCS_char() {
    let a: Vec<char> = "powell".chars().collect();
    let b: Vec<char> = "powershell".chars().collect();

    let lcs = LCS::build(&a, &b);

    assert_eq!(lcs.lcs, 6);
    assert_eq!(lcs.reconstruct(), "powell".chars().collect::<Vec<_>>());
}
