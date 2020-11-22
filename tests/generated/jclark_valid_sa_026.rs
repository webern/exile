// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::{Declaration, Document};

const INPUT_FILE: &str = "jclark_valid_sa_026.xml";

#[test]
/// A valid XML file from the W3C conformance test suite: valid-sa-026
fn valid_sa_026_parse() {
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
    let gen1n0 = root.add_new_child().unwrap();
    gen1n0.set_name(r#"foo"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"foo"#);
    doc
}
