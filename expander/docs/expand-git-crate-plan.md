# Git 由来クレートの自動解決（実装計画）

## 目的
- 入力として「対象ファイル」と「展開するクレート名」を受け取り、対象ファイルの `Cargo.lock` を参照して、
  git 由来の依存クレートであればそのチェックアウトディレクトリを特定し、展開処理に利用できるようにする。

## 想定する挙動
- CLI で `input` と `crate_name` を受け取る。
- `input` から上位ディレクトリを辿って最初に見つかる `Cargo.lock` を使用。
- `Cargo.lock` 内で `crate_name` に一致する package を検索。
- `source` が `git+...` で始まる場合は git 依存とみなし、該当 repo のローカルチェックアウトを特定。
- 特定したディレクトリを `ModuleExpander::new` の `library_path` として扱う。

## 実装ステップ
1. `Cargo.lock` の探索
   - `input` の親からルートに向かって `Cargo.lock` を探索するユーティリティを追加。
   - 見つからない場合はエラーとして明示。

2. `Cargo.lock` パース
   - `cargo-lock` クレートなどで `Cargo.lock` を構造化して読む。
   - `package` から `name == crate_name` のエントリを取得。
   - `source` が `git+` で始まるかを判定。

3. git 依存のローカルディレクトリ特定
   - `source` から URL と rev（`#` 以降）を抽出。
   - `CARGO_HOME`（未設定なら `~/.cargo`）配下の `git/checkouts` を探索。
   - repo 名で候補を絞り、rev が一致するディレクトリを優先して採用。
   - 複数候補や一致なしの場合は診断情報付きでエラー。

4. CLI/設定の拡張
   - `bundler` に `--crate-name` などを追加し、既存の `library` 引数と排他的に扱うか、
     自動解決が優先されるルールを定義。
   - 新機能の使い方を `--help` に反映。

5. 既存の `ModuleExpander` への接続
   - 解決したディレクトリを `ModuleExpander::new` に渡す。
   - 既存の `library_path/src` 仕様を維持し、必要なら git 依存のリポジトリ構成を確認して補正。

6. テスト追加
   - `Cargo.lock` をフィクスチャとして用意し、
     `git+...` の `source` からチェックアウトディレクトリを解決できることを検証。
   - `Cargo.lock` 未検出・crate 未検出・非 git 依存のケースも追加。

## 影響範囲
- `src/bin/bundler.rs`: CLI 引数の追加と解決フローの導入。
- 新規ユーティリティ（例: `src/cargo_lock.rs`）: `Cargo.lock` 探索と解決ロジック。
- `Cargo.toml`: `cargo-lock` など依存追加。

## 未決事項 / 要確認
- git チェックアウトのディレクトリ名推定（Cargo の URL ハッシュ規則）をどこまで厳密に行うか。
- git 依存がワークスペース内でパッチされている場合の取り扱い。
- 既存 CLI の `library` 引数との互換性ルール。
