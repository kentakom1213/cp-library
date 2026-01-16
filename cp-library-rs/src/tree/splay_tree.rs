//! スプレー木の実装

pub mod find {
    #![allow(clippy::type_complexity)]

    use super::{
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
        let mut last = root;
        let mut res = None;
        while let Some(last_inner) = last {
            if cmp(last_inner.key()) {
                res = Some(last_inner);
                last = match last_inner.left().as_ref().copied() {
                    Some(node) => Some(node),
                    None => break,
                };
            } else {
                last = match last_inner.right().as_ref().copied() {
                    Some(node) => Some(node),
                    None => break,
                };
            }
        }
        if let Some(res_inner) = res {
            (Some(splay(res_inner)), Some(res_inner))
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
            (new_root, NodePosition::Sup)
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
            (new_root, NodePosition::Sup)
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
        let (new_root, lb) = find_min(root, |k| k >= x);
        if lb.as_ref().is_some_and(|k| k.key() == x) {
            (new_root, lb)
        } else {
            (new_root, None)
        }
    }
}
pub mod insert {
    use super::pointer::{Node, NodeOps, NodePtr};
    use std::cmp::Ordering;
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
    pub fn insert<K: Ord, V>(root: Option<NodePtr<K, V>>, key: K, value: V) -> NodePtr<K, V> {
        if root.is_none() {
            let new_root = Node::node_ptr(key, value);
            return new_root;
        }
        // 親ノードをたどっていく
        let mut par = root;
        loop {
            let comp = key.cmp(par.as_ref().unwrap().key());
            match comp {
                Ordering::Less => {
                    if let Some(Some(left)) = par.as_ref().map(|node| *node.left()) {
                        par.replace(left);
                    } else {
                        // 左側に挿入
                        break insert_left(par, key, value);
                    }
                }
                Ordering::Equal => {
                    // 置き換える
                    break par.unwrap();
                }
                Ordering::Greater => {
                    if let Some(Some(right)) = par.as_ref().map(|node| *node.right()) {
                        par.replace(right);
                    } else {
                        // 右側に挿入
                        break insert_right(par, key, value);
                    }
                }
            }
        }
    }
    /// nodeの左側に子を追加し，追加された子のポインタを返す
    pub fn insert_left<K: Ord, V>(node: Option<NodePtr<K, V>>, key: K, value: V) -> NodePtr<K, V> {
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
        inner.left_mut().replace(new_node);
        new_node
    }
    /// nodeの右側に子を追加し，追加された子のポインタを返す
    pub fn insert_right<K: Ord, V>(node: Option<NodePtr<K, V>>, key: K, value: V) -> NodePtr<K, V> {
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
        inner.right_mut().replace(new_node);
        new_node
    }
}
pub mod iterator {
    use super::{
        pointer::{NodeOps, NodePtr},
        state::NodeState,
    };
    use std::{cmp, fmt::Debug};
    /// ノードの位置
    #[derive(Debug)]
    pub enum NodePosition<K: Ord, V> {
        /// `K` の下界
        Inf,
        /// ノードの値
        Node(NodePtr<K, V>),
        /// `K` の上界
        Sup,
    }
    impl<K: Ord, V> Clone for NodePosition<K, V> {
        fn clone(&self) -> Self {
            match self {
                NodePosition::Inf => NodePosition::Inf,
                NodePosition::Node(node) => NodePosition::Node(*node),
                NodePosition::Sup => NodePosition::Sup,
            }
        }
    }
    impl<K: Ord, V> PartialEq for NodePosition<K, V> {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (NodePosition::Inf, NodePosition::Inf) => true,
                (NodePosition::Sup, NodePosition::Sup) => true,
                (NodePosition::Node(node1), NodePosition::Node(node2)) => node1.is_same(node2),
                _ => false,
            }
        }
    }
    impl<K: Ord, V> PartialOrd for NodePosition<K, V> {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            match (self, other) {
                (NodePosition::Inf, NodePosition::Inf) => Some(cmp::Ordering::Equal),
                (NodePosition::Sup, NodePosition::Sup) => Some(cmp::Ordering::Equal),
                (NodePosition::Node(node1), NodePosition::Node(node2)) => {
                    Some(node1.key_cmp(node2))
                }
                (NodePosition::Inf, _) => Some(cmp::Ordering::Less),
                (NodePosition::Sup, _) => Some(cmp::Ordering::Greater),
                (_, NodePosition::Inf) => Some(cmp::Ordering::Greater),
                (_, NodePosition::Sup) => Some(cmp::Ordering::Less),
            }
        }
    }
    impl<K: Ord, V> NodePosition<K, V> {
        pub fn is_inf(&self) -> bool {
            matches!(self, NodePosition::Inf)
        }
        pub fn is_sup(&self) -> bool {
            matches!(self, NodePosition::Sup)
        }
        pub fn is_node(&self) -> bool {
            matches!(self, NodePosition::Node(_))
        }
        pub fn is_none(&self) -> bool {
            matches!(self, NodePosition::Inf | NodePosition::Sup)
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
            NodePosition::Inf => NodePosition::Inf,
            NodePosition::Node(mut node) => {
                if let Some(mut prv) = node.left().as_ref().copied() {
                    while let Some(right) = prv.clone().right().as_ref().copied() {
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
                NodePosition::Inf
            }
            NodePosition::Sup => match get_max(*root) {
                Some(node) => NodePosition::Node(node),
                None => NodePosition::Sup,
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
            NodePosition::Inf => match get_min(*root) {
                Some(node) => NodePosition::Node(node),
                None => NodePosition::Inf,
            },
            NodePosition::Node(mut node) => {
                if let Some(mut nxt) = node.right().as_ref().copied() {
                    while let Some(left) = nxt.clone().left().as_ref().copied() {
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
                NodePosition::Sup
            }
            NodePosition::Sup => NodePosition::Sup,
        }
    }
    /// rootを根とする木のうち，最も左側の子を返す
    pub fn get_min<K: Ord, V>(root: Option<NodePtr<K, V>>) -> Option<NodePtr<K, V>> {
        let mut node = root;
        while let left @ Some(_) = node.as_ref().map(|node| *node.left())? {
            node = left;
        }
        node
    }
    /// rootを根とする木のうち，最も右側の子を返す
    pub fn get_max<K: Ord, V>(root: Option<NodePtr<K, V>>) -> Option<NodePtr<K, V>> {
        let mut node = root;
        while let right @ Some(_) = node.as_ref().map(|node| *node.right())? {
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
                pos: NodePosition::Inf,
            }
        }
        /// 右端のイテレータを返す
        pub fn last(root: &'a Option<NodePtr<K, V>>) -> Self {
            NodeIterator {
                root,
                pos: NodePosition::Sup,
            }
        }
    }
    impl<K: Ord, V> Iterator for NodeIterator<'_, K, V> {
        type Item = NodePtr<K, V>;
        fn next(&mut self) -> Option<Self::Item> {
            // posを次に進める
            self.pos = next(self.pos.clone(), self.root);
            let val = self.pos.as_ref().copied()?;
            Some(val)
        }
    }
    impl<K: Ord, V> DoubleEndedIterator for NodeIterator<'_, K, V> {
        fn next_back(&mut self) -> Option<Self::Item> {
            // posを前に進める
            self.pos = prev(self.pos.clone(), self.root);
            let val = self.pos.as_ref().copied()?;
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
    impl<K: Ord, V> Iterator for NodeRangeIterator<'_, K, V> {
        type Item = NodePtr<K, V>;
        fn next(&mut self) -> Option<Self::Item> {
            // 左端を次に進める
            self.left = next(self.left.clone(), self.root);
            // 左端が右端に到達したら終了
            if self.left >= self.right {
                return None;
            }
            let val = self.left.as_ref().copied()?;
            Some(val)
        }
    }
    impl<K: Ord + Debug, V: Debug> DoubleEndedIterator for NodeRangeIterator<'_, K, V> {
        fn next_back(&mut self) -> Option<Self::Item> {
            // 右端を前に進める
            self.right = prev(self.right.clone(), self.root);
            // 右端が左端に到達したら終了
            if self.right <= self.left {
                return None;
            }
            let val = self.right.as_ref().copied()?;
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
            NonNull::new(Box::into_raw(ptr)).unwrap_or_else(|| panic!("Failed to allocate memory"))
        }
    }
    impl<K: Ord + Debug, V: Debug> Debug for Node<K, V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match (&self.left, &self.right) {
                (None, None) => f
                    .debug_struct("Node")
                    .field("key", &self.key)
                    .field("value", &self.value)
                    .finish(),
                (Some(_), None) => f
                    .debug_struct("Node")
                    .field("key", &self.key)
                    .field("value", &self.value)
                    .field("left", &self.left)
                    .finish(),
                (None, Some(_)) => f
                    .debug_struct("Node")
                    .field("key", &self.key)
                    .field("value", &self.value)
                    .field("right", &self.right)
                    .finish(),
                (Some(_), Some(_)) => f
                    .debug_struct("Node")
                    .field("key", &self.key)
                    .field("value", &self.value)
                    .field("left", &self.left)
                    .field("right", &self.right)
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
            NonNull::eq(self, other)
        }
        fn key_cmp(&self, other: &Self) -> std::cmp::Ordering {
            unsafe { self.as_ref().key.cmp(&other.as_ref().key) }
        }
        fn take_parent(&mut self) -> Option<NodePtr<K, V>> {
            unsafe { self.as_mut().parent.take() }
        }
        fn take_parent_strong(&mut self) -> Option<NodePtr<K, V>> {
            unsafe { self.as_mut().parent.take() }
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
    pub fn remove<K: Ord, V>(mut node: NodePtr<K, V>) -> (Option<NodePtr<K, V>>, NodePtr<K, V>) {
        // nodeを根に持ってくる
        let root = splay(node);
        // 左右に分割
        let left = node.take_left();
        let mut right = node.take_right();
        // 右部分木の最小値を取得
        let right_min = get_min(right);
        if let Some(right_min_inner) = right_min {
            right = Some(splay(right_min_inner));
        }
        // right.left <- left
        if let Some(mut left_inner) = left {
            *left_inner.parent_mut() = right;
        }
        if let Some(mut right_inner) = right {
            *right_inner.left_mut() = left;
        } else {
            return (left, root);
        }
        (right, root)
    }
}
pub mod splay {
    use super::{
        pointer::{NodeOps, NodePtr},
        state::NodeState,
    };
    /// nodeを1つ上に持ってくるように回転する
    pub fn rotate<K: Ord, V>(mut node: NodePtr<K, V>) -> NodePtr<K, V> {
        match node.get_state() {
            NodeState::Root => node,
            NodeState::LeftChild => {
                let mut right = node.take_right();
                let par = *node.parent();
                // 自分の右の子の親←親
                if let Some(right_inner) = right.as_mut() {
                    *right_inner.parent_mut() = par;
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
                            *parpar_inner.left_mut() = Some(node);
                        }
                        NodeState::RightChild => {
                            *parpar_inner.right_mut() = Some(node);
                        }
                        _ => (),
                    }
                }
                *node.parent_mut() = *parpar;
                // 自分の右の子←親
                *par_inner.parent_mut() = Some(node);
                node.right_mut().replace(par_inner);
                node
            }
            NodeState::RightChild => {
                let mut left = node.take_left();
                let par = *node.parent();
                // 自分の左の子の親←親
                if let Some(left_inner) = left.as_mut() {
                    *left_inner.parent_mut() = par;
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
                            *parpar_inner.left_mut() = Some(node);
                        }
                        NodeState::RightChild => {
                            *parpar_inner.right_mut() = Some(node);
                        }
                        _ => (),
                    }
                }
                *node.parent_mut() = *parpar;
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
