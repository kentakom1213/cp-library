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
        println!("{}", contents);
    }
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}
