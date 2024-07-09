use expander::utils::{get_library_path, UseVisitor};
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
    let content = r#"
use cp_library_rs::{
    modint::{M998, M109},
    segment_tree::SegmentTree,
};
use cp_library_rs::monoid::{Monoid, examples::*};
use std::collections::BTreeMap;

fn main() {

}
"#;
    let syntax_tree = syn::parse_file(&content).expect("Unable to parse file");

    let mut visitor = UseVisitor { uses: vec![] };
    visitor.visit_file(&syntax_tree);

    debug!(visitor.uses);
}
