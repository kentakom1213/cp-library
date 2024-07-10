use std::{collections::HashSet, path::PathBuf};

use expander::utils::{get_deps, get_library_path, ModuleExpander, UseVisitor};
use syn::visit::Visit;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

#[test]
fn test_get_library_path() {
    let lib_path = get_library_path();

    assert!(lib_path.exists());
}

#[test]
fn test_enum_modules() {
    let source = r#"
use cp_library_rs::{
    modint::{M998, M109},
    segment_tree::SegmentTree,
};
use cp_library_rs::monoid::{Monoid, examples::*};
use std::collections::BTreeMap;

fn main() {

}
"#;

    let syntax_tree = syn::parse_file(&source).expect("Unable to parse file");

    let mut visitor = UseVisitor { uses: vec![] };
    visitor.visit_file(&syntax_tree);

    debug!(visitor.uses);
}

#[test]
fn test_local_deps() {
    let source = r#"
use cp_library_rs::{
    modint::{M998, M109},
    segment_tree::SegmentTree,
};
use cp_library_rs::monoid::{Monoid, examples::*};
use std::collections::BTreeMap;

fn main() {

}
"#;

    assert_eq!(
        get_deps(&source, "cp_library_rs"),
        HashSet::from([
            "modint".to_string(),
            "segment_tree".to_string(),
            "monoid".to_string()
        ])
    );

    assert_eq!(
        get_deps(&source, "std"),
        HashSet::from(["collections".to_string(),])
    );

    assert_eq!(get_deps(&source, "crate"), HashSet::new());
}

#[test]
fn test_parser() {
    let p = PathBuf::from("../cp-library-rs/tests/test_segment_tree.rs");
    let mut res = ModuleExpander::new(p, get_library_path());

    res.solve_dependancies();

    debug!(res.dependancies);
    assert_eq!(
        res.dependancies,
        Some(HashSet::from([
            "segment_tree".to_string(),
            "monoid".to_string(),
            "affine1d".to_string()
        ]))
    );

    let p = PathBuf::from("../cp-library-rs/tests/test_modint_comb.rs");
    let mut res = ModuleExpander::new(p, get_library_path());

    res.solve_dependancies();

    debug!(res.dependancies);
    assert_eq!(
        res.dependancies,
        Some(HashSet::from([
            "modint".to_string(),
            "modint_comb".to_string()
        ]))
    );
}

#[test]
fn test_get_module() {
    let res = ModuleExpander::new(PathBuf::new(), get_library_path());

    eprintln!("{}", res.get_module("monoid").unwrap());
}

#[test]
fn test_expand() {
    let p = PathBuf::from("../cp-library-rs/tests/test_modint_comb.rs");
    let mut res = ModuleExpander::new(p, get_library_path());

    res.expand().ok();
}

#[test]
fn test_restore() {
    let p = PathBuf::from("../cp-library-rs/tests/test_modint_comb.rs");
    let res = ModuleExpander::new(p, get_library_path());

    res.restore().ok();
}
