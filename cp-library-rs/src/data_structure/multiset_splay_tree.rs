//! スプレー木の多重集合

pub use inner::{
    collections::multiset::Multiset,
    node::{
        iterator::{NodeIterator, NodePosition, NodeRangeIterator},
        pointer::{Node, NodeOps, NodePtr},
    },
};

mod inner {
    #![allow(dead_code)]
    pub mod collections {
        pub mod multiset {
            //! 多重集合
            use crate::data_structure::multiset_splay_tree::inner::{
                node::{
                    find::{lower_bound, upper_bound},
                    insert::{insert, insert_right},
                    iterator::{prev, NodeIterator, NodePosition, NodeRangeIterator},
                    pointer::{NodeOps, NodePtr},
                    remove::remove,
                    splay::splay,
                },
                utils::print::print_as_tree,
            };
            use std::{
                fmt::Debug,
                ops::{Bound, RangeBounds},
            };
            /// Multiset
            /// - 多重集合
            pub struct Multiset<K: Ord> {
                pub root: Option<NodePtr<K, usize>>,
                size: usize,
            }
            impl<K: Ord> Multiset<K> {
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
                            (self.root, ub) = upper_bound(self.root.clone(), &key);
                            ub
                        },
                        &self.root,
                    );
                    match upperbound {
                        NodePosition::Node(node) if &*node.key() == key => Some(node),
                        _ => None,
                    }
                }
                /// 要素の追加
                pub fn insert(&mut self, key: K) {
                    // 最も右側の頂点を探索
                    let rightmost = self.find_rightmost_node(&key);
                    let new_node;
                    if let Some(rightmost) = rightmost {
                        let cnt = *rightmost.value();
                        new_node = insert_right(Some(rightmost), key, cnt + 1);
                    } else {
                        (_, new_node, _) = insert(self.root.clone(), key, 1);
                    }
                    self.size += 1;
                    self.root = Some(splay(new_node));
                }
                /// 要素の削除
                pub fn remove(&mut self, key: &K) -> bool {
                    // 最も右側の頂点を探索
                    let Some(rightmost) = self.find_rightmost_node(&key) else {
                        return false;
                    };
                    (self.root, _) = remove(rightmost);
                    self.size -= 1;
                    true
                }
                /// `key` に一致する要素の個数を返す
                pub fn count(&mut self, key: &K) -> usize {
                    // 最も右側の頂点を探索
                    let rightmost = self.find_rightmost_node(&key);
                    if let Some(rightmost) = rightmost {
                        *rightmost.value()
                    } else {
                        0
                    }
                }
                /// 指定した区間のイテレータを返す
                pub fn range<R: RangeBounds<K>>(
                    &mut self,
                    range: R,
                ) -> NodeRangeIterator<K, usize> {
                    let left = match range.start_bound() {
                        Bound::Unbounded => NodePosition::INF,
                        Bound::Included(x) => prev(
                            {
                                let lb;
                                (self.root, lb) = lower_bound(self.root.clone(), &x);
                                lb
                            },
                            &self.root,
                        ),
                        Bound::Excluded(x) => prev(
                            {
                                let ub;
                                (self.root, ub) = upper_bound(self.root.clone(), &x);
                                ub
                            },
                            &self.root,
                        ),
                    };
                    let right = match range.end_bound() {
                        Bound::Unbounded => NodePosition::SUP,
                        Bound::Included(x) => {
                            let ub;
                            (self.root, ub) = upper_bound(self.root.clone(), x);
                            ub
                        }
                        Bound::Excluded(x) => {
                            let lb;
                            (self.root, lb) = lower_bound(self.root.clone(), x);
                            lb
                        }
                    };
                    NodeRangeIterator::new(&self.root, left, right)
                }
                /// ノードのイテレータを返す
                pub fn iter(&self) -> NodeIterator<K, usize> {
                    NodeIterator::first(&self.root)
                }
            }
            impl<K: Ord + Clone + Debug> Debug for Multiset<K> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_set()
                        .entries(NodeIterator::first(&self.root).map(|node| node.key().clone()))
                        .finish()
                }
            }
            impl<K: Ord + Clone + Debug> Multiset<K> {
                pub fn print_as_tree(&self) {
                    print_as_tree(&self.root);
                }
            }
        }
    }
    pub mod node {
        pub mod find {
            use crate::data_structure::multiset_splay_tree::inner::node::{
                iterator::NodePosition,
                pointer::{NodeOps, NodePtr},
                splay::splay,
            };
            /// 比較関数 cmp を満たす最小のノードを返す
            ///
            /// **戻り値**
            /// - `Option<NodePtr<K, V>>`: 検索後の根ノード
            /// - `Option<NodePtr<K, V>>`: 比較関数 cmp を満たす最小のノード
            fn find_min<K: Ord, V, F: Fn(&K) -> bool>(
                root: Option<NodePtr<K, V>>,
                cmp: F,
            ) -> (Option<NodePtr<K, V>>, Option<NodePtr<K, V>>) {
                if root.is_none() {
                    return (None, None);
                }
                let mut last = root.clone();
                let mut res = None;
                while let Some(last_inner) = last.clone() {
                    if cmp(&*last_inner.key()) {
                        res = Some(last_inner.clone());
                        last = match last_inner.left().as_ref().map(|node| node.clone()) {
                            Some(node) => Some(node),
                            None => break,
                        };
                    } else {
                        last = match last_inner.right().as_ref().map(|node| node.clone()) {
                            Some(node) => Some(node),
                            None => break,
                        };
                    }
                }
                if let Some(res_inner) = res {
                    (Some(splay(res_inner.clone())), Some(res_inner))
                } else if let Some(last_inner) = last {
                    (Some(splay(last_inner)), res)
                } else {
                    (Some(splay(root.unwrap())), res)
                }
            }
            /// `x` 以上の値を持つ最小のノードを返す
            ///
            /// **戻り値**
            /// - `Option<NodePtr<K, V>>`: 検索後の根ノード
            /// - `Option<NodePtr<K, V>>`: `x` 以上の値を持つ最小のノード
            pub fn lower_bound<K: Ord, V>(
                root: Option<NodePtr<K, V>>,
                x: &K,
            ) -> (Option<NodePtr<K, V>>, NodePosition<K, V>) {
                let (new_root, node) = find_min(root, |k| k >= x);
                if let Some(inner) = node {
                    (new_root, NodePosition::Node(inner))
                } else {
                    (new_root, NodePosition::SUP)
                }
            }
            /// `x` より大きい値を持つ最小のノードを返す
            ///
            /// **戻り値**
            /// - `Option<NodePtr<K, V>>`: 検索後の根ノード
            /// - `Option<NodePtr<K, V>>`: `x` より大きい値を持つ最小のノード
            pub fn upper_bound<K: Ord, V>(
                root: Option<NodePtr<K, V>>,
                x: &K,
            ) -> (Option<NodePtr<K, V>>, NodePosition<K, V>) {
                let (new_root, node) = find_min(root, |k| k > x);
                if let Some(inner) = node {
                    (new_root, NodePosition::Node(inner))
                } else {
                    (new_root, NodePosition::SUP)
                }
            }
            /// 値 `x` を持つノードを返す
            ///
            /// **戻り値**
            /// - `Option<NodePtr<K, V>>`: 検索後の根ノード
            /// - `Option<NodePtr<K, V>>`: 値 `x` を持つノード
            pub fn find<K: Ord, V>(
                root: Option<NodePtr<K, V>>,
                x: &K,
            ) -> (Option<NodePtr<K, V>>, Option<NodePtr<K, V>>) {
                let (new_root, lb) = find_min(root.clone(), |k| k >= x);
                if lb.as_ref().is_some_and(|k| &*k.key() == x) {
                    (new_root, lb)
                } else {
                    (new_root, None)
                }
            }
        }
        pub mod insert {
            use crate::data_structure::multiset_splay_tree::inner::node::pointer::{
                Node, NodeOps, NodePtr,
            };
            use std::{cmp::Ordering, mem};
            /// rootを根とする木に(key, value)を挿入し，挿入後のノードの参照を返す．
            /// すでに同一のキーを持つノードが存在した場合，値を置き換える．
            ///
            /// **引数**
            /// - node: 挿入対象のノード
            /// - key: キー
            /// - value: 値
            ///
            /// **戻り値**
            /// - Option<NodePtr<K, V>>: 挿入後の根ノード
            /// - Option<NodePtr<K, V>>: 追加されたノード
            /// - Option<V>: 置き換えられた値
            pub fn insert<K: Ord, V>(
                root: Option<NodePtr<K, V>>,
                key: K,
                value: V,
            ) -> (Option<NodePtr<K, V>>, NodePtr<K, V>, Option<V>) {
                if root.is_none() {
                    let new_root = Node::node_ptr(key, value);
                    return (Some(new_root.clone()), new_root, None);
                }
                // 親ノードをたどっていく
                let mut par = root.clone();
                loop {
                    let comp = key.cmp(&par.as_ref().unwrap().key());
                    match comp {
                        Ordering::Less => {
                            if let Some(Some(left)) = par.as_ref().map(|node| node.left().clone()) {
                                par.replace(left);
                            } else {
                                // 左側に挿入
                                break (root, insert_left(par, key, value), None);
                            }
                        }
                        Ordering::Equal => {
                            // 置き換える
                            let old_value =
                                mem::replace(&mut *par.as_mut().unwrap().value_mut(), value);
                            break (root, par.unwrap(), Some(old_value));
                        }
                        Ordering::Greater => {
                            if let Some(Some(right)) = par.as_ref().map(|node| node.right().clone())
                            {
                                par.replace(right);
                            } else {
                                // 右側に挿入
                                break (root, insert_right(par, key, value), None);
                            }
                        }
                    }
                }
            }
            /// nodeの左側に子を追加し，追加された子のポインタを返す
            pub fn insert_left<K: Ord, V>(
                node: Option<NodePtr<K, V>>,
                key: K,
                value: V,
            ) -> NodePtr<K, V> {
                let mut new_node = Node::node_ptr(key, value);
                let Some(mut inner) = node else {
                    return new_node;
                };
                // new_node.left ← node.left
                *new_node.left_mut() = inner.take_left();
                // left.parent ← new_node
                if let Some(left) = new_node.clone().left_mut().as_mut() {
                    *left.parent_mut() = Some(new_node);
                }
                // new_node.parent ← node
                *new_node.parent_mut() = Some(inner);
                // node.left ← new_node
                inner.left_mut().replace(new_node.clone());
                new_node
            }
            /// nodeの右側に子を追加し，追加された子のポインタを返す
            pub fn insert_right<K: Ord, V>(
                node: Option<NodePtr<K, V>>,
                key: K,
                value: V,
            ) -> NodePtr<K, V> {
                let mut new_node = Node::node_ptr(key, value);
                let Some(mut inner) = node else {
                    return new_node;
                };
                // new_node.right ← node.right
                *new_node.right_mut() = inner.take_right();
                // right.parent ← new_node
                let new_node_weak = new_node;
                if let Some(right) = new_node.right_mut().as_mut() {
                    *right.parent_mut() = Some(new_node_weak);
                }
                // new_node.parent ← node
                *new_node.parent_mut() = Some(inner);
                // node.right ← new_node
                inner.right_mut().replace(new_node.clone());
                new_node
            }
        }
        pub mod iterator {
            use crate::data_structure::multiset_splay_tree::inner::node::{
                pointer::{NodeOps, NodePtr},
                state::NodeState,
            };
            use std::{cmp, fmt::Debug};
            /// ノードの位置
            #[derive(Debug)]
            pub enum NodePosition<K: Ord, V> {
                /// `K` の下界
                INF,
                /// ノードの値
                Node(NodePtr<K, V>),
                /// `K` の上界
                SUP,
            }
            impl<K: Ord, V> Clone for NodePosition<K, V> {
                fn clone(&self) -> Self {
                    match self {
                        NodePosition::INF => NodePosition::INF,
                        NodePosition::Node(node) => NodePosition::Node(node.clone()),
                        NodePosition::SUP => NodePosition::SUP,
                    }
                }
            }
            impl<K: Ord, V> PartialEq for NodePosition<K, V> {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (NodePosition::INF, NodePosition::INF) => true,
                        (NodePosition::SUP, NodePosition::SUP) => true,
                        (NodePosition::Node(node1), NodePosition::Node(node2)) => {
                            node1.is_same(node2)
                        }
                        _ => false,
                    }
                }
            }
            impl<K: Ord, V> PartialOrd for NodePosition<K, V> {
                fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
                    match (self, other) {
                        (NodePosition::INF, NodePosition::INF) => Some(cmp::Ordering::Equal),
                        (NodePosition::SUP, NodePosition::SUP) => Some(cmp::Ordering::Equal),
                        (NodePosition::Node(node1), NodePosition::Node(node2)) => {
                            Some(node1.key_cmp(node2))
                        }
                        (NodePosition::INF, _) => Some(cmp::Ordering::Less),
                        (NodePosition::SUP, _) => Some(cmp::Ordering::Greater),
                        (_, NodePosition::INF) => Some(cmp::Ordering::Greater),
                        (_, NodePosition::SUP) => Some(cmp::Ordering::Less),
                    }
                }
            }
            impl<K: Ord, V> NodePosition<K, V> {
                pub fn is_inf(&self) -> bool {
                    match self {
                        NodePosition::INF => true,
                        _ => false,
                    }
                }
                pub fn is_sup(&self) -> bool {
                    match self {
                        NodePosition::SUP => true,
                        _ => false,
                    }
                }
                pub fn is_node(&self) -> bool {
                    match self {
                        NodePosition::Node(_) => true,
                        _ => false,
                    }
                }
                pub fn is_none(&self) -> bool {
                    match self {
                        NodePosition::INF | NodePosition::SUP => true,
                        _ => false,
                    }
                }
                pub fn unwrap(self) -> NodePtr<K, V> {
                    match self {
                        NodePosition::Node(node) => node,
                        _ => panic!("NodePosition::unwrap"),
                    }
                }
                pub fn as_ref(&self) -> Option<&NodePtr<K, V>> {
                    match self {
                        NodePosition::Node(node) => Some(node),
                        _ => None,
                    }
                }
            }
            /// 次に小さい値を持つノードを返す
            ///
            /// - 計算量： `O(1) amotized`
            pub fn prev<K: Ord, V>(
                iter: NodePosition<K, V>,
                root: &Option<NodePtr<K, V>>,
            ) -> NodePosition<K, V> {
                match iter {
                    NodePosition::INF => NodePosition::INF,
                    NodePosition::Node(mut node) => {
                        if let Some(mut prv) = node.left().as_ref().map(|node| node.clone()) {
                            while let Some(right) =
                                prv.clone().right().as_ref().map(|node| node.clone())
                            {
                                prv = right;
                            }
                            return NodePosition::Node(prv);
                        }
                        // 親をたどる
                        while node.is_child() {
                            match node.get_state() {
                                NodeState::LeftChild => {
                                    // 親は存在する
                                    node = node.parent().unwrap();
                                }
                                NodeState::RightChild => {
                                    return NodePosition::Node(node.parent().unwrap());
                                }
                                _ => unreachable!(),
                            }
                        }
                        NodePosition::INF
                    }
                    NodePosition::SUP => match get_max(root.clone()) {
                        Some(node) => NodePosition::Node(node),
                        None => NodePosition::SUP,
                    },
                }
            }
            /// 次に大きい値をもつノードを返す
            ///
            /// - 計算量： `O(1) amotized`
            pub fn next<K: Ord, V>(
                iter: NodePosition<K, V>,
                root: &Option<NodePtr<K, V>>,
            ) -> NodePosition<K, V> {
                match iter {
                    NodePosition::INF => match get_min(root.clone()) {
                        Some(node) => NodePosition::Node(node),
                        None => NodePosition::INF,
                    },
                    NodePosition::Node(mut node) => {
                        if let Some(mut nxt) = node.right().as_ref().map(|node| node.clone()) {
                            while let Some(left) =
                                nxt.clone().left().as_ref().map(|node| node.clone())
                            {
                                nxt = left;
                            }
                            return NodePosition::Node(nxt);
                        }
                        // 親をたどる
                        while node.is_child() {
                            match node.get_state() {
                                NodeState::RightChild => {
                                    // 親は存在する
                                    node = node.parent().unwrap();
                                }
                                NodeState::LeftChild => {
                                    return NodePosition::Node(node.parent().unwrap());
                                }
                                _ => unreachable!(),
                            }
                        }
                        NodePosition::SUP
                    }
                    NodePosition::SUP => NodePosition::SUP,
                }
            }
            /// rootを根とする木のうち，最も左側の子を返す
            pub fn get_min<K: Ord, V>(root: Option<NodePtr<K, V>>) -> Option<NodePtr<K, V>> {
                let mut node = root;
                while let left @ Some(_) = node.as_ref().map(|node| node.left().clone())? {
                    node = left;
                }
                node
            }
            /// rootを根とする木のうち，最も右側の子を返す
            pub fn get_max<K: Ord, V>(root: Option<NodePtr<K, V>>) -> Option<NodePtr<K, V>> {
                let mut node = root;
                while let right @ Some(_) = node.as_ref().map(|node| node.right().clone())? {
                    node = right;
                }
                node
            }
            /// ノードのイテレータ
            pub struct NodeIterator<'a, K: Ord, V> {
                /// 根のポインタ
                root: &'a Option<NodePtr<K, V>>,
                /// 現在の位置
                pos: NodePosition<K, V>,
            }
            impl<'a, K: Ord, V> NodeIterator<'a, K, V> {
                /// 新しいイテレータを返す
                pub fn new(root: &'a Option<NodePtr<K, V>>, node: NodePtr<K, V>) -> Self {
                    NodeIterator {
                        root,
                        pos: NodePosition::Node(node),
                    }
                }
                /// 左端のイテレータを返す
                pub fn first(root: &'a Option<NodePtr<K, V>>) -> Self {
                    NodeIterator {
                        root,
                        pos: NodePosition::INF,
                    }
                }
                /// 右端のイテレータを返す
                pub fn last(root: &'a Option<NodePtr<K, V>>) -> Self {
                    NodeIterator {
                        root,
                        pos: NodePosition::SUP,
                    }
                }
            }
            impl<'a, K: Ord, V> Iterator for NodeIterator<'a, K, V> {
                type Item = NodePtr<K, V>;
                fn next(&mut self) -> Option<Self::Item> {
                    // posを次に進める
                    self.pos = next(self.pos.clone(), self.root);
                    let val = self.pos.as_ref().map(|node| node.clone())?;
                    Some(val)
                }
            }
            impl<'a, K: Ord, V> DoubleEndedIterator for NodeIterator<'a, K, V> {
                fn next_back(&mut self) -> Option<Self::Item> {
                    // posを前に進める
                    self.pos = prev(self.pos.clone(), self.root);
                    let val = self.pos.as_ref().map(|node| node.clone())?;
                    Some(val)
                }
            }
            /// 範囲をもつイテレータ
            pub struct NodeRangeIterator<'a, K: Ord, V> {
                /// 根のポインタ
                root: &'a Option<NodePtr<K, V>>,
                /// 左端の位置
                left: NodePosition<K, V>,
                /// 右端の位置
                right: NodePosition<K, V>,
            }
            impl<'a, K: Ord, V> NodeRangeIterator<'a, K, V> {
                /// 新しいイテレータを返す
                pub fn new(
                    root: &'a Option<NodePtr<K, V>>,
                    left: NodePosition<K, V>,
                    right: NodePosition<K, V>,
                ) -> Self {
                    NodeRangeIterator { root, left, right }
                }
            }
            impl<'a, K: Ord, V> Iterator for NodeRangeIterator<'a, K, V> {
                type Item = NodePtr<K, V>;
                fn next(&mut self) -> Option<Self::Item> {
                    // 左端を次に進める
                    self.left = next(self.left.clone(), self.root);
                    // 左端が右端に到達したら終了
                    if self.left >= self.right {
                        return None;
                    }
                    let val = self.left.as_ref().map(|node| node.clone())?;
                    Some(val)
                }
            }
            impl<'a, K: Ord + Debug, V: Debug> DoubleEndedIterator for NodeRangeIterator<'a, K, V> {
                fn next_back(&mut self) -> Option<Self::Item> {
                    // 右端を前に進める
                    self.right = prev(self.right.clone(), self.root);
                    // 右端が左端に到達したら終了
                    if self.right <= self.left {
                        return None;
                    }
                    let val = self.right.as_ref().map(|node| node.clone())?;
                    Some(val)
                }
            }
        }
        pub mod pointer {
            //! ノードのポインタ
            macro_rules! generate_getters {
                // 不変参照用のgetterを生成
                ($name:ident, $field:ident, $return_type:ty) => {
                    fn $name(&self) -> $return_type {
                        unsafe { &self.as_ref().$field }
                    }
                };
                // 可変参照用のgetterを生成
                ($name:ident, $field:ident, $return_type:ty, mut) => {
                    fn $name(&mut self) -> $return_type {
                        unsafe { &mut self.as_mut().$field }
                    }
                };
            }
            use super::state::NodeState;
            use std::{fmt::Debug, ptr::NonNull};
            /// ノードのポインタ
            pub type NodePtr<K, V> = NonNull<Node<K, V>>;
            /// ノードの構造体
            pub struct Node<K: Ord, V> {
                pub key: K,
                pub value: V,
                pub parent: Option<NodePtr<K, V>>,
                pub left: Option<NodePtr<K, V>>,
                pub right: Option<NodePtr<K, V>>,
            }
            impl<K: Ord, V> Node<K, V> {
                /// 葉ノードを作成する
                pub fn new(key: K, value: V) -> Self {
                    Self {
                        key,
                        value,
                        parent: None,
                        left: None,
                        right: None,
                    }
                }
                /// ノードのポインタを確保する
                pub fn node_ptr(key: K, value: V) -> NodePtr<K, V> {
                    let ptr = Box::new(Self::new(key, value));
                    NonNull::new(Box::into_raw(ptr))
                        .unwrap_or_else(|| panic!("Failed to allocate memory"))
                }
            }
            impl<K: Ord + Debug, V: Debug> Debug for Node<K, V> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match (&self.left, &self.right) {
                        (None, None) => f
                            .debug_struct("Node")
                            .field(&"key", &self.key)
                            .field(&"value", &self.value)
                            .finish(),
                        (Some(_), None) => f
                            .debug_struct("Node")
                            .field(&"key", &self.key)
                            .field(&"value", &self.value)
                            .field(&"left", &self.left)
                            .finish(),
                        (None, Some(_)) => f
                            .debug_struct("Node")
                            .field(&"key", &self.key)
                            .field(&"value", &self.value)
                            .field(&"right", &self.right)
                            .finish(),
                        (Some(_), Some(_)) => f
                            .debug_struct("Node")
                            .field(&"key", &self.key)
                            .field(&"value", &self.value)
                            .field(&"left", &self.left)
                            .field(&"right", &self.right)
                            .finish(),
                    }
                }
            }
            /// ポインタに対する操作
            pub trait NodeOps<K: Ord, V> {
                /// 与えられたノードが子ノードであるかを判定する
                fn is_child(&self) -> bool;
                /// 与えられたノードが
                /// - 空のノード
                /// - 根ノード
                /// - 親の左の子
                /// - 親の右の子
                ///
                /// のどれかを判定する．
                fn get_state(&self) -> NodeState;
                /// ポインタの同一性判定
                fn is_same(&self, other: &Self) -> bool;
                /// ポインタの半順序
                fn key_cmp(&self, other: &Self) -> std::cmp::Ordering;
                /// 親へのポインタを切り離す
                fn take_parent(&mut self) -> Option<NodePtr<K, V>>;
                /// 親へのポインタを切り離し，強参照を取得する
                fn take_parent_strong(&mut self) -> Option<NodePtr<K, V>>;
                /// 左の子へのポインタを切り離す
                fn take_left(&mut self) -> Option<NodePtr<K, V>>;
                /// 右の子へのポインタを切り離す
                fn take_right(&mut self) -> Option<NodePtr<K, V>>;
                /// 親の参照を取得する
                fn parent(&self) -> &Option<NodePtr<K, V>>;
                /// 親の可変参照を取得する
                fn parent_mut(&mut self) -> &mut Option<NodePtr<K, V>>;
                /// 左の子への参照を取得する
                fn left(&self) -> &Option<NodePtr<K, V>>;
                /// 左の子への可変参照を取得する
                fn left_mut(&mut self) -> &mut Option<NodePtr<K, V>>;
                /// 右の子への参照を取得する
                fn right(&self) -> &Option<NodePtr<K, V>>;
                /// 右の子への可変参照を取得する
                fn right_mut(&mut self) -> &mut Option<NodePtr<K, V>>;
                /// キーへの参照を取得する
                fn key(&self) -> &K;
                /// バリューへの参照を取得する
                fn value(&self) -> &V;
                /// バリューへの可変参照を取得する
                fn value_mut(&mut self) -> &mut V;
            }
            impl<K: Ord, V> NodeOps<K, V> for NodePtr<K, V> {
                fn is_child(&self) -> bool {
                    self.parent().is_some()
                }
                fn get_state(&self) -> NodeState {
                    let par = self.parent();
                    if par.is_none() {
                        return NodeState::Root;
                    }
                    if par.is_some_and(|ptr| ptr.left().is_some_and(|left| left.is_same(self))) {
                        return NodeState::LeftChild;
                    }
                    if par.is_some_and(|ptr| ptr.right().is_some_and(|right| right.is_same(self))) {
                        return NodeState::RightChild;
                    }
                    unreachable!()
                }
                fn is_same(&self, other: &Self) -> bool {
                    NonNull::eq(&self, other)
                }
                fn key_cmp(&self, other: &Self) -> std::cmp::Ordering {
                    unsafe { self.as_ref().key.cmp(&other.as_ref().key) }
                }
                fn take_parent(&mut self) -> Option<NodePtr<K, V>> {
                    unsafe { self.as_mut().parent.take() }
                }
                fn take_parent_strong(&mut self) -> Option<NodePtr<K, V>> {
                    unsafe { self.as_mut().parent.take().map(|p| p) }
                }
                fn take_left(&mut self) -> Option<NodePtr<K, V>> {
                    unsafe {
                        let mut left = self.as_mut().left.take();
                        if let Some(left_inner) = left.as_mut() {
                            *left_inner.parent_mut() = None;
                        }
                        left
                    }
                }
                fn take_right(&mut self) -> Option<NodePtr<K, V>> {
                    unsafe {
                        let mut right = self.as_mut().right.take();
                        if let Some(right_inner) = right.as_mut() {
                            *right_inner.parent_mut() = None;
                        }
                        right
                    }
                }
                // 不変参照用のgetterを生成
                generate_getters!(parent, parent, &Option<NodePtr<K, V>>);
                generate_getters!(left, left, &Option<NodePtr<K, V>>);
                generate_getters!(right, right, &Option<NodePtr<K, V>>);
                generate_getters!(key, key, &K);
                generate_getters!(value, value, &V);
                // 可変参照用のgetterを生成
                generate_getters!(parent_mut, parent, &mut Option<NodePtr<K, V>>, mut);
                generate_getters!(left_mut, left, &mut Option<NodePtr<K, V>>, mut);
                generate_getters!(right_mut, right, &mut Option<NodePtr<K, V>>, mut);
                generate_getters!(value_mut, value, &mut V, mut);
            }
        }
        pub mod remove {
            use crate::data_structure::multiset_splay_tree::inner::node::{
                iterator::get_min,
                pointer::{NodeOps, NodePtr},
                splay::splay,
            };
            /// ノード node を削除する
            ///
            /// **引数**
            /// - root: 削除対象の木の根のポインタ
            /// - node: 削除対象のノードのポインタ
            ///
            /// **戻り値**
            /// - NodePtr\<K, V\>: 削除後の木の根のポインタ
            /// - NodePtr\<K, V\>: 削除されたノードのポインタ
            pub fn remove<K: Ord, V>(
                mut node: NodePtr<K, V>,
            ) -> (Option<NodePtr<K, V>>, NodePtr<K, V>) {
                // nodeを根に持ってくる
                let root = splay(node.clone());
                // 左右に分割
                let left = node.take_left();
                let mut right = node.take_right();
                // 右部分木の最小値を取得
                let right_min = get_min(right.clone());
                if let Some(right_min_inner) = right_min {
                    right = Some(splay(right_min_inner));
                }
                // right.left <- left
                if let Some(mut left_inner) = left.clone() {
                    *left_inner.parent_mut() = right.clone();
                }
                if let Some(mut right_inner) = right.clone() {
                    *right_inner.left_mut() = left;
                } else {
                    return (left, root);
                }
                (right, root)
            }
        }
        pub mod splay {
            use crate::data_structure::multiset_splay_tree::inner::node::{
                pointer::{NodeOps, NodePtr},
                state::NodeState,
            };
            /// nodeを1つ上に持ってくるように回転する
            pub fn rotate<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
                match node.get_state() {
                    NodeState::Root => node,
                    NodeState::LeftChild => {
                        let mut right = node.take_right();
                        let par = node.parent().clone();
                        // 自分の右の子の親←親
                        if let Some(right_inner) = right.as_mut() {
                            *right_inner.parent_mut() = par.clone();
                        }
                        // 親はかならず存在する
                        let mut par_inner = par.unwrap();
                        // 親の左の子←自分の右の子
                        *par_inner.left_mut() = right;
                        // 自分の親←親の親
                        let par_state = par_inner.get_state();
                        let parpar = par_inner.parent_mut();
                        if let Some(parpar_inner) = parpar.as_mut() {
                            match par_state {
                                NodeState::LeftChild => {
                                    *parpar_inner.left_mut() = Some(node.clone());
                                }
                                NodeState::RightChild => {
                                    *parpar_inner.right_mut() = Some(node.clone());
                                }
                                _ => (),
                            }
                        }
                        *node.parent_mut() = parpar.clone();
                        // 自分の右の子←親
                        *par_inner.parent_mut() = Some(node);
                        node.right_mut().replace(par_inner);
                        node
                    }
                    NodeState::RightChild => {
                        let mut left = node.take_left();
                        let par = node.parent().clone();
                        // 自分の左の子の親←親
                        if let Some(left_inner) = left.as_mut() {
                            *left_inner.parent_mut() = par.clone();
                        }
                        // 親はかならず存在する
                        let mut par_inner = par.unwrap();
                        // 親の右の子←自分の左の子
                        *par_inner.right_mut() = left;
                        // 自分の親←親の親
                        let par_state = par_inner.get_state();
                        let parpar = par_inner.parent_mut();
                        if let Some(parpar_inner) = parpar.as_mut() {
                            match par_state {
                                NodeState::LeftChild => {
                                    *parpar_inner.left_mut() = Some(node.clone());
                                }
                                NodeState::RightChild => {
                                    *parpar_inner.right_mut() = Some(node.clone());
                                }
                                _ => (),
                            }
                        }
                        *node.parent_mut() = parpar.clone();
                        // 自分の左の子←親
                        *par_inner.parent_mut() = Some(node);
                        node.left_mut().replace(par_inner);
                        node
                    }
                }
            }
            /// スプレー操作によりnodeを根に移動し，新たな根を返す
            pub fn splay<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
                while node.is_child() {
                    // 頂点の状態
                    let state = node.get_state();
                    // 親頂点の状態（親は存在する）
                    let par = node.parent().unwrap();
                    let par_state = par.get_state();
                    match (state, par_state) {
                        (NodeState::Root, _) => {
                            break;
                        }
                        // zig
                        (NodeState::LeftChild | NodeState::RightChild, NodeState::Root) => {
                            node = rotate(node);
                        }
                        // zig-zag
                        (NodeState::LeftChild, NodeState::RightChild)
                        | (NodeState::RightChild, NodeState::LeftChild) => {
                            node = rotate(node);
                            node = rotate(node);
                        }
                        // zig-zig
                        (NodeState::LeftChild, NodeState::LeftChild)
                        | (NodeState::RightChild, NodeState::RightChild) => {
                            // 親を先にrotate（オブジェクトをdropさせないため，変数に代入する）
                            let _par = rotate(node.parent().unwrap());
                            node = rotate(node);
                        }
                    }
                }
                node
            }
        }
        pub mod state {
            //! ノードの状態を返す列挙子
            /// ノードの状態を調べる
            #[derive(Debug, PartialEq)]
            pub enum NodeState {
                /// 根ノード（親を持たない）
                Root,
                /// 親の左の子
                LeftChild,
                /// 親の右の子
                RightChild,
            }
        }
    }
    pub mod utils {
        pub mod print {
            //! 木を整形して表示するための関数
            use crate::data_structure::multiset_splay_tree::inner::node::pointer::{
                NodeOps, NodePtr,
            };
            use std::fmt::Debug;
            const BLUE: &str = "\x1b[94m";
            const END: &str = "\x1b[0m";
            const LEFT: &str = " ┌──";
            const MID: &str = " │  ";
            const RIGHT: &str = " └──";
            const NULL: &str = "";
            const BLANK: &str = "    ";
            /// 2分木として出力する
            pub fn print_as_tree<K: Ord + Debug, V: Debug>(root: &Option<NodePtr<K, V>>) {
                eprintln!("{BLUE}┌─ BinaryTree ──────────────────────────────────────────{END}");
                fmt_inner_binary_tree(root, &mut vec![], NULL);
                eprintln!("{BLUE}└───────────────────────────────────────────────────────{END}");
            }
            /// print recursive
            fn fmt_inner_binary_tree<K: Ord + Debug, V: Debug>(
                node: &Option<NodePtr<K, V>>,
                fill: &mut Vec<&'static str>,
                last: &'static str,
            ) {
                if let Some(node) = node {
                    // 表示の調整
                    let mut tmp = None;
                    if fill.last().is_some_and(|x| x == &last) {
                        tmp = fill.pop();
                        fill.push(BLANK);
                    } else if fill.last().is_some_and(|x| x != &NULL && x != &BLANK) {
                        tmp = fill.pop();
                        fill.push(MID);
                    }
                    fill.push(last);
                    // 左の子
                    fmt_inner_binary_tree(&node.left(), fill, LEFT);
                    // 自分を出力
                    eprintln!(
                        "{BLUE}│{END}{} Node {{ key: {:?}, value: {:?} }}",
                        fill.iter().fold(String::new(), |s, x| s + x),
                        node.key(),
                        node.value(),
                    );
                    // 右の子
                    fmt_inner_binary_tree(&node.right(), fill, RIGHT);
                    fill.pop();
                    // 戻す
                    if let Some(tmp) = tmp {
                        fill.pop();
                        fill.push(tmp);
                    }
                }
            }
        }
    }
}
