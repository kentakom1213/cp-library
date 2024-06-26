use std::mem;

use cp_library_rs::ford_fulkerson::FordFulkerson;
use itertools::Itertools;

fn fill_grid(S: &Vec<&str>) -> usize {
    let N = S.len();
    let M = S[0].len();
    let S = S.iter().map(|s| s.chars().collect_vec()).collect_vec();

    // A: (i+j)%2==0 であるようなマスの集合
    // B: (i+j)%2==1 であるようなマスの集合
    let mut ff = FordFulkerson::new(N * M + 2);

    let s = N * M;
    let t = s + 1;

    let idx = |r: usize, c: usize| -> usize { r * M + c };

    let mut edges = vec![];

    for i in 0..N {
        for j in 0..M {
            // 左右
            if j + 1 < M {
                let l = S[i][j];
                let r = S[i][j + 1];
                let mut a = idx(i, j);
                let mut b = idx(i, j + 1);
                if (i + j) % 2 == 1 {
                    mem::swap(&mut a, &mut b);
                }
                if l == '.' && r == '.' {
                    ff.add_edge(a, b, 1);
                    edges.push((a, b));
                }
            }
            // 上下
            if i + 1 < N {
                let t = S[i][j];
                let d = S[i + 1][j];
                let mut a = idx(i, j);
                let mut b = idx(i + 1, j);
                if (i + j) % 2 == 1 {
                    mem::swap(&mut a, &mut b);
                }
                if t == '.' && d == '.' {
                    ff.add_edge(a, b, 1);
                    edges.push((a, b));
                }
            }
        }
    }

    for i in 0..N {
        for j in 0..M {
            if (i + j) % 2 == 0 {
                ff.add_edge(s, idx(i, j), 1);
            } else {
                ff.add_edge(idx(i, j), t, 1);
            }
        }
    }

    // 最大流を求める
    let mf = ff.max_flow(s, t);

    // グリッドを求める
    let mut res = S;

    for &(mut a, mut b) in &edges {
        if ff.get_flow(a, b) == 1 {
            if a > b {
                mem::swap(&mut a, &mut b);
            }
            let (ai, aj) = (a / M, a % M);
            let (bi, bj) = (b / M, b % M);

            if aj + 1 == bj {
                res[ai][aj] = '>';
                res[bi][bj] = '<';
            } else {
                res[ai][aj] = 'v';
                res[bi][bj] = '^';
            }
        }
    }

    eprintln!("{}", res.iter().map(|s| s.iter().join("")).join("\n"));

    mf
}

#[test]
fn test_fill_grid() {
    let g1 = vec!["....", "....", "...."];
    assert_eq!(fill_grid(&g1), 6);

    let g2 = vec![".#.", "#.#", ".#."];
    assert_eq!(fill_grid(&g2), 0);

    let g3 = vec![".#...#", "..#...", "..#..."];
    assert_eq!(fill_grid(&g3), 6);

    let g4 = vec!["...#........"];
    assert_eq!(fill_grid(&g4), 5);
}
