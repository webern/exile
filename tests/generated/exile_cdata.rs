// generated file, do not edit

use crate::test_utils::{run_output_test, run_parse_test};
use exile::{Declaration, Document};

const INPUT_FILE: &str = "exile_cdata.xml";
const OUTPUT_FILE: &str = "exile_cdata.output.xml";

#[test]
/// a cdata example
fn cdata_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn cdata_serialize() {
    run_output_test(OUTPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: None,
        encoding: None,
    });
    let root = doc.root_mut();
    root.set_name(r#"root"#);
    root.add_text(r#"before"#);
    root.add_cdata(r#">>>>&&<<<<"#).unwrap();
    root.add_text(r#"after"#);
    doc
}
