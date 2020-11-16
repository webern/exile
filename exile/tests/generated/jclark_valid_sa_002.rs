// generated file, do not edit

use crate::test_utils::path;
use exile::Document;
use xdoc::Declaration;

const INPUT_FILE: &str = "jclark_valid_sa_002.xml";

#[test]
/// A valid XML file from the W3C conformance test suite: valid-sa-002
fn valid_sa_002_parse() {
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
