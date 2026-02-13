#![allow(non_snake_case)]

use cp_library_rs::utils::grid::*;
use std::collections::VecDeque;

const INF: usize = usize::MAX;

#[test]
fn directions_4_inner_i32() {
    let p = (2_i32, 3_i32);
    assert_eq!(p.right(), (2, 4));
    assert_eq!(p.up(), (1, 3));
    assert_eq!(p.left(), (2, 2));
    assert_eq!(p.down(), (3, 3));
}

#[test]
fn directions_8_inner_i32() {
    let p = (2_i32, 3_i32);
    assert_eq!(p.upright(), (1, 4));
    assert_eq!(p.upleft(), (1, 2));
    assert_eq!(p.downleft(), (3, 2));
    assert_eq!(p.downright(), (3, 4));
}

#[test]
fn get_adj_4_order_and_filter_inner() {
    // 0..5 x 0..7 の内部点なので 4 方向全部入る
    let p = (2_i32, 3_i32);
    let adj = p.get_adj_4((0, 5), (0, 7));
    // order: right, up, left, down
    assert_eq!(adj, vec![(2, 4), (1, 3), (2, 2), (3, 3)]);
}

#[test]
fn get_adj_8_order_and_filter_inner() {
    // 0..5 x 0..7 の内部点なので 8 方向全部入る
    let p = (2_i32, 3_i32);
    let adj = p.get_adj_8((0, 5), (0, 7));
    // order:
    // right, upright, up, upleft, left, downleft, down, downright
    assert_eq!(
        adj,
        vec![
            (2, 4),
            (1, 4),
            (1, 3),
            (1, 2),
            (2, 2),
            (3, 2),
            (3, 3),
            (3, 4)
        ]
    );
}

#[test]
fn get_adj_4_corner_filters_out_of_range() {
    // 左上隅 (0,0)
    // right と down だけ残るはず（up, left は範囲外）
    let p = (0_i32, 0_i32);
    let adj = p.get_adj_4((0, 3), (0, 3));
    assert_eq!(adj, vec![(0, 1), (1, 0)]);
}

#[test]
fn get_adj_8_corner_filters_out_of_range_and_keeps_order_among_survivors() {
    // 左上隅 (0,0)
    // 8 方向のうち有効なのは
    // right(0,1), down(1,0), downright(1,1) のみ
    // ただし順序は元配列順を保ったままフィルタされる
    let p = (0_i32, 0_i32);
    let adj = p.get_adj_8((0, 3), (0, 3));
    assert_eq!(adj, vec![(0, 1), (1, 0), (1, 1)]);
}

#[test]
fn get_adj_4_edge_filters() {
    // 上辺の中央 (0,1)
    // right, left, down が残る（up は範囲外）
    let p = (0_i32, 1_i32);
    let adj = p.get_adj_4((0, 3), (0, 3));
    assert_eq!(adj, vec![(0, 2), (0, 0), (1, 1)]);
}

#[test]
fn works_with_usize_ranges() {
    // usize でも，範囲外は filter で落ちることを確認
    let p = (1_usize, 1_usize);
    let adj4 = p.get_adj_4((0, 3), (0, 3));
    assert_eq!(adj4, vec![(1, 2), (0, 1), (1, 0), (2, 1)]);

    let q = (0_usize, 0_usize);
    let adj8 = q.get_adj_8((0, 2), (0, 2));
    assert_eq!(adj8, vec![(0, 1), (1, 0), (1, 1)]);
}

#[test]
fn test_grid_4() {
    let (H, W) = (5, 6);
    let S = "
        .#...#
        ..##..
        .#..##
        ...##.
        ##....
        ";

    let grid: Vec<Vec<char>> = S
        .split_whitespace()
        .map(|row| row.chars().collect())
        .collect();

    // BFS
    let dist = {
        let mut arr = vec![vec![INF; W]; H];
        arr[0][0] = 0;
        let mut deq = VecDeque::new();
        deq.push_back((0, 0));

        while let Some((r, c)) = deq.pop_front() {
            for (nr, nc) in (r, c).get_adj_4((0, H), (0, W)) {
                if grid[nr][nc] == '.' && arr[nr][nc] == INF {
                    arr[nr][nc] = arr[r][c] + 1;
                    deq.push_back((nr, nc));
                }
            }
        }

        arr
    };

    assert_eq!(
        &dist,
        &vec![
            vec![0, INF, INF, INF, INF, INF],
            vec![1, 2, INF, INF, INF, INF],
            vec![2, INF, 6, 7, INF, INF],
            vec![3, 4, 5, INF, INF, 10],
            vec![INF, INF, 6, 7, 8, 9],
        ]
    );
}

#[test]
fn test_grid_8() {
    let (H, W) = (5, 6);
    let S = "
        .#...#
        ..##..
        .#..##
        ...##.
        ##....
        ";

    let grid: Vec<Vec<char>> = S
        .split_whitespace()
        .map(|row| row.chars().collect())
        .collect();

    // BFS
    let dist = {
        let mut arr = vec![vec![INF; W]; H];
        arr[0][0] = 0;
        let mut deq = VecDeque::new();
        deq.push_back((0, 0));

        while let Some((r, c)) = deq.pop_front() {
            for (nr, nc) in (r, c).get_adj_8((0, H), (0, W)) {
                if grid[nr][nc] == '.' && arr[nr][nc] == INF {
                    arr[nr][nc] = arr[r][c] + 1;
                    deq.push_back((nr, nc));
                }
            }
        }

        arr
    };

    assert_eq!(
        &dist,
        &vec![
            vec![0, INF, 2, 3, 4, INF],
            vec![1, 1, INF, INF, 4, 5],
            vec![2, INF, 2, 3, INF, INF],
            vec![3, 3, 3, INF, INF, 6],
            vec![INF, INF, 4, 4, 5, 6],
        ]
    );
}
