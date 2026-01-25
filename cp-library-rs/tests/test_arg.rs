use std::cmp::Ordering;

use cp_library_rs::geometry::arg::Arg;

#[test]
fn test_basic_directions() {
    let mut v = vec![
        Arg(0, -1), // -90°
        Arg(-1, 0), // 180°
        Arg(0, 1),  // 90°
        Arg(1, 0),  // 0°
    ];
    v.sort();

    assert_eq!(
        v,
        vec![
            Arg(1, 0),  // 0°
            Arg(0, 1),  // 90°
            Arg(-1, 0), // 180°
            Arg(0, -1), // 270°
        ]
    );
}

#[test]
fn test_quadrant_order() {
    let mut v = vec![Arg(-1, -1), Arg(1, -1), Arg(-1, 1), Arg(1, 1)];
    v.sort();

    assert_eq!(v, vec![Arg(1, 1), Arg(-1, 1), Arg(-1, -1), Arg(1, -1),]);
}

#[test]
fn test_collinear_equal() {
    let a = Arg(1, 1);
    let b = Arg(2, 2);
    let c = Arg(-1, -1);

    assert_eq!(a.cmp(&b), Ordering::Equal);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_partial_eq_and_ord_consistency() {
    let a = Arg(1, 2);
    let b = Arg(2, 4);
    let c = Arg(-1, -2);

    assert!(a == b);
    assert!(a <= b);
    assert!(a >= b);

    assert!(a < c);
    assert!(c > a);
}

#[test]
fn test_sort_is_transitive() {
    let a = Arg(1, 0); // 0°
    let b = Arg(1, 1); // 45°
    let c = Arg(0, 1); // 90°

    assert!(a < b);
    assert!(b < c);
    assert!(a < c);
}

#[test]
fn test_from_tuple() {
    let a: Arg = (3, 4).into();
    let b: Arg = (&(3, 4)).into();

    assert_eq!(a, b);
    assert_eq!(a.to_tuple(), (3, 4));
}
