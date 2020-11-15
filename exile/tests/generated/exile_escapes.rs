// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Version;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const INPUT_FILE: &str = "exile_escapes.xml";
const OUTPUT_FILE: &str = "exile_escapes.output.xml";

fn path(filename: &str) -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(filename);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
/// escape sequences
fn escapes_parse() {
    let path = path(INPUT_FILE);
    let actual = exile::load(&path).unwrap();
    let expected = expected();
    if actual != expected {
        let actual_str = actual.to_string();
        let expected_str = expected.to_string();
        if actual_str != expected_str {
            assert_eq!(expected_str, actual_str);
        } else {
            assert_eq!(expected, actual);
        }
    }
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn escapes_serialize() {
    let doc = expected();
    let actual = doc.to_string();
    let expected = std::fs::read_to_string(path(OUTPUT_FILE)).unwrap();
    assert_eq!(expected, actual);
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
