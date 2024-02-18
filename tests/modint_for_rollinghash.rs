use cp_library_rs::modint_for_rollinghash::*;

#[test]
fn test_neg() {
    let x: Modint = 0.into();
    assert_eq!(-x, 0);

    let y = Modint::new(10);
    assert_eq!(-y, MOD - 10);

    let z = Modint::new(MOD + 200);
    assert_eq!(-z, MOD - 200);
}

#[test]
fn test_sub() {
    let x = Modint::new(0);
    let y = 1000000007;
    assert_eq!(x - y, MOD - y);

    let a: Modint = 288230376151711744.into(); // 1 << 58
    let b: usize = 576460752303423488; // 1 << 59
    let c: usize = 1152921504606846976; // 1 << 60
    assert_eq!(
        -a - b - c,
        MOD - (288230376151711744 + 576460752303423488 + 1152921504606846976)
    );

    let zero = Modint::new(0) + 1 - 1;
    assert_eq!(zero.0, 0);
}

#[test]
fn test_pow() {
    let x = Modint::new(2);
    let y: usize = 1000000007;
    assert_eq!(x.pow(y), 35184372088832);

    let a: Modint = MOD.into();
    let b: usize = 1024;
    assert_eq!(a.pow(b), 0);
}

#[test]
fn test_inv() {
    assert_eq!(Modint::new(1).inv(), 1);
    assert_eq!(Modint::new(2).inv(), 1152921504606846976);
    assert_eq!(Modint::new(1000).inv(), 1035323511136948584);
    assert_eq!(Modint::new(MOD - 1).inv(), MOD - 1);
}

#[test]
fn test_add_assign() {
    let arr = vec![1, 2, 3];
    let mut ans = 0;
    for i in 0..3 {
        ans += arr[i];
    }
    assert_eq!(ans, 6);
}

#[test]
fn test_sub_assign() {
    let mut add = Modint::new(0);
    let mut sub = Modint::new(0);
    for i in 0..20 {
        add += i;
        sub -= i;
    }

    assert_eq!(sub, -add);
}

#[test]
fn test_mul_assign() {
    let mut fact = vec![Modint::new(1); 20];

    // 階乗
    for i in 1..20 {
        let prv = fact[i - 1];
        fact[i] *= prv * i;
    }

    assert_eq!(
        &fact,
        &[
            1,
            1,
            2,
            6,
            24,
            120,
            720,
            5040,
            40320,
            362880,
            3628800,
            39916800,
            479001600,
            6227020800,
            87178291200,
            1307674368000,
            20922789888000,
            355687428096000,
            6402373705728000,
            121645100408832000
        ]
    );
}

#[test]
fn test_sum() {
    assert_eq!(
        (0..20).map(|i| Modint::new(2).pow(i)).sum::<Modint>(),
        Modint::new(2).pow(20) - 1
    );
}

#[test]
fn test_product() {
    assert_eq!(
        (0..100).map(|_| 3.into()).product::<Modint>(),
        Modint::new(3).pow(100)
    );
}
