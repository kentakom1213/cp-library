use cp_library_rs::modint::*;
use rand::prelude::*;

const MOD998: usize = 998244353;

#[test]
fn test_add() {
    let x: Mod998 = 998244355.into();
    let y: usize = 998244359;
    assert_eq!(x + y, 8 + MOD998);

    let a: Mod998 = MOD998.into();
    let b = 1000000007;
    let c = 20021213;
    assert_eq!(a + b + c, 21776867);
    assert_eq!(a + b + c, (21776867 + MOD998));
}

#[test]
fn test_neg() {
    let x: Mod998 = 0.into();
    assert_eq!(-x, 0);

    let y = Mod998::new(10);
    assert_eq!(-y, MOD998 - 10);

    let z = Mod998::new(MOD998 + 200);
    assert_eq!(-z, MOD998 - 200);
}

#[test]
fn test_sub() {
    let x = Mod998::new(0);
    let y = 1000000007;
    assert_eq!(x - y, 996488699);

    let a: Mod998 = 288230376151711744.into(); // 1 << 58
    let b: usize = 576460752303423488; // 1 << 59
    let c: usize = 1152921504606846976; // 1 << 60
    assert_eq!(-a - b - c, 553154679);

    let zero = Mod998::new(0) + 1 - 1;
    assert_eq!(zero.0, 0);
}

#[test]
fn test_pow() {
    let x = Mod998::new(2);
    let y: usize = 1000000007;
    assert_eq!(x.pow(y), 132727571);

    let a: Mod998 = MOD998.into();
    let b: usize = 1024;
    assert_eq!(a.pow(b), 0);
}

#[test]
fn test_inv() {
    assert_eq!(Mod998::new(1).inv(), 1);
    assert_eq!(Mod998::new(2).inv(), 499122177);
    assert_eq!(Mod998::new(1000).inv(), 981274199);
    assert_eq!(Mod998::new(998244352).inv(), 998244352);
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
    let mut add = Mod109::new(0);
    let mut sub = Mod109::new(0);
    for i in 0..20 {
        add += i;
        sub -= i;
    }

    assert_eq!(sub, -add);
}

#[test]
fn test_mul_assign() {
    let mut fact = vec![Mod998::new(1); 20];

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
fn test_sum() {
    assert_eq!(
        (0..20).map(|i| Mod109::new(2).pow(i)).sum::<Mod109>(),
        Mod109::new(2).pow(20) - 1
    );
}

#[test]
fn test_product() {
    assert_eq!(
        (0..100).map(|_| 3.into()).product::<Mod109>(),
        Mod109::new(3).pow(100)
    );
}

#[test]
fn test_from_isize() {
    for _ in 0..200 {
        let x: isize = random();
        let x_mod = (M998 as isize + x % M998 as isize) as usize % M998;
        let y = Mod998::from_isize(x);
        assert_eq!(x_mod, y.0);
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// ランダムな分数を生成する
fn generate_random_fraction(rng: &mut ThreadRng) -> ((isize, isize), Mod998) {
    let n = rng.gen_range(-1000..=1000);
    let d = rng.gen_range(1..=2000);
    let g = gcd(n, d).abs();
    let (n, d) = (n / g, d / g);
    // modintを生成
    let m = Mod998::from_isize(n) / d as usize;
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
}
