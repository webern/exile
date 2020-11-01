// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Version;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "exile_escapes.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
fn escapes() {
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
    let root = doc.root_mut();
    root.set_name(r#"escapes"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"a"#);
    gen1n1.add_attribute(r#"s"#, r#"<😃>"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"_b"#);
    gen1n3.add_attribute(r#"x"#, r#"&"#);
    gen1n3.add_text(r#"'"&"#);
    doc
}
