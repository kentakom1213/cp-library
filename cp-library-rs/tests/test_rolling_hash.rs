#![allow(non_snake_case)]

use cp_library_rs::{number_theory::modint_for_rollinghash::Modint, string::rolling_hash::*};

#[test]
fn test_pattern_match() {
    let base = Modint::generate_base();

    let target = RollingHash::from_str("momomosumomomomomonouchi", base);
    let ptn1 = RollingHash::from_str("sumomo", base);
    let ptn2 = RollingHash::from_str("momo", base);

    // "sumomo"を検索
    let mut res1 = vec![];
    for i in 0..target.len() - ptn1.len() {
        if target.hash(i..i + ptn1.len()) == ptn1.hash(..) {
            res1.push(i);
        }
    }

    assert_eq!(res1, vec![6]);

    // "momo"を検索
    let mut res2 = vec![];
    for i in 0..target.len() - ptn2.len() {
        if target.hash(i..i + ptn2.len()) == ptn2.hash(..) {
            res2.push(i);
        }
    }

    assert_eq!(res2, vec![0, 2, 8, 10, 12, 14]);
}

#[test]
fn test_concat() {
    let base = Modint::generate_base();

    let a = "abc";
    let b = "str";
    let c = "abcstr";

    let hash_a = RollingHash::from_str(a, base);
    let hash_b = RollingHash::from_str(b, base);
    let hash_c = RollingHash::from_str(c, base);

    assert_eq!(hash_a.hash(..).chain(&hash_b.hash(..)), hash_c.hash(..));
    assert_ne!(hash_b.hash(..).chain(&hash_a.hash(..)), hash_c.hash(..));
    assert_eq!(
        hash_a.hash(0..3).chain(&hash_a.hash(3..3),),
        hash_a.hash(..)
    );
    assert_eq!(hash_a.hash(..).chain(&hash_a.hash(0..0),), hash_a.hash(..));
    assert_eq!(hash_a.hash(0..0).chain(&hash_a.hash(..),), hash_a.hash(..));
}

#[test]
fn test_LCP() {
    let rh1 = RollingHash::from_str("humpbump", Modint::generate_base());

    assert_eq!(rh1.get_LCP(0, 4), 0);
    assert_eq!(rh1.get_LCP(1, 5), 3);

    let rh2 = RollingHash::from_str("strangeorange", Modint::generate_base());

    assert_eq!(rh2.get_LCP(2, 8), 5);
    assert_eq!(rh2.get_LCP(3, 9), 4);
}

#[should_panic]
#[test]
fn test_different_base_chain() {
    let rh1 = RollingHash::from_str("abc", Modint::generate_base());
    let rh2 = RollingHash::from_str("def", Modint::generate_base());
    let _ = rh1.hash(..).chain(&rh2.hash(..));
}

#[should_panic]
#[test]
fn test_different_base_eq() {
    let rh1 = RollingHash::from_str("abc", Modint::generate_base());
    let rh2 = RollingHash::from_str("abc", Modint::generate_base());
    let _ = rh1.hash(..) == rh2.hash(..);
}
