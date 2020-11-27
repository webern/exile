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
    // TODO - write doctype information
    doc.add_prolog_comment(r#" comment before doctype "#)
        .unwrap();
    doc.add_prolog_pi(exile::PI {
        target: r#"pi"#.into(),
        data: r#"before doctype "#.into(),
    });
    doc.add_prolog_comment(r#" comment after doctype "#)
        .unwrap();
    doc.add_prolog_pi(exile::PI {
        target: r#"pi"#.into(),
        data: r#"after doctype "#.into(),
    });
    let root = doc.root_mut();
    root.set_name(r#"note"#);
    root.add_comment(r#" comment as element node "#).unwrap();
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"to"#);
    gen1n1.add_text(r#"Tove"#);
    let gen1n2 = root.add_new_child().unwrap();
    gen1n2.set_name(r#"from"#);
    gen1n2.add_text(r#"Jani"#);
    gen1n2.add_pi(exile::PI {
        target: r#"pi"#.into(),
        data: r#"in element "#.into(),
    });
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"heading"#);
    gen1n3.add_text(r#"Reminder"#);
    let gen1n4 = root.add_new_child().unwrap();
    gen1n4.set_name(r#"body"#);
    gen1n4.add_text(r#"Don't forget me this weekend"#);
    doc.add_epilog_comment(r#" at the end "#).unwrap();
    doc.add_epilog_pi(exile::PI {
        target: r#"pi"#.into(),
        data: r#"at the end "#.into(),
    });
    doc
}
