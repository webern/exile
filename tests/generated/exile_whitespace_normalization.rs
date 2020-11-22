// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::{Declaration, Document, Encoding, Version};

const INPUT_FILE: &str = "exile_whitespace_normalization.xml";

#[test]
/// figure out how we handle whitespace in text elements
fn whitespace_normalization_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: Some(Encoding::Utf8),
    });
    let root = doc.root_mut();
    root.set_name(r#"root"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"text"#);
    gen1n1.add_text(r#"Hello World"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"text"#);
    gen1n3.add_text(r#"Hello World"#);
    let gen1n5 = root.add_new_child().unwrap();
    gen1n5.set_name(r#"text"#);
    gen1n5.add_text("\u{00a0}\u{00a0}Hello\u{00a0}\u{00a0}World\u{00a0}\u{00a0}");
    let gen1n7 = root.add_new_child().unwrap();
    gen1n7.set_name(r#"text"#);
    gen1n7.add_text(r#"Hello World"#);
    let gen1n9 = root.add_new_child().unwrap();
    gen1n9.set_name(r#"text"#);
    gen1n9.add_text(r#"Hello World"#);
    let gen1n11 = root.add_new_child().unwrap();
    gen1n11.set_name(r#"text"#);
    gen1n11.add_text("\u{00a0}");
    doc
}
