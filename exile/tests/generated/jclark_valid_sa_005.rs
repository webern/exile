// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::Document;
use xdoc::Declaration;

const INPUT_FILE: &str = "jclark_valid_sa_005.xml";

#[test]
/// A valid XML file from the W3C conformance test suite: valid-sa-005
fn valid_sa_005_parse() {
    run_parse_test(INPUT_FILE, &expected());
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
    root.add_attribute(r#"a1"#, r#"v1"#);
    doc
}
