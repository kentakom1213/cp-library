# Dynamic Lazy Segment Tree (Implicit) + Binary Search

`src/data_structure/dynamic_segment_tree.rs` と
`src/data_structure/lazy_segment_tree.rs` を統合した設計の検討メモ。
目的は「必要な区間だけノードを確保する」セグ木に対して遅延評価と
セグ木上の二分探索 (`max_right` / `min_left`) を載せること。

## 期待する API
- `new(min, max) -> Self` (扱う index は `[min, max)`).
- `apply(range, f)` (区間作用).
- `prod(range) -> X` (区間積).
- `get(index) -> X` (点取得).
- `all_prod() -> X`.
- `max_right(l, pred) -> (X, I)` / `min_left(r, pred) -> (X, I)`.

## 想定する制約
- Lazy は `ExtMonoid` を使う.
- 二分探索は順序付きの `X` が必要.
  - 追加 trait `OrderedExtMonoid` を定義するか,
    `M::X: Ord` を前提にする.
- `aggregate(f, len)` の `len` は `usize`.
  - `I` が `usize` 以外なら `TryInto<usize>` を要求する
    (もしくは `I = usize` 限定にする).

## データ構造
```
pub struct DynamicLazySegmentTree<I, M>
where
    I: PrimInt,
    M: ExtMonoid,
{
    min_index: I,
    max_index: I,
    n: I,
    root: Option<Box<Node<M>>>,
}

struct Node<M: ExtMonoid> {
    sum: M::X,
    lazy: M::F,
    left: Option<Box<Node<M>>>,
    right: Option<Box<Node<M>>>,
}
```

### 不変条件
- `sum` は `lazy` をすでに適用済み (lazy segment tree の `eval` 後と同等).
- `lazy` は「子に未伝搬の作用」.
- `None` は「全要素が `id_x`・lazy 無し」と見なす.

## 遅延評価の設計
### apply
- 完全被覆なら
  - `node.sum = mapping(node.sum, aggregate(f, len))`
  - `node.lazy = composition(node.lazy, f)`
  - ノードが `None` なら生成してから上記を行う.
- 部分被覆なら
  - 子に降りる前に **push** で `lazy` を子へ伝播 (必要なら子を生成).
  - 子更新後に `sum = op(left.sum, right.sum)` を再計算.

### prod / get (読取り)
- 更新を伴わない探索では **ノード生成はしない** 方針.
- 親の `lazy` を子へ push せずに参照するため,
  再帰引数で「祖先からの遅延 `carry`」を持つ.
- `carry` は **後から適用される作用** と解釈する.
  - `None` ノードの値は `mapping(id_x, aggregate(carry, len))`.
  - 既存ノードの値は `mapping(node.sum, aggregate(carry, len))`.
  - 子へ渡すときは `carry_child = composition(node.lazy, carry)`.

この設計は「更新時にしか push しない」ため,
クエリだけでノードが増えず「必要なところだけ作る」を維持できる.

## セグ木上の二分探索
`dynamic_segment_tree.rs` の `max_right/min_left` を lazy 対応に拡張する.

### 要点
- `prod` と同様に `carry` を受け取って実効値を求める.
- 部分的に降りる前に **push しない** (ノードは作らない).
- `None` ノードは `id_x` と `carry` だけで評価.

### 擬似コード (max_right)
```
fn max_right(node, seg_l, seg_r, ql, pred, acc, carry) -> I {
    if seg_r <= ql { return seg_r; }

    let len = seg_r - seg_l;
    let node_sum = match node {
        None => mapping(id_x(), aggregate(carry, len)),
        Some(t) => mapping(t.sum, aggregate(carry, len)),
    };

    if ql <= seg_l {
        let tmp = op(acc, node_sum);
        if pred(tmp.clone()) {
            *acc = tmp;
            return seg_r;
        }
    }

    if seg_r - seg_l == 1 { return seg_l; }

    let mid = (seg_l + seg_r) / 2;
    let child_carry = match node {
        None => carry,
        Some(t) => composition(t.lazy, carry),
    };

    let left_res = max_right(left, seg_l, mid, ql, pred, acc, child_carry);
    if left_res != mid { return left_res; }
    max_right(right, mid, seg_r, ql, pred, acc, child_carry)
}
```

`min_left` は左右逆順 (acc の合成も `op(node_sum, acc)` 側).

## 実装方針 (まとめ)
1) `DynamicLazySegmentTree<I, M>` を新規作成し,
   `dynamic_segment_tree.rs` の構造と `lazy_segment_tree.rs` の遅延評価を統合.
2) 更新系は push して子を生成, 取得系は carry で遅延を解釈する.
3) 二分探索 (`max_right/min_left`) は carry を使って node_sum を評価し,
   ノード生成なしで走らせる.
4) `OrderedExtMonoid` を導入するか `M::X: Ord` を要求して比較を実装.
5) `len` 変換は `I = usize` を許容するか,
   `TryInto<usize>` を要求して失敗時は panic とする.

## テスト観点
- 区間加算 + 区間和 (AddSum) で `apply/prod/max_right`.
- 区間更新 + 区間最小 (UpdateMin) の `min_left`.
- "クエリのみでノード数が増えない" をサイズで確認する.
