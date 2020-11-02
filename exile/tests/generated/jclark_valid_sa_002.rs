// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "jclark_valid_sa_002.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
fn valid_sa_002() {
    let path = path();
    let loaded = exile::load(&path).unwrap();
    let expected = expected();
    if loaded != expected {
        let loaded_str = loaded.to_string();
        let expected_str = expected.to_string();
        if loaded_str != expected_str {
            assert_eq!(loaded_str, expected_str);
        } else {
            assert_eq!(loaded, expected);
        }
    }
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: None,
        encoding: None,
    });
    // TODO - write doctype information
    let root = doc.root_mut();
    root.set_name(r#"doc"#);
    doc
}
