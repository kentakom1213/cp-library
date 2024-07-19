use std::collections::BTreeSet;

use syn::{visit::Visit, UseTree};

use crate::module_path::ModulePath;

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
pub fn get_deps(source: &str, target: &str) -> BTreeSet<ModulePath> {
    // 構文木を構築
    let syntax_tree = syn::parse_file(source).expect("Unable to parse file");
    // 解析
    let mut visitor = UseVisitor { uses: vec![] };
    visitor.visit_file(&syntax_tree);

    // 内部の依存関係を調べる
    let mut local_deps = BTreeSet::new();

    for p in &visitor.uses {
        // if let [dep, file, ..] = &p[..] {
        //     if dep == target {
        //         local_deps.insert(file.clone());
        //     }
        // }
        match &p[..] {
            [crate_name, category, file, ..] if crate_name == target => {
                let dep = ModulePath::Module {
                    category: category.clone(),
                    file: file.clone(),
                };
                local_deps.insert(dep);
            }
            [crate_name, file] if crate_name == target => {
                let dep = ModulePath::Macro { file: file.clone() };
                local_deps.insert(dep);
            }
            _ => (),
        }
    }

    local_deps
}
