// generated file, do not edit
use crate::test_utils::run_parse_test;
use exile::Document;
use xdoc::Declaration;
use xdoc::Encoding;
use xdoc::Version;

const INPUT_FILE: &str = "exile_pi.xml";

#[test]
/// a simple file with processing instructions
fn pi_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V11),
        encoding: Some(Encoding::Utf8),
    });
    doc.push_prolog_misc(xdoc::Misc::PI(xdoc::PI {
        target: r#"a"#.into(),
        instructions: vec![r#"b"#.to_owned()],
    }));
    let root = doc.root_mut();
    root.set_name(r#"c"#);
    root.add_pi(xdoc::PI {
        target: r#"d"#.into(),
        instructions: vec![r#"e"#.to_owned()],
    });
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"f"#);
    root.add_pi(xdoc::PI {
        target: r#"g"#.into(),
        instructions: vec![r#"h"#.to_owned()],
    });
    doc.push_epilog_misc(xdoc::Misc::PI(xdoc::PI {
        target: r#"i"#.into(),
        instructions: vec![r#"j"#.to_owned()],
    }));
    doc
}
