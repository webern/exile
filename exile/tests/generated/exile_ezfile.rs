// generated file, do not edit

use crate::test_utils::path;
use exile::Document;
use xdoc::Declaration;
use xdoc::Encoding;
use xdoc::Version;

const INPUT_FILE: &str = "exile_ezfile.xml";
const OUTPUT_FILE: &str = "exile_ezfile.output.xml";

#[test]
/// a simple, small, well-formed xml file
fn ezfile_parse() {
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
fn ezfile_serialize() {
    let doc = expected();
    let actual = doc.to_string();
    let expected = std::fs::read_to_string(path(OUTPUT_FILE)).unwrap();
    assert_eq!(expected, actual);
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: Some(Encoding::Utf8),
    });
    let root = doc.root_mut();
    root.set_name(r#"cats"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"cat"#);
    gen1n1.add_attribute(r#"name"#, r#"bones"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"cat"#);
    gen1n3.add_attribute(r#"name"#, r#"bishop"#);
    gen1n3.add_text(r#"punks"#);
    doc
}
