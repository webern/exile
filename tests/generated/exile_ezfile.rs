// generated file, do not edit

use crate::test_utils::{run_output_test, run_parse_test};
use exile::{Declaration, Document, Encoding, Version};

const INPUT_FILE: &str = "exile_ezfile.xml";
const OUTPUT_FILE: &str = "exile_ezfile.output.xml";

#[test]
/// a simple, small, well-formed xml file
fn ezfile_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn ezfile_serialize() {
    run_output_test(OUTPUT_FILE, &expected());
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
