use cp_library_rs::{
    linear_algrebra::dynamic_matrix_exp::*, number_theory::modint::M998, utils::num_traits::Zero,
};
use rand::{prelude::*, rngs::ThreadRng};

/// ランダムな値で埋められたDIMxDIM行列を生成する
fn gen_random_matrix(rng: &mut ThreadRng, D: usize) -> Matrix<M998> {
    let mut res = vec![vec![M998::zero(); D]; D];
    for i in 0..D {
        for j in 0..D {
            res[i][j] = rng.gen::<usize>().into();
        }
    }
    Matrix::new(res)
}

/// ランダムな1bitの値で埋められたDIM次元行列を生成する
fn gen_random_vector<const D: usize>(rng: &mut ThreadRng) -> [M998; D] {
    let mut res = [M998::zero(); D];
    for i in 0..D {
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
    let A = gen_random_matrix(&mut rng, 100);
    let B = gen_random_matrix(&mut rng, 100);

    for _ in 0..10 {
        // ランダムなベクトルを生成
        let v: [M998; 100] = gen_random_vector(&mut rng);

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
    let tetra: Matrix<M998> = Matrix::new(vec![
        vec![0.into(), 1.into(), 0.into(), 0.into()],
        vec![0.into(), 0.into(), 1.into(), 0.into()],
        vec![0.into(), 0.into(), 0.into(), 1.into()],
        vec![1.into(), 1.into(), 1.into(), 1.into()],
    ]);

    // 初期値
    let init = [0, 0, 0, 1].map(M998::new);

    // T_{35}を求める
    let T_35 = tetra.pow(35).apply(&init)[0];

    assert_eq!(T_35, 747044834);
}
