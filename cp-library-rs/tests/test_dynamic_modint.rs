use cp_library_rs::number_theory::dynamic_modint::*;
use rand::prelude::*;

const MOD998: usize = 998244353;
const MOD1000: usize = 1000;

#[test]
fn test_add() {
    let x = Modint2::new(998244355, MOD998);
    let y: usize = 998244359;
    assert_eq!(x + y, 8 + MOD998);

    let a = Modint2::new(MOD998, MOD998);
    let b = 1000000007;
    let c = 20021213;
    assert_eq!(a + b + c, 21776867);
    assert_eq!(a + b + c, (21776867 + MOD998));
}

#[test]
fn test_neg() {
    let x = Modint2::new(0, MOD998);
    assert_eq!(-x, 0);

    let y = Modint2::new(10, MOD998);
    assert_eq!(-y, MOD998 - 10);

    let z = Modint2::new(MOD998 + 200, MOD998);
    assert_eq!(-z, MOD998 - 200);
}

#[test]
fn test_sub() {
    let x = Modint2::new(0, MOD998);
    let y = 1000000007;
    assert_eq!(x - y, 996488699);

    let a = Modint2::new(288230376151711744, MOD998); // 1 << 58
    let b: usize = 576460752303423488; // 1 << 59
    let c: usize = 1152921504606846976; // 1 << 60
    assert_eq!(-a - b - c, 553154679);

    let zero = Modint2::new(0, MOD998) + 1 - 1;
    assert_eq!(zero, 0);
}

#[test]
fn test_pow() {
    let x = Modint2::new(2, MOD998);
    let y: usize = 1000000007;
    assert_eq!(x.pow(y), 132727571);

    let a = Modint2::new(MOD998, MOD998);
    let b: usize = 1024;
    assert_eq!(a.pow(b), 0);
}

#[test]
fn test_inv() {
    assert_eq!(Modint2::new(1, MOD998).inv(), 1);
    assert_eq!(Modint2::new(2, MOD998).inv(), 499122177);
    assert_eq!(Modint2::new(1000, MOD998).inv(), 981274199);
    assert_eq!(Modint2::new(998244352, MOD998).inv(), 998244352);
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
    let a = Modint2::new(998244333, MOD998);
    let y: usize = 1001001001001001001;

    assert_eq!(a + y, a + Modint2::new(y, MOD998));
}

#[test]
#[should_panic]
fn test_panic_different_mod() {
    let a = Modint2::new(998244333, MOD998);
    let y: usize = 1001001001001001001;

    let _ = a + Modint2::new(y, MOD1000);
}

#[test]
fn test_sub_assign() {
    let mut add = Modint2::new(0, MOD1000);
    let mut sub = Modint2::new(0, MOD1000);
    for i in 0..20 {
        add += i;
        sub -= i;
    }

    assert_eq!(sub, -add);
}

#[test]
fn test_mul_assign() {
    let mut fact = vec![Modint2::new(1, MOD998); 20];

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
    let mut iota: Vec<Modint2> = (0..10).map(|i| Modint2::new(i, MOD998)).collect();

    eprintln!("{:?}", iota);

    for i in 0..10 {
        iota[i] /= 2;
    }

    eprintln!("{:?}", iota);
}

#[test]
fn test_from_isize() {
    for _ in 0..200 {
        let x: isize = random();
        let x_mod = (998244353 as isize + x % 998244353 as isize) as usize % 998244353;
        let y = Modint2::from_isize(x, MOD998);
        assert_eq!(x_mod, y.value);
    }
}
