#![allow(non_snake_case)]

use cp_library_rs::number_theory::crt::garner_algorithm;
use itertools::Itertools;
use rand::{rng, Rng};

const P: [u64; 9] = [4, 5, 7, 9, 11, 13, 17, 19, 23];
const M998: u64 = 998244353;
const M107: u64 = 1000000007;

#[test]
fn test_crt_small() {
    let mut rng = rng();

    let prod: u64 = P.iter().product();

    for _ in 0..5_000 {
        let x = rng.random_range(0..prod);

        let rems = P.iter().map(|&p| x % p).collect_vec();
        let res = garner_algorithm(&rems, &P);

        assert_eq!(res, x);
    }
}

#[test]
fn test_crt_large() {
    let mut rng = rng();

    let mods = [M107, M998];
    let prod = M107 * M998;

    for _ in 0..5_000 {
        let x = rng.random_range(0..prod);

        let rems = mods.iter().map(|&p| x % p).collect_vec();
        let res = garner_algorithm(&rems, &mods);

        assert_eq!(res, x);
    }
}
