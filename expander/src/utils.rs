use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::{self, Error, ErrorKind, Write},
    path::{Path, PathBuf},
};

use itertools::Itertools;
use syn::{visit::Visit, UseTree};

const LIBRARY_NAME: &str = "cp-library-rs";
const IMPORT_NAME: &str = "cp_library_rs";

/// ライブラリのパスを取得する
pub fn get_library_path() -> PathBuf {
    let mut buf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    buf.pop();
    buf.push(LIBRARY_NAME);
    buf
}

/// use文の解析器
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
            deps_to_vec(&path.tree, paths, current_path);
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

/// ファイルの文字列から，クレート内での依存関係を抜き出す
pub fn get_deps(source: &str, target: &str) -> HashSet<String> {
    // 構文木を構築
    let syntax_tree = syn::parse_file(source).expect("Unable to parse file");
    // 解析
    let mut visitor = UseVisitor { uses: vec![] };
    visitor.visit_file(&syntax_tree);

    // 内部の依存関係を調べる
    let mut local_deps = HashSet::new();

    for p in &visitor.uses {
        if let [dep, file, ..] = &p[..] {
            if dep == target {
                local_deps.insert(file.clone());
            }
        }
    }

    local_deps
}

#[derive(Debug)]
pub struct ModuleExpander {
    /// 呼び出し元のファイル
    pub entry_file: PathBuf,
    /// 使用しているライブラリのパス（ライブラリのパス以降）
    pub dependancies: Option<HashSet<String>>,
    /// ライブラリのパス
    lib_path: PathBuf,
}

impl ModuleExpander {
    /// パーサーの初期化
    pub fn new(entry_file: PathBuf, mut lib_path: PathBuf) -> Self {
        lib_path.push("src");

        ModuleExpander {
            entry_file,
            dependancies: None,
            lib_path,
        }
    }

    /// 再帰的に依存関係を解析する
    /// 結果を`self.dependancies`に保存する
    pub fn solve_dependancies(&mut self) -> Result<(), io::Error> {
        let mut deps = HashSet::new();

        // entry_fileを解析
        let source = match fs::read_to_string(&self.entry_file) {
            Ok(source) => source,
            Err(err) => return Err(err),
        };

        for dep in get_deps(&source, IMPORT_NAME) {
            deps.insert(dep.clone());
            Self::dfs(&dep, &self.lib_path, &mut deps)?;
        }

        self.dependancies = Some(deps);

        Ok(())
    }

    /// 再帰的に依存関係を解析し，depsに追加する
    fn dfs(dep: &str, lib_path: &Path, deps: &mut HashSet<String>) -> Result<(), io::Error> {
        // パスの生成
        let p = Self::make_path(dep, lib_path);

        let source = match fs::read_to_string(p) {
            Ok(source) => source,
            Err(err) => return Err(err),
        };

        for dep in get_deps(&source, "crate") {
            if deps.contains(&dep) {
                continue;
            }
            deps.insert(dep.clone());
            // 再帰的に探索
            Self::dfs(&dep, lib_path, deps)?;
        }

        Ok(())
    }

    /// 依存関係を展開する
    pub fn expand(&mut self) -> Result<(), io::Error> {
        // 依存関係の解析
        if self.dependancies.is_none() {
            self.solve_dependancies()?;
        }

        // 依存していない場合
        if self.dependancies.as_ref().unwrap().is_empty() {
            log::info!("non dependancies");
            return Ok(());
        }

        // 元ファイルを編集
        let mut contents = fs::read_to_string(&self.entry_file)?;
        let mut file = File::create(&self.entry_file)?;

        // "${IMPORT_NAME}" -> "crate"
        contents = contents.replace(IMPORT_NAME, &format!("crate::{IMPORT_NAME}"));

        // 書き換えた内容を書き込み
        file.write_all(contents.as_bytes())?;

        write!(
            file,
            "
// ==================== {} ====================
mod {} {{
    #![allow(dead_code)]
",
            LIBRARY_NAME, IMPORT_NAME
        )?;

        // 各モジュールを展開
        for dep in self.dependancies.as_ref().unwrap() {
            let dep_str = self.get_module(dep)?;
            file.write_all(dep_str.as_bytes())?;
        }

        file.write_all("}".as_bytes())?;

        Ok(())
    }

    /// ファイルをモジュールに出力
    pub fn get_module(&self, dep: &str) -> Result<String, io::Error> {
        let p = Self::make_path(dep, &self.lib_path);
        let mut file = fs::read_to_string(p)?;

        // "crate" -> "crate::${IMPORT_NAME}"
        file = file.replace("crate", &format!("crate::{IMPORT_NAME}"));

        let mut res = format!("    pub mod {dep} {{\n");

        // 各行を追加
        for line in file.lines().filter(|l| !l.is_empty()) {
            res += "        ";
            res += line;
            res += "\n";
        }

        res += "    }\n";

        Ok(res)
    }

    /// 復元する
    pub fn restore(&self) -> Result<(), io::Error> {
        // 現在のファイルの内容
        let mut file = fs::read_to_string(&self.entry_file)?;

        // "crate::${IMPORT_NAME}" -> "${IMPORT_NAME}"
        file = file.replace(&format!("crate::{IMPORT_NAME}"), IMPORT_NAME);

        // ライブラリの位置
        let library_begin = format!(
            "// ==================== {} ====================",
            LIBRARY_NAME
        );

        // library以降を削除
        file = file.lines().take_while(|&l| l != &library_begin).join("\n");

        // 元のファイルに書き出し
        fs::write(&self.entry_file, file)?;

        Ok(())
    }

    /// ファイル名からパスを構成する
    fn make_path(dep: &str, lib_path: &Path) -> PathBuf {
        let mut p = PathBuf::from(&lib_path);
        p.push(format!("{}.rs", dep));
        p
    }
}
