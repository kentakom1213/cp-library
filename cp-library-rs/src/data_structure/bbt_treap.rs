//! implicit treap
//!
//! 列を管理するデータ構造

use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Bound::*, RangeBounds},
};

use rand::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    algebraic_structure::actedmonoid::ActedMonoid,
    tree::arena::{Arena, ArenaNode, Ptr},
    utils::show_binary_tree::ShowBinaryTree,
};

type A<K, M> = Arena<TreapNode<K, M>>;

// ========== node ==========

/// Treap のノード
pub struct TreapNode<K: Ord, M: ActedMonoid> {
    /// キー
    key: K,
    /// 値
    val: M::Val,
    /// 集約値
    sum: M::Val,
    /// 作用
    act: M::Act,
    /// ヒープの重み値
    prio: u32,
    /// 部分木のサイズ
    size: usize,
    // ポインタ
    left: Option<Ptr>,
    right: Option<Ptr>,
}

impl<K: Ord, M: ActedMonoid> TreapNode<K, M> {
    /// val と prio からノードを生成
    pub fn new(key: K, val: M::Val, prio: u32) -> Self {
        Self {
            key,
            sum: val.clone(),
            val,
            act: M::id(),
            prio,
            size: 1,
            left: None,
            right: None,
        }
    }
}

impl<K: Ord, M: ActedMonoid> ArenaNode for TreapNode<K, M> {}

// ========== binary search ==========

/// 境界の種類
#[derive(Clone, Copy)]
enum BoundKind {
    /// lowerbound: (key < x, key >= x)
    Lower,
    /// upperbound: (key <= x, key > x)
    Upper,
}

// ========== balanced binary tree ==========

/// 平衡二分木
pub struct BalancedBinaryTree<K: Ord, M: ActedMonoid> {
    arena: Arena<TreapNode<K, M>>,
    root: Option<Ptr>,
    rng: XorShiftRng,
}

impl<K: Ord, M: ActedMonoid> Default for BalancedBinaryTree<K, M> {
    fn default() -> Self {
        Self {
            arena: A::new(),
            root: None,
            rng: XorShiftRng::from_os_rng(),
        }
    }
}

impl<K: Ord + Clone, M: ActedMonoid> BalancedBinaryTree<K, M> {
    /// key, val を挿入する（lowerbound 版）
    /// - 既存の同一 key は右側（>=）に送るので，新要素は「同一 key の先頭」に入る
    pub fn insert_lower(&mut self, key: K, val: M::Val) {
        let prio = self.rng.next_u32();

        // (key < k, key >= k)
        let (l, r) = self.split_key_lower(self.root, &key);

        let ptr = self.arena.alloc(TreapNode::new(key, val, prio));
        let mid = self.merge(l, Some(ptr));
        self.root = self.merge(mid, r);
    }

    /// key, val を挿入する（upperbound 版）
    /// - 既存の同一 key は左側（<=）に送るので，新要素は「同一 key の末尾」に入る
    pub fn insert_upper(&mut self, key: K, val: M::Val) {
        let prio = self.rng.next_u32();

        // (key <= k, key > k)
        let (l, r) = self.split_key_upper(self.root, &key);

        let ptr = self.arena.alloc(TreapNode::new(key, val, prio));
        let mid = self.merge(l, Some(ptr));
        self.root = self.merge(mid, r);
    }

    /// key がまだ存在しないときだけ (key, val) を挿入する
    /// - 返り値: 挿入できたら true, 既に存在していたら false
    pub fn insert_unique(&mut self, key: K, val: M::Val) -> bool {
        let prio = self.rng.next_u32();

        // A: key < k,  BC: key >= k
        let (a, bc) = self.split_key_lower(self.root, &key);

        // B: key == k, C: key > k
        //
        // bc の中は全て key >= k なので，
        // upperbound 分割 (<=k, >k) をすると B が key==k の塊になる．
        let (b, c) = self.split_key_upper(bc, &key);

        if b.is_some() {
            // 既に存在するので挿入しない：元に戻す
            let bc2 = self.merge(b, c);
            self.root = self.merge(a, bc2);
            false
        } else {
            // 存在しないので挿入：A + new + C にして戻す
            let ptr = self.arena.alloc(TreapNode::new(key, val, prio));
            let ac = self.merge(a, Some(ptr));
            self.root = self.merge(ac, c);
            true
        }
    }

    /// key をもつ要素を削除する（存在すれば全て削除）
    /// - 返り値: 1 個以上削除できたら true
    pub fn remove(&mut self, key: &K) -> bool {
        // A: key < k, BC: key >= k
        let (a, bc) = self.split_key_lower(self.root, key);

        // B: key == k, C: key > k
        let (b, c) = self.split_key_upper(bc, key);

        let removed = b.is_some();

        // B を捨てて A + C に戻す
        self.root = self.merge(a, c);

        removed
    }

    /// 区間（key の範囲）の集約値を返す
    pub fn get_range<R: RangeBounds<K> + Debug>(&mut self, range: R) -> M::Val {
        // まず左境界で split: A | BC
        let (a, bc) = match range.start_bound() {
            Unbounded => (None, self.root),
            Included(l) => self.split_key_lower(self.root, l), // key < l | key >= l
            Excluded(l) => self.split_key_upper(self.root, l), // key <= l | key > l なので左が <=l
        };

        // 次に右境界で split: B | C （B が範囲内）
        let (b, c) = match range.end_bound() {
            Unbounded => (bc, None),
            Included(r) => {
                // 欲しいのは key <= r を左に残す（= upperbound）
                self.split_key_upper(bc, r) // key <= r | key > r
            }
            Excluded(r) => {
                // 欲しいのは key < r を左に残す（= lowerbound）
                self.split_key_lower(bc, r) // key < r | key >= r
            }
        };

        let ans = b
            .map(|ptr| self.arena.get(ptr).sum.clone())
            .unwrap_or(M::e());

        // 戻す
        let bc2 = self.merge(b, c);
        self.root = self.merge(a, bc2);

        ans
    }

    /// 区間（key の範囲）に作用を適用する
    pub fn apply<R: RangeBounds<K> + Debug>(&mut self, range: R, act: M::Act) {
        let (a, bc) = match range.start_bound() {
            Unbounded => (None, self.root),
            Included(l) => self.split_key_lower(self.root, l), // key < l | key >= l
            Excluded(l) => self.split_key_upper(self.root, l), // key <= l | key > l
        };

        let (b, c) = match range.end_bound() {
            Unbounded => (bc, None),
            Included(r) => self.split_key_upper(bc, r), // key <= r | key > r
            Excluded(r) => self.split_key_lower(bc, r), // key < r | key >= r
        };

        if let Some(ptr) = b {
            self.apply_lazy(ptr, &act);
        }

        let bc2 = self.merge(b, c);
        self.root = self.merge(a, bc2);
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

    /// 部分木のサイズ
    #[inline]
    fn size_of(&self, ptr: Option<Ptr>) -> usize {
        ptr.map(|ptr| self.arena.get(ptr).size).unwrap_or(0)
    }

    /// 部分木の集約
    #[inline]
    fn sum_of(&self, ptr: Option<Ptr>) -> M::Val {
        ptr.map(|ptr| self.arena.get(ptr).sum.clone())
            .unwrap_or(M::e())
    }

    /// ノード `ptr` が表す部分木全体に作用を適用
    #[inline]
    fn apply_lazy(&mut self, ptr: Ptr, act: &M::Act) {
        let (nval, nsum) = {
            let v = self.arena.get(ptr);
            (M::mapping(&v.val, act), M::mapping(&v.sum, act))
        };
        let composed = {
            let v = self.arena.get(ptr);
            M::compose(&v.act, act)
        };

        let v = self.arena.get_mut(ptr);
        v.val = nval;
        v.sum = nsum;
        v.act = composed;
    }

    /// 子の情報を吸い上げる
    #[inline]
    fn pull(&mut self, ptr: Ptr) {
        let (l, r, val) = {
            let v = self.arena.get(ptr);
            (v.left, v.right, v.val.clone())
        };

        let lsize = self.size_of(l);
        let rsize = self.size_of(r);

        let lsum = self.sum_of(l);
        let rsum = self.sum_of(r);

        // 集約: L + [val] + R
        let sum = M::op(&M::op(&lsum, &val), &rsum);

        let v = self.arena.get_mut(ptr);
        v.size = lsize + rsize + 1;
        v.sum = sum;
    }

    /// 子に伝播する
    #[inline]
    fn push(&mut self, ptr: Ptr) {
        let (act, l, r) = {
            let v = self.arena.get(ptr);
            (v.act.clone(), v.left, v.right)
        };

        // 作用の伝播
        if act != M::id() {
            if let Some(lp) = l {
                self.apply_lazy(lp, &act);
            }
            if let Some(rp) = r {
                self.apply_lazy(rp, &act);
            }
            self.arena.get_mut(ptr).act = M::id();
        }
    }

    /// 共通実装
    /// - Lower:  L = {key < x},  R = {key >= x}
    /// - Upper:  L = {key <= x}, R = {key > x}
    fn split_key_impl(
        &mut self,
        ptr: Option<Ptr>,
        x: &K,
        kind: BoundKind,
    ) -> (Option<Ptr>, Option<Ptr>)
    where
        K: Clone,
    {
        let Some(ptr) = ptr else {
            return (None, None);
        };

        self.push(ptr);

        // 借用衝突を避けるため，必要情報をコピーしてから再帰する
        let (key, left, right) = {
            let v = self.arena.get(ptr);
            (v.key.clone(), v.left, v.right)
        };

        // 左側に残るべきか？（true なら ptr は L 側，false なら R 側）
        let go_left = match (key.cmp(x), kind) {
            (Ordering::Less, _) => true,
            (Ordering::Equal, BoundKind::Lower) => false, // key == x は右へ
            (Ordering::Equal, BoundKind::Upper) => true,  // key == x は左へ
            (Ordering::Greater, _) => false,
        };

        if go_left {
            // ptr は L 側に残す：右部分木を分割して，左片を ptr.right に戻す
            let (l, r) = self.split_key_impl(right, x, kind);
            self.arena.get_mut(ptr).right = l;
            self.pull(ptr);
            (Some(ptr), r)
        } else {
            // ptr は R 側に残す：左部分木を分割して，右片を ptr.left に戻す
            let (l, r) = self.split_key_impl(left, x, kind);
            self.arena.get_mut(ptr).left = r;
            self.pull(ptr);
            (l, Some(ptr))
        }
    }

    /// lowerbound 版： (key < x, key >= x)
    fn split_key_lower(&mut self, ptr: Option<Ptr>, x: &K) -> (Option<Ptr>, Option<Ptr>)
    where
        K: Clone,
    {
        self.split_key_impl(ptr, x, BoundKind::Lower)
    }

    /// upperbound 版： (key <= x, key > x)
    fn split_key_upper(&mut self, ptr: Option<Ptr>, x: &K) -> (Option<Ptr>, Option<Ptr>)
    where
        K: Clone,
    {
        self.split_key_impl(ptr, x, BoundKind::Upper)
    }

    /// ptr を根とする木を，左から n 番目までのノードとそれ以外のノードに分解する
    fn split_nth(&mut self, ptr: Option<Ptr>, n: usize) -> (Option<Ptr>, Option<Ptr>) {
        let Some(ptr) = ptr else {
            return (None, None);
        };

        self.push(ptr);

        let node = self.arena.get(ptr);
        let lsize = self.size_of(node.left);

        if n <= lsize {
            let (l, r) = self.split_nth(node.left, n);
            self.arena.get_mut(ptr).left = r;
            self.pull(ptr);

            (l, Some(ptr))
        } else {
            let (l, r) = self.split_nth(node.right, n - lsize - 1);
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
                    self.push(left);
                    let lr = self.arena.get(left).right;
                    self.arena.get_mut(left).right = self.merge(lr, Some(right));
                    self.pull(left);

                    Some(left)
                } else {
                    // right を根にする
                    self.push(right);
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

impl<K, M> ShowBinaryTree<Ptr> for BalancedBinaryTree<K, M>
where
    K: Ord + Debug,
    M: ActedMonoid,
    M::Val: Debug,
    M::Act: Debug,
{
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
        format!(
            "[key:{:?}, val:{:?}, sum:{:?}, act:{:?}]",
            self.arena.get(*ptr).key,
            self.arena.get(*ptr).val,
            self.arena.get(*ptr).sum,
            self.arena.get(*ptr).act,
        )
    }
}
