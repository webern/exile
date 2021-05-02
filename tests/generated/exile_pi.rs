// generated file, do not edit

use crate::test_utils::{run_output_test, run_parse_test};
use exile::{Declaration, Document, Encoding, Version};

const INPUT_FILE: &str = "exile_pi.xml";
const OUTPUT_FILE: &str = "exile_pi.output.xml";

#[test]
/// a simple file with processing instructions
fn pi_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

#[test]
/// Check that the serialization of this XML document matches what we expect.
fn pi_serialize() {
    run_output_test(OUTPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V11),
        encoding: Some(Encoding::Utf8),
    });
    doc.add_prolog_pi(exile::Pi::new(r#"a"#, r#"b"#).unwrap());
    let root = doc.root_mut();
    root.set_name(r#"c"#);
    root.add_pi(exile::Pi::new(r#"d"#, r#"e"#).unwrap());
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"f"#);
    root.add_pi(exile::Pi::new(r#"g"#, r#"h"#).unwrap());
    doc.add_epilog_pi(exile::Pi::new(r#"i"#, r#"j"#).unwrap());
    doc
}
