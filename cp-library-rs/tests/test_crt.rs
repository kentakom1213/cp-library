use cp_library_rs::crt::garner_algorithm;
use itertools::Itertools;
use rand::{thread_rng, Rng};

const P: [usize; 9] = [4, 5, 7, 9, 11, 13, 17, 19, 23];
const M998: usize = 998244353;
const M107: usize = 1000000007;

#[test]
fn test_crt_small() {
    let mut rng = thread_rng();

    let prod: usize = P.iter().product();

    for _ in 0..5_000 {
        let x = rng.gen_range(0..prod);

        let rems = P.iter().map(|&p| x % p).collect_vec();
        let res = garner_algorithm(&rems, &P);

        assert_eq!(res, x);
    }
}

#[test]
fn test_crt_large() {
    let mut rng = thread_rng();

    let mods = [M107, M998];
    let prod = M107 * M998;

    for _ in 0..5_000 {
        let x = rng.gen_range(0..prod);

        let rems = mods.iter().map(|&p| x % p).collect_vec();
        let res = garner_algorithm(&rems, &mods);

        assert_eq!(res, x);
    }
}
