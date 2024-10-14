use cp_library_rs::number_theory::pollard_rho_algorithm::*;

#[test]
fn test_algo_method() {
    assert_eq!(factorize_pollard_rho(4033), vec![37, 109]);
    assert_eq!(factorize_pollard_rho(4681), vec![31, 151]);
    assert_eq!(factorize_pollard_rho(1000000007), vec![1000000007]);
    assert_eq!(
        factorize_pollard_rho(9999999999999),
        vec![3, 3, 53, 79, 265371653]
    );
    assert_eq!(
        factorize_pollard_rho(341550054645379),
        vec![341550054645379]
    );
    assert_eq!(
        factorize_pollard_rho(347484690041206937),
        vec![381727069, 910296173]
    );
}
