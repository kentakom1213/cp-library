use cp_library_rs::comb::*;

const SIZE: usize = 5050505;

#[test]
fn test_comb() {
    let cmb = Comb::new(SIZE);

    assert_eq!(cmb.comb(5, 2), 10);
    assert_eq!(cmb.comb(100, 50), 198626801);
    assert_eq!(cmb.comb(100000, 50000), 710154335);
}

#[test]
fn test_perm() {
    let cmb = Comb::new(SIZE);

    assert_eq!(cmb.perm(5, 2), 20);
    assert_eq!(cmb.perm(100000, 50000), 801648426);
    assert_eq!(cmb.perm(100000, 30000), 87629341);
}
