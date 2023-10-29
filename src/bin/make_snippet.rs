use std::{fs, path::Path};

pub type FileTree = Vec<(usize, String)>;

const SRC: &str = "./src";

fn main() {
    let files = fs::read_dir(SRC).unwrap();

    // ファイルを列挙
    for entry in files {
        let path = entry.unwrap().path();

        // ディレクトリはskip
        if path.is_dir() {
            continue;
        }

        let contents = read_file(&path);
        println!("{:?}\n{}\n", &path, contents.join("\n"));
    }
}

/// ファイルを読み込み、不要な部分を削除する
fn read_file(path: &Path) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .filter(|&line| !line.is_empty() && !line.starts_with("//!")) // ファイルのdoc-commentは無視
        .take_while(|&line| !line.starts_with("#[cfg(test)]")) // テストは無視
        .map(|line| line.replace("    ", "\\t")) // タブ文字に置換
        .map(|line| line.to_string())
        .collect()
}
