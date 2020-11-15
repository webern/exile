// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Encoding;
use xdoc::Version;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const INPUT_FILE: &str = "exile_pi.xml";

fn path(filename: &str) -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(filename);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
/// a simple file with processing instructions
fn pi_parse() {
    let path = path(INPUT_FILE);
    let actual = exile::load(&path).unwrap();
    let expected = expected();
    if actual != expected {
        let actual_str = actual.to_string();
        let expected_str = expected.to_string();
        if actual_str != expected_str {
            assert_eq!(expected_str, actual_str);
        } else {
            assert_eq!(expected, actual);
        }
    }
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
