// generated file, do not edit

use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const INPUT_FILE: &str = "jclark_not_wf_sa_004.xml";

fn path(filename: &str) -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(filename);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
/// A not-well-formed test file from the W3C conformance test suite: not-wf-sa-004
fn not_wf_sa_004_test() {
    let result = exile::load(path(INPUT_FILE));
    assert!(result.is_err());
    let e = result.err().unwrap();
    match e {
        exile::error::Error::Parse(_) => {}
        _ => panic!("expected parse error."),
    }
}
