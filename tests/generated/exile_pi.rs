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
    doc.push_prolog_misc(exile::Misc::PI(exile::PI {
        target: r#"a"#.into(),
        data: r#"b"#.into(),
    }));
    let root = doc.root_mut();
    root.set_name(r#"c"#);
    root.add_pi(exile::PI {
        target: r#"d"#.into(),
        data: r#"e"#.into(),
    });
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"f"#);
    root.add_pi(exile::PI {
        target: r#"g"#.into(),
        data: r#"h"#.into(),
    });
    doc.push_epilog_misc(exile::Misc::PI(exile::PI {
        target: r#"i"#.into(),
        data: r#"j"#.into(),
    }));
    doc
}
