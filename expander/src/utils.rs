use std::{
    collections::HashSet,
    env,
    path::{Path, PathBuf},
};

use syn::{visit::Visit, UseTree};

/// ライブラリのパスを取得する
pub fn get_library_path() -> PathBuf {
    let mut cur = env::current_dir().unwrap();
    cur.pop();
    cur.push("cp-library-rs");
    cur
}

/// use文のパーサー
pub struct UseVisitor {
    pub uses: Vec<Vec<String>>,
}

impl<'ast> Visit<'ast> for UseVisitor {
    fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
        let use_tree = &item_use.tree;
        let mut paths = Vec::new();
        deps_to_vec(use_tree, &mut paths, Vec::new());
        self.uses.extend(paths);
        // Continue visiting the rest of the file
        syn::visit::visit_item_use(self, item_use);
    }
}

fn deps_to_vec(tree: &UseTree, paths: &mut Vec<Vec<String>>, mut current_path: Vec<String>) {
    match tree {
        UseTree::Path(path) => {
            current_path.push(path.ident.to_string());
            deps_to_vec(&*path.tree, paths, current_path);
        }
        UseTree::Name(name) => {
            current_path.push(name.ident.to_string());
            paths.push(current_path);
        }
        UseTree::Rename(rename) => {
            current_path.push(rename.ident.to_string());
            paths.push(current_path);
        }
        UseTree::Glob(_) => {
            current_path.push("*".to_string());
            paths.push(current_path);
        }
        UseTree::Group(group) => {
            for item in &group.items {
                deps_to_vec(item, paths, current_path.clone());
            }
        }
    }
}

pub struct ModuleParser {
    /// 呼び出し元のファイル
    pub entry_file: PathBuf,
    /// 使用しているライブラリのパス（ライブラリのパス以降）
    pub dependancies: Option<HashSet<String>>,
    /// ライブラリのパス
    lib_path: PathBuf,
}

impl ModuleParser {
    /// パーサーの初期化
    pub fn new(entry_file: PathBuf) -> Self {
        ModuleParser {
            entry_file,
            dependancies: None,
            lib_path: get_library_path(),
        }
    }

    /// 再帰的に依存関係を解析する
    /// 結果を`self.dependancies`に保存する
    pub fn solve_dependancies(&mut self) {}

    pub fn dfs() {}
}
