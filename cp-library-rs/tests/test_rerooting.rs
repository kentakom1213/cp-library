use cp_library_rs::rerooting::Rerooting;

#[test]
fn test_diameter() {
    let mut dp: Rerooting<isize, _, _, _> = Rerooting::new(6, || 0, |a, b| *a.max(b), |x, w| x + w);

    dp.add_edge2(0, 1, 1);
    dp.add_edge2(0, 2, 1);
    dp.add_edge2(2, 3, 1);
    dp.add_edge2(2, 4, 1);
    dp.add_edge2(4, 5, 1);

    dp.build();

    assert_eq!(dp.ans, vec![3, 4, 2, 3, 3, 4]);
}
