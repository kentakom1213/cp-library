# `indexedset.rs` の整理

`cp-library-rs/src/data_structure/indexedset.rs` を、`Box` ベースの splay tree から `cp-library-rs/src/tree/arena.rs` の `Arena` / `Ptr` を使う treap 実装へ置き換えた。

## 変更点

- ノード管理を `Box<Node<T>>` から `Arena<Node<T>>` に変更した。
- 重複要素を別ノードで持たず、各ノードの `count` に集約する multiset に変更した。
- 部分木サイズを
  - `len`: 重複込み要素数
  - `distinct_len`: 異なるキー数
  の 2 種類で管理するようにした。
- `insert` は置換ではなく「1 個追加」に変更した。戻り値は挿入後のそのキーの個数。
- `remove` はそのキーを 1 個だけ削除する挙動にした。
- `remove_n` と `remove_all` を追加し、複数削除を $`O(log n)`$ で処理できるようにした。
- イテレータと `Debug` 出力は multiset として重複をそのまま並べるようにした。

## 関数名の整理

Rust の一般的な命名に寄せて、曖昧だった名前を以下へ置き換えた。

- `contains_key` -> `contains`
- `lower_bound` -> `first_ge`
- `upper_bound` -> `first_gt`
- `lower_bound_rev` -> `last_le`
- `upper_bound_rev` -> `last_lt`
- `get_by_index` -> `nth`
- `index` -> `position`

補助的に以下も追加した。

- `count(&T) -> usize`
- `remove_n(&T, usize) -> usize`
- `remove_all(&T) -> usize`
- `first() -> Option<&T>`
- `last() -> Option<&T>`
- `distinct_len() -> usize`

## 高速化・改善として入れた内容

- ノード確保をアリーナにまとめ、挿入ごとのヒープ確保とポインタ追跡のオーバーヘッドを削減した。
- 重複要素を `count` に畳み込んだので、同値キーが多いケースで木の高さとノード数が増えにくい。
- 旧実装にあった `T: Clone` 制約を除去した。
- 探索系メソッドは `&mut self` ではなく `&self` で呼べるようにした。splay による再配置が不要になったため。
- `nth` / `position` は部分木サイズから $`O(log n)`$ で計算する。

## 注意点

- `Arena` は解放機構を持たないため、削除されたノードの領域は再利用されない。競プロ用途では通常問題になりにくいが、長寿命ワークロードで大量の挿入削除を繰り返すなら free list 付きの `Arena` 拡張を検討する余地がある。
- ランダム優先度付き treap なので期待計算量は $`O(log n)`$。最悪計算量の保証が必要なら AA tree など別系統の実装を使い分ける。

## 確認

- `cargo fmt`
- `cargo test --test test_indexedset -- --nocapture`
