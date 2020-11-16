// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::{Declaration, Document, Encoding, Version};

const INPUT_FILE: &str = "exile_single_quotes.xml";

#[test]
/// a simple file with single-quoted attributes
fn single_quotes_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V11),
        encoding: Some(Encoding::Utf8),
    });
    let root = doc.root_mut();
    root.set_name(r#"foo"#);
    root.add_attribute(r#"attr1"#, r#"bones"#);
    root.add_attribute(r#"attr2"#, r#"bish"#);
    doc
}
