#![allow(non_snake_case)]

use cp_library_rs::utils::iterutil::IterUtil;

#[test]
fn test_join() {
    let arr = vec![2, 3, 5, 7, 11];

    assert_eq!(arr.iter().join(" "), "2 3 5 7 11");
    assert_eq!(arr.iter().join(", "), "2, 3, 5, 7, 11");

    let alph = ["alpha", "beta", "gamma", "delta", "epsilon"];

    assert_eq!(alph.iter().join(","), "alpha,beta,gamma,delta,epsilon");
    assert_eq!(alph.iter().join("\n"), "alpha\nbeta\ngamma\ndelta\nepsilon");
}
