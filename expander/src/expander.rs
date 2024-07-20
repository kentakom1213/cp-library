use std::{
    collections::BTreeSet,
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use itertools::Itertools;

use crate::{module_path::ModulePath, parser::get_deps};

/// ライブラリの名前
pub const LIBRARY_NAME: &str = "cp-library-rs";

/// インポートされる場合の名前
pub const IMPORT_NAME: &str = "cp_library_rs";

/// タブ文字
const TAB: &str = "    ";

/// ライブラリのパスを取得する
pub fn get_library_path() -> PathBuf {
    let mut buf = PathBuf::from(env::var("KYOPURO_LIBRARY_DIR").unwrap());
    buf.push(LIBRARY_NAME);
    buf
}

#[derive(Debug)]
pub struct ModuleExpander {
    /// 呼び出し元のファイル
    pub entry_file: PathBuf,
    /// 使用しているライブラリのパス（ライブラリのパス以降）
    pub dependancies: Option<BTreeSet<ModulePath>>,
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
        let mut deps = BTreeSet::new();

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
    fn dfs(
        dep: &ModulePath,
        lib_path: &Path,
        deps: &mut BTreeSet<ModulePath>,
    ) -> Result<(), io::Error> {
        // パスの生成
        let p = dep.to_pathbuf(lib_path.to_path_buf());

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

        // // 各モジュールを展開
        // for dep in self.dependancies.as_ref().unwrap() {
        //     let dep_str = self.get_module(dep)?;
        //     file.write_all(dep_str.as_bytes())?;
        // }

        // file.write_all("}".as_bytes())?;

        // カテゴリごとに展開
        let mut prev_category = "";

        for dep in self.dependancies.as_ref().unwrap() {
            prev_category = match dep {
                ModulePath::Macro { .. } => {
                    if !prev_category.is_empty() {
                        writeln!(file, "{}}}", TAB.repeat(1))?;
                    }
                    file.write_all(self.get_module(dep, 1)?.as_bytes())?;
                    ""
                }
                ModulePath::Module { category, .. } => {
                    if category == prev_category {
                        file.write_all(self.get_module(dep, 2)?.as_bytes())?;
                    } else {
                        if !prev_category.is_empty() {
                            writeln!(file, "{}}}", TAB.repeat(1))?;
                        }
                        writeln!(file, "{}pub mod {category} {{", TAB.repeat(1))?;
                        file.write_all(self.get_module(dep, 2)?.as_bytes())?;
                    }
                    &category
                }
            };
        }

        if !prev_category.is_empty() {
            writeln!(file, "{}}}", TAB.repeat(1))?;
        }
        file.write_all("}".as_bytes())?;

        Ok(())
    }

    /// ファイルをモジュールに出力
    pub fn get_module(&self, dep: &ModulePath, indent: usize) -> Result<String, io::Error> {
        let p = dep.to_pathbuf(self.lib_path.to_path_buf());
        let mut file = fs::read_to_string(p)?;

        // "crate" -> "crate::${IMPORT_NAME}"
        file = file.replace("crate", &format!("crate::{IMPORT_NAME}"));

        let mut res = format!("{}pub mod {dep} {{\n", TAB.repeat(indent));

        // 各行を追加
        for line in file.lines().filter(|l| !l.is_empty()) {
            res += &TAB.repeat(indent + 1);
            res += line;
            res += "\n";
        }

        res += &format!("{}}}\n", TAB.repeat(indent));

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
}
