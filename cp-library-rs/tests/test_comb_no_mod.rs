use cp_library_rs::number_theory::comb_no_mod::comb;

#[test]
fn test_comb_random() {
    assert_eq!(comb(24, 44), 0);
    assert_eq!(comb(20, 41), 0);
    assert_eq!(comb(50, 17), 9847379391150);
    assert_eq!(comb(51, 9), 3042312350);
    assert_eq!(comb(44, 18), 1029530696964);
    assert_eq!(comb(5, 58), 0);
    assert_eq!(comb(25, 10), 3268760);
    assert_eq!(comb(56, 3), 27720);
    assert_eq!(comb(5, 32), 0);
    assert_eq!(comb(19, 60), 0);
    assert_eq!(comb(53, 58), 0);
    assert_eq!(comb(10, 28), 0);
    assert_eq!(comb(56, 50), 32468436);
    assert_eq!(comb(29, 3), 3654);
    assert_eq!(comb(43, 35), 145008513);
    assert_eq!(comb(54, 40), 3245372870670);
    assert_eq!(comb(24, 24), 1);
    assert_eq!(comb(56, 22), 2142582442263900);
    assert_eq!(comb(5, 17), 0);
    assert_eq!(comb(56, 25), 5574440580220512);
    assert_eq!(comb(36, 25), 600805296);
    assert_eq!(comb(6, 5), 6);
    assert_eq!(comb(45, 11), 10150595910);
    assert_eq!(comb(56, 48), 1420494075);
    assert_eq!(comb(24, 9), 1307504);
    assert_eq!(comb(7, 23), 0);
    assert_eq!(comb(39, 22), 51021117810);
    assert_eq!(comb(32, 60), 0);
    assert_eq!(comb(14, 10), 1001);
    assert_eq!(comb(25, 48), 0);
}
