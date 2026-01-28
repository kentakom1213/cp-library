//! AA木による動的セグ木
//! - 遅延評価なし

use crate::algebraic_structure::monoid::Monoid;

use std::{
    fmt::{self, Debug},
    ops::{Bound::Unbounded, Deref, DerefMut, RangeBounds},
};

use crate::tree::aa_tree::{delete, get, get_range, insert, Node, NodeInner};

/// 平衡2分木
/// - 1点更新，区間取得
/// - 遅延評価なし
pub struct BalancedBinaryTree<K: Ord, M: Monoid> {
    pub root: Node<K, M>,
    size: usize,
    /// getメソッドで返すための一時的な単位元
    tmp_e: M::Val,
}

impl<K: Ord, M: Monoid> BalancedBinaryTree<K, M> {
    /// 1点取得（不変参照）
    /// - 値 `key` を持つノードの不変参照を取得する
    pub fn get(&self, key: &K) -> &M::Val {
        if let Some(NodeInner { value, .. }) = get(&self.root, key) {
            value
        } else {
            &self.tmp_e
        }
    }

    /// 1点取得（可変参照）
    /// - 値 `key` を持つノードの可変参照を取得する
    pub fn get_mut(&mut self, key: K) -> NodeEntry<'_, K, M> {
        let (new_root, old_key_val) = delete(self.root.take(), &key);
        self.root = new_root;

        if let Some((key, value)) = old_key_val {
            NodeEntry {
                root: &mut self.root,
                key: Some(key),
                value: Some(value),
            }
        } else {
            // ノードの新規作成
            self.size += 1;
            NodeEntry {
                root: &mut self.root,
                key: Some(key),
                value: Some(M::e()),
            }
        }
    }

    /// 要素の更新
    /// - `key`：更新するキー
    /// - `value`：更新後の値
    pub fn insert(&mut self, key: K, value: M::Val) {
        let (new_root, old_key_value) = insert(self.root.take(), key, value);
        self.root = new_root;
        // 要素が追加された場合
        if old_key_value.is_none() {
            self.size += 1;
        }
    }

    /// 要素の削除
    /// - `key`：削除するキー
    pub fn remove(&mut self, key: &K) -> Option<M::Val> {
        let (new_root, old_key_value) = delete(self.root.take(), key);
        self.root = new_root;
        // 削除された要素を返す
        if let Some((_, old_value)) = old_key_value {
            self.size -= 1;
            Some(old_value)
        } else {
            None
        }
    }

    /// 区間の取得
    /// - 区間 `range` の要素を集約する
    pub fn get_range<R: RangeBounds<K>>(&self, range: R) -> M::Val {
        let l = range.start_bound();
        let r = range.end_bound();
        get_range(&self.root, l, r, Unbounded, Unbounded)
    }

    /// 要素数を取得
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

/// ノードの可変参照
pub struct NodeEntry<'a, K: Ord, M: 'a + Monoid> {
    root: &'a mut Node<K, M>,
    key: Option<K>,
    value: Option<M::Val>,
}

impl<K, M> Debug for NodeEntry<'_, K, M>
where
    K: Ord + Debug,
    M: Monoid,
    M::Val: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeEntry")
            .field("key", &self.key.as_ref().unwrap())
            .field("value", &self.value.as_ref().unwrap())
            .finish()
    }
}

impl<K: Ord, M: Monoid> Drop for NodeEntry<'_, K, M> {
    fn drop(&mut self) {
        let root = self.root.take();
        let key = self.key.take().unwrap();
        let value = self.value.take().unwrap();
        (*self.root, _) = insert(root, key, value);
    }
}

impl<K: Ord, M: Monoid> Deref for NodeEntry<'_, K, M> {
    type Target = M::Val;
    fn deref(&self) -> &Self::Target {
        self.value.as_ref().unwrap()
    }
}

impl<K: Ord, M: Monoid> DerefMut for NodeEntry<'_, K, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.as_mut().unwrap()
    }
}

impl<K: Ord, M: Monoid> Default for BalancedBinaryTree<K, M> {
    /// 動的セグ木の初期化
    fn default() -> Self {
        Self {
            root: None,
            size: 0,
            tmp_e: M::e(),
        }
    }
}
