//! セグ木のノード

#![allow(non_snake_case)]
#![allow(clippy::type_complexity)]

use crate::algebraic_structure::monoid::Monoid;

use std::{
    cmp::Ordering,
    fmt::Debug,
    mem,
    ops::Bound::{self, *},
};

/// AA木のノード
pub type Node<K, M> = Option<Box<NodeInner<K, M>>>;

pub struct NodeInner<K: Ord, M: Monoid> {
    /// キー
    pub key: K,
    /// ノードが持つ値
    pub value: M::Val,
    /// 部分木を集約した値
    pub sum: M::Val,
    /// ノードの高さ
    pub level: usize,
    pub left: Node<K, M>,
    pub right: Node<K, M>,
}

impl<K: Ord, M: Monoid> NodeInner<K, M> {
    /// ノードの作成
    pub fn new(key: K, value: M::Val) -> Node<K, M> {
        Some(Box::new(NodeInner {
            key,
            value: value.clone(),
            sum: value,
            level: 1,
            left: None,
            right: None,
        }))
    }

    /// ノードの値を再計算する
    fn eval(&mut self) {
        // ノードの値を再計算
        self.sum = match (&self.left, &self.right) {
            (Some(l), Some(r)) => M::op(&M::op(&l.sum, &self.value), &r.sum),
            (Some(l), _) => M::op(&l.sum, &self.value),
            (_, Some(r)) => M::op(&self.value, &r.sum),
            _ => self.value.clone(),
        };
    }
}

impl<K, M> Debug for NodeInner<K, M>
where
    K: Ord + Debug,
    M: Monoid,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("sum", &self.sum)
            .finish()
    }
}

/// skew操作
/// ```text
///   |        ⇓           ⇓        
/// 2 |    L ← T           L → T    
///   |   ↙ ↘   ↘   ==>   ↙   ↙ ↘   
/// 1 |  A   B   R       A   B   R  
/// ```
fn skew<K: Ord, M: Monoid>(node: Node<K, M>) -> Node<K, M> {
    let mut T = node?;
    if T.left.is_none() {
        Some(T)
    } else if T.level == T.left.as_ref().unwrap().level {
        let mut L = T.left.unwrap();
        // Tを更新
        T.left = L.right;
        T.eval();
        // Lを更新
        L.right = Some(T);
        L.eval();
        Some(L)
    } else {
        Some(T)
    }
}

/// split操作
/// ```text
///   |                         ⇓    
/// 3 |                         R    
///   |    ⇓                   ↙ ↘   
/// 2 |    T → R → X   ==>    T   X  
///   |   ↙   ↙              ↙ ↘     
/// 1 |  A   B              A   B    
/// ```
fn split<K: Ord, M: Monoid>(node: Node<K, M>) -> Node<K, M> {
    let mut T = node?;
    if T.right.is_none() || T.right.as_ref().unwrap().right.is_none() {
        Some(T)
    } else if T.level == T.right.as_ref().unwrap().right.as_ref().unwrap().level {
        let mut R = T.right.unwrap();
        // Tを更新
        T.right = R.left;
        T.eval();
        // Rを更新
        R.left = Some(T);
        R.eval();
        R.level += 1; // Rのレベルを1上げる
        Some(R)
    } else {
        Some(T)
    }
}

/// 値 `key` を持つノードの不変参照を取得する
pub fn get<'a, K: Ord, M: Monoid>(root: &'a Node<K, M>, key: &K) -> Option<&'a NodeInner<K, M>> {
    let Some(T) = root else {
        return None;
    };
    match key.cmp(&T.key) {
        Ordering::Less => get(&T.left, key),
        Ordering::Greater => get(&T.right, key),
        Ordering::Equal => Some(T),
    }
}

/// 区間
type Segment<K> = (Bound<K>, Bound<K>);

/// 区間 `x` と `y` が共通部分を持たないか判定
fn has_no_intersection<K: Ord>((l, r): Segment<&K>, (begin, end): Segment<&K>) -> bool {
    (match (r, begin) {
        (Included(r), Included(b)) => r < b,
        (Included(r), Excluded(b)) => r <= b,
        (Excluded(r), Included(b)) => r <= b,
        (Excluded(r), Excluded(b)) => r <= b,
        _ => false,
    } || match (end, l) {
        (Included(e), Included(l)) => e < l,
        (Included(e), Excluded(l)) => e <= l,
        (Excluded(e), Included(l)) => e <= l,
        (Excluded(e), Excluded(l)) => e <= l,
        _ => false,
    })
}

/// 区間 `x`（引数1） が区間 `y`（引数2） を包含するか
fn includes<K: Ord>((l, r): Segment<&K>, (begin, end): Segment<&K>) -> bool {
    (match (l, begin) {
        (Unbounded, _) => true,
        (_, Unbounded) => false,
        (Included(l), Included(b)) => l <= b,
        (Included(l), Excluded(b)) => l <= b,
        (Excluded(l), Included(b)) => l < b,
        (Excluded(l), Excluded(b)) => l <= b,
    } && match (end, r) {
        (_, Unbounded) => true,
        (Unbounded, _) => false,
        (Included(e), Included(r)) => e <= r,
        (Included(e), Excluded(r)) => e < r,
        (Excluded(e), Included(r)) => e <= r,
        (Excluded(e), Excluded(r)) => e <= r,
    })
}

/// 区間 `[l,r)` 中のノードの値を集約する
pub fn get_range<K: Ord, M: Monoid>(
    root: &Node<K, M>,
    l: Bound<&K>,
    r: Bound<&K>,
    begin: Bound<&K>,
    end: Bound<&K>,
) -> M::Val {
    let Some(T) = root else {
        return M::e();
    };
    // 区間を含まない
    if has_no_intersection((l, r), (begin, end)) {
        M::e()
    }
    // 区間を包含する
    else if includes((l, r), (begin, end)) {
        T.sum.clone()
    }
    // 区間が一部重なる
    else {
        let mid = &T.key;
        let l_val = get_range(&T.left, l, r, begin, Excluded(mid));
        let m_val = if includes((l, r), (Included(mid), Included(mid))) {
            T.value.clone()
        } else {
            M::e()
        };
        let r_val = get_range(&T.right, l, r, Excluded(mid), end);
        M::op(&M::op(&l_val, &m_val), &r_val)
    }
}

/// 値 `key` に `value` を挿入する
/// - 値がすでに存在する場合には更新し，もとの値を返す
pub fn insert<K: Ord, M: Monoid>(
    root: Node<K, M>,
    key: K,
    value: M::Val,
) -> (Node<K, M>, Option<(K, M::Val)>) {
    let Some(mut T) = root else {
        return (NodeInner::new(key, value), None);
    };
    // 挿入
    let old_key_value = match key.cmp(&T.key) {
        Ordering::Less => {
            let (new_left, old_key_value) = insert(T.left, key, value);
            T.left = new_left;
            old_key_value
        }
        Ordering::Greater => {
            let (new_right, old_key_value) = insert(T.right, key, value);
            T.right = new_right;
            old_key_value
        }
        Ordering::Equal => Some((
            mem::replace(&mut T.key, key),
            mem::replace(&mut T.value, value),
        )),
    };
    // ノードの評価
    T.eval();
    // 再平衡化
    let mut root = Some(T);
    root = skew(root);
    root = split(root);
    (root, old_key_value)
}

/// 値 `key` をもつノードを削除し，削除されたノードを返す
/// - `root`：削除する木の根
pub fn delete<K: Ord, M: Monoid>(root: Node<K, M>, key: &K) -> (Node<K, M>, Option<(K, M::Val)>) {
    let Some(mut T) = root else {
        return (None, None);
    };
    let (mut new_root, old_key_value) = match key.cmp(&T.key) {
        Ordering::Less => {
            let (new_left, old_key_value) = delete(T.left, key);
            T.left = new_left;
            (Some(T), old_key_value)
        }
        Ordering::Greater => {
            let (new_right, old_key_value) = delete(T.right, key);
            T.right = new_right;
            (Some(T), old_key_value)
        }
        Ordering::Equal => {
            if T.left.is_none() {
                (T.right, Some((T.key, T.value)))
            } else if T.right.is_none() {
                (T.left, Some((T.key, T.value)))
            } else {
                // 左右の子を持つ場合，左の子の最大値を現在のノードに代入
                let (new_left, right_most) = delete_and_get_max(T.left.take());
                if let Some(L) = new_left {
                    T.left.replace(L);
                }
                let Some(right_most) = right_most else {
                    unreachable!("T.left is not None");
                };
                let old_key_value = (
                    mem::replace(&mut T.key, right_most.key),
                    mem::replace(&mut T.value, right_most.value),
                );
                (Some(T), Some(old_key_value))
            }
        }
    };
    // 評価
    if let Some(T) = &mut new_root {
        T.eval();
    }
    // バランスの修正
    let rebalanced = rebarance(new_root);
    (rebalanced, old_key_value)
}

/// 削除後の頂点を再平衡化
fn rebarance<K: Ord, M: Monoid>(root: Node<K, M>) -> Node<K, M> {
    let mut T = root?;
    let left_level = T.left.as_ref().map_or(0, |node| node.level);
    let right_level = T.right.as_ref().map_or(0, |node| node.level);
    if left_level.min(right_level) < T.level - 1 {
        T.level -= 1;
        // 右が大きい場合，下げる
        if right_level > T.level {
            T.right.as_mut().unwrap().level = T.level;
        }
        // 同じレベルのノードをskew
        T = skew(Some(T)).unwrap();
        T.right = skew(T.right);
        if let Some(mut right) = T.right.take() {
            right.right = skew(right.right);
            T.right.replace(right);
        }
        // 同じレベルのノードをsplit
        T = split(Some(T)).unwrap();
        T.right = split(T.right);
        // ノードの再評価
        T.eval();
    }
    Some(T)
}

/// nodeを根とする木のうち，値が最大のものを削除する
/// - 戻り値：(新しい根, 削除されたノード)
fn delete_and_get_max<K: Ord, M: Monoid>(
    root: Node<K, M>,
) -> (Node<K, M>, Option<NodeInner<K, M>>) {
    let Some(mut T) = root else {
        return (None, None);
    };
    // 右の子の取り出し
    let (new_right, right_most) = delete_and_get_max(T.right.take());
    let Some(right_most) = right_most else {
        return (None, Some(*T));
    };
    if let Some(R) = new_right {
        T.right.replace(R);
    }
    // ノードを再評価
    T.eval();
    let mut new_root = Some(T);
    // 削除したので，再平衡化
    new_root = rebarance(new_root);
    (new_root, Some(right_most))
}
