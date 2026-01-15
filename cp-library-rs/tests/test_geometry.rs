#![allow(non_snake_case)]

use cp_library_rs::geometry::{
    basic::{Segment, EPS},
    vec2::Vec2,
};

#[test]
fn test_add_sub_mul_vec2() {
    let a = Vec2(2, 3);
    let b = Vec2(5, -2);

    assert_eq!(a + b, Vec2(7, 1));
    assert_eq!(a - b, Vec2(-3, 5));

    let a2 = a * -2;
    let b2 = b * 3;
    assert_eq!(a2 + b2, Vec2(11, -12));
}

#[test]
fn test_dot() {
    let a = Vec2(2.0, -5.0);
    let b = Vec2(10.0, 4.0);

    assert_eq!(a.dot(b), 0.0);
}

#[test]
fn test_dist2() {
    let zero = Vec2(0, 0);
    let a = Vec2(1, 2);
    let b = Vec2(2, 1);

    let dist_0_a = a.dist2(zero);
    let dist_0_b = b.dist2(zero);
    assert_eq!(dist_0_a, dist_0_b);

    let dist_a_b = a.dist2(b);
    assert_eq!(dist_a_b, 2);
}

#[test]
fn test_collision_line() {
    let ab = Segment(Vec2(3.0, 1.0), Vec2(-3.0, 1.0));

    let line1 = Segment(Vec2(1.0, 2.0), Vec2(2.0, 2.0));

    let line2 = Segment(Vec2(1.0, 2.0), Vec2(1.0, 0.0));

    assert!(!ab.has_intersection(&line1));
    assert!(ab.has_intersection(&line2));
}

#[test]
fn test_norm_dist() {
    let a = Vec2(1.0, 3.0);
    let b = Vec2(1.0, 2.0);
    let c = Vec2(3.0, 0.0);

    assert!((a.norm() - 10_f64.sqrt()).abs() < EPS);
    assert!((b.norm() - 5_f64.sqrt()).abs() < EPS);
    assert!((c.norm() - 3.0).abs() < EPS);

    assert!((a.dist(b) - 1.0).abs() < EPS);
    assert!((a.dist(c) - 13_f64.sqrt()).abs() < EPS);
    assert!((b.dist(c) - 8_f64.sqrt()).abs() < EPS);
}
