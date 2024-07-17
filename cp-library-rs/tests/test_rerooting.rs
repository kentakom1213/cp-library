use cp_library_rs::rerooting::{Rerooting, TreeMonoid};

#[test]
fn test_diameter() {
    let mut dp: Rerooting<Diameter> = Rerooting::new(6);

    dp.add_edge2(0, 1, 1);
    dp.add_edge2(0, 2, 1);
    dp.add_edge2(2, 3, 1);
    dp.add_edge2(2, 4, 1);
    dp.add_edge2(4, 5, 1);

    dp.build();

    assert_eq!(dp.ans, vec![3, 4, 2, 3, 3, 4]);
}

struct Diameter;

impl TreeMonoid for Diameter {
    type T = isize;
    fn id() -> Self::T {
        0
    }
    fn merge(x: &Self::T, y: &Self::T) -> Self::T {
        *x.max(y)
    }
    fn put_edge(x: &Self::T, weight: &Self::T) -> Self::T {
        x + weight
    }
}
