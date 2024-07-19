use std::{fmt::Display, path::PathBuf};

/// パス処理用の構造体
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum ModulePath {
    /// 通常のモジュール
    /// （カテゴリいかに格納される）
    Module { category: String, file: String },
    /// マクロ
    /// （ライブラリ直下に格納される）
    Macro { file: String },
}

impl ModulePath {
    pub fn to_pathbuf(&self, mut lib_path: PathBuf) -> PathBuf {
        match self {
            ModulePath::Macro { file } => {
                lib_path.push(file);
            }
            ModulePath::Module { category, file } => {
                lib_path.push(category);
                lib_path.push(file);
            }
        }
        // ".rs"拡張子をつける
        lib_path.with_extension("rs")
    }
}

impl Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModulePath::Macro { file } => f.write_str(file),
            ModulePath::Module { file, .. } => f.write_str(file),
        }
    }
}
