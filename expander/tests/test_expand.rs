use std::{collections::BTreeSet, env, path::PathBuf};

use expander::{
    expander::ModuleExpander,
    module_path::ModulePath,
    parser::{get_deps, UseVisitor},
};
use syn::visit::Visit;

/// ライブラリのパスを取得する
fn get_library_path() -> PathBuf {
    let mut buf = PathBuf::from(env::var("KYOPURO_LIBRARY_DIR").unwrap());
    buf.push("cp-library-rs");
    buf
}

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

const SOURCE: &str = r#"
use cp_library_rs::{
    number_theory::modint::{M998, M109},
    data_structure::segment_tree::SegmentTree,
};
use cp_library_rs::algebraic_structure::monoid::{Monoid, examples::*};
use std::collections::BTreeMap;

fn main() {

}
"#;

#[test]
fn test_get_library_path() {
    let lib_path = get_library_path();

    assert!(lib_path.exists());
}

#[test]
fn test_enum_modules() {
    let syntax_tree = syn::parse_file(&SOURCE).expect("Unable to parse file");

    let mut visitor = UseVisitor { uses: vec![] };
    visitor.visit_file(&syntax_tree);

    debug!(visitor.uses);
}

#[test]
fn test_local_deps() {
    assert_eq!(
        get_deps(&SOURCE, "cp_library_rs"),
        BTreeSet::from([
            ModulePath::Module {
                category: "number_theory".to_string(),
                file: "modint".to_string()
            },
            ModulePath::Module {
                category: "data_structure".to_string(),
                file: "segment_tree".to_string()
            },
            ModulePath::Module {
                category: "algebraic_structure".to_string(),
                file: "monoid".to_string()
            },
        ])
    );

    assert_eq!(get_deps(&SOURCE, "crate"), BTreeSet::new());
}

#[test]
fn test_parser() {
    let p = PathBuf::from("../cp-library-rs/tests/test_segment_tree.rs");
    let mut res = ModuleExpander::new(p, get_library_path()).unwrap();

    res.solve_dependancies().ok();

    debug!(res.dependancies);
    assert_eq!(
        res.dependancies,
        Some(BTreeSet::from([
            ModulePath::Module {
                category: "algebraic_structure".to_string(),
                file: "monoid".to_string()
            },
            ModulePath::Module {
                category: "data_structure".to_string(),
                file: "segment_tree".to_string()
            },
            ModulePath::Module {
                category: "utils".to_string(),
                file: "num_traits".to_string()
            },
        ]))
    );

    let p = PathBuf::from("../cp-library-rs/tests/test_modint_comb.rs");
    let mut res = ModuleExpander::new(p, get_library_path()).unwrap();

    res.solve_dependancies().ok();

    debug!(res.dependancies);
    assert_eq!(
        res.dependancies,
        Some(BTreeSet::from([
            ModulePath::Module {
                category: "number_theory".to_string(),
                file: "modint".to_string(),
            },
            ModulePath::Module {
                category: "number_theory".to_string(),
                file: "modint_comb".to_string(),
            },
            ModulePath::Module {
                category: "utils".to_string(),
                file: "num_traits".to_string(),
            }
        ]))
    );
}

#[test]
fn test_get_module() {
    let res = ModuleExpander::new(PathBuf::new(), get_library_path()).unwrap();

    eprintln!(
        "{}",
        res.get_module(
            &ModulePath::Module {
                category: "algebraic_structure".to_string(),
                file: "monoid".to_string()
            },
            1
        )
        .unwrap()
    );
}

#[test]
fn test_expand_restore() {
    let p = PathBuf::from("../cp-library-rs/tests/test_modint_comb.rs");
    let mut res = ModuleExpander::new(p, get_library_path()).unwrap();

    // 展開
    res.expand().ok();

    // 復元
    res.restore().ok();
}
