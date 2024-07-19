use cp_library_rs::number_theory::modint_traits::*;

#[test]
fn test_madd() {
    let x: usize = 998244355;
    let y: usize = 998244359;
    assert_eq!(x.madd(y), 8);

    let a: usize = 998244353;
    let b: usize = 1000000007;
    let c: usize = 20021213;
    assert_eq!(a.madd(b).madd(c), 21776867);
}

#[test]
fn test_mneg() {
    let x: usize = 0;
    assert_eq!(x.mneg(), 0);
}

#[test]
fn test_msub() {
    let x: usize = 0;
    let y: usize = 1000000007;
    assert_eq!(x.msub(y), 996488699);

    let a: usize = 288230376151711744; // 1 << 58
    let b: usize = 576460752303423488; // 1 << 59
    let c: usize = 1152921504606846976; // 1 << 60
    assert_eq!(a.mneg().msub(b).msub(c), 553154679);
}

#[test]
fn test_mpow() {
    let x: usize = 2;
    let y: usize = 1000000007;
    assert_eq!(x.mpow(y), 132727571);

    let a: usize = 998244353;
    let b: usize = 1024;
    assert_eq!(a.mpow(b), 0);
}

#[test]
fn test_minv() {
    assert_eq!(1.minv(), 1);
    assert_eq!(2.minv(), 499122177);
    assert_eq!(1000.minv(), 981274199);
    assert_eq!((MOD - 1).minv(), 998244352);
}

#[test]
#[should_panic]
fn test_minv_err() {
    0.minv();
}

#[test]
#[should_panic]
fn test_mdiv_err() {
    1.mdiv(0);
}

#[test]
fn test_madd_assign() {
    let arr = vec![1, 2, 3];
    let mut ans = 0;
    for i in 0..3 {
        ans.madd_assign(arr[i]);
    }
    assert_eq!(ans, 6);
}
