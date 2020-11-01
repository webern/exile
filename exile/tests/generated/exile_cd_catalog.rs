// generated file, do not edit

use exile::Document;
use std::path::PathBuf;
use xdoc::Declaration;
use xdoc::Encoding;
use xdoc::Version;

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const INPUT_DATA: &str = "input_data";
const FILENAME: &str = "exile_cd_catalog.xml";

fn path() -> PathBuf {
    let p = PathBuf::from(MANIFEST_DIR)
        .join("tests")
        .join(INPUT_DATA)
        .join(FILENAME);
    p.canonicalize()
        .expect(format!("bad path: {}", p.display()).as_str())
}

#[test]
fn cd_catalog() {
    let path = path();
    let loaded = exile::load(&path).unwrap();
    let expected = expected();
    if loaded != expected {
        let loaded_str = loaded.to_string();
        let expected_str = expected.to_string();
        if loaded_str != expected_str {
            assert_eq!(loaded_str, expected_str);
        } else {
            assert_eq!(loaded, expected);
        }
    }
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: Some(Encoding::Utf8),
    });
    let mut root = doc.root_mut();
    root.set_name(r#"CATALOG"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"CD"#);
    let gen2n1 = gen1n1.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n1.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n1.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n1.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n1.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n1.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"CD"#);
    let gen2n1 = gen1n3.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n3.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n3.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n3.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n3.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n3.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n5 = root.add_new_child().unwrap();
    gen1n5.set_name(r#"CD"#);
    let gen2n1 = gen1n5.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n5.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n5.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n5.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n5.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n5.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n7 = root.add_new_child().unwrap();
    gen1n7.set_name(r#"CD"#);
    let gen2n1 = gen1n7.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n7.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n7.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n7.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n7.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n7.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n9 = root.add_new_child().unwrap();
    gen1n9.set_name(r#"CD"#);
    let gen2n1 = gen1n9.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n9.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n9.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n9.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n9.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n9.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n11 = root.add_new_child().unwrap();
    gen1n11.set_name(r#"CD"#);
    let gen2n1 = gen1n11.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n11.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n11.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n11.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n11.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n11.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n13 = root.add_new_child().unwrap();
    gen1n13.set_name(r#"CD"#);
    let gen2n1 = gen1n13.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n13.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n13.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n13.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n13.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n13.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n15 = root.add_new_child().unwrap();
    gen1n15.set_name(r#"CD"#);
    let gen2n1 = gen1n15.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n15.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n15.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n15.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n15.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n15.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n17 = root.add_new_child().unwrap();
    gen1n17.set_name(r#"CD"#);
    let gen2n1 = gen1n17.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n17.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n17.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n17.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n17.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n17.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n19 = root.add_new_child().unwrap();
    gen1n19.set_name(r#"CD"#);
    let gen2n1 = gen1n19.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n19.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n19.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n19.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n19.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n19.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n21 = root.add_new_child().unwrap();
    gen1n21.set_name(r#"CD"#);
    let gen2n1 = gen1n21.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n21.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n21.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n21.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n21.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n21.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n23 = root.add_new_child().unwrap();
    gen1n23.set_name(r#"CD"#);
    let gen2n1 = gen1n23.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n23.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n23.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n23.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n23.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n23.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n25 = root.add_new_child().unwrap();
    gen1n25.set_name(r#"CD"#);
    let gen2n1 = gen1n25.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n25.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n25.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n25.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n25.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n25.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n27 = root.add_new_child().unwrap();
    gen1n27.set_name(r#"CD"#);
    let gen2n1 = gen1n27.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n27.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n27.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n27.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n27.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n27.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n29 = root.add_new_child().unwrap();
    gen1n29.set_name(r#"CD"#);
    let gen2n1 = gen1n29.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n29.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n29.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n29.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n29.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n29.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n31 = root.add_new_child().unwrap();
    gen1n31.set_name(r#"CD"#);
    let gen2n1 = gen1n31.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n31.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n31.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n31.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n31.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n31.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n33 = root.add_new_child().unwrap();
    gen1n33.set_name(r#"CD"#);
    let gen2n1 = gen1n33.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n33.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n33.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n33.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n33.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n33.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n35 = root.add_new_child().unwrap();
    gen1n35.set_name(r#"CD"#);
    let gen2n1 = gen1n35.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n35.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n35.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n35.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n35.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n35.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n37 = root.add_new_child().unwrap();
    gen1n37.set_name(r#"CD"#);
    let gen2n1 = gen1n37.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n37.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n37.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n37.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n37.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n37.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n39 = root.add_new_child().unwrap();
    gen1n39.set_name(r#"CD"#);
    let gen2n1 = gen1n39.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n39.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n39.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n39.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n39.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n39.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n41 = root.add_new_child().unwrap();
    gen1n41.set_name(r#"CD"#);
    let gen2n1 = gen1n41.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n41.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n41.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n41.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n41.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n41.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n43 = root.add_new_child().unwrap();
    gen1n43.set_name(r#"CD"#);
    let gen2n1 = gen1n43.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n43.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n43.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n43.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n43.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n43.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n45 = root.add_new_child().unwrap();
    gen1n45.set_name(r#"CD"#);
    let gen2n1 = gen1n45.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n45.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n45.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n45.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n45.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n45.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n47 = root.add_new_child().unwrap();
    gen1n47.set_name(r#"CD"#);
    let gen2n1 = gen1n47.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n47.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n47.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n47.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n47.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n47.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n49 = root.add_new_child().unwrap();
    gen1n49.set_name(r#"CD"#);
    let gen2n1 = gen1n49.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n49.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n49.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n49.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n49.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n49.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    let gen1n51 = root.add_new_child().unwrap();
    gen1n51.set_name(r#"CD"#);
    let gen2n1 = gen1n51.add_new_child().unwrap();
    gen2n1.set_name(r#"TITLE"#);
    let gen2n3 = gen1n51.add_new_child().unwrap();
    gen2n3.set_name(r#"ARTIST"#);
    let gen2n5 = gen1n51.add_new_child().unwrap();
    gen2n5.set_name(r#"COUNTRY"#);
    let gen2n7 = gen1n51.add_new_child().unwrap();
    gen2n7.set_name(r#"COMPANY"#);
    let gen2n9 = gen1n51.add_new_child().unwrap();
    gen2n9.set_name(r#"PRICE"#);
    let gen2n11 = gen1n51.add_new_child().unwrap();
    gen2n11.set_name(r#"YEAR"#);
    doc
}
