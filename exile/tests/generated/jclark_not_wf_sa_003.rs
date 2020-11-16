// generated file, do not edit

use crate::test_utils::path;

const INPUT_FILE: &str = "jclark_not_wf_sa_003.xml";

#[test]
/// A not-well-formed test file from the W3C conformance test suite: not-wf-sa-003
fn not_wf_sa_003_test() {
    let result = exile::load(path(INPUT_FILE));
    assert!(result.is_err());
    let e = result.err().unwrap();
    match e {
        exile::error::Error::Parse(_) => {}
        _ => panic!("expected parse error."),
    }
}
