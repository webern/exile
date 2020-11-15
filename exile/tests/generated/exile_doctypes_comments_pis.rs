// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Version;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const INPUT_FILENAME: &str = "exile_doctypes_comments_pis.xml";

fn path(filename: &str) -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(filename);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
fn doctypes_comments_pis_parse() {
    let path = path(INPUT_FILENAME);
    let loaded = exile::load(&path).unwrap();
    let expected = expected();
    if loaded != expected {
        let loaded_str = loaded.to_string();
        let expected_str = expected.to_string();
        if loaded_str != expected_str {
            assert_eq!(loaded_str, expected_str);
        } else {
            assert_eq!(loaded, expected);
        }
    }
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: None,
    });
    // TODO - write doctype information
    doc.push_prolog_misc(xdoc::Misc::PI(xdoc::PI {
        target: r#"pi"#.into(),
        instructions: vec![r#"before"#.to_owned(), r#"doctype"#.to_owned()],
    }));
    doc.push_prolog_misc(xdoc::Misc::PI(xdoc::PI {
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
    gen1n5.add_pi(xdoc::PI {
        target: r#"pi"#.into(),
        instructions: vec![r#"in"#.to_owned(), r#"element"#.to_owned()],
    });
    let gen1n7 = root.add_new_child().unwrap();
    gen1n7.set_name(r#"heading"#);
    gen1n7.add_text(r#"Reminder"#);
    let gen1n9 = root.add_new_child().unwrap();
    gen1n9.set_name(r#"body"#);
    gen1n9.add_text(r#"Don't forget me this weekend"#);
    doc.push_epilog_misc(xdoc::Misc::PI(xdoc::PI {
        target: r#"pi"#.into(),
        instructions: vec![r#"at"#.to_owned(), r#"the"#.to_owned(), r#"end"#.to_owned()],
    }));
    doc
}
