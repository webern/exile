// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::{Declaration, Document, Version};

const INPUT_FILE: &str = "exile_doctypes_comments_pis.xml";

#[test]
/// a file with doctypes, processing instructions and comments
fn doctypes_comments_pis_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: None,
    });
    // TODO - support doctype https://github.com/webern/exile/issues/22
    doc.set_doctype(
"<!DOCTYPE note [\n<!ELEMENT note (to,from,heading,body)>\n<!ELEMENT to (#PCDATA)>\n<!ELEMENT from (#PCDATA)>\n<!ELEMENT heading (#PCDATA)>\n<!ELEMENT body (#PCDATA)>\n]>"
).unwrap();
    doc.add_prolog_comment(r#" comment before doctype "#)
        .unwrap();
    doc.add_prolog_pi(exile::Pi::new(r#"pi"#, r#"before doctype "#).unwrap());
    doc.add_prolog_comment(r#" comment after doctype "#)
        .unwrap();
    doc.add_prolog_pi(exile::Pi::new(r#"pi"#, r#"after doctype "#).unwrap());
    let root = doc.root_mut();
    root.set_name(r#"note"#);
    root.add_comment(r#" comment as element node "#).unwrap();
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"to"#);
    gen1n1.add_text(r#"Tove"#);
    let gen1n2 = root.add_new_child().unwrap();
    gen1n2.set_name(r#"from"#);
    gen1n2.add_text(r#"Jani"#);
    gen1n2.add_pi(exile::Pi::new(r#"pi"#, r#"in element "#).unwrap());
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"heading"#);
    gen1n3.add_text(r#"Reminder"#);
    let gen1n4 = root.add_new_child().unwrap();
    gen1n4.set_name(r#"body"#);
    gen1n4.add_text(r#"Don't forget me this weekend"#);
    doc.add_epilog_comment(r#" at the end "#).unwrap();
    doc.add_epilog_pi(exile::Pi::new(r#"pi"#, r#"at the end "#).unwrap());
    doc
}
