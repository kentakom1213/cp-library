use std::{
    collections::BTreeSet,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use anyhow::bail;
use itertools::Itertools;

use crate::{module_path::ModulePath, parser::get_deps};

/// タブ文字
const TAB: &str = "    ";

#[derive(Debug)]
pub struct ModuleExpander {
    /// 呼び出し元のファイル
    pub entry_file: PathBuf,
    /// 使用しているライブラリのパス（ライブラリのパス以降）
    pub dependancies: Option<BTreeSet<ModulePath>>,
    /// ライブラリのパス
    library_path: PathBuf,
    /// インポート時の名前
    pub import_name: String,
    /// ライブラリ自体の名前
    pub library_name: String,
}

impl ModuleExpander {
    /// パーサーの初期化
    pub fn new(entry_file: PathBuf, mut library_path: PathBuf) -> anyhow::Result<Self, String> {
        // ライブラリ名
        let Some(library_name) = library_path
            .file_name()
            .map(|s| s.to_str().unwrap().to_string())
        else {
            return Err("No such library: {library_path:?}".to_string());
        };

        // インポート時の名前
        let import_name = library_name.replace('-', "_");

        library_path.push("src");

        Ok(ModuleExpander {
            entry_file,
            dependancies: None,
            library_path,
            import_name,
            library_name,
        })
    }

    /// 再帰的に依存関係を解析する
    /// 結果を`self.dependancies`に保存する
    pub fn solve_dependancies(&mut self) -> anyhow::Result<(), anyhow::Error> {
        let mut deps = BTreeSet::new();

        // entry_fileを解析
        let source = match fs::read_to_string(&self.entry_file) {
            Ok(contents) => contents,
            Err(err) => bail!("{}: {:?}", err, self.entry_file),
        };

        for dep in get_deps(&source, &self.import_name) {
            deps.insert(dep.clone());
            Self::dfs(&dep, &self.library_path, &mut deps)?;
        }

        self.dependancies = Some(deps);

        Ok(())
    }

    /// 再帰的に依存関係を解析し，depsに追加する
    fn dfs(
        dep: &ModulePath,
        library_path: &Path,
        deps: &mut BTreeSet<ModulePath>,
    ) -> anyhow::Result<(), anyhow::Error> {
        // パスの生成
        let p = dep.to_pathbuf(library_path.to_path_buf());

        let source = match fs::read_to_string(&p) {
            Ok(source) => source,
            Err(err) => bail!("{}: {:?}", err, p),
        };

        for dep in get_deps(&source, "crate") {
            if deps.contains(&dep) {
                continue;
            }
            deps.insert(dep.clone());
            // 再帰的に探索
            Self::dfs(&dep, library_path, deps)?;
        }

        Ok(())
    }

    /// 依存関係を展開する（文字列の形で返す）
    pub fn expand(&mut self) -> anyhow::Result<String, anyhow::Error> {
        // 依存関係の解析
        if self.dependancies.is_none() {
            self.solve_dependancies()?;
        }

        // 元ファイル読み取り
        let mut contents = match fs::read_to_string(&self.entry_file) {
            Ok(contents) => contents,
            Err(err) => bail!("{}: {:?}", err, self.entry_file),
        };

        // 依存していない場合
        if self.dependancies.as_ref().unwrap().is_empty() {
            log::info!("non dependancies");
            return Ok(contents);
        }

        // "${IMPORT_NAME}" -> "crate"
        contents = contents.replace(&self.import_name, &format!("crate::{}", self.import_name));

        write!(
            &mut contents,
            "
// ==================== {} ====================
pub mod {} {{
    #![allow(dead_code)]
",
            self.library_name, self.import_name
        )
        .unwrap();

        // カテゴリごとに展開
        let mut prev_category = "";

        for dep in self.dependancies.as_ref().unwrap() {
            prev_category = match dep {
                ModulePath::Macro { .. } => {
                    if !prev_category.is_empty() {
                        writeln!(&mut contents, "{TAB}}}").unwrap();
                    }
                    contents += &self.get_module(dep, 1)?;
                    ""
                }
                ModulePath::Module { category, .. } => {
                    if category == prev_category {
                        contents += &self.get_module(dep, 2)?;
                    } else {
                        if !prev_category.is_empty() {
                            writeln!(&mut contents, "{TAB}}}").unwrap();
                        }
                        writeln!(&mut contents, "{TAB}pub mod {category} {{").unwrap();
                        contents += &self.get_module(dep, 2)?;
                    }
                    category
                }
            };
        }

        if !prev_category.is_empty() {
            writeln!(&mut contents, "{TAB}}}").unwrap();
        }
        contents += "}";

        Ok(contents)
    }

    /// ファイルをモジュールに出力
    pub fn get_module(
        &self,
        dep: &ModulePath,
        indent: usize,
    ) -> anyhow::Result<String, anyhow::Error> {
        let p = dep.to_pathbuf(self.library_path.to_path_buf());
        let mut file = match fs::read_to_string(&p) {
            Ok(contents) => contents,
            Err(err) => bail!("{}: {:?}", err, p),
        };

        // "crate" -> "crate::${IMPORT_NAME}"
        file = file.replace("crate", &format!("crate::{}", self.import_name));

        let mut res = format!("{}pub mod {dep} {{\n", TAB.repeat(indent));

        // 各行を追加
        for line in file.lines().filter(|l| !l.is_empty()) {
            res += &TAB.repeat(indent + 1);
            res += line;
            res += "\n";
        }

        writeln!(&mut res, "{}}}", TAB.repeat(indent)).unwrap();

        Ok(res)
    }

    /// 依存関係をファイルに展開する
    pub fn expand_inplace(&mut self) -> anyhow::Result<(), anyhow::Error> {
        let contents = self.expand()?;

        // ファイルに書き込み
        match fs::write(&self.entry_file, contents) {
            Ok(_) => {}
            Err(err) => bail!("{}: {:?}", err, self.entry_file),
        }

        Ok(())
    }

    /// 復元する
    pub fn restore(&self) -> anyhow::Result<(), anyhow::Error> {
        // 現在のファイルの内容
        let mut file = match fs::read_to_string(&self.entry_file) {
            Ok(contents) => contents,
            Err(err) => bail!("{}: {:?}", err, self.entry_file),
        };

        // "crate::${IMPORT_NAME}" -> "${IMPORT_NAME}"
        file = file.replace(&format!("crate::{}", self.import_name), &self.import_name);

        // ライブラリの位置
        let library_begin = format!(
            "// ==================== {} ====================",
            self.library_name
        );

        // library以降を削除
        file = file.lines().take_while(|&l| l != library_begin).join("\n");

        // 元のファイルに書き出し
        match fs::write(&self.entry_file, file) {
            Ok(_) => {}
            Err(err) => bail!("{}: {:?}", err, self.entry_file),
        }

        Ok(())
    }
}
