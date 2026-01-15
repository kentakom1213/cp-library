#![allow(non_snake_case)]
#![allow(clippy::needless_range_loop)]

use cp_library_rs::{data_structure::acc3d::acc3D, debug2D};
use itertools::iproduct;
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[test]
fn test_acc3d_simple_1() {
    let arr = vec![vec![vec![1, 2], vec![3, 4]], vec![vec![5, 6], vec![7, 8]]];

    let sum = acc3D(&arr);

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                assert_eq!(sum(i, i + 1, j, j + 1, k, k + 1), arr[i][j][k]);
            }
        }
    }

    assert_eq!(sum(0, 2, 1, 2, 0, 1), 10);
    assert_eq!(sum(1, 2, 0, 2, 0, 2), 26);
}

#[test]
fn test_ramdom() {
    const N_MAX: usize = 100;
    const X_MAX: isize = 1000;
    const Q: usize = 1000;

    let mut rng = thread_rng();

    let X = rng.gen_range(1..=N_MAX);
    let Y = rng.gen_range(1..=N_MAX);
    let Z = rng.gen_range(1..=N_MAX);

    let mut arr = vec![vec![vec![0; Z]; Y]; X];

    for i in 0..X {
        for j in 0..Y {
            for k in 0..Z {
                arr[i][j][k] = rng.gen_range(-X_MAX..=X_MAX);
            }
        }
    }

    debug2D!(arr);

    // 累積和
    let sum = acc3D(&arr);

    let rand_range = |rng: &mut ThreadRng, max: usize| {
        let a = rng.gen_range(0..=max);
        let b = rng.gen_range(0..=max);
        if a > b {
            (b, a)
        } else {
            (a, b)
        }
    };

    for _ in 0..Q {
        let (lx, rx) = rand_range(&mut rng, X);
        let (ly, ry) = rand_range(&mut rng, Y);
        let (lz, rz) = rand_range(&mut rng, Z);

        let res = sum(lx, rx, ly, ry, lz, rz);

        // 愚直
        let ans = iproduct!(lx..rx, ly..ry, lz..rz)
            .map(|(i, j, k)| arr[i][j][k])
            .sum::<isize>();

        assert_eq!(res, ans);
    }
}
