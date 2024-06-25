macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

use syn::{visit::Visit, UseTree};

struct UseVisitor {
    uses: Vec<Vec<String>>,
}

impl<'ast> Visit<'ast> for UseVisitor {
    fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
        let use_tree = &item_use.tree;
        let mut paths = Vec::new();
        use_tree_to_vec(use_tree, &mut paths, Vec::new());
        self.uses.extend(paths);
        // Continue visiting the rest of the file
        syn::visit::visit_item_use(self, item_use);
    }
}

fn use_tree_to_vec(tree: &UseTree, paths: &mut Vec<Vec<String>>, mut current_path: Vec<String>) {
    match tree {
        UseTree::Path(path) => {
            current_path.push(path.ident.to_string());
            use_tree_to_vec(&*path.tree, paths, current_path);
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
                use_tree_to_vec(item, paths, current_path.clone());
            }
        }
    }
}

#[test]
fn test() {
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
