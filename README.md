# cp-library

競技プログラミング用のライブラリ

## cp-library-rs

Rust 用のライブラリ

## expander

以下のコマンドでインストールします．

```sh
cargo install --git https://github.com/kentakom1213/cp-library expander
```

`expander` は，指定した Rust ファイル中で利用しているライブラリを 1 ファイルに展開する CLI です．
ライブラリの場所は `--library` で直接指定するか，`--crate-name` で `Cargo.lock` から解決します．

### 使い方

```sh
$ expander [OPTIONS] <INPUT>
```

- `<INPUT>`: 展開対象の Rust ファイル
- `-l, --library <LIBRARY>`: ライブラリのルートディレクトリを直接指定
- `--crate-name <CRATE_NAME>`: `Cargo.lock` から git 依存 crate のパスを解決
- `-i, --inplace`: 標準出力ではなく入力ファイルを直接書き換える
- `-r, --restore`: 展開済みファイルを元に戻す

### 展開

```sh
$ expander path/to/main.rs --library /path/to/cp-library-rs
```

標準出力に展開結果を出します．ファイルを直接更新したい場合は `-i` を付けます．

```sh
$ expander path/to/main.rs --library /path/to/cp-library-rs -i
```

`Cargo.lock` にある git 依存からライブラリパスを解決する場合は `--crate-name` を使います．

```sh
$ expander path/to/main.rs --crate-name cp-library-rs -i
```

### 復元

```sh
$ expander path/to/main.rs --library /path/to/cp-library-rs --restore
```

## cp-library-py

Python 用のライブラリ
