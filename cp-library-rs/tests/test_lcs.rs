use cp_library_rs::{debug2D, string::lcs::*};

#[test]
fn test_LCS_isize() {
    let a: Vec<isize> = vec![0, 1, 2, 3, 4, 5];
    let b: Vec<isize> = vec![-1, 1, 3, 5, 7, 9];

    let lcs = LCS::build(&a, &b);

    assert_eq!(lcs.lcs, 3);
    assert_eq!(lcs.reconstruct(), vec![&1, &3, &5]);
}

#[test]
fn test_LCS_char() {
    let a: Vec<char> = "powell".chars().collect();
    let b: Vec<char> = "powershell".chars().collect();
    let c: Vec<char> = "towel".chars().collect();

    let lcs_ab = LCS::build(&a, &b);
    assert_eq!(lcs_ab.lcs, 6);
    assert_eq!(
        lcs_ab.reconstruct(),
        vec![&'p', &'o', &'w', &'e', &'l', &'l',]
    );

    let lcs_bc = LCS::build(&b, &c);
    assert_eq!(lcs_bc.lcs, 4);
    assert_eq!(lcs_bc.reconstruct(), vec![&'o', &'w', &'e', &'l']);
}
