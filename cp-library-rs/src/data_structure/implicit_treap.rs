//! implicit treap
//!
//! 列を管理するデータ構造

use std::{
    fmt::Debug,
    ops::{Bound::*, RangeBounds},
};

use rand::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;

use crate::{
    algebraic_structure::actedmonoid::ActedMonoid,
    tree::arena::{Arena, ArenaNode, Ptr},
    tree::show_binary_tree::ShowBinaryTree,
};

type A<M> = Arena<TreapNode<M>>;

// ========== node ==========

/// Treap のノード
pub struct TreapNode<M: ActedMonoid> {
    /// 値
    val: M::Val,
    /// 集約値（正順）
    sum: M::Val,
    /// 集約値（逆順）
    rsum: M::Val,
    /// 作用
    act: M::Act,
    /// 反転フラグ
    rev: bool,
    /// ヒープの重み値
    prio: u32,
    /// 部分木のサイズ
    size: usize,
    // ポインタ
    left: Option<Ptr>,
    right: Option<Ptr>,
}

impl<M: ActedMonoid> TreapNode<M> {
    /// val と prio からノードを生成
    pub fn new(val: M::Val, prio: u32) -> Self {
        Self {
            sum: val.clone(),
            rsum: val.clone(),
            val,
            act: M::id(),
            rev: false,
            prio,
            size: 1,
            left: None,
            right: None,
        }
    }
}

impl<M: ActedMonoid> ArenaNode for TreapNode<M> {}

// ========== implicit treap ==========

/// Implicit Treap
///
/// - 動的な列を管理するデータ構造
pub struct ImplicitTreap<M: ActedMonoid> {
    arena: Arena<TreapNode<M>>,
    pub root: Option<Ptr>,
    rng: XorShiftRng,
}

impl<M: ActedMonoid> Default for ImplicitTreap<M> {
    fn default() -> Self {
        Self {
            arena: A::new(),
            root: None,
            rng: XorShiftRng::from_os_rng(),
        }
    }
}

impl<M: ActedMonoid> ImplicitTreap<M> {
    /// i 番目の要素の値を取得する（clone）
    pub fn get(&mut self, i: usize) -> M::Val {
        assert!(i < self.len());

        let (a, bc) = self.split_nth(self.root, i);
        let (b, c) = self.split_nth(bc, 1);

        let val = match b {
            Some(ptr) => {
                // b は 1 要素の木だが，念のため遅延を落としてから読む
                self.push(ptr);
                self.arena.get(ptr).val.clone()
            }
            None => unreachable!(),
        };

        let bc = self.merge(b, c);
        self.root = self.merge(a, bc);

        val
    }

    /// i 番目に要素 val を挿入する
    /// - `self.len() <= i` の場合，末尾に追加する．
    pub fn insert(&mut self, i: usize, val: M::Val) {
        let prio = self.rng.next_u32();
        let (l, r) = self.split_nth(self.root, i.min(self.len()));
        let ptr = self.arena.alloc(TreapNode::new(val, prio));
        let mid = self.merge(l, Some(ptr));
        self.root = self.merge(mid, r);
    }

    /// 末尾に要素 val を挿入する
    pub fn push_back(&mut self, val: M::Val) {
        self.insert(self.len(), val);
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

    /// 区間の集約値を返す
    pub fn get_range<R: RangeBounds<usize> + Debug>(&mut self, range: R) -> M::Val {
        let (l, r) = self.parse_range(range);

        let (a, bc) = self.split_nth(self.root, l);
        let (b, c) = self.split_nth(bc, r - l);

        let ans = b
            .map(|ptr| self.arena.get(ptr).sum.clone())
            .unwrap_or(M::e());

        let bc = self.merge(b, c);
        self.root = self.merge(a, bc);
        ans
    }

    /// 区間に作用を適用する
    pub fn apply<R: RangeBounds<usize> + Debug>(&mut self, range: R, act: M::Act) {
        let (l, r) = self.parse_range(range);

        let (a, bc) = self.split_nth(self.root, l);
        let (b, c) = self.split_nth(bc, r - l);

        if let Some(ptr) = b {
            self.apply_lazy(ptr, &act);
        }

        let bc = self.merge(b, c);
        self.root = self.merge(a, bc);
    }

    /// 区間反転
    pub fn reverse<R: RangeBounds<usize> + Debug>(&mut self, range: R) {
        let (l, r) = self.parse_range(range);

        let (a, bc) = self.split_nth(self.root, l);
        let (b, c) = self.split_nth(bc, r - l);

        if let Some(ptr) = b {
            self.apply_rev(ptr);
        }

        let bc = self.merge(b, c);
        self.root = self.merge(a, bc);
    }

    /// 空であるか判定する
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 要素数をカウントする
    pub fn len(&self) -> usize {
        self.root.map(|p| self.arena.get(p).size).unwrap_or(0)
    }

    // ========== split / merge ==========

    /// ptr を根とする木を，左から n 番目までのノードとそれ以外のノードに分解する
    pub fn split_nth(&mut self, ptr: Option<Ptr>, n: usize) -> (Option<Ptr>, Option<Ptr>) {
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
    pub fn merge(&mut self, left: Option<Ptr>, right: Option<Ptr>) -> Option<Ptr> {
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

    // ========== internal ==========

    /// 区間の取得
    #[inline]
    fn parse_range<R: RangeBounds<usize> + Debug>(&self, range: R) -> (usize, usize) {
        let start = match range.start_bound() {
            Unbounded => 0,
            Excluded(&v) => v + 1,
            Included(&v) => v,
        };
        let end = match range.end_bound() {
            Unbounded => self.len(),
            Excluded(&v) => v,
            Included(&v) => v + 1,
        };
        if start <= end && end <= self.len() {
            (start, end)
        } else {
            panic!("The given range is wrong: {:?}", range)
        }
    }

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

    // 逆順集約
    #[inline]
    fn rsum_of(&self, ptr: Option<Ptr>) -> M::Val {
        ptr.map(|ptr| self.arena.get(ptr).rsum.clone())
            .unwrap_or(M::e())
    }

    /// ノード `ptr` が表す部分木全体に作用を適用
    #[inline]
    fn apply_lazy(&mut self, ptr: Ptr, act: &M::Act) {
        let (nval, nsum, nrsum) = {
            let v = self.arena.get(ptr);
            (
                M::mapping(&v.val, act),
                M::mapping(&v.sum, act),
                M::mapping(&v.rsum, act),
            )
        };
        let composed = {
            let v = self.arena.get(ptr);
            M::compose(&v.act, act)
        };

        let v = self.arena.get_mut(ptr);
        v.val = nval;
        v.sum = nsum;
        v.rsum = nrsum;
        v.act = composed;
    }

    /// ノード `ptr` が表す部分木全体を反転（遅延）
    #[inline]
    fn apply_rev(&mut self, ptr: Ptr) {
        let (l, r, sum, rsum) = {
            let v = self.arena.get(ptr);
            (v.left, v.right, v.sum.clone(), v.rsum.clone())
        };
        let v = self.arena.get_mut(ptr);
        v.left = r;
        v.right = l;
        v.sum = rsum;
        v.rsum = sum;
        v.rev = !v.rev;
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

        let lrsum = self.rsum_of(l);
        let rrsum = self.rsum_of(r);

        // 正順: L + [val] + R
        let sum = M::op(&M::op(&lsum, &val), &rsum);
        // 逆順: reverse(R) + [val] + reverse(L)
        let rev_sum = M::op(&M::op(&rrsum, &val), &lrsum);

        let v = self.arena.get_mut(ptr);
        v.size = lsize + rsize + 1;
        v.sum = sum;
        v.rsum = rev_sum;
    }

    /// 子に伝播する（act と rev の両方）
    #[inline]
    fn push(&mut self, ptr: Ptr) {
        let (act, rev, l, r) = {
            let v = self.arena.get(ptr);
            (v.act.clone(), v.rev, v.left, v.right)
        };

        // 反転の伝播
        if rev {
            if let Some(lp) = l {
                self.apply_rev(lp);
            }
            if let Some(rp) = r {
                self.apply_rev(rp);
            }
            self.arena.get_mut(ptr).rev = false;
        }

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
}

// ========== binary search (like segtree max_right / min_left) ==========

impl<M: ActedMonoid> ImplicitTreap<M> {
    /// 左端固定二分探索（segtree の max_right と同じ）
    ///
    /// 返り値：(`get_range(l..x)`, `x`)
    /// 条件：`f(M::e()) == true`
    pub fn max_right<F>(&mut self, l: usize, f: F) -> (M::Val, usize)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(l <= self.len());
        assert!(f(M::e()));

        // [0, l) と [l, n) に分割
        let (a, b) = self.split_nth(self.root, l);

        let mut acc = M::e();
        let take = self.max_right_inner(b, &f, &mut acc);

        // 元に戻す
        self.root = self.merge(a, b);

        (acc, l + take)
    }

    /// `ptr` が表す列の prefix を，左から何個取れるか
    /// 返り値：取れる要素数
    fn max_right_inner<F>(&mut self, ptr: Option<Ptr>, f: &F, acc: &mut M::Val) -> usize
    where
        F: Fn(M::Val) -> bool,
    {
        let Some(p) = ptr else {
            return 0;
        };

        self.push(p);

        let (l, r, val) = {
            let v = self.arena.get(p);
            (v.left, v.right, v.val.clone())
        };

        let lsize = self.size_of(l);
        let lsum = self.sum_of(l);

        // まず左部分木を丸ごと入れられるか？
        let tmp = M::op(acc, &lsum);
        if !f(tmp.clone()) {
            // 左部分木の中で探す
            return self.max_right_inner(l, f, acc);
        }

        // 左部分木は全て入れられる
        *acc = tmp;

        // 次に現在ノード val を入れられるか？
        let tmp2 = M::op(acc, &val);
        if !f(tmp2.clone()) {
            // val は入れられないので，取れるのは左部分木まで
            return lsize;
        }

        // val も入れられる
        *acc = tmp2;

        // 右部分木へ
        lsize + 1 + self.max_right_inner(r, f, acc)
    }

    /// 右端固定二分探索（segtree の min_left と同じ）
    ///
    /// 返り値：(`get_range(x..r)`, `x`)
    /// 条件：`f(M::e()) == true`
    pub fn min_left<F>(&mut self, r: usize, f: F) -> (M::Val, usize)
    where
        F: Fn(M::Val) -> bool,
    {
        assert!(r <= self.len());
        assert!(f(M::e()));

        // [0, r) と [r, n) に分割
        let (a, b) = self.split_nth(self.root, r);

        let mut acc = M::e();
        let take = self.min_left_inner(a, &f, &mut acc);

        // 元に戻す
        self.root = self.merge(a, b);

        (acc, r - take)
    }

    /// `ptr` が表す列の suffix を，右から何個取れるか
    /// 返り値：取れる要素数
    fn min_left_inner<F>(&mut self, ptr: Option<Ptr>, f: &F, acc: &mut M::Val) -> usize
    where
        F: Fn(M::Val) -> bool,
    {
        let Some(p) = ptr else {
            return 0;
        };

        self.push(p);

        let (l, r, val) = {
            let v = self.arena.get(p);
            (v.left, v.right, v.val.clone())
        };

        let rsize = self.size_of(r);
        let rsum = self.sum_of(r);

        // まず右部分木を丸ごと「前に付け足して」良いか？
        // suffix を伸ばすので，新規部分は常に acc の前に来る：op(new, acc)
        let tmp = M::op(&rsum, acc);
        if !f(tmp.clone()) {
            // 右部分木の中で探す
            return self.min_left_inner(r, f, acc);
        }

        // 右部分木は全て入れられる
        *acc = tmp;

        // 次に現在ノード val を前に付け足せるか？
        let tmp2 = M::op(&val, acc);
        if !f(tmp2.clone()) {
            // val は入れられないので，取れるのは右部分木まで
            return rsize;
        }

        // val も入れられる
        *acc = tmp2;

        // 左部分木へ（さらに左に伸ばす）
        rsize + 1 + self.min_left_inner(l, f, acc)
    }
}

// ========== debug ==========

impl<M> ShowBinaryTree<Ptr> for ImplicitTreap<M>
where
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
            "[val:{:?}, sum:{:?}, act:{:?}, rev:{:?}]",
            self.arena.get(*ptr).val,
            self.arena.get(*ptr).sum,
            self.arena.get(*ptr).act,
            self.arena.get(*ptr).rev,
        )
    }
}
