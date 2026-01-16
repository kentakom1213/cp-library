//! implicit treap

use std::fmt::Debug;

use rand::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    tree::arena::{Arena, ArenaNode, Ptr},
    utils::show_binary_tree::ShowBinaryTree,
};

type A<T> = Arena<TreapNode<T>>;

// ========== node ==========
/// Treap のノード
pub struct TreapNode<T> {
    value: T,
    /// ヒープの重み値
    prio: u32,
    /// 部分木のサイズ
    size: usize,
    // ポインタ
    left: Option<Ptr>,
    right: Option<Ptr>,
}

impl<T> TreapNode<T> {
    /// value と prio からノードを生成
    pub fn new(value: T, prio: u32) -> Self {
        Self {
            value,
            prio,
            size: 1,
            left: None,
            right: None,
        }
    }
}

impl<T> ArenaNode for TreapNode<T> {}

// ========== implicit treap ==========

/// Implicit Treap
///
/// - 動的な列を管理するデータ構造
pub struct ImplicitTreap<T> {
    arena: Arena<TreapNode<T>>,
    root: Option<Ptr>,
    rng: XorShiftRng,
}

impl<T> Default for ImplicitTreap<T> {
    fn default() -> Self {
        Self {
            arena: A::new(),
            root: None,
            rng: XorShiftRng::from_os_rng(),
        }
    }
}

impl<T> ImplicitTreap<T> {
    /// i 番目の要素の可変参照を取得する
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if self.len() <= i {
            return None;
        }
        let (l, r) = self.split_nth(self.root, i);
        let (mid, r) = self.split_nth(r, 1);

        let new_l = self.merge(l, mid);
        self.root = self.merge(new_l, r);

        mid.map(|ptr| &mut self.arena.get_mut(ptr).value)
    }

    /// i 番目の要素の不変参照を取得する
    pub fn get(&mut self, i: usize) -> Option<&T> {
        self.get_mut(i).map(|t| &*t)
    }

    /// i 番目に要素 value を挿入する
    /// - `self.len() <= i` の場合，末尾に追加する．
    pub fn insert(&mut self, i: usize, value: T) {
        let prio = self.rng.next_u32();
        let (l, r) = self.split_nth(self.root, i.min(self.len()));
        let ptr = self.arena.alloc(TreapNode::new(value, prio));
        let mid = self.merge(l, Some(ptr));
        self.root = self.merge(mid, r);
    }

    /// 末尾に要素 value を挿入する
    pub fn push_back(&mut self, value: T) {
        self.insert(self.len(), value);
    }

    /// i 番目の要素を削除する
    /// - `self.len() <= i` の場合，None を返す．
    pub fn remove(&mut self, i: usize) {
        if self.len() <= i {
            return;
        }
        let (l, r) = self.split_nth(self.root, i);
        let (_, r) = self.split_nth(r, 1);
        self.root = self.merge(l, r);
    }

    /// 空であるか判定する
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 要素数をカウントする
    pub fn len(&self) -> usize {
        self.root.map(|p| self.arena.get(p).size).unwrap_or(0)
    }

    // ========== internal ==========

    /// 子の情報を吸い上げる
    #[inline]
    fn pull(&mut self, ptr: Ptr) {
        let lsize = self
            .arena
            .get(ptr)
            .left
            .map(|ptr| self.arena.get(ptr).size)
            .unwrap_or(0);
        let rsize = self
            .arena
            .get(ptr)
            .right
            .map(|ptr| self.arena.get(ptr).size)
            .unwrap_or(0);
        self.arena.get_mut(ptr).size = lsize + rsize + 1;
    }

    /// ptr を根とする木を，左から n 番目までのノードとそれ以外のノードに分解する
    fn split_nth(&mut self, ptr: Option<Ptr>, n: usize) -> (Option<Ptr>, Option<Ptr>) {
        let Some(ptr) = ptr else {
            return (None, None);
        };

        let lsize = self
            .arena
            .get(ptr)
            .left
            .map(|p| self.arena.get(p).size)
            .unwrap_or(0);

        if n <= lsize {
            let lch = self.arena.get(ptr).left;
            let (l, r) = self.split_nth(lch, n);
            self.arena.get_mut(ptr).left = r;
            self.pull(ptr);

            (l, Some(ptr))
        } else {
            let rch = self.arena.get(ptr).right;
            let (l, r) = self.split_nth(rch, n - lsize - 1);
            self.arena.get_mut(ptr).right = l;
            self.pull(ptr);

            (Some(ptr), r)
        }
    }

    /// left, right を根とする木を併合する
    fn merge(&mut self, left: Option<Ptr>, right: Option<Ptr>) -> Option<Ptr> {
        match (left, right) {
            (None, ptr) | (ptr, None) => ptr,
            (Some(left), Some(right)) => {
                let pl = self.arena.get(left).prio;
                let pr = self.arena.get(right).prio;

                if pl < pr {
                    // left を根にする
                    let lr = self.arena.get(left).right;
                    self.arena.get_mut(left).right = self.merge(lr, Some(right));
                    self.pull(left);

                    Some(left)
                } else {
                    // right を根にする
                    let rl = self.arena.get(right).left;
                    self.arena.get_mut(right).left = self.merge(Some(left), rl);
                    self.pull(right);

                    Some(right)
                }
            }
        }
    }
}

// ========== debug ==========

impl<T: Debug> Debug for TreapNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreapNode")
            .field("value", &self.value)
            .field("size", &self.size)
            .field("prio", &self.prio)
            .finish()
    }
}

impl<T: Debug> ShowBinaryTree<Ptr> for ImplicitTreap<T> {
    fn get_root(&self) -> Option<Ptr> {
        self.root
    }
    fn get_left(&self, ptr: &Ptr) -> Option<Ptr> {
        self.arena.get(*ptr).left
    }
    fn get_right(&self, ptr: &Ptr) -> Option<Ptr> {
        self.arena.get(*ptr).right
    }
    fn print_node(&self, ptr: &Ptr) -> String {
        format!("{:?}", self.arena.get(*ptr))
    }
}
