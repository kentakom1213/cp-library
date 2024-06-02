use cp_library_rs::affine1d::{Affine, AffineTransform};

#[test]
fn test_affine() {
    let f: Affine<isize> = (2, 1);
    let g: Affine<isize> = (-3, 2);

    for x in 0..20 {
        assert_eq!(f.apply(x), 2 * x + 1);
        assert_eq!(g.apply(x), -3 * x + 2);
    }

    // 積
    let fg = f.compose(&g);
    assert_eq!(fg, (-6, 5));

    let gf = g.compose(&f);
    assert_eq!(gf, (-6, -1));

    // 累乗
    let f0 = f.pow(0);
    assert_eq!(f0, (1, 0));

    let f1 = f.pow(1);
    assert_eq!(f1, (2, 1));

    let f2 = f.pow(2);
    assert_eq!(f2, (4, 3));

    let f3 = f.pow(3);
    assert_eq!(f3, (8, 7));

    let f4 = f.pow(4);
    assert_eq!(f4, (16, 15));

    let g0 = g.pow(0);
    assert_eq!(g0, (1, 0));

    let g1 = g.pow(1);
    assert_eq!(g1, (-3, 2));

    let g2 = g.pow(2);
    assert_eq!(g2, (9, -4));

    let g3 = g.pow(3);
    assert_eq!(g3, (-27, 14));

    let g4 = g.pow(4);
    assert_eq!(g4, (81, -40));
}
