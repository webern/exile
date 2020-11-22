// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::{Declaration, Document, Version};

const INPUT_FILE: &str = "jclark_valid_sa_029.xml";

#[test]
/// A valid XML file from the W3C conformance test suite: valid-sa-029
fn valid_sa_029_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: None,
    });
    // TODO - write doctype information
    let root = doc.root_mut();
    root.set_name(r#"doc"#);
    doc
}
