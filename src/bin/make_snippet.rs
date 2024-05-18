use itertools::Itertools;
use regex::Regex;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::Path,
};

/// スニペットの断片
#[derive(Serialize)]
struct SnippetPiece {
    prefix: String,
    body: Vec<String>,
    description: String,
}

/// スニペット
// #[derive(Serialize)]
// struct Snippets {
//     kyopro: Vec<SnippetPiece>,
// }

#[derive(Serialize)]
struct Snippets(HashMap<String, SnippetPiece>);

const SRC: &str = "./src";
const TARGET: &str = "./rust.code-snippets";

fn main() -> Result<(), io::Error> {
    eprint!("Making snippet file ... ");

    let src = fs::read_dir(SRC)?;

    // スニペット
    let mut snippet = Snippets(HashMap::new());

    // ファイルを列挙
    for entry in src {
        let path = entry.unwrap().path();

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

        snippet.0.insert(snippet_name, snippet_piece);
    }

    // 書き込み
    let target = fs::File::create(TARGET)?;
    write!(
        &target,
        "{}",
        &serde_json::to_string_pretty(&snippet).unwrap()
    )?;

    eprintln!("Complete!");
    Ok(())
}

/// スニペットの各部分を生成
fn make_snippet_piece(path: &Path) -> SnippetPiece {
    // ファイル名
    let prefix = path.file_stem().unwrap().to_str().unwrap().to_string();
    // コードの説明
    let description = get_snippet_description(path);
    // 中身
    let mut body = vec![];
    make_snippet_body(path, &mut body);

    SnippetPiece {
        prefix,
        body,
        description,
    }
}

/// スニペットの概要を取得
fn get_snippet_description(path: &Path) -> String {
    // 先頭の`//!`から始まっている部分を抽出
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .take_while(|s| s.starts_with("//!"))
        .map(|s| s.replace("//! ", ""))
        .join("\n")
}

/// ファイルを読み込み、不要な部分を削除する
/// （依存関係を解析して再帰的に生成）
fn make_snippet_body(path: &Path, body: &mut Vec<String>) {
    // 依存関係を解決
    let contents = fs::read_to_string(path).unwrap();

    // 参照されているクレートを列挙
    let re = Regex::new(r"crate::(?<stem>.*?)::").unwrap();
    for c in re.captures_iter(&contents) {
        // 参照されているパス
        let mut ref_path = String::new();
        c.expand("./src/$stem.rs", &mut ref_path);
        // 再帰呼び出し
        let ref_path = Path::new(&ref_path);
        make_snippet_body(ref_path, body);
    }

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
}
