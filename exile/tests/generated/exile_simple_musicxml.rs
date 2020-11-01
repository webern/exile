// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Encoding;
use xdoc::Version;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "exile_simple_musicxml.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
fn simple_musicxml() {
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
        encoding: Some(Encoding::Utf8),
    });
    let root = doc.root_mut();
    root.set_name(r#"score-partwise"#);
    root.add_attribute(r#"version"#, r#"3.0"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"part-list"#);
    let gen2n1 = gen1n1.add_new_child().unwrap();
    gen2n1.set_name(r#"score-part"#);
    gen2n1.add_attribute(r#"id"#, r#"P1"#);
    let gen3n1 = gen2n1.add_new_child().unwrap();
    gen3n1.set_name(r#"part-name"#);
    gen3n1.add_text(r#"No Name"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"part"#);
    gen1n3.add_attribute(r#"id"#, r#"P1"#);
    let gen2n1 = gen1n3.add_new_child().unwrap();
    gen2n1.set_name(r#"measure"#);
    gen2n1.add_attribute(r#"number"#, r#"1"#);
    doc
}
