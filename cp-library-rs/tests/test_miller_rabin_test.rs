#![allow(non_snake_case)]

use cp_library_rs::number_theory::miller_rabin_test::*;

#[test]
fn test_algo_method() {
    assert_eq!(is_prime_MR(4033), false);
    assert_eq!(is_prime_MR(4681), false);
    assert_eq!(is_prime_MR(341550054645379), true);
    assert_eq!(is_prime_MR(347484690041206937), false);
}
