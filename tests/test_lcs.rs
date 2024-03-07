use cp_library_rs::lcs::*;

#[test]
fn test_LCS_isize() {
    let a: Vec<isize> = vec![0, 1, 2, 3, 4, 5];
    let b: Vec<isize> = vec![-1, 1, 3, 5, 7, 9];

    assert_eq!(LCS(&a, &b), 3);
}

#[test]
fn test_LCS_char() {
    let a: Vec<char> = "powell".chars().collect();
    let b: Vec<char> = "powershell".chars().collect();

    assert_eq!(LCS(&a, &b), 6);
}

#[test]
fn test_LCS_with_Vec_usize() {
    let a: Vec<isize> = vec![0, 1, 2, 3, 4, 5];
    let b: Vec<isize> = vec![-1, 1, 3, 5, 7, 9];

    assert_eq!(LCS_with_Vec(&a, &b), vec![1, 3, 5]);
}

#[test]
fn test_LCS_with_Vec_char() {
    let a: Vec<char> = "powell".chars().collect();
    let b: Vec<char> = "powershell".chars().collect();

    assert_eq!(LCS_with_Vec(&a, &b), vec!['p', 'o', 'w', 'e', 'l', 'l']);
}
