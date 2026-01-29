use std::marker::PhantomData;

/// CartesianTree
///
/// inorder が元配列の順になる（二分木としての中順走査が 0..n-1）
///
/// `build_max` は max-heap（親の値 \(\ge\) 子の値）．
/// `build_min` は min-heap（親の値 \(\le\) 子の値）．
#[derive(Debug, Clone)]
pub struct CartesianTree<T> {
    pub n: usize,
    pub root: usize,
    pub parent: Vec<Option<usize>>,
    pub left: Vec<Option<usize>>,
    pub right: Vec<Option<usize>>,
    _phantom: PhantomData<T>,
}

impl<T: Ord> CartesianTree<T> {
    /// 共通実装．
    ///
    /// `should_pop(top, cur)` が true の間，スタックを pop する．
    pub fn build_by<F>(arr: &[T], mut should_pop: F) -> Self
    where
        F: FnMut(&T, &T) -> bool,
    {
        let n = arr.len();
        assert!(n > 0);

        let mut parent = vec![None; n];
        let mut left = vec![None; n];
        let mut right = vec![None; n];

        let mut st: Vec<usize> = Vec::with_capacity(n);

        for i in 0..n {
            let mut last: Option<usize> = None;

            while let Some(&j) = st.last() {
                if !should_pop(&arr[j], &arr[i]) {
                    break;
                }
                last = Some(st.pop().unwrap());
            }

            // 最後に pop された `last` が `i` の左部分木（左の子の根）
            if let Some(l) = last {
                parent[l] = Some(i);
                left[i] = Some(l);
            }

            // 残ったスタック top の右の子が `i`
            if let Some(&top) = st.last() {
                parent[i] = Some(top);
                right[top] = Some(i);
            }

            st.push(i);
        }

        let root = (0..n).find(|&i| parent[i].is_none()).unwrap();

        Self {
            n,
            root,
            parent,
            left,
            right,
            _phantom: PhantomData,
        }
    }

    /// max-heap の Cartesian Tree を構築する（同値は右側優先）．
    ///
    /// すなわち，区間の最大値（同値なら最右）を根にして再帰分割した木になる．
    pub fn build_max(arr: &[T]) -> Self {
        // stack top `top` が `cur` より「優先度が低い」なら pop する
        // max-heap かつ同値は右優先なので，`top <= cur` の間 pop
        Self::build_by(arr, |top, cur| top <= cur)
    }

    /// min-heap の Cartesian Tree を構築する（同値は右側優先）．
    ///
    /// すなわち，区間の最小値（同値なら最右）を根にして再帰分割した木になる．
    pub fn build_min(arr: &[T]) -> Self {
        // min-heap かつ同値は右優先なので，`top >= cur` の間 pop
        Self::build_by(arr, |top, cur| top >= cur)
    }
}
