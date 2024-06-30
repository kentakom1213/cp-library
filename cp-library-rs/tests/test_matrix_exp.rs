use cp_library_rs::{matrix_exp::*, modint::M998};
use num_traits::Zero;
use rand::{prelude::*, rngs::ThreadRng};

const REPEAT_TIME: usize = 100;

/// ランダムな値で埋められたDIMxDIM行列を生成する
fn gen_random_matrix(rng: &mut ThreadRng) -> [[M998; DIM]; DIM] {
    let mut res = [[M998::zero(); DIM]; DIM];
    for i in 0..DIM {
        for j in 0..DIM {
            res[i][j] = rng.gen::<usize>().into();
        }
    }
    res
}

/// ランダムな1bitの値で埋められたDIM次元行列を生成する
fn gen_random_vector(rng: &mut ThreadRng) -> [M998; DIM] {
    let mut res = [M998::zero(); DIM];
    for i in 0..DIM {
        res[i] = (rng.gen::<bool>() as usize).into();
    }
    res
}

/// ## test_dot
/// 行列積の正当性の検証
/// - [乱択アルゴリズム紹介(行列乗算の検査&多項式等価性の検査)](https://tech.preferred.jp/ja/blog/matrix-multiplication-and-polynomial-identity/)
#[test]
fn test_dot() {
    let mut rng = rand::thread_rng();

    // ランダムな行列を生成
    let A = gen_random_matrix(&mut rng);
    let B = gen_random_matrix(&mut rng);

    for _ in 0..REPEAT_TIME {
        // ランダムなベクトルを生成
        let v = gen_random_vector(&mut rng);

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
    let tetra: [[M998; 4]; 4] = [
        [0, 1, 0, 0].map(M998::new),
        [0, 0, 1, 0].map(M998::new),
        [0, 0, 0, 1].map(M998::new),
        [1, 1, 1, 1].map(M998::new),
    ];

    // 初期値
    let init = [0, 0, 0, 1].map(M998::new);

    // T_{35}を求める
    let T_35 = tetra.pow(35).apply(&init)[0];

    assert_eq!(T_35, 747044834);
}
