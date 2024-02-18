use cp_library_rs::vec2::*;

#[test]
fn test_add_sub_mul_vec2() {
    let a = (2, 3);
    let b = (5, -2);

    assert_eq!(a.add(b), (7, 1));
    assert_eq!(a.sub(b), (-3, 5));

    let a2 = a.mul(-2);
    let b2 = b.mul(3);
    assert_eq!(a2.add(b2), (11, -12));
}

#[test]
fn test_dot() {
    let a = (2.0, -5.0);
    let b = (10.0, 4.0);

    assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dist2() {
    let zero = (0, 0);
    let a = (1, 2);
    let b = (2, 1);

    let dist_0_a = a.dist2(zero);
    let dist_0_b = b.dist2(zero);
    assert_eq!(dist_0_a, dist_0_b);

    let dist_a_b = a.dist2(b);
    assert_eq!(dist_a_b, 2);
}

#[test]
fn test_collision_line() {
    let ab: Line<isize> = ((3, 1), (-3, 1));

    let line1: Line<isize> = ((1, 2), (2, 2));

    let line2: Line<isize> = ((1, 2), (1, 0));

    assert_eq!(is_collided(ab, line1), false);
    assert_eq!(is_collided(ab, line2), true);
}
