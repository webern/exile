// generated file, do not edit
use std::io::Cursor;
use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "jclark_valid_sa_005.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .expect(format!("bad path: {}", p.display()).as_str())
}

#[test]
fn valid_sa_005() {
    let path = path();
    let original = exile::load(&path).unwrap();
    let mut buff = Cursor::new(Vec::new());
    original.write(&mut buff).unwrap();
    let data = buff.into_inner();
    let data = std::str::from_utf8(data.as_slice()).unwrap();
    assert_eq!(data, r#""#);
}
