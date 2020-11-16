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
    doc.push_prolog_misc(exile::Misc::PI(exile::PI {
        target: r#"pi"#.into(),
        instructions: vec![r#"before"#.to_owned(), r#"doctype"#.to_owned()],
    }));
    doc.push_prolog_misc(exile::Misc::PI(exile::PI {
        target: r#"pi"#.into(),
        instructions: vec![r#"after"#.to_owned(), r#"doctype"#.to_owned()],
    }));
    let root = doc.root_mut();
    root.set_name(r#"note"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"to"#);
    gen1n3.add_text(r#"Tove"#);
    let gen1n5 = root.add_new_child().unwrap();
    gen1n5.set_name(r#"from"#);
    gen1n5.add_text(r#"Jani"#);
    gen1n5.add_pi(exile::PI {
        target: r#"pi"#.into(),
        instructions: vec![r#"in"#.to_owned(), r#"element"#.to_owned()],
    });
    let gen1n7 = root.add_new_child().unwrap();
    gen1n7.set_name(r#"heading"#);
    gen1n7.add_text(r#"Reminder"#);
    let gen1n9 = root.add_new_child().unwrap();
    gen1n9.set_name(r#"body"#);
    gen1n9.add_text(r#"Don't forget me this weekend"#);
    doc.push_epilog_misc(exile::Misc::PI(exile::PI {
        target: r#"pi"#.into(),
        instructions: vec![r#"at"#.to_owned(), r#"the"#.to_owned(), r#"end"#.to_owned()],
    }));
    doc
}
