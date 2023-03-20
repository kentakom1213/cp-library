#[allow(dead_code)]
use rand::prelude::*;

#[derive(Debug, Clone)]
struct MatrixExpMod {
    order: usize,  // MODの値
    dim: usize,  // 次元
    arr: Vec<Vec<usize>>,  // 行列
}

impl MatrixExpMod {
    /// ## e
    /// 単位行列を返す
    fn e(order: usize, dim: usize) -> Self {
        let mut arr = vec![vec![0; dim]; dim];
        for i in 0..dim {
            arr[i][i] = 1;
        }
        Self { order, dim, arr }
    }

    pub fn new(order: usize, arr: Vec<Vec<usize>>) -> Self {
        Self { order, dim: arr.len(), arr }
    }

    /// ## apply
    /// ベクトル`x`と行列`A`について、`Ax`を返す
    pub fn apply(&self, vec: &Vec<usize>) -> Vec<usize> {
        let mut res = vec![0; self.dim];
        for i in 0..self.dim {
            for j in 0..self.dim {
                res[i] += self.arr[i][j] * vec[j] % self.order;
                res[i] %= self.order;
            }
        }
        res
    }

    /// ## pow
    /// 行列の累乗を返す（繰り返し2乗法）
    pub fn pow(&self, mut e: usize) -> Self {
        let mut res = Self::e(self.order, self.dim);
        let mut tmp = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                res = tmp.dot(&res);
            }
            tmp = tmp.dot(&tmp);
            e >>= 1;
        }
        res
    }

    /// ## dot
    /// 行列のドット積
    fn dot(&self, other: &Self) -> Self {
        let mut arr = vec![vec![0; self.dim]; self.dim];
        for i in 0..self.dim {
            for j in 0..self.dim {
                for k in 0..self.dim {
                    arr[i][j] += self.arr[i][k] * other.arr[k][j] % self.order;
                    arr[i][j] %= self.order;
                }
            }
        }
        Self { order: self.order, dim: self.dim, arr }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    const REPEAT_TIME: usize = 100;
    const MOD9: usize = 998_244_353;
    const MOD1: usize = 1_000_000_007;

    /// ランダムな値で埋められたDIMxDIM行列を生成する
    fn gen_random_matrix(order: usize, dim: usize, rng: &mut ThreadRng) -> MatrixExpMod {
        let mut arr = vec![vec![0; dim]; dim];
        for i in 0..dim {
            for j in 0..dim {
                arr[i][j] = rng.gen::<usize>() % order;
            }
        }
        MatrixExpMod { order, dim, arr }
    }

    /// ランダムな1bitの値で埋められたDIM次元行列を生成する
    fn gen_random_vector(dim: usize, rng: &mut ThreadRng) -> Vec<usize> {
        let mut res = vec![0; dim];
        for i in 0..dim {
            res[i] = rng.gen::<bool>() as usize;
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
        let A = gen_random_matrix(MOD1, 5, &mut rng);
        let B = gen_random_matrix(MOD1, 5, &mut rng);

        for _ in 0..REPEAT_TIME {
            // ランダムなベクトルを生成
            let v = gen_random_vector(5, &mut rng);

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
        let tetra = vec![
            vec![0, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
            vec![1, 1, 1, 1],
        ];

        let tetra4pow = MatrixExpMod::new(MOD9, tetra);

        // 初期値
        let init = vec![0, 0, 0, 1];
        
        // T_{35}を求める
        let T_35 = tetra4pow.pow(35).apply(&init)[0];

        assert_eq!(T_35, 747044834);
    }
}
