# cp-library

競技プログラミング用のライブラリ

ドキュメント：https://kentakom1213.github.io/cp-library/cp_library_rs/

## 使い方

`.git/hooks/pre-commit`に以下を記述

```sh
#!/bin/zsh

# snippetの修正
cargo run --bin make_snippet --manifest-path expander/Cargo.toml || exit 1

# ステージング
git add rust.json
```
