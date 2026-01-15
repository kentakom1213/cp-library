# Lazy Segment Tree: 二分探索の調査メモ

`src/data_structure/lazy_segment_tree.rs` に `max_right / min_left` を追加するための
調査・方針まとめ。

## 前提と制約
- 既存の `LazySegmentTree<M: ExtMonoid>` は配列型の遅延セグ木 (1-indexed).
- `data[idx]` は **ノード区間の集約値** を保持し、`lazy[idx]` は未伝搬の作用。
  - `apply_inner` が `data` を常に更新するため、**ノード自身の `data` は正しい**。
  - ただし `lazy` が残っている場合、**子の `data` は古い** ため、
    子へ降りるときは `eval` で伝搬が必要。
- `max_right/min_left` の条件は通常の monoid 二分探索:
  - `f(id_x) = true`
  - ある位置で false になったら、その先も false になる (単調性)。
- `OrderedMonoid` は不要。`ExtMonoid` だけで十分。
  (既存 `segment_tree.rs` は `OrderedMonoid` だが、実際は `f` の単調性しか使っていない)

## 実装の基本方針
### 1) eval/push の使い方
二分探索で子へ降りる前に必ず `eval(idx, len)` を呼び、
`lazy[idx]` を子へ伝搬して **子の `data` を正しくする**。

### 2) ループ構造
`segment_tree.rs` の 2 段階探索を踏襲できる。
違いは「ノードの値を使う前に eval を挟む」こと。

### 3) 追加の補助関数
`eval` は既存のものを使い回せる。
二分探索用に `push` を切り出す必要はないが、
読みやすくするなら `eval` を使った `push` 風ヘルパを追加しても良い。

## max_right 実装案 (反復型)
`segment_tree.rs` をほぼ移植して、`eval` を足す形。
```
pub fn max_right<F>(&mut self, mut l: usize, f: F) -> (M::X, usize)
where
    F: Fn(M::X) -> bool,
{
    assert!(f(M::id_x()));
    if l >= self.size { return (M::id_x(), self.size); }

    l += self.offset;
    let mut sum = M::id_x();

    // 第1段階: 条件を満たさない区間を探す
    loop {
        while l & 1 == 0 { l >>= 1; }
        self.eval(l, self.offset >> l.trailing_zeros());
        let tmp = M::op(&sum, &self.data[l]);
        if !f(tmp.clone()) { break; }
        sum = tmp;
        l += 1;
        if (l & l.wrapping_neg()) == l { return (sum, self.size); }
    }

    // 第2段階: 子へ降りながら探索
    while l < self.offset {
        l <<= 1;
        self.eval(l, self.offset >> l.trailing_zeros());
        let tmp = M::op(&sum, &self.data[l]);
        if f(tmp.clone()) {
            sum = tmp;
            l += 1;
        }
    }

    (sum, l - self.offset)
}
```

### 注意点
- `eval` には区間長が必要。`len = end - begin` が必要なので、
  `idx` から長さを逆算する手段が必要。
  - `len = self.offset >> depth` を使う方法 (idx のビット長から depth を計算)。
  - もしくは `max_right_inner(idx, l, r, ...)` の再帰で区間長を渡す。
- Rust では `idx.trailing_zeros()` だけでは深さを表せないので、
  `leading_zeros()` で高さを求めるなど工夫が必要。
  再帰の方が実装が単純。

## min_left 実装案 (反復型)
`max_right` と左右対称。
`eval` を挟みつつ `tmp = op(data[r], sum)` の順序を守る。

## 代替: 再帰型 (おすすめ)
`dynamic_segment_tree.rs` に近い実装で、
`max_right_inner(idx, seg_l, seg_r, ql, f, acc)` とする。
この場合 `len` を明示的に持てるので `eval` が簡単。

### 再帰のポイント
- `eval(idx, seg_r - seg_l)` を呼んでから分岐。
- 区間が丸ごと入るなら `op(acc, data[idx])` を試す。
- 子へ降りる際に `eval` で子の `data` を正しい状態にする。

## まとめ (実装指針)
1) `max_right/min_left` を `LazySegmentTree` に追加。
2) 子へ降りる直前に `eval` を必ず実行。
3) 区間長を安全に渡すため、再帰実装が無難。
4) `M: ExtMonoid` だけで実装できる (OrderedMonoid は不要)。

## テスト観点
- `ExtMonoid::examples::AddSum` で
  - 区間加算後の `max_right` が期待位置になるか。
- 既存 `get` と `max_right` の整合性:
  `max_right(l, f)` の返す `x` が
  `f(get(l..x)) = true` かつ `f(get(l..x+1)) = false` を満たす。
