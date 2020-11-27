// generated file, do not edit

use crate::test_utils::{run_output_test, run_parse_test};
use exile::{Declaration, Document, Encoding, Version};

const INPUT_FILE: &str = "exile_single_quotes.xml";
const OUTPUT_FILE: &str = "exile_single_quotes.output.xml";

#[test]
/// a simple file with single-quoted attributes
fn single_quotes_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn single_quotes_serialize() {
    run_output_test(OUTPUT_FILE, &expected());
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
