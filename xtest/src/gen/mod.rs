use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::{Syntax, XmlFile};

pub fn generate_tests<P: AsRef<Path>>(output_filepath: P) {
    let xtest = crate::load_all();
    let _ = std::fs::remove_file(&output_filepath);
    let mut f = std::fs::File::create(&output_filepath).unwrap();
    write_test_file_header(&mut f);

    for xml_file in xtest.iter() {
        start_test_and_load_metadata(&mut f, xml_file);
        match xml_file.metadata.syntax {
            Syntax::Bad {
                character_position,
                line,
                column,
            } => write_bad_syntax_test(&mut f, character_position, line, column),
            Syntax::Good {} => write_good_syntax_test(&mut f, &xml_file),
        }
        finish_test(&mut f);
    }

    Command::new("cargo")
        .args(&["fmt", "--", output_filepath.as_ref().to_str().unwrap()])
        .output()
        .expect("failed to execute process");
}

fn finish_test(f: &mut File) {
    f.write_all(b"}\n\n").unwrap();
}

fn start_test_and_load_metadata(f: &mut File, xml_file: &XmlFile) {
    f.write_all(b"#[test]").unwrap();
    let test_fn = testname(xml_file);
    writeln!(f, "{}", test_fn).unwrap();
    writeln!(f, "    let info = xtest::load(\"{}\");", xml_file.name).unwrap();
}

fn testname(xml_file: &XmlFile) -> String {
    match xml_file.metadata.syntax {
        Syntax::Bad { .. } => bad_syntax_testname(xml_file),
        Syntax::Good {} => good_syntax_testname(xml_file),
    }
}

fn good_syntax_testname(xml_file: &XmlFile) -> String {
    format!(
        "fn good_syntax_{}_test() {{\n",
        xml_file.name.replace("-", "_")
    )
}

fn bad_syntax_testname(xml_file: &XmlFile) -> String {
    format!(
        "fn bad_syntax_{}_test() {{\n",
        xml_file.name.replace("-", "_")
    )
}

fn write_test_file_header(f: &mut File) {
    f.write_all(b"//! `parse_tests.rs` is generated by build.rs\n")
        .unwrap();
}

fn write_bad_syntax_test(f: &mut File, character_position: u64, line: u64, column: u64) {
    writeln!(f, "let xml_str = info.read_xml_file();").unwrap();
    writeln!(f, "let parse_result = exile::parse(xml_str.as_str());").unwrap();
    writeln!(f, "assert!(parse_result.is_err());").unwrap();
    writeln!(f, "let err = parse_result.err().unwrap();").unwrap();
    writeln!(f, "match err {{").unwrap();
    writeln!(f, "exile::error::Error::Parse(parse_error) => {{").unwrap();
    writeln!(
        f,
        "assert_eq!({}, parse_error.xml_site.position);",
        character_position
    )
    .unwrap();
    writeln!(f, "assert_eq!({}, parse_error.xml_site.line);", line).unwrap();
    writeln!(f, "assert_eq!({}, parse_error.xml_site.column);", column).unwrap();
    writeln!(f, "}}").unwrap();
    writeln!(f, "_ => panic!(\"Error was expected to be of type exile::error::Error::Parse, but was not.\")").unwrap();
    writeln!(f, "}}").unwrap();
}

fn write_good_syntax_test(f: &mut File, xml_file: &XmlFile) {
    writeln!(f, "let xml_str = info.read_xml_file();").unwrap();
    writeln!(f, "let parse_result = exile::parse(xml_str.as_str());").unwrap();
    writeln!(f, "if let Err(e) = parse_result {{").unwrap();
    writeln!(
        f,
        "panic!(\"expected parse_result to be Ok, got Err: {{}}\", e);"
    )
    .unwrap();
    writeln!(f, "}}").unwrap();
    if xml_file.metadata.expected.is_some() {
        writeln!(f, "let actual = parse_result.as_ref().unwrap();").unwrap();
        writeln!(
            f,
            "let expected = info.metadata.expected.as_ref().unwrap();"
        )
        .unwrap();
        writeln!(f, "let equal = expected == actual;").unwrap();
        writeln!(f, "if !equal {{").unwrap();
        // We prefer to assert that the strings are not equal for the visual aid when debugging.
        writeln!(f, "let expected_str = expected.to_string();").unwrap();
        writeln!(f, "let actual_str = actual.to_string();").unwrap();
        writeln!(f, "if expected_str != actual_str {{").unwrap();
        writeln!(f, "assert_eq!(expected_str, actual_str);").unwrap();
        writeln!(f, "}} else {{").unwrap();
        writeln!(f, "assert!(equal);").unwrap();
        writeln!(f, "}}").unwrap();
        writeln!(f, "}}").unwrap();
        // now serialize and check that it matches the expected.xml file (if there is one)
        if xml_file.expected_write.is_some() {
            writeln!(
                f,
                "let expected_serialization = info.read_expected_write().unwrap();"
            )
            .unwrap();
            writeln!(f, "assert_eq!(expected_serialization, actual.to_string());").unwrap();
        }
    }
}
