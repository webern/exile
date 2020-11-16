// generated file, do not edit

use crate::test_utils::{run_output_test, run_parse_test};
use exile::Document;
use xdoc::Declaration;
use xdoc::Encoding;
use xdoc::Version;

const INPUT_FILE: &str = "exile_simple_musicxml.xml";
const OUTPUT_FILE: &str = "exile_simple_musicxml.output.xml";

#[test]
/// cd_catalog example from https://www.w3schools.com/xml/xml_examples.asp
fn simple_musicxml_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn simple_musicxml_serialize() {
    run_output_test(OUTPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: Some(Encoding::Utf8),
    });
    let root = doc.root_mut();
    root.set_name(r#"score-partwise"#);
    root.add_attribute(r#"version"#, r#"3.0"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"part-list"#);
    let gen2n1 = gen1n1.add_new_child().unwrap();
    gen2n1.set_name(r#"score-part"#);
    gen2n1.add_attribute(r#"id"#, r#"P1"#);
    let gen3n1 = gen2n1.add_new_child().unwrap();
    gen3n1.set_name(r#"part-name"#);
    gen3n1.add_text(r#"No Name"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"part"#);
    gen1n3.add_attribute(r#"id"#, r#"P1"#);
    let gen2n1 = gen1n3.add_new_child().unwrap();
    gen2n1.set_name(r#"measure"#);
    gen2n1.add_attribute(r#"number"#, r#"1"#);
    doc
}
