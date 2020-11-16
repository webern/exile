#[allow(unused_imports)]
use exile::error::XMLSite;
#[allow(unused_imports)]
use exile::Document;
#[allow(unused_imports)]
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";

/// Given a filename, gives the path to it in the `inpud_data` directory.
#[allow(dead_code)]
pub fn path(filename: &str) -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(filename);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

/// Loads `filename` and compares it to `expected` asserting equality.
#[allow(dead_code)]
pub fn run_parse_test(filename: &str, expected: &Document) {
    let path = path(filename);
    let actual = exile::load(&path).unwrap();
    if actual != *expected {
        let actual_str = actual.to_string();
        let expected_str = expected.to_string();
        if actual_str != expected_str {
            assert_eq!(expected_str, actual_str);
        } else {
            assert_eq!(*expected, actual);
        }
    }
}

/// Loads `filename` which is expected to have bad XML syntax. If `throw_site` is given, asserts
/// that `throw_site` is returned in the parse error.
#[allow(dead_code)]
pub fn run_not_well_formed_test(filename: &str, throw_site: Option<XMLSite>) {
    let path = path(filename);
    let result = exile::load(path);
    assert!(result.is_err());
    let e = result.err().unwrap();
    match e {
        exile::error::Error::Parse(parse_error) => {
            if let Some(actual_site) = throw_site {
                assert_eq!(actual_site, parse_error.xml_site);
            }
        }
        _ => panic!("expected parse error."),
    }
}

/// Check that the serialization of this XML document matches what we expect. `output_filename` is
/// an XML file in the `input_data` directory that represents the expected outcome when serializing
/// `doc`.
#[allow(dead_code)]
pub fn run_output_test(output_filename: &str, doc: &exile::Document) {
    let actual = doc.to_string();
    let output_filepath = path(output_filename);
    let expected = std::fs::read_to_string(output_filepath).unwrap();
    assert_eq!(expected, actual);
}
