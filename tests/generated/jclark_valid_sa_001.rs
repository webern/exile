// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::{Declaration, Document};

const INPUT_FILE: &str = "jclark_valid_sa_001.xml";

#[test]
/// A valid XML file from the W3C conformance test suite: valid-sa-001
fn valid_sa_001_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: None,
        encoding: None,
    });
    // TODO - support doctype https://github.com/webern/exile/issues/22
    doc.set_doctype("<!DOCTYPE doc [\r\n<!ELEMENT doc (#PCDATA)>\r\n]>")
        .unwrap();
    let root = doc.root_mut();
    root.set_name(r#"doc"#);
    doc
}
