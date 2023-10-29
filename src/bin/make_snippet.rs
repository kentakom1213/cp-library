use serde::Serialize;
use std::{
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
#[derive(Serialize)]
struct Snippets {
    kyopro: Vec<SnippetPiece>,
}

const SRC: &str = "./src";
const TARGET: &str = "./rust.code-snippet";

fn main() -> Result<(), io::Error> {
    let src = fs::read_dir(SRC)?;

    // スニペット
    let mut snippet = Snippets { kyopro: vec![] };

    // ファイルを列挙
    for entry in src {
        let path = entry.unwrap().path();

        // ディレクトリはskip
        if path.is_dir() {
            continue;
        }

        // ファイルの中身
        let snippet_piece = read_file(&path);

        snippet.kyopro.push(snippet_piece);
    }

    // 書き込み
    let target = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&TARGET)?;
    write!(
        &target,
        "{}",
        &serde_json::to_string_pretty(&snippet).unwrap()
    )?;
    Ok(())
}

/// ファイルを読み込み、不要な部分を削除する
fn read_file(path: &Path) -> SnippetPiece {
    // ファイル名
    let prefix = path.file_name().unwrap().to_str().unwrap().to_string();
    // ファイルの中身
    let content: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|line| line.to_string())
        .collect();

    let mut description = String::new(); // コードの説明
    let mut body = vec![]; // コードの中身

    let mut idx = 0;
    while idx < content.len() && content[idx].starts_with("//!") {
        description += content[idx].as_str().replace("//! ", "").as_str();
        idx += 1;
    }

    // テスト以外の中身を追加
    while idx < content.len() && !content[idx].starts_with("#![cfg(test)]") {
        if content[idx] != "" {
            let line = content[idx]
                .replace("    ", "\t")
                .replace("$", "\\$")
                .replace("\"", "\\\"");
            body.push(line);
        }
        idx += 1;
    }

    SnippetPiece {
        prefix,
        body,
        description,
    }
}
