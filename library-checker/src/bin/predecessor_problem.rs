#![allow(non_snake_case)]

use cp_library_rs::get;
use multiset::{node::pointer::NodeOps, Multiset};

fn main() {
    let (_N, Q) = get!(usize, usize);
    let T = get!(String);

    let mut set = T
        .chars()
        .enumerate()
        .fold(Multiset::new(), |mut set, (x, e)| {
            if e == '1' {
                set.insert(x as isize);
            }
            set
        });

    // set.print_as_tree();

    for _ in 0..Q {
        let (c, k) = get!(usize, isize);

        match c {
            0 => {
                if set.count(&k) == 0 {
                    set.insert(k);
                }
            }
            1 => {
                set.remove(&k);
            }
            2 => {
                println!("{}", set.count(&k));
            }
            3 => {
                if let Some(Some(val)) = set.range(k..).next().map(|node| node.key().map(|k| *k)) {
                    println!("{}", val);
                } else {
                    println!("-1");
                }
            }
            4 => {
                if let Some(Some(res)) = set
                    .range(..=k)
                    .next_back()
                    .map(|node| node.key().map(|k| *k))
                {
                    println!("{}", res);
                } else {
                    println!("-1");
                }
            }
            _ => (),
        }

        // set.print_as_tree();
    }
}

pub mod multiset {
    //! 多重集合

    use std::{
        fmt::Debug,
        ops::{Bound, RangeBounds},
    };

    use node::{
        find::{lower_bound, upper_bound},
        insert::{insert, insert_right},
        iterator::{prev, NodeIterator, NodePosition, NodeRangeIterator},
        pointer::{NodeOps, NodePtr},
        remove::remove,
        splay::splay,
    };
    use print_util::print_as_tree;

    /// Multiset
    /// - 多重集合
    pub struct Multiset<K: Ord> {
        pub root: NodePtr<K, usize>,
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
        fn find_rightmost_node(&mut self, key: &K) -> NodePtr<K, usize> {
            let upperbound = prev(
                {
                    let ub;
                    (self.root, ub) = upper_bound(self.root.clone(), &key);
                    if ub.is_some() {
                        NodePosition::Node(ub)
                    } else {
                        NodePosition::SUP
                    }
                },
                &self.root,
            );

            match upperbound {
                NodePosition::Node(node) if node.key().is_some_and(|k| &*k == key) => node,
                _ => None,
            }
        }

        /// 要素の追加
        pub fn insert(&mut self, key: K) {
            // 最も右側の頂点を探索
            let rightmost = self.find_rightmost_node(&key);

            let new_node;
            if rightmost.is_some() {
                let cnt = *rightmost.value().unwrap();
                new_node = insert_right(rightmost, key, cnt + 1);
            } else {
                (_, new_node, _) = insert(self.root.clone(), key, 1);
            }

            self.size += 1;
            self.root = splay(new_node);
        }

        /// 要素の削除
        pub fn remove(&mut self, key: &K) -> bool {
            // 最も右側の頂点を探索
            let rightmost = self.find_rightmost_node(&key);

            if rightmost.is_none() {
                return false;
            }

            (self.root, _) = remove(self.root.clone(), rightmost);

            self.size -= 1;
            true
        }

        /// `key` に一致する要素の個数を返す
        pub fn count(&mut self, key: &K) -> usize {
            // 最も右側の頂点を探索
            let rightmost = self.find_rightmost_node(&key);

            if rightmost.is_some() {
                *rightmost.value().unwrap()
            } else {
                0
            }
        }

        /// 指定した区間のイテレータを返す
        pub fn range<R: RangeBounds<K>>(&mut self, range: R) -> NodeRangeIterator<K, usize> {
            let left = match range.start_bound() {
                Bound::Unbounded => NodePosition::INF,
                Bound::Included(x) => prev(
                    {
                        let lb;
                        (self.root, lb) = lower_bound(self.root.clone(), &x);
                        if lb.is_some() {
                            NodePosition::Node(lb)
                        } else {
                            NodePosition::SUP
                        }
                    },
                    &self.root,
                ),
                Bound::Excluded(x) => prev(
                    {
                        let ub;
                        (self.root, ub) = upper_bound(self.root.clone(), &x);
                        if ub.is_some() {
                            NodePosition::Node(ub)
                        } else {
                            NodePosition::SUP
                        }
                    },
                    &self.root,
                ),
            };
            let right = match range.end_bound() {
                Bound::Unbounded => NodePosition::SUP,
                Bound::Included(x) => {
                    let right;
                    (self.root, right) = upper_bound(self.root.clone(), x);
                    if right.is_some() {
                        NodePosition::Node(right)
                    } else {
                        NodePosition::SUP
                    }
                }
                Bound::Excluded(x) => {
                    let right;
                    (self.root, right) = lower_bound(self.root.clone(), x);
                    if right.is_some() {
                        NodePosition::Node(right)
                    } else {
                        NodePosition::SUP
                    }
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
                .entries(NodeIterator::first(&self.root).map(|node| node.key().unwrap().clone()))
                .finish()
        }
    }

    impl<K: Ord + Clone + Debug> Multiset<K> {
        pub fn print_as_tree(&self) {
            print_as_tree(&self.root);
        }
    }

    pub mod node {
        pub mod find {
            use super::{
                pointer::{NodeOps, NodePtr},
                splay::splay,
            };

            /// 比較関数 cmp を満たす最小のノードを返す
            ///
            /// **戻り値**
            /// - `NodePtr<K, V>`: 検索後の根ノード
            /// - `NodePtr<K, V>`: 比較関数 cmp を満たす最小のノード
            fn find_min<K: Ord, V, F: Fn(&K) -> bool>(
                root: NodePtr<K, V>,
                cmp: F,
            ) -> (NodePtr<K, V>, NodePtr<K, V>) {
                let mut last = root.clone();
                let mut res = None;

                while last.is_some() {
                    if last.key().is_some_and(|k| cmp(&k)) {
                        res = last.clone();
                        last = match last.left().map(|node| node.clone()) {
                            Some(node) => node,
                            None => break,
                        };
                    } else {
                        last = match last.right().map(|node| node.clone()) {
                            Some(node) => node,
                            None => break,
                        };
                    }
                }

                if res.is_some() {
                    (splay(res.clone()), res)
                } else if last.is_some() {
                    (splay(last), res)
                } else {
                    (splay(root), res)
                }
            }

            /// `x` 以上の値を持つ最小のノードを返す
            ///
            /// **戻り値**
            /// - `NodePtr<K, V>`: 検索後の根ノード
            /// - `NodePtr<K, V>`: `x` 以上の値を持つ最小のノード
            pub fn lower_bound<K: Ord, V>(
                root: NodePtr<K, V>,
                x: &K,
            ) -> (NodePtr<K, V>, NodePtr<K, V>) {
                find_min(root, |k| k >= x)
            }

            /// `x` より大きい値を持つ最小のノードを返す
            ///
            /// **戻り値**
            /// - `NodePtr<K, V>`: 検索後の根ノード
            /// - `NodePtr<K, V>`: `x` より大きい値を持つ最小のノード
            pub fn upper_bound<K: Ord, V>(
                root: NodePtr<K, V>,
                x: &K,
            ) -> (NodePtr<K, V>, NodePtr<K, V>) {
                find_min(root, |k| k > x)
            }

            /// 値 `x` を持つノードを返す
            ///
            /// **戻り値**
            /// - `NodePtr<K, V>`: 検索後の根ノード
            /// - `NodePtr<K, V>`: 値 `x` を持つノード
            pub fn find<K: Ord, V>(root: NodePtr<K, V>, x: &K) -> (NodePtr<K, V>, NodePtr<K, V>) {
                let (new_root, lb) = lower_bound(root.clone(), x);
                if lb.key().is_some_and(|k| &*k == x) {
                    (new_root, lb)
                } else {
                    (new_root, None)
                }
            }
        }

        pub mod insert {
            use std::{cmp::Ordering, mem};

            use super::pointer::NodePtr;
            use super::pointer::{Node, NodeOps};

            /// rootを根とする木に(key, value)を挿入し，挿入後のノードの参照を返す．
            /// すでに同一のキーを持つノードが存在した場合，値を置き換える．
            ///
            /// **引数**
            /// - node: 挿入対象のノード
            /// - key: キー
            /// - value: 値
            ///
            /// **戻り値**
            /// - NodePtr<K, V>: 挿入後の根ノード
            /// - NodePtr<K, V>: 追加されたノード
            /// - Option<V>: 置き換えられた値
            pub fn insert<K: Ord, V>(
                root: NodePtr<K, V>,
                key: K,
                value: V,
            ) -> (NodePtr<K, V>, NodePtr<K, V>, Option<V>) {
                if root.is_none() {
                    let new_node = Node::node_ptr(key, value);
                    return (new_node.clone(), new_node, None);
                }

                // 親ノードをたどっていく
                let mut par = root.clone();

                loop {
                    let comp = key.cmp(&par.key().unwrap());
                    match comp {
                        Ordering::Less => {
                            if let Some(left) = par.left().map(|node| node.clone()).unwrap() {
                                par = Some(left);
                            } else {
                                // 左側に挿入
                                break (root, insert_left(par, key, value), None);
                            }
                        }
                        Ordering::Equal => {
                            // 置き換える
                            let old_value = mem::replace(&mut *par.value_mut().unwrap(), value);
                            break (root, par, Some(old_value));
                        }
                        Ordering::Greater => {
                            if let Some(right) = par.right().map(|node| node.clone()).unwrap() {
                                par = Some(right);
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
                mut node: NodePtr<K, V>,
                key: K,
                value: V,
            ) -> NodePtr<K, V> {
                let mut new_node = Node::node_ptr(key, value);

                if node.is_none() {
                    return new_node;
                }

                // new_node.left ← node.left
                *new_node.left_mut().unwrap() = node.take_left();

                // left.parent ← new_node
                let new_node_weak = new_node.to_weak_ptr();
                if let Some(mut left_par) = new_node.left_mut().unwrap().parent_mut() {
                    *left_par = new_node_weak;
                }

                // new_node.parent ← node
                *new_node.parent_mut().unwrap() = node.to_weak_ptr();

                // node.left ← new_node
                if let Some(mut left) = node.left_mut() {
                    *left = new_node.clone();
                }

                new_node
            }

            /// nodeの右側に子を追加し，追加された子のポインタを返す
            pub fn insert_right<K: Ord, V>(
                mut node: NodePtr<K, V>,
                key: K,
                value: V,
            ) -> NodePtr<K, V> {
                let mut new_node = Node::node_ptr(key, value);

                if node.is_none() {
                    return new_node;
                }

                // new_node.right ← node.right
                *new_node.right_mut().unwrap() = node.take_right();

                // right.parent ← new_node
                let new_node_weak = new_node.to_weak_ptr();
                if let Some(mut right_par) = new_node.right_mut().unwrap().parent_mut() {
                    *right_par = new_node_weak;
                }

                // new_node.parent ← node
                *new_node.parent_mut().unwrap() = node.to_weak_ptr();

                // node.right ← new_node
                if let Some(mut right) = node.right_mut() {
                    *right = new_node.clone();
                }

                new_node
            }
        }

        pub mod iterator {
            use std::cmp;

            use super::{
                pointer::{NodeOps, NodePtr},
                state::NodeState,
            };

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
                            node1.partial_cmp(node2)
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
                root: &NodePtr<K, V>,
            ) -> NodePosition<K, V> {
                match iter {
                    NodePosition::INF => NodePosition::INF,
                    NodePosition::Node(mut node) => {
                        if let Some(left) = node.left().map(|node| node.clone()) {
                            if let Some(mut prv) = left {
                                while let Some(right) =
                                    Some(prv.clone()).right().map(|node| node.clone()).unwrap()
                                {
                                    prv = right;
                                }
                                return NodePosition::Node(Some(prv));
                            }
                        }

                        // 親をたどる
                        while node.is_child() {
                            match node.get_state() {
                                NodeState::LeftChild => {
                                    node = node.get_parent_ptr();
                                }
                                NodeState::RightChild => {
                                    return NodePosition::Node(node.get_parent_ptr());
                                }
                                _ => unreachable!(),
                            }
                        }

                        NodePosition::INF
                    }
                    NodePosition::SUP => NodePosition::Node(get_max(root.clone())),
                }
            }

            /// 次に大きい値をもつノードを返す
            ///
            /// - 計算量： `O(1) amotized`
            pub fn next<K: Ord, V>(
                iter: NodePosition<K, V>,
                root: &NodePtr<K, V>,
            ) -> NodePosition<K, V> {
                match iter {
                    NodePosition::INF => NodePosition::Node(get_min(root.clone())),
                    NodePosition::Node(mut node) => {
                        if let Some(right) = node.right().map(|node| node.clone()) {
                            if let Some(mut nxt) = right {
                                while let Some(left) =
                                    Some(nxt.clone()).left().map(|node| node.clone()).unwrap()
                                {
                                    nxt = left;
                                }
                                return NodePosition::Node(Some(nxt));
                            }
                        }

                        // 親をたどる
                        while node.is_child() {
                            match node.get_state() {
                                NodeState::RightChild => {
                                    node = node.get_parent_ptr();
                                }
                                NodeState::LeftChild => {
                                    return NodePosition::Node(node.get_parent_ptr());
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
            pub fn get_min<K: Ord, V>(root: NodePtr<K, V>) -> NodePtr<K, V> {
                let mut node = root;

                while let left @ Some(_) = node.left().map(|node| node.clone())? {
                    node = left;
                }

                node
            }

            /// rootを根とする木のうち，最も右側の子を返す
            pub fn get_max<K: Ord, V>(root: NodePtr<K, V>) -> NodePtr<K, V> {
                let mut node = root;

                while let right @ Some(_) = node.right().map(|node| node.clone())? {
                    node = right;
                }

                node
            }

            /// ノードのイテレータ
            pub struct NodeIterator<'a, K: Ord, V> {
                /// 根のポインタ
                root: &'a NodePtr<K, V>,
                /// 現在の位置
                pos: NodePosition<K, V>,
            }

            impl<'a, K: Ord, V> NodeIterator<'a, K, V> {
                /// 新しいイテレータを返す
                pub fn new(root: &'a NodePtr<K, V>, node: NodePtr<K, V>) -> Self {
                    NodeIterator {
                        root,
                        pos: NodePosition::Node(node),
                    }
                }

                /// 左端のイテレータを返す
                pub fn first(root: &'a NodePtr<K, V>) -> Self {
                    NodeIterator {
                        root,
                        pos: NodePosition::INF,
                    }
                }

                /// 右端のイテレータを返す
                pub fn last(root: &'a NodePtr<K, V>) -> Self {
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

                    let val = self.pos.as_ref().map(|node| node.clone())??;

                    Some(Some(val))
                }
            }

            impl<'a, K: Ord, V> DoubleEndedIterator for NodeIterator<'a, K, V> {
                fn next_back(&mut self) -> Option<Self::Item> {
                    // posを前に進める
                    self.pos = prev(self.pos.clone(), self.root);

                    let val = self.pos.as_ref().map(|node| node.clone())??;

                    Some(Some(val))
                }
            }

            /// 範囲をもつイテレータ
            pub struct NodeRangeIterator<'a, K: Ord, V> {
                /// 根のポインタ
                root: &'a NodePtr<K, V>,
                /// 左端の位置
                left: NodePosition<K, V>,
                /// 右端の位置
                right: NodePosition<K, V>,
            }

            impl<'a, K: Ord, V> NodeRangeIterator<'a, K, V> {
                /// 新しいイテレータを返す
                pub fn new(
                    root: &'a NodePtr<K, V>,
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

                    let val = self.left.as_ref().map(|node| node.clone())??;

                    Some(Some(val))
                }
            }

            impl<'a, K: Ord, V> DoubleEndedIterator for NodeRangeIterator<'a, K, V> {
                fn next_back(&mut self) -> Option<Self::Item> {
                    // 右端を前に進める
                    self.right = prev(self.right.clone(), self.root);

                    // 右端が左端に到達したら終了
                    if self.right <= self.left {
                        return None;
                    }

                    let val = self.right.as_ref().map(|node| node.clone())??;

                    Some(Some(val))
                }
            }
        }

        pub mod pointer {
            //! ノードのポインタ

            macro_rules! generate_getters {
                // 不変参照用のgetterを生成
                ($name:ident, $field:ident, $return_type:ty) => {
                    fn $name(&self) -> Option<$return_type> {
                        let node_ref = self.as_ref()?.borrow();
                        Some(std::cell::Ref::map(node_ref, |node| &node.$field))
                    }
                };

                // 可変参照用のgetterを生成
                ($name:ident, $field:ident, $return_type:ty, mut) => {
                    fn $name(&mut self) -> Option<$return_type> {
                        let node_mut = self.as_ref()?.borrow_mut();
                        Some(std::cell::RefMut::map(node_mut, |node| &mut node.$field))
                    }
                };
            }

            use std::{
                cell::{Ref, RefCell, RefMut},
                fmt::Debug,
                rc::{Rc, Weak},
            };

            use super::state::NodeState;

            /// ノードの構造体
            pub struct Node<K: Ord, V> {
                pub key: K,
                pub value: V,
                pub parent: Option<Weak<RefCell<Node<K, V>>>>,
                pub left: Option<Rc<RefCell<Node<K, V>>>>,
                pub right: Option<Rc<RefCell<Node<K, V>>>>,
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
                    Some(Rc::new(RefCell::new(Self::new(key, value))))
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

            /// ノードのポインタ
            pub type NodePtr<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

            /// 親ノードのポインタ
            pub type ParentPtr<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

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
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>;

                /// 親のポインタを取得する
                fn get_parent_ptr(&self) -> Self;

                /// 親へのポインタを切り離す
                fn take_parent(&mut self) -> ParentPtr<K, V>;
                /// 親へのポインタを切り離し，強参照を取得する
                fn take_parent_strong(&mut self) -> NodePtr<K, V>;
                /// 左の子へのポインタを切り離す
                fn take_left(&mut self) -> NodePtr<K, V>;
                /// 右の子へのポインタを切り離す
                fn take_right(&mut self) -> NodePtr<K, V>;

                /// 親の参照を取得する
                fn parent(&self) -> Option<Ref<ParentPtr<K, V>>>;
                /// 親の可変参照を取得する
                fn parent_mut(&mut self) -> Option<RefMut<ParentPtr<K, V>>>;
                /// 左の子への参照を取得する
                fn left(&self) -> Option<Ref<NodePtr<K, V>>>;
                /// 左の子への可変参照を取得する
                fn left_mut(&mut self) -> Option<RefMut<NodePtr<K, V>>>;
                /// 右の子への参照を取得する
                fn right(&self) -> Option<Ref<NodePtr<K, V>>>;
                /// 右の子への可変参照を取得する
                fn right_mut(&mut self) -> Option<RefMut<NodePtr<K, V>>>;

                /// キーへの参照を取得する
                fn key(&self) -> Option<Ref<K>>;
                /// バリューへの参照を取得する
                fn value(&self) -> Option<Ref<V>>;
                /// バリューへの可変参照を取得する
                fn value_mut(&mut self) -> Option<RefMut<V>>;

                /// 親ポインタに変換する
                fn to_weak_ptr(&self) -> ParentPtr<K, V>;
            }

            impl<K: Ord, V> NodeOps<K, V> for NodePtr<K, V> {
                fn is_child(&self) -> bool {
                    self.parent().is_some_and(|node| node.is_some())
                }

                fn get_state(&self) -> NodeState {
                    if self.is_none() {
                        return NodeState::Nil;
                    }

                    let par = self.get_parent_ptr();

                    if par.is_none() {
                        return NodeState::Root;
                    }

                    if par.left().is_some_and(|left| left.is_same(self)) {
                        return NodeState::LeftChild;
                    }

                    if par.right().is_some_and(|right| right.is_same(self)) {
                        return NodeState::RightChild;
                    }

                    unreachable!()
                }

                fn is_same(&self, other: &Self) -> bool {
                    self.as_ref()
                        .zip(other.as_ref())
                        .map(|(s, o)| Rc::ptr_eq(s, o))
                        .unwrap_or(false)
                }

                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.key().zip(other.key()).map(|(s, o)| s.cmp(&o))
                }

                fn get_parent_ptr(&self) -> Self {
                    self.parent()?.to_strong_ptr()
                }

                fn take_parent(&mut self) -> ParentPtr<K, V> {
                    self.as_ref()?.borrow_mut().parent.take()
                }

                fn take_parent_strong(&mut self) -> NodePtr<K, V> {
                    self.as_ref()?
                        .borrow_mut()
                        .parent
                        .take()
                        .map(|p| p.upgrade().unwrap())
                }

                fn take_left(&mut self) -> NodePtr<K, V> {
                    let mut left = self.as_ref()?.borrow_mut().left.take();
                    if let Some(mut left_par) = left.parent_mut() {
                        *left_par = None;
                    }
                    left
                }

                fn take_right(&mut self) -> NodePtr<K, V> {
                    let mut right = self.as_ref()?.borrow_mut().right.take();
                    if let Some(mut right_par) = right.parent_mut() {
                        *right_par = None;
                    }
                    right
                }

                fn to_weak_ptr(&self) -> ParentPtr<K, V> {
                    self.as_ref().map(|node| Rc::downgrade(node))
                }

                // 不変参照用のgetterを生成
                generate_getters!(parent, parent, Ref<ParentPtr<K, V>>);
                generate_getters!(left, left, Ref<NodePtr<K, V>>);
                generate_getters!(right, right, Ref<NodePtr<K, V>>);
                generate_getters!(key, key, Ref<K>);
                generate_getters!(value, value, Ref<V>);

                // 可変参照用のgetterを生成
                generate_getters!(parent_mut, parent, RefMut<ParentPtr<K, V>>, mut);
                generate_getters!(left_mut, left, RefMut<NodePtr<K, V>>, mut);
                generate_getters!(right_mut, right, RefMut<NodePtr<K, V>>, mut);
                generate_getters!(value_mut, value, RefMut<V>, mut);
            }

            /// 弱参照の操作
            pub trait ParentOps<K: Ord, V> {
                /// NodePtrへの変換
                fn to_strong_ptr(&self) -> NodePtr<K, V>;
            }

            impl<K: Ord, V> ParentOps<K, V> for ParentPtr<K, V> {
                fn to_strong_ptr(&self) -> NodePtr<K, V> {
                    self.as_ref()?.upgrade()
                }
            }
        }

        pub mod remove {
            use super::{
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
                mut root: NodePtr<K, V>,
                node: NodePtr<K, V>,
            ) -> (NodePtr<K, V>, NodePtr<K, V>) {
                // nodeが存在しない場合
                if node.is_none() {
                    return (root, node);
                }

                // nodeを根に持ってくる
                root = splay(node);

                // 左右に分割
                let mut left = root.take_left();
                let mut right = root.take_right();

                // 右部分木の最小値を取得
                let right_min = get_min(right.clone());

                right = splay(right_min);

                // right.left <- left
                if let Some(mut left_par) = left.parent_mut() {
                    *left_par = right.to_weak_ptr();
                }
                if let Some(mut right_left) = right.left_mut() {
                    *right_left = left;
                } else {
                    return (left, root);
                }

                (right, root)
            }
        }

        pub mod splay {
            use std::rc::Rc;

            use super::{
                pointer::{NodeOps, NodePtr, ParentOps},
                state::NodeState,
            };

            /// nodeを1つ上に持ってくるように回転する
            pub fn rotate<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
                match node.get_state() {
                    NodeState::Nil | NodeState::Root => node,
                    NodeState::LeftChild => {
                        let mut right = node.right_mut()?.take();
                        let par = node.parent()?.clone();

                        // 親の左の子←自分の右の子
                        if let Some(mut right_parent) = right.parent_mut() {
                            *right_parent = par.clone();
                        }
                        let mut par = par.to_strong_ptr();
                        *par.left_mut()? = right;

                        // 自分の親←親の親
                        let par_state = par.get_state();
                        let mut parpar = par.take_parent_strong();

                        match par_state {
                            NodeState::LeftChild => {
                                *parpar.left_mut()? = node.clone();
                            }
                            NodeState::RightChild => {
                                *parpar.right_mut()? = node.clone();
                            }
                            _ => (),
                        }

                        *node.parent_mut()? = parpar.map(|f| Rc::downgrade(&f));

                        // 自分の右の子←親
                        *par.parent_mut()? = node.to_weak_ptr();
                        node.right_mut()?.replace(par?);

                        node
                    }
                    NodeState::RightChild => {
                        let mut left = node.left_mut()?.take();
                        let par = node.parent()?.clone();

                        // 親の右の子←自分の左の子
                        if let Some(mut left_parent) = left.parent_mut() {
                            *left_parent = par.clone();
                        }
                        let mut par = par.to_strong_ptr();
                        *par.right_mut()? = left;

                        // 自分の親←親の親
                        let par_state = par.get_state();
                        let mut parpar = par.take_parent_strong();

                        match par_state {
                            NodeState::LeftChild => {
                                *parpar.left_mut()? = node.clone();
                            }
                            NodeState::RightChild => {
                                *parpar.right_mut()? = node.clone();
                            }
                            _ => (),
                        }

                        *node.parent_mut()? = parpar.map(|f| Rc::downgrade(&f));

                        // 自分の左の子←親
                        *par.parent_mut()? = node.to_weak_ptr();
                        node.left_mut()?.replace(par?);

                        node
                    }
                }
            }

            /// スプレー操作によりnodeを根に移動し，新たな根を返す
            pub fn splay<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
                while node.is_child() {
                    // 頂点の状態
                    let state = node.get_state();
                    // 親頂点の状態
                    let par = node.get_parent_ptr();
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
                            let _par = rotate(node.get_parent_ptr());
                            node = rotate(node);
                        }
                        _ => unreachable!(),
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
                /// ノードが存在しない
                Nil,
                /// 根ノード（親を持たない）
                Root,
                /// 親の左の子
                LeftChild,
                /// 親の右の子
                RightChild,
            }
        }
    }

    pub mod print_util {
        //! 木を整形して表示するための関数

        use std::fmt::Debug;

        use super::node::pointer::NodePtr;

        const BLUE: &str = "\x1b[94m";
        const END: &str = "\x1b[0m";
        const LEFT: &str = " ┌──";
        const MID: &str = " │  ";
        const RIGHT: &str = " └──";
        const NULL: &str = "";
        const BLANK: &str = "    ";

        /// 2分木として出力する
        pub fn print_as_tree<K: Ord + Debug, V: Debug>(root: &NodePtr<K, V>) {
            eprintln!("{BLUE}┌─ BinaryTree ──────────────────────────────────────────{END}");
            fmt_inner_binary_tree(root, &mut vec![], NULL);
            eprintln!("{BLUE}└───────────────────────────────────────────────────────{END}");
        }

        /// print recursive
        fn fmt_inner_binary_tree<K: Ord + Debug, V: Debug>(
            node: &NodePtr<K, V>,
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
                fmt_inner_binary_tree(&node.borrow().left, fill, LEFT);
                // 自分を出力
                eprintln!(
                    "{BLUE}│{END}{} Node {{ key: {:?}, value: {:?} }}",
                    fill.iter().fold(String::new(), |s, x| s + x),
                    node.borrow().key,
                    node.borrow().value,
                );
                // 右の子
                fmt_inner_binary_tree(&node.borrow().right, fill, RIGHT);
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
