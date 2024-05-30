use cp_library_rs::enum_pairs::*;

#[test]
fn test_enum_pairs() {
    let mut res = vec![];
    dfs((0..6).collect(), vec![], &mut res);

    assert_eq!(
        res,
        vec![
            vec![(0, 1), (2, 3), (4, 5)],
            vec![(0, 1), (2, 4), (3, 5)],
            vec![(0, 1), (2, 5), (3, 4)],
            vec![(0, 2), (1, 3), (4, 5)],
            vec![(0, 2), (1, 4), (3, 5)],
            vec![(0, 2), (1, 5), (3, 4)],
            vec![(0, 3), (1, 2), (4, 5)],
            vec![(0, 3), (1, 4), (2, 5)],
            vec![(0, 3), (1, 5), (2, 4)],
            vec![(0, 4), (1, 2), (3, 5)],
            vec![(0, 4), (1, 3), (2, 5)],
            vec![(0, 4), (1, 5), (2, 3)],
            vec![(0, 5), (1, 2), (3, 4)],
            vec![(0, 5), (1, 3), (2, 4)],
            vec![(0, 5), (1, 4), (2, 3)],
        ]
    );
}

fn dfs(rem: Vec<usize>, pairs: Vec<(usize, usize)>, res: &mut Vec<Vec<(usize, usize)>>) {
    if rem.len() < 2 {
        res.push(pairs.clone());
        return;
    }
    for i in 1..rem.len() {
        let mut new_rem = rem.clone();
        let snd = new_rem.remove(i);
        let fst = new_rem.remove(0);
        let mut new_pairs = pairs.clone();
        new_pairs.push((fst, snd));
        // 再帰呼出し
        dfs(new_rem, new_pairs, res);
    }
}

#[test]
fn test_iter_pairs_4() {
    let mut pairs4 = vec![];
    dfs((0..4).collect(), vec![], &mut pairs4);

    assert_eq!(pairs_usize(4).collect::<Vec<_>>(), pairs4);
}

#[test]
fn test_iter_pairs_12() {
    let mut pairs12 = vec![];
    dfs((0..12).collect(), vec![], &mut pairs12);

    assert_eq!(pairs_usize(12).collect::<Vec<_>>(), pairs12);
}
