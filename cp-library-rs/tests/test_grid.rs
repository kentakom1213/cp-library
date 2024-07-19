use cp_library_rs::utils::grid::*;
use std::collections::VecDeque;

const INF: usize = usize::MAX;

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
            for (nr, nc) in (r, c).get_adj_4(H, W) {
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
            for (nr, nc) in (r, c).get_adj_8(H, W) {
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
