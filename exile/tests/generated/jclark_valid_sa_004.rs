// generated file, do not edit

use std::path::PathBuf;
use exile::Document;

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
// dt.getEntities()...
// dt.getNotations()...
// dt.getAttributes()...
// dt.getPublicId(): null
// dt.getInternalSubset(): <!ELEMENT doc (#PCDATA)>
<!ATTLIST doc a1 CDATA #IMPLIED>

// dt.getSystemId(): null
// dt.getBaseURI(): null
// dt.getLocalName(): null
// dt.getNodeName(): doc
doc
}
