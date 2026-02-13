//! スプレー木の多重集合

pub use multiset_splay_tree_::MultiSet;

mod multiset_splay_tree_ {
    //! 多重集合
    use crate::{
        tree::show_binary_tree::ShowBinaryTree,
        tree::splay_tree::{
            find::{lower_bound, upper_bound},
            insert::{insert, insert_right},
            iterator::{prev, NodeIterator, NodePosition, NodeRangeIterator},
            pointer::{Node, NodeOps, NodePtr},
            remove::remove,
            splay::splay,
        },
    };
    use std::{
        fmt::Debug,
        ops::{Bound, RangeBounds},
        ptr::NonNull,
    };
    /// MultiSet
    /// - 多重集合
    pub struct MultiSet<K: Ord> {
        pub root: Option<NodePtr<K, usize>>,
        size: usize,
    }
    impl<K: Ord> Default for MultiSet<K> {
        fn default() -> Self {
            Self::new()
        }
    }
    impl<K: Ord> MultiSet<K> {
        /// 新規作成
        pub fn new() -> Self {
            Self {
                root: None,
                size: 0,
            }
        }
        /// 要素数
        pub fn len(&self) -> usize {
            self.size
        }
        /// 空判定
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }
        /// 値 `x` を持つノードのうち，最も右側にあるものを探索する
        fn find_rightmost_node(&mut self, key: &K) -> Option<NodePtr<K, usize>> {
            let upperbound = prev(
                {
                    let ub;
                    (self.root, ub) = upper_bound(self.root, key);
                    ub
                },
                &self.root,
            );
            match upperbound {
                NodePosition::Node(node) if node.key() == key => Some(node),
                _ => None,
            }
        }
        /// 要素の追加
        pub fn insert(&mut self, key: K) {
            // 最も右側の頂点を探索
            let rightmost = self.find_rightmost_node(&key);
            let new_node = if let Some(rightmost) = rightmost {
                let cnt = *rightmost.value();
                insert_right(Some(rightmost), key, cnt + 1)
            } else {
                insert(self.root, key, 1)
            };
            self.size += 1;
            self.root = Some(splay(new_node));
        }
        /// 要素の削除
        pub fn remove(&mut self, key: &K) -> bool {
            // 最も右側の頂点を探索
            let Some(rightmost) = self.find_rightmost_node(key) else {
                return false;
            };
            (self.root, _) = remove(rightmost);
            self.size -= 1;
            true
        }
        /// `key` に一致する要素の個数を返す
        pub fn count(&mut self, key: &K) -> usize {
            // 最も右側の頂点を探索
            let rightmost = self.find_rightmost_node(key);
            if let Some(rightmost) = rightmost {
                *rightmost.value()
            } else {
                0
            }
        }
        /// 指定した区間のイテレータを返す
        pub fn range<R: RangeBounds<K>>(&mut self, range: R) -> NodeRangeIterator<'_, K, usize> {
            let left = match range.start_bound() {
                Bound::Unbounded => NodePosition::Inf,
                Bound::Included(x) => prev(
                    {
                        let lb;
                        (self.root, lb) = lower_bound(self.root, x);
                        lb
                    },
                    &self.root,
                ),
                Bound::Excluded(x) => prev(
                    {
                        let ub;
                        (self.root, ub) = upper_bound(self.root, x);
                        ub
                    },
                    &self.root,
                ),
            };
            let right = match range.end_bound() {
                Bound::Unbounded => NodePosition::Sup,
                Bound::Included(x) => {
                    let ub;
                    (self.root, ub) = upper_bound(self.root, x);
                    ub
                }
                Bound::Excluded(x) => {
                    let lb;
                    (self.root, lb) = lower_bound(self.root, x);
                    lb
                }
            };
            NodeRangeIterator::new(&self.root, left, right)
        }
        /// ノードのイテレータを返す
        pub fn iter(&self) -> NodeIterator<'_, K, usize> {
            NodeIterator::first(&self.root)
        }
    }
    impl<K: Ord + Clone + Debug> Debug for MultiSet<K> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_set()
                .entries(NodeIterator::first(&self.root).map(|node| node.key().clone()))
                .finish()
        }
    }

    // ==================== ShowBinaryTree ====================
    /// ShowBinaryTree 用の「ポインタ」
    #[derive(Clone, Copy)]
    pub struct TreePtr<K: Ord>(NonNull<Node<K, usize>>);

    impl<K: Ord> TreePtr<K> {
        #[inline]
        fn mk_ptr(node: &Node<K, usize>) -> TreePtr<K> {
            TreePtr(NonNull::from(node))
        }
    }

    impl<K: Ord + Debug> ShowBinaryTree<TreePtr<K>> for MultiSet<K> {
        fn get_left(&self, ptr: &TreePtr<K>) -> Option<TreePtr<K>> {
            // 読み取り専用だが，trait が &mut self を要求する
            let t = unsafe { ptr.0.as_ref() };
            let left = unsafe { t.left.as_ref()?.as_ref() };
            Some(TreePtr::mk_ptr(left))
        }

        fn get_right(&self, ptr: &TreePtr<K>) -> Option<TreePtr<K>> {
            let t = unsafe { ptr.0.as_ref() };
            let right = unsafe { t.right.as_ref()?.as_ref() };
            Some(TreePtr::mk_ptr(right))
        }

        fn get_root(&self) -> Option<TreePtr<K>> {
            let root = unsafe { self.root.as_ref()?.as_ref() };
            Some(TreePtr::mk_ptr(root))
        }

        fn print_node(&self, ptr: &TreePtr<K>) -> String {
            let t = unsafe { ptr.0.as_ref() };
            format!("{t:?}")
        }
    }
}
