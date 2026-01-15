#![allow(non_snake_case)]

use cp_library_rs::number_theory::miller_rabin_test::*;

#[test]
fn test_algo_method() {
    assert!(!is_prime_MR(4033));
    assert!(!is_prime_MR(4681));
    assert!(is_prime_MR(341550054645379));
    assert!(!is_prime_MR(347484690041206937));
}
