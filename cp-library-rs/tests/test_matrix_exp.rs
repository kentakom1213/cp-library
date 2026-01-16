#![allow(non_snake_case)]
#![allow(clippy::needless_range_loop)]

use cp_library_rs::{linear_algrebra::matrix_exp::*, number_theory::modint::M998};
use rand::{prelude::*, rngs::ThreadRng};

use num_traits::Zero;

const REPEAT_TIME: usize = 100;

/// ランダムな値で埋められたDIMxDIM行列を生成する
fn gen_random_matrix<const D: usize>(rng: &mut ThreadRng) -> Matrix<D, M998> {
    let mut res = [[M998::zero(); D]; D];
    for i in 0..D {
        for j in 0..D {
            res[i][j] = (rng.random::<u64>() as usize).into();
        }
    }
    Matrix(res)
}

/// ランダムな1bitの値で埋められたDIM次元行列を生成する
fn gen_random_vector<const D: usize>(rng: &mut ThreadRng) -> [M998; D] {
    let mut res = [M998::zero(); D];
    for i in 0..D {
        res[i] = (rng.random::<bool>() as usize).into();
    }
    res
}

/// ## test_dot
/// 行列積の正当性の検証
/// - [乱択アルゴリズム紹介(行列乗算の検査&多項式等価性の検査)](https://tech.preferred.jp/ja/blog/matrix-multiplication-and-polynomial-identity/)
#[test]
fn test_dot() {
    let mut rng = rand::rng();

    // ランダムな行列を生成
    let A = gen_random_matrix(&mut rng);
    let B = gen_random_matrix(&mut rng);

    for _ in 0..REPEAT_TIME {
        // ランダムなベクトルを生成
        let v: [M998; 4] = gen_random_vector(&mut rng);

        // left = (A @ B) @ v
        let left = A.dot(&B).apply(&v);
        // right = A @ (B @ v)
        let right = A.apply(&B.apply(&v));

        assert_eq!(left, right);
    }
}

/// ## test_pow
/// 行列累乗の正当性の検証
#[test]
fn test_pow() {
    // テトラナッチ数
    let tetra: Matrix<4, M998> = Matrix([
        [0, 1, 0, 0].map(M998::new),
        [0, 0, 1, 0].map(M998::new),
        [0, 0, 0, 1].map(M998::new),
        [1, 1, 1, 1].map(M998::new),
    ]);

    // 初期値
    let init = [0, 0, 0, 1].map(M998::new);

    // T_{35}を求める
    let T_35 = tetra.pow(35).apply(&init)[0];

    assert_eq!(T_35, 747044834);
}
