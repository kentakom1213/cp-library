use std::path::PathBuf;

use expander::module_path::ModulePath;

#[test]
fn test_to_pathbuf() {
    let p1 = ModulePath::Macro {
        file: "get".to_string(),
    };
    assert_eq!(p1.to_pathbuf(PathBuf::from(".")), PathBuf::from("./get.rs"));

    let p2 = ModulePath::Module {
        category: "number_theory".to_string(),
        file: "crt".to_string(),
    };
    assert_eq!(
        p2.to_pathbuf(PathBuf::from("../cp-library-rs")),
        PathBuf::from("../cp-library-rs/number_theory/crt.rs")
    );
}

#[test]
fn test_display() {
    let p1 = ModulePath::Macro {
        file: "get".to_string(),
    };
    assert_eq!(&p1.to_string(), "get");

    let p2 = ModulePath::Module {
        category: "number_theory".to_string(),
        file: "crt".to_string(),
    };
    assert_eq!(p2.to_string(), "number_theory::crt");
}
