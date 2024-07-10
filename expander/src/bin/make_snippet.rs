#![allow(non_snake_case)]

use itertools::Itertools;
use serde::Serialize;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

/// スニペットの断片
#[derive(Serialize)]
struct SnippetPiece {
    prefix: String,
    body: Vec<String>,
    description: String,
}

#[derive(Serialize)]
struct Snippets(HashMap<String, SnippetPiece>);

fn main() -> Result<(), io::Error> {
    let ROOT = env::var("CARGO_MANIFEST_DIR").unwrap();
    let SRC: PathBuf = [&ROOT, "..", "cp-library-rs", "src"].iter().collect();
    let TARGET: PathBuf = [&ROOT, "..", "rust.json"].iter().collect();

    eprint!("Making snippet file ... ");

    let src = fs::read_dir(SRC)?;

    // スニペット
    let mut snippet = Snippets(HashMap::new());

    // ファイルを列挙
    for entry in src {
        let path = entry?.path();

        // ディレクトリはskip
        if path.is_dir() {
            continue;
        }
        // lib.rsはskip
        if path.file_name().unwrap().to_str().unwrap() == "lib.rs" {
            continue;
        }
        // ファイルの中身
        let snippet_piece = make_snippet_piece(&path);

        // スニペット名
        let snippet_name = path.file_name().unwrap().to_str().unwrap().to_string();

        snippet.0.insert(snippet_name, snippet_piece?);
    }

    // 書き込み
    let target = fs::File::create(TARGET)?;
    write!(&target, "{}", &serde_json::to_string_pretty(&snippet)?)?;

    eprintln!("Complete!");
    Ok(())
}

/// スニペットの各部分を生成
fn make_snippet_piece(path: &Path) -> Result<SnippetPiece, io::Error> {
    // ファイル名
    let prefix = path.file_stem().unwrap().to_str().unwrap().to_string();
    // コードの説明
    let description = get_snippet_description(path)?;
    // 中身
    let mut body = vec![];
    make_snippet_body(path, &mut body)?;

    Ok(SnippetPiece {
        prefix,
        body,
        description,
    })
}

/// スニペットの概要を取得
fn get_snippet_description(path: &Path) -> Result<String, io::Error> {
    // 先頭の`//!`から始まっている部分を抽出
    Ok(fs::read_to_string(path)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, format!("No such file: {:?}", path)))?
        .split('\n')
        .take_while(|s| s.starts_with("//!"))
        .map(|s| s.replace("//! ", ""))
        .join("\n"))
}

/// ファイルを読み込み、不要な部分を削除する
/// （依存関係を解析して再帰的に生成）
fn make_snippet_body(path: &Path, body: &mut Vec<String>) -> Result<(), io::Error> {
    // 依存関係を解決
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("No such file: {:?}", path),
            ));
        }
    };

    // スニペットの生成
    // pathからstemを取り出す
    let stem = path.file_stem().unwrap().to_str().unwrap();

    // もしすでに追加されていれば，空白を追加
    if !body.is_empty() {
        body.push(String::new());
    }

    // コードの中身（modに切り分け）
    body.push(format!("mod {stem} {{"));

    // 不要なコードをwarnしない
    body.push("\t#![allow(dead_code)]".to_string());

    // 必要部分の置換
    for line in contents.split('\n') {
        if !line.is_empty() {
            let line = format!("\t{}", line.replace("    ", "\t").replace('$', "\\$"));
            body.push(line);
        }
    }
    body.push("}".to_string());

    Ok(())
}
