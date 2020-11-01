// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Encoding;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "exile_single_quotes.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .expect(format!("bad path: {}", p.display()).as_str())
}

#[test]
fn single_quotes() {
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
        encoding: Some(Encoding::Utf8),
    });
    let mut root = doc.root_mut();
    root.set_name(r#"foo"#);
    root.add_attribute(r#"attr1"#, r#"bones"#);
    root.add_attribute(r#"attr2"#, r#"bish"#);
    doc
}
