//! 木の重心を求める

// 部分木のサイズを求める
pub fn subtree_size(p: usize, u: usize, G: &Vec<Vec<usize>>, W: &[usize], res: &mut [usize]) {
    // 自分を足す
    res[u] += W[u];
    // 集約
    for &v in &G[u] {
        if p == v {
            continue;
        }
        subtree_size(u, v, G, W, res);
        res[u] += res[v];
    }
}

/// 重心を求める
pub fn get_centroid(p: usize, u: usize, G: &Vec<Vec<usize>>, S: &[usize], root: usize) -> usize {
    let mut is_centroid = true;
    // 隣接頂点を調べる
    for &v in &G[u] {
        if v == p {
            continue;
        }
        let res = get_centroid(u, v, G, S, root);
        if res < usize::MAX {
            return res;
        }

        if S[v] > S[root] / 2 {
            is_centroid = false;
        }
    }
    if (S[root] - S[u]) > S[root] / 2 {
        is_centroid = false;
    }

    if is_centroid {
        u
    } else {
        usize::MAX
    }
}
