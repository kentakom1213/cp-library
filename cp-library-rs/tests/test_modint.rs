use cp_library_rs::number_theory::modint::*;
use num_traits::{One, Zero};
use rand::prelude::*;

const MOD998: usize = 998244353;

#[test]
fn test_add() {
    let x: M998 = 998244355.into();
    let y: usize = 998244359;
    assert_eq!(x + y, 8 + MOD998);

    let a: M998 = MOD998.into();
    let b = 1000000007;
    let c = 20021213;
    assert_eq!(a + b + c, 21776867);
    assert_eq!(a + b + c, (21776867 + MOD998));
}

#[test]
fn test_neg() {
    let x = M998::zero();
    assert_eq!(-x, 0);

    let y = M998::new(10);
    assert_eq!(-y, MOD998 - 10);

    let z = M998::new(MOD998 + 200);
    assert_eq!(-z, MOD998 - 200);
}

#[test]
fn test_sub() {
    let x = M998::zero();
    let y = 1000000007;
    assert_eq!(x - y, 996488699);

    let a: M998 = 288230376151711744.into(); // 1 << 58
    let b: usize = 576460752303423488; // 1 << 59
    let c: usize = 1152921504606846976; // 1 << 60
    assert_eq!(-a - b - c, 553154679);

    let zero = M998::zero() + 1 - 1;
    assert_eq!(zero.0, 0);
}

#[test]
fn test_pow() {
    let x = M998::new(2);
    let y: usize = 1000000007;
    assert_eq!(x.pow(y), 132727571);

    let a: M998 = MOD998.into();
    let b: usize = 1024;
    assert_eq!(a.pow(b), 0);
}

#[test]
fn test_inv() {
    assert_eq!(M998::one().inv(), 1);
    assert_eq!(M998::new(2).inv(), 499122177);
    assert_eq!(M998::new(1000).inv(), 981274199);
    assert_eq!(M998::new(998244352).inv(), 998244352);
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
fn test_add_usize() {
    let a = M998::new(998244333);
    let y: usize = 1001001001001001001;

    assert_eq!(a + y, a + M998::new(y));
}

#[test]
fn test_sub_assign() {
    let mut add = M107::zero();
    let mut sub = M107::zero();
    for i in 0..20 {
        add += i;
        sub -= i;
    }

    assert_eq!(sub, -add);
}

#[test]
fn test_mul_assign() {
    let mut fact = vec![M998::one(); 20];

    // 階乗
    for i in 1..20 {
        let prv = fact[i - 1];
        fact[i] *= prv * i;
    }

    assert_eq!(
        &fact,
        &[
            1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 237554682,
            331032489, 972509923, 586493473, 986189864, 781263551, 868586527
        ]
    );
}

#[test]
fn test_div_assign() {
    let mut iota: Vec<M998> = (0..10).map(|i| i.into()).collect();

    eprintln!("{:?}", iota);

    for i in 0..10 {
        iota[i] /= 2;
    }

    eprintln!("{:?}", iota);
}

#[test]
fn test_sum() {
    assert_eq!(
        (0..20).map(|i| M107::new(2).pow(i)).sum::<M107>(),
        M107::new(2).pow(20) - 1
    );
}

#[test]
fn test_product() {
    assert_eq!(
        (0..100).map(|_| 3.into()).product::<M107>(),
        M107::new(3).pow(100)
    );
}

#[test]
fn test_from_isize() {
    for _ in 0..200 {
        let x: isize = random();
        let x_mod = (998244353 as isize + x % 998244353 as isize) as usize % 998244353;
        let y = M998::from_isize(x);
        assert_eq!(x_mod, y.0);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ランダムな分数を生成する
fn generate_random_fraction(rng: &mut ThreadRng) -> ((usize, usize), M998) {
    let n = rng.gen_range(0..=4000);
    let d = rng.gen_range(1..=4000);
    let g = gcd(n, d);
    let (n, d) = (n / g, d / g);
    // modintを生成
    let m = M998::new(n) / d;
    ((n, d), m)
}

#[test]
fn test_rational_reconstruction() {
    let mut rng = thread_rng();

    for _ in 0..200 {
        let ((n, d), m) = generate_random_fraction(&mut rng);
        if let Some((p, q)) = m.rational_reconstruction() {
            println!("x: {m:?} -> {:?}", (p, q));
            assert_eq!((n, d), (p, q));
        }
    }

    // 分母1
    for i in 0..=20 {
        let x = M998::new(i);
        let (p, q) = x.rational_reconstruction().unwrap();
        println!("x: {x:?} -> {:?}", (p, q));
        assert_eq!((i, 1), (p, q));
    }

    // 分母1
    for i in 1..23 {
        let x = M998::new(i) / 23;
        let (p, q) = x.rational_reconstruction().unwrap();
        println!("x: {x:?} -> {:?}", (p, q));
        assert_eq!((i, 23), (p, q));
    }
}
