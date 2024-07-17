use cp_library_rs::euler_tour::EulerTour;

#[test]
fn test_euler_tour() {
    let mut tree = EulerTour::new(6);

    //      0
    //     / \
    //    1  2
    //   / \
    //  3  4
    //  |
    //  5

    tree.add_edge(0, 1);
    tree.add_edge(0, 2);
    tree.add_edge(1, 3);
    tree.add_edge(1, 4);
    tree.add_edge(3, 5);

    tree.build(0);

    assert_eq!(&tree.in_, &vec![0, 1, 9, 2, 6, 3]);
    assert_eq!(&tree.out, &vec![11, 8, 10, 5, 7, 4]);
}
