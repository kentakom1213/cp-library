use cp_library_rs::string::rolling_hash::*;

#[test]
fn test_pattern_match() {
    let base = 20021213;

    let target = RollingHash::from_str("momomosumomomomomonouchi", base);
    let ptn1 = RollingHash::from_str("sumomo", base);
    let ptn2 = RollingHash::from_str("momo", base);
    let (tlen, p1len, p2len) = (24, 6, 4);

    // "sumomo"を検索
    let mut res1 = vec![];
    for i in 0..tlen - p1len {
        if target.get(i, i + p1len) == ptn1.full() {
            res1.push(i);
        }
    }

    assert_eq!(res1, vec![6]);

    // "momo"を検索
    let mut res2 = vec![];
    for i in 0..tlen - p2len {
        if target.get(i, i + p2len) == ptn2.full() {
            res2.push(i);
        }
    }

    assert_eq!(res2, vec![0, 2, 8, 10, 12, 14]);
}

#[test]
fn test_concat() {
    const BASE: usize = 998244353;

    let a = "abc";
    let b = "str";
    let c = "abcstr";

    let hash_a = RollingHash::from_str(a, BASE);
    let hash_b = RollingHash::from_str(b, BASE);
    let hash_c = RollingHash::from_str(c, BASE);

    assert_eq!(
        hash_a.concat(hash_a.full(), hash_b.full(), 3),
        hash_c.full()
    );
    assert_ne!(
        hash_a.concat(hash_b.full(), hash_a.full(), 3),
        hash_c.full()
    );
    assert_eq!(
        hash_a.concat(hash_a.get(0, 3), hash_a.get(3, 3), 0),
        hash_a.full()
    );
    assert_eq!(
        hash_a.concat(hash_a.full(), hash_a.get(0, 0), 0),
        hash_a.full()
    );
    assert_eq!(
        hash_a.concat(hash_a.get(0, 0), hash_a.full(), a.len()),
        hash_a.full()
    );
}

#[test]
fn test_LCP() {
    let rh1 = RollingHash::from_str(&"humpbump", 2023);

    assert_eq!(rh1.getLCP(0, 4), 0);
    assert_eq!(rh1.getLCP(1, 5), 3);

    let rh2 = RollingHash::from_str(&"strangeorange", 19);

    assert_eq!(rh2.getLCP(2, 8), 5);
    assert_eq!(rh2.getLCP(3, 9), 4);
}
