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
const TARGET: &str = "./rust.code-snippets";

fn main() -> Result<(), io::Error> {
    eprint!("Making snippet file ... ");

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
        // lib.rsはskip
        if path.file_name().unwrap().to_str().unwrap() == "lib.rs" {
            continue;
        }

        // ファイルの中身
        let snippet_piece = read_file(&path);

        snippet.kyopro.push(snippet_piece);
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

/// ファイルを読み込み、不要な部分を削除する
fn read_file(path: &Path) -> SnippetPiece {
    // ファイル名
    let prefix = path.file_name().unwrap().to_str().unwrap().to_string();

    // コードの説明
    let mut description = String::new();
    // コードの中身（modに切り分け）
    let mut body = vec![format!("mod {} {{", &prefix[..prefix.len() - 3])];

    for line in fs::read_to_string(path).unwrap().split('\n') {
        if line.starts_with("//!") {
            description += line.replace("//! ", "").as_str();
        }
        if !line.is_empty() {
            let line = format!("\t{}", line.replace("    ", "\t").replace('$', "\\$"));
            body.push(line);
        }
    }

    body.push("}".to_string());

    SnippetPiece {
        prefix,
        body,
        description,
    }
}
