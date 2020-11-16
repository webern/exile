// generated file, do not edit

use crate::test_utils::{run_output_test, run_parse_test};
use exile::Document;
use xdoc::Declaration;
use xdoc::Version;

const INPUT_FILE: &str = "exile_escapes.xml";
const OUTPUT_FILE: &str = "exile_escapes.output.xml";

#[test]
/// escape sequences
fn escapes_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn escapes_serialize() {
    run_output_test(OUTPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: None,
    });
    let root = doc.root_mut();
    root.set_name(r#"escapes"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"a"#);
    gen1n1.add_attribute(r#"s"#, r#"<😃>"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"_b"#);
    gen1n3.add_attribute(r#"x"#, r#"&"#);
    gen1n3.add_text(r#"'"&"#);
    doc
}
