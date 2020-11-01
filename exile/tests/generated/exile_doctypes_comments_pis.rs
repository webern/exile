// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Version;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "exile_doctypes_comments_pis.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .expect(format!("bad path: {}", p.display()).as_str())
}

#[test]
fn doctypes_comments_pis() {
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
        version: Some(Version::V10),
        encoding: None,
    });
    // TODO - write doctype information
    let mut root = doc.root_mut();
    root.set_name(r#"note"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"to"#);
    let gen1n5 = root.add_new_child().unwrap();
    gen1n5.set_name(r#"from"#);
    let gen1n7 = root.add_new_child().unwrap();
    gen1n7.set_name(r#"heading"#);
    let gen1n9 = root.add_new_child().unwrap();
    gen1n9.set_name(r#"body"#);
    doc
}
