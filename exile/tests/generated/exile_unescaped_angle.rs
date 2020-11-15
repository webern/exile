// generated file, do not edit

use std::path::PathBuf;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const INPUT_FILE: &str = "exile_unescaped_angle.xml";

fn path(filename: &str) -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(filename);
    p.canonicalize()
        .unwrap_or_else(|e| panic!("bad path: {}: {}", p.display(), e))
}

#[test]
fn unescaped_angle_test() {
    let result = exile::load(path(INPUT_FILE));
    assert!(result.is_err());
    let e = result.err().unwrap();
    match e {
        exile::error::Error::Parse(parse_error) => {
            assert_eq!(95, parse_error.xml_site.position);
            assert_eq!(4, parse_error.xml_site.line);
            assert_eq!(39, parse_error.xml_site.column);
        }
        _ => panic!("expected parse error."),
    }
}
