// generated file, do not edit

use exile::Document;
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "jclark_valid_sa_004.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .expect(format!("bad path: {}", p.display()).as_str())
}

#[test]
fn valid_sa_004() {
    let path = path();
    let _doc = exile::load(&path).unwrap();
}

fn expected() -> Document {
    let mut doc = Document::new();
    // TODO - write doctype information
    /*
    <!ELEMENT doc (#PCDATA)>
    <!ATTLIST doc a1 CDATA #IMPLIED>
    */
    doc
}
