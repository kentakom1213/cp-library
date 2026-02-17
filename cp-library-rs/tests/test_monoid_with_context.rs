use cp_library_rs::algebraic_structure::k_best::KBest;
use cp_library_rs::algebraic_structure::monoid_with_context::MonoidCtx;

// 便利: Some(x) を詰めて残りを None にする
fn pack<const K: usize, T: Copy>(xs: &[T]) -> [Option<T>; K] {
    std::array::from_fn(|i| xs.get(i).copied().map(Some).unwrap_or(None))
}

#[test]
fn kbest_e_is_all_none() {
    const K: usize = 3;
    let m = KBest::<K, i32>::new(|a, b| a >= b);
    assert_eq!(m.e(), [None, None, None]);
}

#[test]
fn kbest_basic_topk() {
    const K: usize = 3;
    let m = KBest::<K, i32>::new(|a, b| a >= b);

    let a = pack::<K, _>(&[5, 1, -2]); // already sorted desc
    let b = pack::<K, _>(&[4, 4, 3]); // already sorted desc

    // 上位3個は 5,4,4
    let got = m.op(&a, &b);
    assert_eq!(got, [Some(5), Some(4), Some(4)]);
}

#[test]
fn kbest_identity_left_right() {
    const K: usize = 4;
    let m = KBest::<K, i32>::new(|a, b| a >= b);

    let x = pack::<K, _>(&[10, 7, 7, 1]);

    assert_eq!(m.op(&m.e(), &x), x);
    assert_eq!(m.op(&x, &m.e()), x);
}

#[test]
fn kbest_associative_when_ge_is_transitive() {
    const K: usize = 2;
    let m = KBest::<K, i32>::new(|a, b| a >= b);

    let a = pack::<K, _>(&[3, 0]);
    let b = pack::<K, _>(&[2, 1]);
    let c = pack::<K, _>(&[5, 4]);

    let left = m.op(&m.op(&a, &b), &c);
    let right = m.op(&a, &m.op(&b, &c));
    assert_eq!(left, right);
}
